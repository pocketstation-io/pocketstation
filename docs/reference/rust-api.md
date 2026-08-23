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
| sym-d45896d5cc6abbfcd3e2 | `INITIAL` | assoc_const | Provides the initial value for `PermissionEpoch`. | `src/capture/authorization.rs:270` |
| sym-c22033b2bb1da21bc723 | `INITIAL` | assoc_const | Provides the initial value for `SourceGeneration`. | `src/capture/events.rs:15` |
| sym-74f10710a2809a6dd708 | `Error` | assoc_type | Specifies the error type returned by `SidecarMessageKind` operations. | `src/runtime/lifecycle/sidecar_protocol.rs:23` |
| sym-f711aa823577922f7731 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MAJOR` | constant | Defines the major version of extension ABI. | `src/abi/extension.rs:7` |
| sym-062a6480e9f37de71402 | `pocketstation::abi::extension::PKS_EXTENSION_ABI_MINOR` | constant | Defines the minor version of extension ABI. | `src/abi/extension.rs:8` |
| sym-27e7db0a9ab3edb1a125 | `pocketstation::capture::capture_owner::CAPTURE_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Monotonic timestamp domain used by native capture backends. | `src/capture/capture_owner.rs:20` |
| sym-3ddc5124a70a7fd4c6e0 | `pocketstation::capture::events::MAX_SOURCE_RUNTIME_EVENT_OWNED_BYTES` | constant | Maximum heap storage retained by one queued capture-runtime event. | `src/capture/events.rs:72` |
| sym-e337789afb958cf9dbf0 | `pocketstation::codec::constants::OPUS_FRAME_SAMPLES` | constant | 20 ms frame = 960 samples at 48 kHz (AUDIO-012). | `src/codec/constants.rs:5` |
| sym-b9f643402b922afb3fbb | `pocketstation::codec::constants::OPUS_MAX_PACKET_BYTES` | constant | Maximum number of bytes the Opus encoder can emit per 20 ms frame. libopus guarantees this upper bound. | `src/codec/constants.rs:13` |
| sym-ccf921e9b7f8fda11b0c | `pocketstation::codec::constants::OPUS_SAMPLE_RATE_HZ` | constant | 48 000 Hz, mono, VOIP application profile (AUDIO-012 default). | `src/codec/constants.rs:2` |
| sym-3a17d11b79ead1143bbc | `pocketstation::codec::constants::VOICE_AGENT_FRAME_SAMPLES` | constant | 10 ms frame = 480 samples at 48 kHz (voice-agent low-latency mode, RFC 6716 §3.1). Ten milliseconds of mono PCM at 48 kHz. | `src/codec/constants.rs:9` |
| sym-b0cbabd45361f449853a | `pocketstation::conformance::EXTENSION_ENDPOINT_ID` | constant | Defines the public extension endpoint identifier value. | `src/conformance.rs:559` |
| sym-1ba908e28c3afd8e497b | `pocketstation::conformance::EXTENSION_ENDPOINT_INPUT_PORT` | constant | Defines the public extension endpoint input port value. | `src/conformance.rs:564` |
| sym-d6798661143e74612f0b | `pocketstation::conformance::EXTENSION_ENDPOINT_NODE_ID` | constant | Defines the public extension endpoint node identifier value. | `src/conformance.rs:560` |
| sym-1ccb78cd002be7ff6123 | `pocketstation::conformance::EXTENSION_INPUT_PAYLOAD` | constant | Defines the public extension input payload value. | `src/conformance.rs:565` |
| sym-646cf2b7ebca65dc752c | `pocketstation::conformance::EXTENSION_OPERATOR_ID` | constant | Defines the public extension operator identifier value. | `src/conformance.rs:557` |
| sym-c2e22ca17c3c863edabf | `pocketstation::conformance::EXTENSION_OPERATOR_INPUT_PORT` | constant | Defines the public extension operator input port value. | `src/conformance.rs:562` |
| sym-49ffa91fc9e00a49e55a | `pocketstation::conformance::EXTENSION_OPERATOR_NODE_ID` | constant | Defines the public extension operator node identifier value. | `src/conformance.rs:558` |
| sym-10c76587f1101d8d3e31 | `pocketstation::conformance::EXTENSION_OPERATOR_OUTPUT_PORT` | constant | Defines the public extension operator output port value. | `src/conformance.rs:563` |
| sym-dfecdbff090be3e1cd77 | `pocketstation::conformance::EXTENSION_OUTPUT_PAYLOAD` | constant | Defines the public extension output payload value. | `src/conformance.rs:566` |
| sym-4bb49cdac9a896e1bcad | `pocketstation::conformance::EXTENSION_ROLE_ID` | constant | Defines the public extension role identifier value. | `src/conformance.rs:555` |
| sym-cdb79e20b2f13c62be5e | `pocketstation::conformance::EXTENSION_SCHEMA_ID` | constant | Defines the public extension schema identifier value. | `src/conformance.rs:554` |
| sym-05f7da8f3e8fbde09c92 | `pocketstation::conformance::EXTENSION_SIGNAL_ID` | constant | Defines the public extension signal identifier value. | `src/conformance.rs:553` |
| sym-b162c69892b2440e428b | `pocketstation::conformance::EXTENSION_SOURCE_PORT` | constant | Defines the public extension source port value. | `src/conformance.rs:561` |
| sym-8be82ac72d1a5deecb00 | `pocketstation::conformance::EXTENSION_SOURCE_TYPE_ID` | constant | Defines the public extension source type identifier value. | `src/conformance.rs:556` |
| sym-9a1d83f3ed613eadcaf4 | `pocketstation::conformance::FRAMES_PER_SOURCE` | constant | Frames emitted per source by the finite deterministic fixture. | `src/conformance.rs:39` |
| sym-187a1836ba3ee59985d2 | `pocketstation::conformance::OBSERVED_CONNECTOR_OPERATOR_ID` | constant | Defines the public observed connector operator identifier value. | `src/conformance.rs:40` |
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
| sym-331441db632dcf64c0b7 | `pocketstation::frame::audio::POOL_SLOT_SAMPLES` | constant | Defines the public pool slot samples value. | `src/frame/audio.rs:10` |
| sym-14e035ed0b0d39c2bb72 | `pocketstation::frame::audio::SAMPLE_RATE_HZ` | constant | Defines the public sample rate hertz value. | `src/frame/audio.rs:6` |
| sym-d76758b7388e90ce34c5 | `pocketstation::frame::pool::POOL_MAX_SLOTS` | constant | Defines the public pool max slots value. | `src/frame/pool.rs:11` |
| sym-39765f9cd0c0fb34c75a | `pocketstation::graph::operator::OPERATOR_ID_SYNTAX_VERSION` | constant | Version of the serialized operator-identifier syntax. | `src/graph/operator.rs:6` |
| sym-c3da8a156b60fde775c7 | `pocketstation::graph::plan::EDGE_RECEIVER_MAX_IN_FLIGHT_FRAMES` | constant | A sequential edge receiver may retain the frame it just popped while it processes that frame. Copy-pool sizing must cover that owned frame in addition to every frame that can still be queued. | `src/graph/plan.rs:16` |
| sym-06754e50ae4dbb11aece | `pocketstation::graph::plan::EDGE_RING_CAPACITY_FRAMES` | constant | Defines the public edge ring capacity frames value. | `src/graph/plan.rs:12` |
| sym-bfcff1df5a0c4e946955 | `pocketstation::graph::plan::FRAME_BYTES_MONO_48K` | constant | Defines the public frame bytes mono 48 k value. | `src/graph/plan.rs:11` |
| sym-45f8a54a67e90ca0e98a | `pocketstation::graph::plan::MAX_EDGE_RING_CAPACITY_FRAMES` | constant | Sets the maximum supported edge ring capacity frames. | `src/graph/plan.rs:17` |
| sym-8965f2238e75bd29390b | `pocketstation::graph::ports::MAX_ASYNC_PAYLOAD_BYTES` | constant | Sets the maximum supported async payload bytes. | `src/graph/ports.rs:13` |
| sym-374159181c3213969fb7 | `pocketstation::native_extension::EXTENSION_LIBRARY_ENTRYPOINT_V1` | constant | Exact exported symbol required from a native Extension ABI v1 dynamic library. The suffix follows the ABI major; compatible minor revisions use the same entrypoint. | `src/native_extension/mod.rs:24` |
| sym-96f659bd569e1533d2d8 | `pocketstation::recording::endpoint::MULTISTEM_GROUP_CONFIGURATION_KEY` | constant | Defines the public multistem group configuration key value. | `src/recording/endpoint.rs:24` |
| sym-aa9aa2ac45f752b6f16a | `pocketstation::recording::endpoint::MULTISTEM_NAME_CONFIGURATION_KEY` | constant | Defines the public multistem name configuration key value. | `src/recording/endpoint.rs:25` |
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
| sym-8c9708e9a20bf5143276 | `pocketstation::timing::PROCESS_MONOTONIC_CLOCK_DOMAIN_ID` | constant | Clock-domain identity for timestamps produced by PocketStation's shared process-wide monotonic clock. | `src/timing/mod.rs:20` |
| sym-8ad24580c5c47968ab09 | `pocketstation::SessionCancelDisposition` | enum | Classifies the observable session cancel disposition. | `src/lib.rs:1085` |
| sym-71d2dc01f8d101bd5f68 | `pocketstation::SessionEndpointError` | enum | Classifies failures reported as session endpoint error. | `src/lib.rs:1035` |
| sym-b28b9481e9a8ae46b85d | `pocketstation::SessionOperatorError` | enum | Classifies failures reported as session operator error. | `src/lib.rs:1051` |
| sym-3f457dcebaf1e7367342 | `pocketstation::SessionRuntimeError` | enum | Classifies failures reported as session runtime error. | `src/lib.rs:1073` |
| sym-343ab031df379030a5f2 | `pocketstation::SessionSidecarError` | enum | Classifies failures reported as session sidecar error. | `src/lib.rs:1045` |
| sym-753083e26461ba34616f | `pocketstation::SessionSourceError` | enum | Classifies failures reported as session source error. | `src/lib.rs:1057` |
| sym-3f39126331442d84ecd6 | `pocketstation::SessionStartErrorKind` | enum | Selects the session start error kind used by PocketStation. | `src/lib.rs:1063` |
| sym-f01466d96b73789c1c2a | `pocketstation::SessionStopDisposition` | enum | Classifies the observable session stop disposition. | `src/lib.rs:1079` |
| sym-36cd76830d91616ed45a | `pocketstation::abi::extension::PksExtensionKind` | enum | Selects the extension kind used by PocketStation. | `src/abi/extension.rs:32` |
| sym-664ec5a3767a0555bb69 | `pocketstation::abi::extension::PksExtensionPortDirection` | enum | Selects the extension port direction used by PocketStation. | `src/abi/extension.rs:40` |
| sym-082a210661ef8384587f | `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Enumerates the supported session status code cases. | `src/abi/session/abi.rs:79` |
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
| sym-1396fee9ba1a6fac3da4 | `pocketstation::codec::decoder::OpusDecodeError` | enum | Classifies failures reported as opus decode error. | `src/codec/decoder.rs:25` |
| sym-76c962dec9e247b27a0b | `pocketstation::codec::encoder::OpusApplication` | enum | Selects the Opus encoder mode used to tune speech or general audio. | `src/codec/encoder.rs:58` |
| sym-05ac9bbc5f498492100a | `pocketstation::codec::encoder::OpusChannels` | enum | Typed channel count for Opus — prevents silent u8 misuse. | `src/codec/encoder.rs:27` |
| sym-a8a0d170d170e25329c3 | `pocketstation::codec::encoder::OpusEncodeError` | enum | Classifies failures reported as opus encode error. | `src/codec/encoder.rs:131` |
| sym-96b7637b5d8bfeeb0b34 | `pocketstation::codec::encoder::OpusFrameDuration` | enum | Supported Opus frame duration at 48 kHz. | `src/codec/encoder.rs:7` |
| sym-8afbed3a6983510a1a34 | `pocketstation::codec::encoder::OpusSampleRate` | enum | Typed sample rate. Opus internally always uses 48 kHz; this type makes the constraint explicit rather than hiding it behind a `u32` constant. | `src/codec/encoder.rs:44` |
| sym-aa25e74e66f28fe8efaf | `pocketstation::codec::profile::StreamProfile` | enum | Enumerates the supported stream profile cases. | `src/codec/profile.rs:11` |
| sym-e7d9f671a0008406878d | `pocketstation::conformance::ObservedEndpointError` | enum | Classifies failures reported as observed endpoint error. | `src/conformance.rs:345` |
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
| sym-04d73a434600a5d3f46b | `pocketstation::frame::audio::AudioFrameBuildError` | enum | Classifies failures reported as audio frame build error. | `src/frame/audio.rs:51` |
| sym-2ef18de7c50ff69f9668 | `pocketstation::frame::audio::FrameLineageError` | enum | Classifies failures reported as frame lineage error. | `src/frame/audio.rs:250` |
| sym-4126c27a577e7dc06b43 | `pocketstation::frame::audio::SampleFormat` | enum | Selects the sample format used by PocketStation. | `src/frame/audio.rs:13` |
| sym-1ec57531bcf0b3720cf6 | `pocketstation::frame::lineage::FrameLineageBuildError` | enum | Classifies failures reported as frame lineage build error. | `src/frame/lineage.rs:93` |
| sym-f9317dbcae260dd5577a | `pocketstation::frame::platform::Platform` | enum | Enumerates the supported platform cases. | `src/frame/platform.rs:4` |
| sym-7ee17b056959ed78ed91 | `pocketstation::frame::pool::AudioBufferWriteError` | enum | Classifies failures reported as audio buffer write error. | `src/frame/pool.rs:14` |
| sym-4fcb9723ef545df20ac8 | `pocketstation::graph::compile::resolve::CompileError` | enum | Classifies failures reported as compile error. | `src/graph/compile/resolve.rs:26` |
| sym-c3a1ecffd346ee433f7d | `pocketstation::graph::node::ConfigError` | enum | Classifies failures reported as config error. | `src/graph/node.rs:141` |
| sym-f025a37eca498e64758e | `pocketstation::graph::node::NodeDescriptorError` | enum | Classifies failures reported as node descriptor error. | `src/graph/node.rs:252` |
| sym-e77de4e7bba3768a4f6c | `pocketstation::graph::node::NodeError` | enum | Classifies failures reported as node error. | `src/graph/node.rs:149` |
| sym-cc20f40d48d0b1cb1f05 | `pocketstation::graph::partition::ExecutionPartition` | enum | WHERE an operator runs. | `src/graph/partition.rs:18` |
| sym-029e7dbe36cf7d064f9a | `pocketstation::graph::partition::SafetyContract` | enum | WHAT an operator guarantees about its runtime behaviour. | `src/graph/partition.rs:82` |
| sym-af7ad85c4fbca3400638 | `pocketstation::graph::plan::PlanError` | enum | Classifies failures reported as plan error. | `src/graph/plan.rs:21` |
| sym-483b55a8b79b880f182d | `pocketstation::graph::ports::BackpressurePolicy` | enum | Selects the backpressure policy used by PocketStation. | `src/graph/ports.rs:265` |
| sym-4c00608939c517ac97d7 | `pocketstation::graph::ports::ChannelLayout` | enum | Enumerates the supported channel layout cases. | `src/graph/ports.rs:27` |
| sym-6b73e2a8a016f8e04982 | `pocketstation::graph::ports::ClockDomain` | enum | Enumerates the supported clock domain cases. | `src/graph/ports.rs:249` |
| sym-eab5bcb627544f2e4f84 | `pocketstation::graph::ports::CopyPolicy` | enum | Selects the copy policy used by PocketStation. | `src/graph/ports.rs:280` |
| sym-ff6064aef4e4fce40483 | `pocketstation::graph::ports::DeliverySemantics` | enum | Selects the delivery semantics used by PocketStation. | `src/graph/ports.rs:273` |
| sym-d66bbe8dc359d2f3928f | `pocketstation::graph::ports::EdgeObservabilityLevel` | enum | Selects the edge observability level used by PocketStation. | `src/graph/ports.rs:294` |
| sym-eb1f585bd7270ee1a32c | `pocketstation::graph::ports::LossPolicy` | enum | Selects the loss policy used by PocketStation. | `src/graph/ports.rs:287` |
| sym-3c90a46c18851e763795 | `pocketstation::graph::ports::MediaCaps` | enum | Enumerates the supported media caps cases. | `src/graph/ports.rs:85` |
| sym-d7f179dd53d4e0981b38 | `pocketstation::graph::ports::MediaKind` | enum | Selects the media kind used by PocketStation. | `src/graph/ports.rs:16` |
| sym-2ceeae7585bdacd3fc7a | `pocketstation::graph::ports::Multiplicity` | enum | Enumerates the supported multiplicity cases. | `src/graph/ports.rs:169` |
| sym-137cad195a6ea3af43e0 | `pocketstation::graph::ports::PortDirection` | enum | Selects the port direction used by PocketStation. | `src/graph/ports.rs:163` |
| sym-48116bf4757dd0ea87d6 | `pocketstation::graph::ports::PortSpecError` | enum | Classifies failures reported as port spec error. | `src/graph/ports.rs:239` |
| sym-439a0bf5cb33e6b94cb5 | `pocketstation::graph::registry::NodeDefinitionRef` | enum | Enumerates the supported node definition ref cases. | `src/graph/registry.rs:32` |
| sym-b6d81f6ae57aa0d62bfa | `pocketstation::graph::registry::NodeRegistrationError` | enum | Classifies failures reported as node registration error. | `src/graph/registry.rs:57` |
| sym-d955a14f909addfc1475 | `pocketstation::graph::signal::continuity::SignalContinuityError` | enum | Classifies failures reported as signal continuity error. | `src/graph/signal/continuity.rs:89` |
| sym-bcc991272a84381d89b1 | `pocketstation::graph::signal::envelope::SignalEnvelopeError` | enum | Classifies failures reported as signal envelope error. | `src/graph/signal/envelope.rs:137` |
| sym-6970d81cb09c75a72210 | `pocketstation::graph::signal::lineage::SignalDerivationError` | enum | Classifies failures reported as signal derivation error. | `src/graph/signal/lineage.rs:159` |
| sym-aafc34922d43197db130 | `pocketstation::graph::signal::lineage::SignalLineageError` | enum | Classifies failures reported as signal lineage error. | `src/graph/signal/lineage.rs:86` |
| sym-f7159cc45b5de1e0732c | `pocketstation::graph::signal::operator::AsyncOperatorManifestError` | enum | Classifies failures reported as async operator manifest error. | `src/graph/signal/operator.rs:321` |
| sym-26d6430f6471c39c45bb | `pocketstation::graph::signal::operator::OperatorCancellationPolicy` | enum | Selects the operator cancellation policy used by PocketStation. | `src/graph/signal/operator.rs:57` |
| sym-1c4c408d47a3d6396cfb | `pocketstation::graph::signal::operator::OperatorFailurePolicy` | enum | Selects the operator failure policy used by PocketStation. | `src/graph/signal/operator.rs:63` |
| sym-9868bde57bc5fc380680 | `pocketstation::graph::signal::payload::SignalPayload` | enum | Enumerates the supported signal payload cases. | `src/graph/signal/payload.rs:10` |
| sym-3aaa0287f72b8a594c51 | `pocketstation::graph::signal::spec::BinaryFormat` | enum | Binary encoding hint for `SignalClass::Binary`. | `src/graph/signal/spec.rs:141` |
| sym-66ab9aa90b84727d686a | `pocketstation::graph::signal::spec::Codec` | enum | Audio encoding format for `SignalClass::EncodedAudio`. | `src/graph/signal/spec.rs:113` |
| sym-d1913dcde8b63010c8c6 | `pocketstation::graph::signal::spec::EventFormat` | enum | Event structure hint for `SignalClass::Event`. | `src/graph/signal/spec.rs:132` |
| sym-e97c39ab79fc17dd35dd | `pocketstation::graph::signal::spec::SignalClass` | enum | The fundamental class of data flowing through a port. | `src/graph/signal/spec.rs:156` |
| sym-3c1758bf6bba6308e9df | `pocketstation::graph::signal::spec::SignalSpecError` | enum | Classifies failures reported as signal spec error. | `src/graph/signal/spec.rs:351` |
| sym-f83afadac94e75998e53 | `pocketstation::graph::signal::spec::TextFormat` | enum | Text encoding hint for `SignalClass::Text`. | `src/graph/signal/spec.rs:124` |
| sym-5818bdec6a1b05fc739b | `pocketstation::graph::signal::timing::SignalTimingError` | enum | Classifies failures reported as signal timing error. | `src/graph/signal/timing.rs:89` |
| sym-350b38ad4fa5a2014d84 | `pocketstation::native_extension::NativeExtensionKind` | enum | Selects the native extension kind used by PocketStation. | `src/native_extension/mod.rs:27` |
| sym-3c3f70e43d0556be3c9e | `pocketstation::native_extension::NativeExtensionLibraryErrorCode` | enum | Enumerates the supported native extension library error code cases. | `src/native_extension/mod.rs:78` |
| sym-257559049c741e0b7fdd | `pocketstation::recording::config::PermissionDecision` | enum | Enumerates the supported permission decision cases. | `src/recording/config.rs:43` |
| sym-c1c408fed30ef9989c9e | `pocketstation::recording::config::PermissionScope` | enum | Selects the permission scope used by PocketStation. | `src/recording/config.rs:50` |
| sym-d088b4347207780b0458 | `pocketstation::recording::config::RecorderLineageField` | enum | Enumerates the supported recorder lineage field cases. | `src/recording/config.rs:10` |
| sym-9703518ae35e1e6a80d6 | `pocketstation::recording::error_code::RecordingErrorCode` | enum | Stable language-neutral code for a recording failure. | `src/recording/error_code.rs:9` |
| sym-d6ebc5c666e65cd800a5 | `pocketstation::recording::writer::DiscontinuityKind` | enum | Selects the discontinuity kind used by PocketStation. | `src/recording/writer.rs:104` |
| sym-3bdbbc813bb88358314c | `pocketstation::recording::writer::RecorderError` | enum | Classifies failures reported as recorder error. | `src/recording/writer.rs:23` |
| sym-3e43b04c63f0fb274f6e | `pocketstation::recording::writer::RecordingState` | enum | Selects the recording state used by PocketStation. | `src/recording/writer.rs:85` |
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
| sym-5aae2cabada09c6ba8a6 | `AsyncNode::cancel` | function | Requests cancellation of `AsyncNode`. | `src/graph/signal/operator.rs:36` |
| sym-e6e78ffac15439249c55 | `AsyncNode::close` | function | Closes `AsyncNode` to further work. | `src/graph/signal/operator.rs:40` |
| sym-60c43b986b92ec3bfda3 | `AsyncNode::flush` | function | Flushes pending output from `AsyncNode` at the end of a run. | `src/graph/signal/operator.rs:32` |
| sym-4c83f61648e2a06176df | `AsyncNode::prepare` | function | Prepares resources required by `AsyncNode`. | `src/graph/signal/operator.rs:14` |
| sym-21b01f5c6ed3a11f1a20 | `AsyncNode::process` | function | Processes an input value through `AsyncNode`. | `src/graph/signal/operator.rs:19` |
| sym-0942eba5d5fceac41988 | `AsyncNode::process_port` | function | Returns the process port held by `AsyncNode`. | `src/graph/signal/operator.rs:24` |
| sym-708ec8c1d50ce7123227 | `AsyncOperatorFactory::create` | function | Creates the runtime implementation described by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:378` |
| sym-bcfefc27332935ccca94 | `AsyncOperatorFactory::manifest` | function | Returns the manifest held by `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:369` |
| sym-5e256a2c9ff41b5bdcfd | `AsyncOperatorFactory::resolve_manifest` | function | Resolves manifest for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:371` |
| sym-7c0f476638ec6b056d69 | `AsyncOperatorFactory::validate_config` | function | Validates config for `AsyncOperatorFactory`. | `src/graph/signal/operator.rs:370` |
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
| sym-92eb8f203e1ebecb6f82 | `NodeDefinition::descriptor` | function | Returns the descriptor associated with `NodeDefinition`. | `src/graph/registry.rs:22` |
| sym-9d7949ea0b416de4d1cb | `NodeDefinition::validate_config` | function | Validates config for `NodeDefinition`. | `src/graph/registry.rs:23` |
| sym-eaddd3481c0f9e30d8c0 | `NodeFactory::descriptor` | function | Returns the descriptor associated with `NodeFactory`. | `src/graph/registry.rs:12` |
| sym-dcd8abe9dd507babe210 | `NodeFactory::instantiate` | function | Instantiates the runtime node described by `NodeFactory`. | `src/graph/registry.rs:14` |
| sym-f94849632be857a7354a | `NodeFactory::validate_config` | function | Validates config for `NodeFactory`. | `src/graph/registry.rs:13` |
| sym-f70e9390f0489e77ec23 | `PreparedCaptureBackend::open` | function | Opens the resource represented by `PreparedCaptureBackend`. | `src/capture/capture_owner.rs:89` |
| sym-775076f9f636c432e69c | `PreparedEndpointDriver::cancel_preparation` | function | Cancels preparation for `PreparedEndpointDriver`. | `src/endpoint/runtime.rs:328` |
| sym-5c9f2e79390cbd3a6554 | `PreparedEndpointDriver::start` | function | Makes endpoint resources ready behind the supplied closed gate. | `src/endpoint/runtime.rs:323` |
| sym-f0e8cdf84d48aca086d6 | `RunningEndpointDriver::join_and_finalize` | function | Joins and finalize for `RunningEndpointDriver`. | `src/endpoint/runtime.rs:346` |
| sym-459a9a074e0856994405 | `RunningEndpointDriver::observations` | function | Returns the observations exposed by `RunningEndpointDriver`. | `src/endpoint/runtime.rs:337` |
| sym-4cb08eb58cfec6f939e5 | `RunningEndpointDriver::request_shutdown` | function | Requests the selected shutdown mode from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:341` |
| sym-d1f376b21fdab92dc979 | `RunningEndpointDriver::request_stop` | function | Requests a graceful stop from `RunningEndpointDriver`. | `src/endpoint/runtime.rs:339` |
| sym-f2fa4350e7a859527850 | `RuntimeNode::prepare` | function | Prepares resources required by `RuntimeNode`. | `src/graph/runtime_node.rs:8` |
| sym-2ef7f17b8694f3d96cbd | `RuntimeNode::process` | function | Processes an input value through `RuntimeNode`. | `src/graph/runtime_node.rs:9` |
| sym-617d7fa081197246ac8a | `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| sym-b93a63c9f08d180a1385 | `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| sym-1791481d286846241ac9 | `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| sym-2e8c31657a884516d1db | `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| sym-6a128ee236756cc8905e | `SourceFactory::manifest` | function | Returns the manifest held by `SourceFactory`. | `src/session/extensions/source.rs:277` |
| sym-6cd2abb6461a40ec9b16 | `SourceFactory::validate_config` | function | Validates config for `SourceFactory`. | `src/session/extensions/source.rs:278` |
| sym-5d1bf5166addc34ce5d2 | `StreamSignal::signal_spec` | function | Returns the signal spec held by `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| sym-c1c61c7d9b4bef0b1508 | `accepts` | function | Returns whether accepts is true for `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:75` |
| sym-1674b375b35b6e757720 | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorDeliveryReadiness`. | `src/connector/status.rs:10` |
| sym-1a7508978d9c24e664bb | `accepts_delivery` | function | Returns whether accepts delivery applies to `ConnectorServiceStatus`. | `src/connector/status.rs:74` |
| sym-2ba2c6e834e0e56d1b87 | `accumulated_error_ns` | function | Returns the accumulated error nanoseconds held by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:62` |
| sym-17b81c499a05173f1deb | `acquire` | function | Attempts to acquire an available buffer slot from `AudioBufferPool`. | `src/frame/pool.rs:75` |
| sym-c1e159b4daa087bdfc39 | `acquire_failures` | function | Returns the acquire failures associated with `AudioBufferPool`. | `src/frame/pool.rs:68` |
| sym-87003503ff089a9dd863 | `add_node` | function | Adds node for `Pipeline`. | `src/graph/dsl.rs:44` |
| sym-0760bc020ba894b3f5bf | `advance_from_source_position` | function | Returns a buffer's source-time start from its native sample-frame position. Forward gaps are preserved in the returned timestamp without separately advancing this clock from an aggregate drop counter. | `src/capture/timeline.rs:80` |
| sym-fa0ffc427c129433bceb | `after_failed_open` | function | Records a backend open failure without guessing that permission was denied. | `src/capture/authorization.rs:45` |
| sym-d0f5605e55cfb06011a6 | `after_successful_open` | function | Records the evidence available after an explicitly selected source opens. | `src/capture/authorization.rs:31` |
| sym-6ce1a2cb5ee1f86493cf | `any` | function | Convenience constructor for a deliberately open boundary port. | `src/graph/signal/spec.rs:264` |
| sym-e92fd812c090f3a0cb3c | `api_revision` | function | Returns the API revision held by `ConnectorManifest`. | `src/connector/manifest.rs:124` |
| sym-b458165e8136073dfb32 | `application` | function | Returns the application held by `StreamProfile`. | `src/codec/profile.rs:41` |
| sym-ef79fdde27b96aa64bd9 | `application` | function | Returns the application held by `Source`. | `src/session/declaration/selector.rs:140` |
| sym-f9871f0dbb9b665b5715 | `as_mut_slice` | function | Borrows `AudioBufferHandle` as mut slice. | `src/frame/pool.rs:218` |
| sym-4a7e2a80ef427e839b4d | `as_slice` | function | Borrows `AudioBufferHandle` as slice. | `src/frame/pool.rs:214` |
| sym-0cf65d4a441597a1309e | `as_slice` | function | Borrows `SharedAudioBufferHandle` as slice. | `src/frame/pool.rs:300` |
| sym-ff0a5c3647be65f22b11 | `as_str` | function | Returns the stable string representation of `ConnectorConfigurationErrorCode`. | `src/connector/configuration.rs:585` |
| sym-97f88351f6898933637f | `as_str` | function | Returns the stable string representation of `ConnectorErrorCode`. | `src/connector/error.rs:29` |
| sym-095f9600d9b7441f9d03 | `as_str` | function | Returns the stable string representation of `EndpointGroupId`. | `src/endpoint/identity.rs:16` |
| sym-d65e4bd95d21a2527dcc | `as_str` | function | Returns the stable string representation of `NodeTypeId`. | `src/graph/node.rs:16` |
| sym-f50d405ba82ebeb40541 | `as_str` | function | Returns the stable string representation of `OperatorId`. | `src/graph/operator.rs:23` |
| sym-cbd818fcc6341db1dea3 | `as_str` | function | Returns the stable string representation of `SignalId`. | `src/graph/signal/spec.rs:29` |
| sym-c19bc7d1b15405d7ef23 | `as_str` | function | Returns the stable string representation of `SemanticRole`. | `src/graph/signal/spec.rs:64` |
| sym-7089f618e899e9dab49b | `as_str` | function | Returns the stable string representation of `SchemaRef`. | `src/graph/signal/spec.rs:94` |
| sym-25e46e42e4bde888d842 | `as_str` | function | Returns the stable string representation of `NativeExtensionLibraryErrorCode`. | `src/native_extension/mod.rs:97` |
| sym-e380c2ab6ae4344224a7 | `as_str` | function | Returns the stable string representation of `StemLabel`. | `src/recording/config.rs:36` |
| sym-77781a10d7c76e7960c7 | `as_str` | function | Returns the stable string representation of `RecordingErrorCode`. | `src/recording/error_code.rs:32` |
| sym-bf6c00beab1af63e1860 | `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| sym-24fd498b139cb8cb0ac2 | `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| sym-d690b5d01072ac18d623 | `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| sym-bae0d7a3f3cb88aef9ae | `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| sym-a050f7fae6bde7725ea6 | `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| sym-e7079625485c166b9318 | `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| sym-ea9faf951c1432c15b29 | `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| sym-02909264b072a2cd7279 | `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| sym-53240e78e7e5e31d64d1 | `async_factory` | function | Returns the async factory associated with `NodeRegistry`. | `src/graph/registry.rs:144` |
| sym-4aa59105bd7b79996be5 | `async_factory_by_operator` | function | Returns the async factory by operator associated with `NodeRegistry`. | `src/graph/registry.rs:151` |
| sym-f10e1d3bd85962bda1c4 | `async_node_type_id` | function | Returns the async node type identifier held by `NodeRegistry`. | `src/graph/registry.rs:160` |
| sym-b8325e913de8e0179332 | `audio` | function | Convenience constructor for PCM audio ports. | `src/graph/signal/spec.rs:269` |
| sym-2bae5d019dea07a6f21b | `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| sym-a7067ae4a47027892edf | `audio_input` | function | Opens a bounded input for audio already owned by the embedding application. | `src/lib.rs:447` |
| sym-3de80c3fe1915fe5508e | `audio_observations` | function | Returns the audio observations held by `RunningSession`. | `src/lib.rs:806` |
| sym-c151cd3ac57b46c765f0 | `audio_reentry_metrics` | function | Returns exact queue, pool, loss, and lifecycle accounting for every Session-owned typed-PCM reentry into the specialized audio lane. | `src/lib.rs:874` |
| sym-3619d61a5d57a170ea24 | `audio_reentry_metrics` | function | Returns the audio reentry metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:217` |
| sym-7f4c17f9bbbf1584724e | `audio_stem_id` | function | Returns the audio stem identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:88` |
| sym-1735fdd774ca425845f1 | `available_slots` | function | Returns the available slots associated with `AudioBufferPool`. | `src/frame/pool.rs:71` |
| sym-63f1aac4dd3f9eaa49fd | `backpressure` | function | Returns the backpressure associated with `EdgeContract`. | `src/graph/ports.rs:341` |
| sym-2ecc6be8dc695c834430 | `before_open` | function | Records a source that was resolved but not opened because another required source failed first. | `src/capture/authorization.rs:60` |
| sym-5cf5030b216c16fe810e | `binary` | function | Convenience constructor for opaque or schema-backed binary ports. | `src/graph/signal/spec.rs:299` |
| sym-685ef51f186153db0929 | `bitrate_kbps` | function | Returns the bitrate kbps associated with `StreamProfile`. | `src/codec/profile.rs:51` |
| sym-0d4a11b9d5ef2e26e62f | `bounded_async` | function | Generic bounded asynchronous edge. Connected ports supply the payload representation and the envelope preserves its producer clock. | `src/graph/ports.rs:413` |
| sym-16bd4425261fffd04ba8 | `branch_copy_pool_bytes` | function | Returns the branch copy pool bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:57` |
| sym-9d712fad464358ad5323 | `branch_copy_pool_capacity_frames` | function | Returns the branch copy pool capacity frames held by `EdgeBufferPlan`. | `src/graph/plan.rs:48` |
| sym-043e1ab34abcd3592088 | `browser` | function | Declares a browser/remote receiver. Register its transport implementation with [`Self::register_browser_driver`]. | `src/lib.rs:534` |
| sym-aad1ecb21c3c011a267f | `browser` | function | Returns the browser associated with `Session`. | `src/session/declaration/draft.rs:443` |
| sym-9bec835b5bcd6e2656e8 | `build` | function | Builds the Session declaration owner. | `src/lib.rs:336` |
| sym-9bf522ae209685bd39bf | `build` | function | Consumes all setup state so no partially populated registry can escape. | `src/session/lifecycle/engine.rs:176` |
| sym-2d52a1225970e7513f70 | `build` | function | Builds its owned operation for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:344` |
| sym-5bdc2470e7c237e69120 | `builder` | function | Creates a builder for declaring `Session` sources, routes, and endpoints. | `src/lib.rs:374` |
| sym-8dcf1610f287d6f7139c | `bundle_id` | function | Returns the bundle identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| sym-6926ce73538b1c25722c | `cancel` | function | Cancels active asynchronous Operators, then finalizes capture, runtime, endpoints, and recording through the same bounded Session authority. | `src/lib.rs:903` |
| sym-07d5fd0f7862cc3e0ddf | `cancel` | function | Requests cancellation of `SourceRuntime`. | `src/session/extensions/source.rs:575` |
| sym-cd665b6efd7a7a26e678 | `cancel` | function | Requests cancellation of `RunningSession`. | `src/session/lifecycle/running.rs:417` |
| sym-3761e5ac37050c354529 | `cancel_and_join` | function | Cancels and join for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:184` |
| sym-c6eb9568705b06fea58e | `cancel_and_join` | function | Cancels and join for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:925` |
| sym-363ceb64721f1577a42d | `cancel_and_reap` | function | Cancels and reap for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:322` |
| sym-60ff45fe284ba7b6ebb7 | `cancellation` | function | Returns the cancellation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:212` |
| sym-cd9034aece59b4e2f5ad | `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| sym-6a03bc81241e552b45a7 | `cancellation_total` | function | Returns the cancellation total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| sym-a23752ab1db9abce0522 | `canonical_path` | function | Returns the canonical path associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:68` |
| sym-71c8248552577ce31a95 | `capabilities` | function | Returns the capabilities held by `ConnectorManifest`. | `src/connector/manifest.rs:152` |
| sym-099e7f35723bb117ce7d | `capacity_frames` | function | Returns the capacity frames held by `CapturedFrameStream`. | `src/capture/frame_stream.rs:165` |
| sym-0347dd7ca21a781e74ea | `capacity_frames` | function | Returns the capacity frames held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| sym-e1727a00565651dc8f64 | `capacity_signals` | function | Returns the capacity signals associated with `PortPrepareContext`. | `src/graph/node.rs:361` |
| sym-de8b13ef7f5471f84012 | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/lib.rs:382` |
| sym-53e7fa01c1555e29c482 | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:347` |
| sym-9020569a5515cdf79f9e | `capture_backends` | function | Uses caller-owned capture backends while retaining the canonical Session compiler, runtime, endpoint lifecycle, and recording ownership. | `src/lib.rs:308` |
| sym-8f46c10e198bfff05070 | `capture_finalization_failures_total` | function | Returns the capture finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:330` |
| sym-b75c58a2c4f1f420edb8 | `capture_mode` | function | Returns the capture mode held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:57` |
| sym-1e412832c9049ffd81cf | `capture_mode` | function | Returns the capture mode held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:44` |
| sym-70771741ca5628c66f0f | `channel_count` | function | Returns the channel count held by `ChannelLayout`. | `src/graph/ports.rs:34` |
| sym-be0ad9663b006de72f97 | `channels` | function | Returns the channel count represented by `StreamProfile`. | `src/codec/profile.rs:21` |
| sym-e0ccb37acd23f29eaeeb | `channels` | function | Returns the channel count represented by `EndpointAudioFrame`. | `src/endpoint/contract.rs:47` |
| sym-7114c597af9660b246ff | `channels` | function | Returns the channel count represented by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:239` |
| sym-74fbbf79422965ab388b | `channels` | function | Returns the channel count represented by `AudioFrame`. | `src/frame/audio.rs:130` |
| sym-bdb64c3d02ab60c407e2 | `channels` | function | Returns the channel count represented by `SharedAudioFrame`. | `src/frame/audio.rs:200` |
| sym-7bfc0bf29cb72d1895d8 | `channels` | function | Returns the channel count represented by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:70` |
| sym-c7f680c12d1b3a72ec22 | `class` | function | Returns the class associated with `SignalSpec`. | `src/graph/signal/spec.rs:215` |
| sym-f0ea5047357f5a71e926 | `clipped_samples` | function | Returns the clipped samples held by `MixerTelemetry`. | `src/runtime/nodes.rs:260` |
| sym-da095c09dd3f9a6c970b | `clock` | function | Returns the clock associated with `EdgeContract`. | `src/graph/ports.rs:329` |
| sym-40e0d80cb91a22bab5a3 | `clock_id` | function | Returns the clock identifier held by `FrameLineage`. | `src/frame/lineage.rs:65` |
| sym-dc5fff3ffd689363e387 | `clock_id` | function | Returns the clock identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:68` |
| sym-5974737f23bbd935c441 | `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| sym-fe6bbea7f6396ae52e3f | `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| sym-3533308edab8aa54e59a | `close_and_reap` | function | Closes `SidecarHost` and reaps its child process. | `src/runtime/lifecycle/sidecar_host.rs:326` |
| sym-7d045272667522a51932 | `code` | function | Returns the stable error or status code represented by `ConnectorConfigurationError`. | `src/connector/configuration.rs:615` |
| sym-cfeaffe44c2c31baa3a1 | `code` | function | Returns the stable error or status code represented by `ConnectorError`. | `src/connector/error.rs:109` |
| sym-91cb556f2fd4381a2927 | `code` | function | Returns the stable error or status code represented by `EndpointFailure`. | `src/endpoint/runtime.rs:212` |
| sym-b70c506801b7d44f0f84 | `code` | function | Returns the stable error or status code represented by `SessionRuntimeError`. | `src/error_code.rs:5` |
| sym-62656e15659d007eff26 | `code` | function | Returns the stable error or status code represented by `SessionStopResult`. | `src/error_code.rs:15` |
| sym-ebd65e069ca248666f88 | `code` | function | Returns the stable error or status code represented by `SessionStartError`. | `src/lib.rs:957` |
| sym-cfb1c8bf367e13c8d444 | `code` | function | Returns the stable error or status code represented by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:131` |
| sym-e11f8c63a3184f4b90fa | `code` | function | Returns the stable error or status code represented by `RecorderError`. | `src/recording/error_code.rs:59` |
| sym-910af977649e51eb0d87 | `compile` | function | Compiles its owned operation for `Compiler`. | `src/graph/compile/resolve.rs:464` |
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
| sym-255c1250509b1e464cbc | `connect` | function | Connects the requested ports through `Pipeline`. | `src/graph/dsl.rs:55` |
| sym-97b189b635cb7c6dbcaf | `connect` | function | Connects the requested ports through `StemHandle`. | `src/session/declaration/draft.rs:819` |
| sym-b5ff73fcfbbc618dae3f | `connect` | function | Connects the requested ports through `SourceOutputHandle`. | `src/session/declaration/draft.rs:955` |
| sym-4720f8c89561ea4b9ea0 | `connect` | function | Connects the requested ports through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1051` |
| sym-bbad897aeb7a5e08149e | `connect_with` | function | Connects pipeline ports using the supplied edge contract on `Pipeline`. | `src/graph/dsl.rs:59` |
| sym-6b73136640e4945a0e3d | `connections` | function | Returns the connections associated with `SessionSpec`. | `src/session/declaration/spec.rs:347` |
| sym-f7f8fd71c410fec90af8 | `connector` | function | Declares an external connector. Register its implementation after route identities are available with [`Self::register_connector_driver`]. | `src/lib.rs:507` |
| sym-8381579d195d96495dd5 | `connector` | function | Declares a connector endpoint on `Session` with the supplied operator identity and configuration. | `src/session/declaration/draft.rs:411` |
| sym-13ba1f8b4e64872e2f75 | `connector_id` | function | Returns the connector identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:32` |
| sym-1f4adc143d29ff96861c | `connector_id` | function | Returns the connector identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:231` |
| sym-411c5f107d46d687613f | `connector_id` | function | Returns the connector identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:138` |
| sym-e1d54b23cb99c6c6b7a7 | `connector_id` | function | Returns the connector identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:153` |
| sym-f4f0b73539dae0c559db | `connector_id` | function | Returns the connector identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:607` |
| sym-cf7d333c61a83867f2a0 | `connector_id` | function | Returns the connector identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:175` |
| sym-1165bb75ef97e65e025a | `constraints` | function | Returns the constraints held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:222` |
| sym-28f290acaf29fc919a23 | `contains` | function | Returns whether contains is true for `NodeRegistry`. | `src/graph/registry.rs:164` |
| sym-765b19e5b736c4a1abb1 | `context` | function | Returns the context held by `EndpointPortInput`. | `src/endpoint/contract.rs:219` |
| sym-616a7202b6ff0092c073 | `control` | function | Convenience constructor for control ports. | `src/graph/signal/spec.rs:294` |
| sym-be7fa54e9ea9043544f7 | `copy_policy` | function | Returns the copy policy held by `EdgeContract`. | `src/graph/ports.rs:353` |
| sym-ec92f1b89383f257d00b | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedAudioFrame`. | `src/frame/audio.rs:233` |
| sym-7417542e02f081e0c7b8 | `copy_to_pool` | function | Copies the shared frame into storage acquired from the supplied pool for `SharedLineagedAudioFrame`. | `src/frame/audio.rs:319` |
| sym-b64a7061c2e4a315d65e | `count` | function | Returns the count associated with `OpusChannels`. | `src/codec/encoder.rs:33` |
| sym-b9dec8c2506c097ed3e7 | `current` | function | Returns the current value observed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:196` |
| sym-9368d177ae3e8cbdd0aa | `custom` | function | Convenience constructor for custom / vendor extension ports. | `src/graph/signal/spec.rs:304` |
| sym-7f3c3aecd675c182eddd | `deadline` | function | Returns the deadline associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:208` |
| sym-c8fe3feaf7fa7ca52e7d | `declare` | function | Adds the declaration represented by `RegisteredConnector` to its Session. | `src/connector/mod.rs:159` |
| sym-54fecfe30fb4f38fba2b | `declares_multistem_recording` | function | Returns whether `Session` declares multistem recording. | `src/session/extensions/recording.rs:98` |
| sym-dc7ecd45f2947dea2ccb | `decode` | function | Decodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:93` |
| sym-53ba1550fb3c2f8d1dba | `decode` | function | Decodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:403` |
| sym-8d92b0a47a80648a77d8 | `decode` | function | Decodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:121` |
| sym-ce095fd9d80ee307bb44 | `decode_into` | function | Decode a compressed Opus packet into i16 samples, then convert to f32. | `src/codec/decoder.rs:81` |
| sym-6b088b00b1c70ccd8a09 | `decode_plc_into` | function | Conceal one missing packet while preserving libopus decoder state. | `src/codec/decoder.rs:116` |
| sym-b02bba0830f20e8d62da | `default` | function | Returns the default `OpusDecoder` value. | `src/codec/decoder.rs:175` |
| sym-68385e80df96d1bfbd0b | `default` | function | Returns the default `OpusConfig` value. | `src/codec/encoder.rs:92` |
| sym-bf39d5649ed2cf6816ae | `default` | function | Returns the default `OpusEncoder` value. | `src/codec/encoder.rs:303` |
| sym-6a7daffc3610023cad91 | `default` | function | Returns the default `PolledAudioEndpointConfig` value. | `src/endpoint/polled_audio_driver.rs:30` |
| sym-81cb0c79d1667584fca9 | `default` | function | Returns the default `RuntimePlanner` value. | `src/graph/compile/plan.rs:349` |
| sym-ae5d332bf994af7d868e | `default` | function | Returns the default `Compiler` value. | `src/graph/compile/resolve.rs:513` |
| sym-d5fdd1e97e84aa2d91f8 | `default` | function | Returns the default `SessionBuilder` value. | `src/lib.rs:279` |
| sym-b8afbdbbed22fffaf243 | `default` | function | Returns the default `Session` value. | `src/lib.rs:780` |
| sym-5d74bb4ba301e13b6548 | `default` | function | Returns the default `PlanRunnerCancellation` value. | `src/runtime/audio/runner.rs:110` |
| sym-7db8245fb4f216c6a814 | `default` | function | Returns the default `SidecarDeadlines` value. | `src/runtime/lifecycle/sidecar_host.rs:61` |
| sym-af19b8b3405091440643 | `default` | function | Returns the default `SidecarProtocolLimits` value. | `src/runtime/lifecycle/sidecar_protocol.rs:51` |
| sym-9c03c0728aa593e1d73f | `default` | function | Returns the default `Session` value. | `src/session/declaration/draft.rs:577` |
| sym-b4a0bbefbec170268134 | `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| sym-fbbdd70b9d27a4341930 | `default` | function | Returns the default `NativeSessionEngineHostOptions` value. | `src/session/lifecycle/host.rs:172` |
| sym-7d2662192d98d7cb188f | `default` | function | Returns the default `SessionStartOptions` value. | `src/session/lifecycle/start_contract.rs:33` |
| sym-0b26dc8e32052d6f3db2 | `default` | function | Returns the default `ClockCorrectionController` value. | `src/timing/clock_correction.rs:52` |
| sym-7a31f0974065a9809fd1 | `default` | function | Returns the default `ClockDriftEstimator` value. | `src/timing/clock_drift.rs:115` |
| sym-73080b2b6d7efead95e0 | `definition` | function | Returns the definition associated with `NodeRegistry`. | `src/graph/registry.rs:134` |
| sym-471995bd2d8dd6e45a0a | `delivery` | function | Returns the delivery associated with `EdgeContract`. | `src/graph/ports.rs:345` |
| sym-263af9445d9a180f56bc | `delivery_readiness` | function | Returns the delivery readiness associated with `ConnectorServiceStatus`. | `src/connector/status.rs:42` |
| sym-6bc2402fe269fe7f6b4e | `deprecated` | function | Returns the deprecated held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:201` |
| sym-a1444469e193165c6f22 | `deprecation` | function | Returns the deprecation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:226` |
| sym-3e9506472d6a607ff82f | `derivation` | function | Returns the derivation associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:82` |
| sym-12355a31167104ea1ffd | `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| sym-0a891da3911ec35e5ef0 | `derived_route_count` | function | Returns the derived route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| sym-6a2e0c13ae37d15c1e19 | `derived_route_metrics` | function | Returns one observation handle per derived operator-output route. | `src/lib.rs:868` |
| sym-f48d91e269c7742941c0 | `derived_route_metrics` | function | Returns the derived route metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:213` |
| sym-8277e57458dee8c11f29 | `descriptor` | function | Returns the descriptor associated with `PassthroughFactory`. | `src/graph/builtins.rs:70` |
| sym-476fbda9bf1e13a7449f | `descriptor` | function | Returns the descriptor associated with `GainFactory`. | `src/graph/builtins.rs:110` |
| sym-cd9a178a1ded665bb67e | `descriptor` | function | Returns the descriptor associated with `MonoMixFactory`. | `src/graph/builtins.rs:169` |
| sym-165eb2ce7542087de402 | `descriptor` | function | Returns the descriptor associated with `NodeDefinitionRef`. | `src/graph/registry.rs:39` |
| sym-458cc89a27928f87c11f | `descriptor` | function | Returns the descriptor associated with `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:87` |
| sym-3bc3623fa5be2ed7f264 | `descriptor` | function | Returns the descriptor associated with `BridgeSinkFactory`. | `src/runtime/nodes.rs:191` |
| sym-ddbb7babdefa2c49ba99 | `direction` | function | Returns the direction associated with `PortPrepareContext`. | `src/graph/node.rs:345` |
| sym-93a75b6d781d2c33cac2 | `direction` | function | Returns the direction associated with `PortSpec`. | `src/graph/ports.rs:217` |
| sym-65059d3406a37d9860f0 | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `FrameLineage`. | `src/frame/lineage.rs:80` |
| sym-f8b730c83e4adefb9406 | `discontinuity_epoch` | function | Returns the discontinuity epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:77` |
| sym-f7b04b27aaa8ae652791 | `discover` | function | Discovers the resources visible to `LocalSourceProvider`. | `src/capture/query.rs:55` |
| sym-c80e661237ec531c7b87 | `dispatch_from` | function | Routes one lineaged audio frame from the named plan output through `PlanEdgeRouter`. | `src/runtime/audio/router.rs:748` |
| sym-7737822215bedc45c9e3 | `display_name` | function | Returns the display name held by `NodeDescriptor`. | `src/graph/node.rs:226` |
| sym-9511b7169319b0166c57 | `disposition` | function | Returns the disposition associated with `SessionCancelResult`. | `src/lib.rs:1097` |
| sym-c5dd8992e341048840df | `disposition` | function | Returns the disposition associated with `SessionStopResult`. | `src/lib.rs:1117` |
| sym-91a5f8f773d352edab66 | `documentation` | function | Returns the documentation held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:218` |
| sym-4b3bc0edf728e8a49312 | `documentation` | function | Returns the documentation held by `ConnectorCapability`. | `src/connector/manifest.rs:34` |
| sym-fdf4039f27c61720c6a2 | `documentation` | function | Returns the documentation held by `ConnectorRequirement`. | `src/connector/manifest.rs:69` |
| sym-28a178cc9d8d140d32f1 | `drift_ppm` | function | Returns the drift ppm associated with `ClockDriftEstimator`. | `src/timing/clock_drift.rs:59` |
| sym-7a9a3832c93c1ee7b3f2 | `drop` | function | Releases resources owned by `MacosInputSource`. | `src/capture/platform/macos/input.rs:251` |
| sym-c4021746d79020bddaed | `drop` | function | Releases resources owned by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:299` |
| sym-7e10b6736a3774e1a045 | `drop` | function | Releases resources owned by `ConnectorSecret`. | `src/connector/configuration.rs:49` |
| sym-4d610cca23cad69c4723 | `drop` | function | Releases resources owned by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:195` |
| sym-88cc3ed40c8d28013862 | `drop` | function | Releases resources owned by `AudioBufferHandle`. | `src/frame/pool.rs:265` |
| sym-ec8f2db8bb6488a7e187 | `drop` | function | Releases resources owned by `SharedAudioBufferHandle`. | `src/frame/pool.rs:322` |
| sym-7d4e6f0d540eb10ca70a | `drop` | function | Releases resources owned by `MultistemRecording`. | `src/recording/writer.rs:355` |
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
| sym-4bfaa030ceda41db736b | `drop_rate_pct` | function | Returns the drop rate pct held by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:171` |
| sym-4295ba8996165d5da0d8 | `duration_ns` | function | Returns the duration nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:74` |
| sym-cf1f8ff43a349e09aa72 | `duration_ns` | function | Returns the duration nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:83` |
| sym-60d8cf973db65857a614 | `edge_buffer` | function | Returns the edge buffer associated with `MemoryPlan`. | `src/graph/plan.rs:71` |
| sym-10fb748905467401f2f8 | `edge_contract` | function | Returns the edge contract held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:52` |
| sym-40b0c8b332b971a5a10f | `edge_contract` | function | Returns the edge contract held by `EndpointPortInput`. | `src/endpoint/contract.rs:215` |
| sym-205f20aeeb83d05bf2fb | `edge_contract` | function | Returns the edge contract held by `PortPrepareContext`. | `src/graph/node.rs:357` |
| sym-d19f73749fda9ced227b | `edge_count` | function | Returns the edge count held by `GraphIr`. | `src/graph/ir.rs:43` |
| sym-609b3b1ad4daabfeee08 | `edge_count` | function | Returns the edge count held by `GraphSpec`. | `src/graph/spec.rs:69` |
| sym-a1b37e1c7532d4731a9a | `edge_count` | function | Returns the edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:52` |
| sym-72fa5deb567e0fd70a5e | `edge_id` | function | Returns the edge identifier held by `PortPrepareContext`. | `src/graph/node.rs:337` |
| sym-0a3df42f064ce049f1df | `edge_id` | function | Returns the edge identifier held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:508` |
| sym-84dc2a5a1d2002f1b94c | `encode` | function | Encodes input through `ConnectorConfigurationRecord`. | `src/connector/transport.rs:61` |
| sym-2519efc461a4e280fd1b | `encode` | function | Encodes input through `ConnectorAudioRecord`. | `src/connector/transport.rs:347` |
| sym-2e3dadd8ca135c9e03ab | `encode` | function | Encodes input through `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:86` |
| sym-bd08dd5e0b763265e78c | `encode_into` | function | Encode an interleaved PCM slice into `out`. | `src/codec/encoder.rs:235` |
| sym-e5bb12d7a197ff37dba3 | `encoded_audio` | function | Convenience constructor for encoded audio ports. | `src/graph/signal/spec.rs:274` |
| sym-b0327767e2217655fd1e | `endpoint` | function | Declares an endpoint on `Session` and returns its Session-scoped handle. | `src/lib.rs:482` |
| sym-7ee11f80ffee60ed1ac8 | `endpoint` | function | Declares an endpoint on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:406` |
| sym-af23e03424aef049afd6 | `endpoint_declarations` | function | Returns the endpoint declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:42` |
| sym-d6fa3d41c3b036a454b2 | `endpoint_failures` | function | Returns the endpoint failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:279` |
| sym-a85ec1d660da506d3778 | `endpoint_finalization_failures_total` | function | Returns the endpoint finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:334` |
| sym-6ca9531125094143aa37 | `endpoint_id` | function | Returns the endpoint identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:28` |
| sym-db2683f156864f46467c | `endpoint_id` | function | Returns the endpoint identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:219` |
| sym-c641b13adaad8a6c02d4 | `endpoint_id` | function | Returns the endpoint identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:129` |
| sym-891dd071941cfeda08be | `endpoint_id` | function | Returns the endpoint identifier held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:150` |
| sym-8d63ce59b3bf9526f8bc | `endpoint_id` | function | Returns the endpoint identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:253` |
| sym-267630205817f07547fe | `endpoint_observations` | function | Returns the endpoint observations held by `ConnectorContext`. | `src/connector/worker/coordination.rs:139` |
| sym-59145b978f237fdd96f9 | `endpoints` | function | Returns the endpoints associated with `SessionSpec`. | `src/session/declaration/spec.rs:339` |
| sym-c43653a3d9e4adb903ba | `engine` | function | Returns the engine associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:158` |
| sym-8a798becca22d42621db | `engine_builder` | function | Borrows the mutable engine builder owned by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:277` |
| sym-a72f5fdbce5729e7a88e | `error` | function | Returns the error associated with `SessionStartFailure`. | `src/session/lifecycle/start_contract.rs:278` |
| sym-1c652c741942e93ae3e7 | `error_class` | function | Returns the error class associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:97` |
| sym-ed7b6e0d91dc8a84ae4d | `event` | function | Convenience constructor for event ports. | `src/graph/signal/spec.rs:284` |
| sym-f3a21262c49f983d794b | `event` | function | Returns the event associated with `SessionSourceFailure`. | `src/session/lifecycle/events.rs:118` |
| sym-0bbd9c20b73138afe8cc | `event_observations` | function | Returns the event observations held by `RunningSession`. | `src/lib.rs:820` |
| sym-f3d0f8aafd878c92c09e | `event_queue` | function | Returns the event queue associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:67` |
| sym-55a49f2fc0acfe356a28 | `execute` | function | Executes its owned operation for `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:65` |
| sym-978803573e99b67ca948 | `execute_from` | function | Executes from for `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:149` |
| sym-7205f6787d2ee1b7b007 | `execution` | function | Returns the execution held by `NodeDescriptor`. | `src/graph/node.rs:238` |
| sym-f8fa19026eb6310fea3d | `execution` | function | Returns the execution held by `SourceManifest`. | `src/session/extensions/source.rs:174` |
| sym-dcc0672d33b86f0c40dd | `execution_partition` | function | Returns the execution partition associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:58` |
| sym-fc869d5bdd95e2de6ca5 | `expose_secret` | function | Exposes the secret to the owning connector during setup or worker use. | `src/connector/configuration.rs:37` |
| sym-21b76e2d1246ca5b5daa | `external_source` | function | Returns the external source held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:87` |
| sym-abb9da13f404a699b9e5 | `external_source_count` | function | Returns the external source count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:83` |
| sym-eba44dfab1bc80191141 | `external_source_declarations` | function | Returns the external source declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:37` |
| sym-bca70b827839e0d661ae | `external_source_metrics` | function | Returns one observation handle per Session-owned external source. | `src/lib.rs:837` |
| sym-e674d51e49a4a20a99ed | `external_source_metrics` | function | Returns the external source metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:209` |
| sym-e6c0ce7d51dfcbaf96e4 | `failure` | function | Returns the failure held by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:55` |
| sym-1a0f975ff6202dab6437 | `failure` | function | Returns the failure held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:216` |
| sym-0543e802fc11f7a0a88f | `failure` | function | Returns the failure held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:158` |
| sym-25581e9181e17ff4fd33 | `failure` | function | Returns the failure held by `SessionRollbackFailure`. | `src/session/lifecycle/events.rs:179` |
| sym-9de3fdac7f06790f4286 | `failure` | function | Returns the failure held by `SessionFinalizationFailure`. | `src/session/lifecycle/events.rs:203` |
| sym-55a45397d674b3703375 | `failure_codes` | function | Returns the failure codes associated with `SessionStopResult`. | `src/error_code.rs:26` |
| sym-b603c77500103cbf4489 | `failure_threshold` | function | Returns the failure threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:55` |
| sym-2145392fb082e13eb627 | `field` | function | Returns the field held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:255` |
| sym-fc4c5f29bbc17d42d8db | `field` | function | Returns the field held by `ConnectorConfigurationError`. | `src/connector/configuration.rs:619` |
| sym-5b8b978acff4848278a9 | `fields` | function | Returns the fields held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:251` |
| sym-fcd9400e7f3ed4c3bf83 | `finalization_failures` | function | Returns the finalization failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:287` |
| sym-68e27ca34c656c2a0992 | `finish` | function | Finishes work owned by `MultistemRecording`. | `src/recording/writer.rs:278` |
| sym-1c3150cdcd10e04cc1d2 | `finish` | function | Finishes work owned by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:359` |
| sym-926b2b8d7c23849e658d | `finish` | function | Finishes work owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:201` |
| sym-9f81457d2803b2181343 | `finish_and_join` | function | Finishes input to `GeneratedAudioBridge`, joins its worker, and returns the terminal result. | `src/runtime/bridge/audio.rs:178` |
| sym-73129516fd04e3f04d43 | `finish_and_join` | function | Finishes input to `AsyncOperatorWorker`, joins its worker, and returns the terminal result. | `src/runtime/signal/operator.rs:933` |
| sym-fb998d5e97976d9cf219 | `fmt` | function | Formats `ConnectorSecret` with the requested formatter. | `src/connector/configuration.rs:43` |
| sym-64b09159091bd3b96f5b | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:35` |
| sym-e029f0d778ba91592b0f | `fmt` | function | Formats `ConnectorErrorCode` with the requested formatter. | `src/connector/error.rs:44` |
| sym-244cebda9945d000bb40 | `fmt` | function | Formats `AudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:273` |
| sym-3ccd0b6574bc3bbb1327 | `fmt` | function | Formats `SharedAudioBufferHandle` with the requested formatter. | `src/frame/pool.rs:328` |
| sym-52a6fbc698f955b2cd96 | `fmt` | function | Formats `NodeTypeId` with the requested formatter. | `src/graph/node.rs:31` |
| sym-3312c0e9fada600ccfb1 | `fmt` | function | Formats `NodeConfig` with the requested formatter. | `src/graph/node.rs:116` |
| sym-f1a93945c2a5d0f69de9 | `fmt` | function | Formats `PlanEdgeFrame` with the requested formatter. | `src/runtime/audio/router.rs:100` |
| sym-e8c488865e3375f09416 | `fmt` | function | Formats `Session` with the requested formatter. | `src/session/declaration/draft.rs:583` |
| sym-8a0ecff6fe54145935ec | `fmt` | function | Formats `OperatorInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:770` |
| sym-cc86641a71921316a15e | `fmt` | function | Formats `OperatorInputHandle` with the requested formatter. | `src/session/declaration/draft.rs:780` |
| sym-ac1519c3256765bfb90c | `fmt` | function | Formats `SourceInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:911` |
| sym-aba9e6a0e1a3572f5246 | `fmt` | function | Formats `SourceOutputHandle` with the requested formatter. | `src/session/declaration/draft.rs:991` |
| sym-a2a1bf9ab2e8e35a2e87 | `fmt` | function | Formats `DerivedStreamHandle` with the requested formatter. | `src/session/declaration/draft.rs:1114` |
| sym-3d2b76bbe3301d06a387 | `fmt` | function | Formats `StemHandle` with the requested formatter. | `src/session/declaration/draft.rs:1125` |
| sym-a40ac12165e8ee6a29e2 | `fmt` | function | Formats `EndpointConfiguration` with the requested formatter. | `src/session/declaration/endpoint.rs:85` |
| sym-3748f2efe3ceb0007216 | `fmt` | function | Formats `AudioInputBuffer` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:52` |
| sym-baabda23a6b760df3426 | `fmt` | function | Formats `AudioInputWriter` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:261` |
| sym-4c0874f66857519560b2 | `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:336` |
| sym-15b6f2306a2edb6ff483 | `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:346` |
| sym-3b26f425ead593ae4178 | `fmt` | function | Formats `AudioInput` with the requested formatter. | `src/session/extensions/audio_input/mod.rs:143` |
| sym-e6b4b022643b4c70158d | `fmt` | function | Formats `PcmSource` with the requested formatter. | `src/session/extensions/audio_input/source.rs:74` |
| sym-c55e56c92bd816bda5b2 | `fmt` | function | Formats `SourceTypeId` with the requested formatter. | `src/session/extensions/source.rs:742` |
| sym-ecc57a6f0e22006bb8ca | `fmt` | function | Formats `SessionStartFailure` with the requested formatter. | `src/session/lifecycle/start_contract.rs:296` |
| sym-0e92e7ecb8481aedc211 | `format` | function | Returns the format associated with `AudioFrame`. | `src/frame/audio.rs:134` |
| sym-2429c171fd87812f1d03 | `format` | function | Returns the format associated with `SharedAudioFrame`. | `src/frame/audio.rs:204` |
| sym-8ea70ec71b283e82bb5e | `frame` | function | Returns the frame held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:186` |
| sym-f56967a038cd84555880 | `frame` | function | Returns the frame held by `LineagedAudioFrame`. | `src/frame/audio.rs:277` |
| sym-e0b65c424a06edc7d26f | `frame` | function | Returns the frame held by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:304` |
| sym-4a326d4b77f2919cddc5 | `frame_capacity_samples` | function | Returns the frame capacity samples held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:340` |
| sym-659bb287ac18a802b571 | `frame_duration` | function | Returns the frame duration associated with `StreamProfile`. | `src/codec/profile.rs:31` |
| sym-1d82ffe4b3fa9063e868 | `frame_ms` | function | Returns the frame milliseconds held by `StreamProfile`. | `src/codec/profile.rs:60` |
| sym-80fa7314bf0e0f7e13d6 | `frame_samples_for_duration_ms` | function | Returns the frame samples for duration milliseconds held by `SampleSpec`. | `src/frame/audio.rs:33` |
| sym-fc7d805f26f6a9dfbf27 | `frame_samples_per_channel` | function | Returns the frame samples per channel associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:67` |
| sym-5eba9d84e7e7e2217921 | `frame_stream_closed` | function | Returns the frame stream closed associated with `CaptureOwner`. | `src/capture/capture_owner.rs:251` |
| sym-e0308dc7a8bcf8b82c01 | `frames_attempted_total` | function | Returns the frames attempted total held by `EdgeObservations`. | `src/runtime/audio/router.rs:160` |
| sym-ccae372acdb4065bcaf9 | `frames_captured` | function | Returns the frames captured associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:55` |
| sym-ed04cf67bf3449078c30 | `frames_emitted` | function | Returns the frames emitted associated with `SystemOutputTelemetry`. | `src/runtime/nodes.rs:58` |
| sym-19c5d4424aefe07bf723 | `frames_mixed` | function | Returns the frames mixed associated with `MixerTelemetry`. | `src/runtime/nodes.rs:254` |
| sym-ac35a0c1639c349ca85b | `frames_pushed` | function | Returns the frames pushed associated with `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:162` |
| sym-bdb1ea7aca5c1c2514a4 | `freeze` | function | Freezes mutable storage owned by `AudioFrame` into its shared immutable form. | `src/frame/audio.rs:150` |
| sym-a13662ae36441b39d89b | `freeze` | function | Freezes mutable storage owned by `LineagedAudioFrame` into its shared immutable form. | `src/frame/audio.rs:289` |
| sym-1b1890c3445f6f82f996 | `freeze` | function | Freezes mutable storage owned by `AudioBufferHandle` into its shared immutable form. | `src/frame/pool.rs:246` |
| sym-9d53c012f65576e10641 | `freeze` | function | Freezes mutable storage owned by `Session` into its shared immutable form. | `src/session/declaration/draft.rs:466` |
| sym-712dd0ed5d758d0259fc | `from` | function | Converts the supplied value into `NodeTypeId`. | `src/graph/node.rs:37` |
| sym-0260a0359d6a61e4abd7 | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:41` |
| sym-6a1a06a410e59afb704f | `from` | function | Converts the supplied value into `SignalId`. | `src/graph/signal/spec.rs:47` |
| sym-57adf6fa9c960a194659 | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:70` |
| sym-d21313b85a5a4f9b112b | `from` | function | Converts the supplied value into `SemanticRole`. | `src/graph/signal/spec.rs:76` |
| sym-63c44e108dc20375eb36 | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:100` |
| sym-01078514b0053e0d48e4 | `from` | function | Converts the supplied value into `SchemaRef`. | `src/graph/signal/spec.rs:106` |
| sym-6caba771539ec7409e9d | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:987` |
| sym-68c83f152d5f649d83b0 | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:996` |
| sym-bc0e5a1fc522158a6926 | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:1010` |
| sym-6a15025eb042f92a1a93 | `from` | function | Converts the supplied value into `SessionStartError`. | `src/lib.rs:1026` |
| sym-08222e995b79ce013f0a | `from` | function | Converts the supplied value into `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:513` |
| sym-0fcf855969ace6f0828f | `from` | function | Converts the supplied value into `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:325` |
| sym-3b0b482d47e67dd82b8d | `from_audio` | function | Creates `SignalEnvelope` from audio. | `src/graph/signal/envelope.rs:27` |
| sym-a80bc336b907a200eb6c | `from_config` | function | Create an encoder from an explicit OpusConfig. | `src/codec/encoder.rs:173` |
| sym-c8e8aac676d5eebd09b3 | `from_frame` | function | Creates `SignalLineage` from frame. | `src/graph/signal/lineage.rs:46` |
| sym-23617123d6583636fd64 | `from_frame` | function | Creates `SignalTiming` from frame. | `src/graph/signal/timing.rs:56` |
| sym-9ff05aa034f6fd7eb466 | `from_index` | function | Creates a stable runtime node identifier for externally assembled plans. | `src/graph/spec.rs:12` |
| sym-6fa3e8d79c890c7717ff | `from_item` | function | Creates `ConnectorAudioRecord` from item. | `src/connector/transport.rs:314` |
| sym-a1d924cbc0478bd1398e | `from_monotonic_timestamp_ns` | function | Creates `SessionTimelineOrigin` from monotonic timestamp nanoseconds. | `src/endpoint/runtime.rs:18` |
| sym-5d13de0df7ad71b668dd | `from_node` | function | Creates `ExecError` from node. | `src/runtime/audio/executor.rs:26` |
| sym-e258bcaf92e5c1d9247d | `from_open_observations` | function | Records platform authorization observations without inferring them from a generic backend result. Callers must pass `NotObservable` when their platform has no authoritative query for the requested capture class. | `src/capture/authorization.rs:76` |
| sym-33b9ee5cdb6059b6e5f9 | `from_resolved` | function | Creates `ConnectorConfigurationRecord` from resolved. | `src/connector/transport.rs:45` |
| sym-37099899d4a6590b672f | `from_source` | function | Creates `EndpointRouteContext` from source. | `src/endpoint/runtime.rs:57` |
| sym-e1053a7046106d7c1ce4 | `from_source_output` | function | Wraps a public external-source output in the same typed Rust façade. Runtime identity remains the output's stable `SignalSpec` and schema. | `src/session/declaration/typed_stream.rs:118` |
| sym-1aa6bb6281fa2a93bcb2 | `from_stem` | function | Creates `EndpointRouteContext` from stem. | `src/endpoint/runtime.rs:50` |
| sym-1079adfc5870e06d1bc3 | `from_stem` | function | Creates `Stream` from stem. | `src/session/declaration/typed_stream.rs:103` |
| sym-4d24e0374ad96fee281d | `generated_audio_ingresses` | function | Returns the generated audio ingresses associated with `SessionSpec`. | `src/session/declaration/spec.rs:335` |
| sym-66709b3ff0592c65224c | `generation` | function | Returns the generation associated with `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:84` |
| sym-abb31d14a22ef594f1e0 | `generation` | function | Returns the generation associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:184` |
| sym-ff7327324294fdd0e490 | `generation` | function | Returns the generation associated with `NativeExtensionRegistration`. | `src/native_extension/mod.rs:54` |
| sym-1e34f2399e99837264ce | `generation` | function | Returns the implementation generation. | `src/session/extensions/source.rs:166` |
| sym-a8f5198e79b286269788 | `get` | function | Returns the value held by `ConnectorConfiguration`. | `src/connector/configuration.rs:134` |
| sym-4d9bc8528c0202d5425f | `get` | function | Returns the value held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:394` |
| sym-6ecc9053f5721cdadb3c | `get` | function | Returns the value held by `ClockDomainId`. | `src/frame/identity.rs:36` |
| sym-6cca7b42b19a7963f00d | `get` | function | Returns the value held by `NodeConfig`. | `src/graph/node.rs:92` |
| sym-4ae7e2e7746961ddcb78 | `get` | function | Returns the value held by `NodeRegistry`. | `src/graph/registry.rs:127` |
| sym-fd78059adf00a80f26ff | `get` | function | Returns the value held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:60` |
| sym-7ec0100b785ff9974c5a | `get` | function | Returns the value held by `ProcessId`. | `src/session/declaration/selector.rs:13` |
| sym-ba96a13281a8ccf9fdfc | `get` | function | Returns the value held by `SourceConfiguration`. | `src/session/extensions/source.rs:100` |
| sym-13fa24385bb0481aa0e4 | `get_f32` | function | Returns the get f32 associated with `NodeConfig`. | `src/graph/node.rs:100` |
| sym-ffb635f4663ee56ddfa8 | `get_u32` | function | Returns the get u32 associated with `NodeConfig`. | `src/graph/node.rs:104` |
| sym-11060a30b8e68b52a0a1 | `group_id` | function | Returns the group identifier held by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:70` |
| sym-91ef7083bf48f44efe44 | `handle` | function | Returns the handle associated with `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:197` |
| sym-8540ee158dc2da2131fc | `health` | function | Returns the health held by `ConnectorServiceStatus`. | `src/connector/status.rs:46` |
| sym-8e222ad1c1c59e1ccb04 | `health_reason_code` | function | Returns the health reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:58` |
| sym-e605d954cd2a4de7a570 | `hz` | function | Returns the hz associated with `OpusSampleRate`. | `src/codec/encoder.rs:49` |
| sym-59a056aeb32ea6af5817 | `id` | function | Returns the id held by `ConnectorCapability`. | `src/connector/manifest.rs:30` |
| sym-d1a311cfb7fb6b0beb5a | `id` | function | Returns the id held by `ConnectorRequirement`. | `src/connector/manifest.rs:61` |
| sym-45d23c64ab95d9d99186 | `id` | function | Returns the id held by `NodeHandle`. | `src/graph/dsl.rs:15` |
| sym-46e559b1c0bd1721a52c | `id` | function | Returns the id held by `ResolvedNode`. | `src/graph/ir.rs:16` |
| sym-ffa270cde2c84187da54 | `id` | function | Returns the id held by `Session`. | `src/lib.rs:378` |
| sym-fabf53aef85f6d57579d | `id` | function | Returns the id held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:42` |
| sym-176f01e820896ed91ecd | `id` | function | Returns the id held by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:253` |
| sym-cf85afd4d1673c928bfb | `id` | function | Returns the id held by `Session`. | `src/session/declaration/draft.rs:343` |
| sym-bde0a2263b883d2a02a8 | `id` | function | Returns the id held by `EndpointHandle`. | `src/session/declaration/draft.rs:603` |
| sym-38ad6acc195ce4b793a7 | `id` | function | Returns the id held by `StemHandle`. | `src/session/declaration/draft.rs:795` |
| sym-4a56d9cedda87c1a390e | `id` | function | Returns the id held by `DeviceSelector`. | `src/session/declaration/selector.rs:117` |
| sym-d424573816adf7a448a6 | `id` | function | Returns the id held by `StemSpec`. | `src/session/declaration/spec.rs:151` |
| sym-3caab190b0a5cb98ec46 | `id` | function | Returns the id held by `EndpointSpec`. | `src/session/declaration/spec.rs:171` |
| sym-e0678a06b7826d0eddf6 | `id` | function | Returns the id held by `ConnectionSpec`. | `src/session/declaration/spec.rs:259` |
| sym-26e629a3439fbbf76585 | `identity_strength` | function | Returns the identity strength held by `CaptureSource`. | `src/capture/identity.rs:94` |
| sym-0ef1efd3620a46e54fa2 | `implementation_generation` | function | Monotonic implementation generation for this manifest revision. | `src/session/extensions/source.rs:158` |
| sym-8e17aa73649cd14f1cc5 | `in_` | function | Selects a named input port from `NodeHandle`. | `src/graph/dsl.rs:24` |
| sym-bedba89a88c30341a367 | `index` | function | Returns the index held by `AudioBufferHandle`. | `src/frame/pool.rs:211` |
| sym-5a6667fe9280b73e0c0f | `index` | function | Returns the index held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:296` |
| sym-040f459afdad39ad3886 | `index` | function | Returns the index held by `NodeId`. | `src/graph/spec.rs:16` |
| sym-a8af8d2eab0172a0d7a6 | `index` | function | Returns the index held by `EdgeId`. | `src/graph/spec.rs:25` |
| sym-3a94fa1fdb1571c006dd | `ingress_rejected_total` | function | Returns the ingress rejected total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:364` |
| sym-126179a82c1dac44596e | `input` | function | Returns the input held by `ConnectorItem`. | `src/connector/worker/driver.rs:74` |
| sym-144e506a1cf7c0aa078b | `input` | function | Returns the input held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:729` |
| sym-b5532cd7a4e1647076b4 | `input_attempted_total` | function | Returns the input attempted total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:413` |
| sym-0adaa4f1d995ef49edde | `input_delivered_total` | function | Returns the input delivered total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:421` |
| sym-30e6822ffaff9ba57387 | `input_dropped_total` | function | Returns the input dropped total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:425` |
| sym-c97cfd4baf922e230a94 | `input_edge` | function | Returns the input edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:192` |
| sym-7af29d2a0e0d8b4ae509 | `input_edge` | function | Returns the input edge associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:153` |
| sym-ed86c8da9240a3754da0 | `input_edge` | function | Returns the input edge associated with `EndpointSpec`. | `src/session/declaration/spec.rs:191` |
| sym-7172910d40925f998ab6 | `input_enqueued_total` | function | Returns the input enqueued total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:417` |
| sym-ed19944ab65c48c7ca45 | `input_mut` | function | Returns the input mut associated with `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:917` |
| sym-d504de821ea6433aaf3f | `input_port` | function | Returns the input port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:75` |
| sym-5aeb7cdb9acbef2e78f0 | `input_port` | function | Returns the input port held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:398` |
| sym-2498228c5bbd13648656 | `input_ports` | function | Returns the input ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:224` |
| sym-a62e16f22523087f53c7 | `input_queue_capacity_frames` | function | Returns the input queue capacity frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:401` |
| sym-50c8b0a61d9b37bf4574 | `input_queue_depth_frames` | function | Returns the input queue depth frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:405` |
| sym-5e399a46305d990f070f | `input_queue_peak_frames` | function | Returns the input queue peak frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:409` |
| sym-700a1293c724cb18159c | `input_spec` | function | Returns the input spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:83` |
| sym-60352d699b6436aa0952 | `inputs` | function | Returns the inputs associated with `NodeDescriptor`. | `src/graph/node.rs:230` |
| sym-de549fe8cfa5d7f7b830 | `inputs` | function | Returns the inputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:62` |
| sym-3625c2cf36ae58dfe77c | `insert` | function | Inserts a typed configuration value into `ConnectorConfiguration`. | `src/connector/configuration.rs:126` |
| sym-1d1c0fc50a816563d4e9 | `insert` | function | Adds declared source configuration. | `src/session/extensions/source.rs:96` |
| sym-af589dfe78ceae0eac52 | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:725` |
| sym-3a7568856b61a68fec16 | `instance_id` | function | Returns the instance identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:858` |
| sym-97b7f04bf7c77ff2c93f | `instance_id` | function | Returns the instance identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:76` |
| sym-78a0a0b71d22895f295e | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:245` |
| sym-571a00a0e0dad50368ff | `instantiate` | function | Instantiates the runtime node described by `PassthroughFactory`. | `src/graph/builtins.rs:86` |
| sym-9fdce2b6e76eecf4e15f | `instantiate` | function | Instantiates the runtime node described by `GainFactory`. | `src/graph/builtins.rs:135` |
| sym-df4c461fa6eaa9f738b2 | `instantiate` | function | Instantiates the runtime node described by `MonoMixFactory`. | `src/graph/builtins.rs:185` |
| sym-55f54ca7b33bd5a63b62 | `instantiate` | function | Instantiates the runtime node described by `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:103` |
| sym-90fcec8e9310c96f2ab4 | `instantiate` | function | Instantiates the runtime node described by `BridgeSinkFactory`. | `src/runtime/nodes.rs:207` |
| sym-79881e8872c141336a40 | `integral_error_ns` | function | Returns the integral error nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:42` |
| sym-4dda4bf993c35495f761 | `integral_ns` | function | Returns the integral nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:46` |
| sym-4e8e191c150a473e6652 | `into_callback` | function | Converts `CapturedFrameSender` into callback. | `src/capture/frame_stream.rs:132` |
| sym-478867138b32cee52cc9 | `into_configuration` | function | Converts `ConnectorConfigurationRecord` into configuration. | `src/connector/transport.rs:57` |
| sym-1adbe724a6a8d4fff4de | `into_endpoint_failure` | function | Converts `ConnectorError` into endpoint failure. | `src/connector/error.rs:125` |
| sym-03cf916de4a45a48b3e9 | `into_error` | function | Converts `SessionStartFailure` into error. | `src/session/lifecycle/start_contract.rs:290` |
| sym-db4321629969fd8fe29f | `into_parts` | function | Consumes `EndpointPortInput` and returns its component values. | `src/endpoint/contract.rs:227` |
| sym-10764124105b472e68ff | `into_parts` | function | Consumes `LineagedAudioFrame` and returns its component values. | `src/frame/audio.rs:285` |
| sym-b79469e92c616d61fc7a | `into_parts` | function | Consumes `PcmSource` and returns its component values. | `src/session/extensions/audio_input/source.rs:68` |
| sym-6d6b21a9f385a15c2b4d | `into_payload` | function | Converts `SignalEnvelope` into payload. | `src/graph/signal/envelope.rs:86` |
| sym-49c367c52869a94bb6e6 | `into_pcm_source` | function | Converts the convenience façade into explicit source, output, and producer ownership. | `src/session/extensions/audio_input/mod.rs:137` |
| sym-fed3cd219cf45a7c7ee6 | `into_plan_edge_receiver` | function | Converts `EndpointAudioReceiver` into plan edge receiver. | `src/endpoint/contract.rs:82` |
| sym-7f0fcfd19bd6a964571b | `into_rejected` | function | Converts `SignalEdgeSendError` into rejected. | `src/runtime/signal/edge.rs:123` |
| sym-f67b760d2d44af55cc4f | `into_rejected` | function | Converts `AudioInputWriteError` into rejected. | `src/session/extensions/audio_input/buffer.rs:319` |
| sym-8423f122d3f100838ddb | `into_spec` | function | Converts `Pipeline` into spec. | `src/graph/dsl.rs:90` |
| sym-a3a6b6c0accc419fd555 | `into_start_failure` | function | Converts `SessionEngineStartError` into start failure. | `src/session/lifecycle/engine.rs:336` |
| sym-c2cd40085283728ed9b7 | `invalid_total` | function | Returns the invalid total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:352` |
| sym-348e8a4240b8f282b928 | `is_abandoned` | function | Returns whether abandoned applies to `EndpointAudioReceiver`. | `src/endpoint/contract.rs:92` |
| sym-ea2545c4423c5ea821a4 | `is_abandoned` | function | Returns whether abandoned applies to `EndpointSignalReceiver`. | `src/endpoint/contract.rs:140` |
| sym-110de2cce94828ffaefe | `is_abandoned` | function | Returns whether abandoned applies to `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:549` |
| sym-b419412d315c2cb0fede | `is_abort_requested` | function | Returns whether abort requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:36` |
| sym-fd7c2353ef0ba142e77a | `is_audio` | function | Returns `true` for classes that carry real-time audio on the hot path. | `src/graph/signal/spec.rs:180` |
| sym-9ffca2dbd9c8863ff15b | `is_cancelled` | function | Returns whether cancelled applies to `SessionStartError`. | `src/lib.rs:981` |
| sym-681c06335d54e88a43ce | `is_cancelled` | function | Returns whether cancelled applies to `SourceCancellation`. | `src/session/extensions/source.rs:255` |
| sym-8cedd9b3063f0be9d99e | `is_closed` | function | Returns whether closed applies to `CapturedFrameStream`. | `src/capture/frame_stream.rs:170` |
| sym-6957b53fa20c046cd164 | `is_compatible_with` | function | Returns whether compatible with applies to `ChannelLayout`. | `src/graph/ports.rs:42` |
| sym-4a48d42a2bb469618c1a | `is_compatible_with` | function | Returns whether compatible with applies to `AudioCaps`. | `src/graph/ports.rs:56` |
| sym-b66c66799f3942868330 | `is_compatible_with` | function | Returns whether compatible with applies to `MediaCaps`. | `src/graph/ports.rs:110` |
| sym-60d37dec7979b459d3fd | `is_compatible_with` | function | Returns `true` if two signal classes are compatible for edge wiring. | `src/graph/signal/spec.rs:188` |
| sym-6d2f696c2f0bcf134b2a | `is_compatible_with` | function | Returns `true` if this spec can connect to `other` on an edge. | `src/graph/signal/spec.rs:324` |
| sym-2ebbe7c81fcbaaed8838 | `is_complete` | function | Returns whether complete applies to `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:80` |
| sym-3a89509bef9042425729 | `is_empty` | function | Returns whether `ConnectorConfiguration` contains no values. | `src/connector/configuration.rs:146` |
| sym-cdc39e098fa8525e68c5 | `is_empty` | function | Returns whether `PolledAudioBatchLease` contains no values. | `src/endpoint/polled_audio_driver.rs:182` |
| sym-aca5edc5ef1101ccaa3d | `is_empty` | function | Returns whether `AudioBufferHandle` contains no values. | `src/frame/pool.rs:208` |
| sym-8e8655689ed1798e3144 | `is_empty` | function | Returns whether `SharedAudioBufferHandle` contains no values. | `src/frame/pool.rs:292` |
| sym-ced2c36e825774462916 | `is_empty` | function | Returns whether `NodeRegistry` contains no values. | `src/graph/registry.rs:174` |
| sym-4ff0ee20cadb8ece9974 | `is_in_use` | function | Returns whether in use applies to `AudioBufferPool`. | `src/frame/pool.rs:98` |
| sym-c40844ae30b605f327e7 | `is_open` | function | Returns whether open applies to `EndpointStartGate`. | `src/endpoint/runtime.rs:376` |
| sym-c4242678455f12a2a79b | `is_portable` | function | Reports whether this value is a portable implementation contract ID. | `src/graph/operator.rs:31` |
| sym-53f9b768dede3d88ded9 | `is_portable` | function | Reports whether this custom signal ID is portable across packages, languages, and processes. | `src/graph/signal/spec.rs:35` |
| sym-3067364c756e1dbf3c95 | `is_realtime` | function | Returns whether realtime applies to `ClockDomain`. | `src/graph/ports.rs:259` |
| sym-6b84dae9ee40e790945c | `is_requested` | function | Returns whether requested applies to `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:104` |
| sym-aac50ad341fd43dc62ea | `is_requested` | function | Returns whether requested applies to `SessionStartCancellation`. | `src/session/lifecycle/start_contract.rs:107` |
| sym-25c1de9f07a865e0fe97 | `is_sensitive` | function | Returns whether sensitive applies to `NodeConfig`. | `src/graph/node.rs:96` |
| sym-3d30799c0f256dbe4678 | `is_sensitive` | function | Returns whether sensitive applies to `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:64` |
| sym-0f4929d08758870a7625 | `is_stateful` | function | Returns whether stateful applies to `NodeDescriptor`. | `src/graph/node.rs:246` |
| sym-a456b018e1b43fa68057 | `is_stereo` | function | Returns whether stereo applies to `StreamProfile`. | `src/codec/profile.rs:69` |
| sym-0ced882e7a0707ac4cf2 | `is_stop_requested` | function | Returns whether stop requested applies to `ConnectorContext`. | `src/connector/worker/coordination.rs:28` |
| sym-3c60241f8e5f1ab0b930 | `is_success` | function | Returns whether success applies to `SessionCancelResult`. | `src/lib.rs:1105` |
| sym-1fd789aa9f8a1cf7741f | `is_success` | function | Returns whether success applies to `SessionStopResult`. | `src/lib.rs:1125` |
| sym-26bd8d847c4f74770432 | `is_success` | function | Returns whether success applies to `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:320` |
| sym-62cadbfa63ba01f06a9c | `is_terminal` | function | Returns whether terminal applies to `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:86` |
| sym-ba34c52c37d7de4461cc | `is_valid_for` | function | Returns `true` if this contract is compatible with the given partition. | `src/graph/partition.rs:107` |
| sym-95cdf3ba054aa12d8cdd | `is_well_formed` | function | Reports whether this value follows the portable node-type syntax. | `src/graph/node.rs:25` |
| sym-b513b191cf132d40afa6 | `iter` | function | Iterates over the values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:138` |
| sym-f8a82a36670eedb24825 | `iter` | function | Iterates over the values held by `ResolvedConnectorConfiguration`. | `src/connector/configuration.rs:398` |
| sym-1b878040951ec109e51e | `iter` | function | Iterates over the values held by `NodeConfig`. | `src/graph/node.rs:108` |
| sym-87f7b411cebdb4212f3b | `iter` | function | Iterates over the values held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:68` |
| sym-cc9676d435b55045859f | `iter` | function | Iterates over the values held by `SourceConfiguration`. | `src/session/extensions/source.rs:104` |
| sym-43b9a3ee7a683e657bf1 | `jitter_budget_ms` | function | Returns the jitter budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:337` |
| sym-30958807622184a2ef06 | `join` | function | Joins its owned operation for `SourceRuntime`. | `src/session/extensions/source.rs:583` |
| sym-7de8e0a776eff6f4e656 | `joined` | function | Returns whether joined is true for `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:376` |
| sym-de42a0012afbd21fb026 | `kind` | function | Returns the kind represented by `ConnectorConfigurationValue`. | `src/connector/configuration.rs:77` |
| sym-6b625be43fd3c89865e4 | `kind` | function | Returns the kind represented by `MediaCaps`. | `src/graph/ports.rs:97` |
| sym-0a2f1d7e88362d622f09 | `kind` | function | Returns the kind represented by `SessionStartError`. | `src/lib.rs:965` |
| sym-f62a0b3d0c053e53cfe7 | `kind` | function | Returns the kind represented by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:46` |
| sym-f177dc226e20122ed2bd | `kind` | function | Returns the kind represented by `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:315` |
| sym-97d70f67e4cda1a66bad | `kind` | function | Returns the kind represented by `SessionEvent`. | `src/session/lifecycle/events.rs:322` |
| sym-b54e214adc9db1ca7010 | `lane_underruns` | function | Returns the lane underruns associated with `MixerTelemetry`. | `src/runtime/nodes.rs:257` |
| sym-e748db1c1f6bdf4274bb | `last_correction_ns` | function | Returns the last correction nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:39` |
| sym-f1c643a525e98a619677 | `last_offset_ns` | function | Returns the last offset nanoseconds held by `ClockCorrectionController`. | `src/timing/clock_correction.rs:36` |
| sym-c170db02f7c98809c93c | `last_transition_elapsed_ns` | function | Returns the last transition elapsed nanoseconds held by `ConnectorServiceStatus`. | `src/connector/status.rs:70` |
| sym-38f0dccf4d9126f4c381 | `latency_budget_ms` | function | Returns the latency budget milliseconds held by `EdgeContract`. | `src/graph/ports.rs:333` |
| sym-6a4f6ad61d1cb38db315 | `len` | function | Returns the number of values held by `ConnectorConfiguration`. | `src/connector/configuration.rs:142` |
| sym-9f927e55ff48c6179c6b | `len` | function | Returns the number of values held by `PolledAudioBatchLease`. | `src/endpoint/polled_audio_driver.rs:178` |
| sym-4c98f24a698c9d467f1f | `len` | function | Returns the number of values held by `AudioBufferHandle`. | `src/frame/pool.rs:205` |
| sym-4b32d1471be01559d2be | `len` | function | Returns the number of values held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:288` |
| sym-774089c61834270d3508 | `len` | function | Returns the number of values held by `NodeRegistry`. | `src/graph/registry.rs:169` |
| sym-83c3cf6e8b3e4e8859a4 | `lineage` | function | Returns the frame lineage carried by `EndpointAudioFrame`. | `src/endpoint/contract.rs:59` |
| sym-f9bbeca469782ea9a7c2 | `lineage` | function | Returns the frame lineage carried by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:215` |
| sym-6f85ec7dff6aef7ebe86 | `lineage` | function | Returns the frame lineage carried by `LineagedAudioFrame`. | `src/frame/audio.rs:281` |
| sym-f74c3ed1333764b92d58 | `lineage` | function | Returns the frame lineage carried by `SharedLineagedAudioFrame`. | `src/frame/audio.rs:308` |
| sym-1dd2284550c9c2085802 | `lineage` | function | Returns the frame lineage carried by `SignalEnvelope`. | `src/graph/signal/envelope.rs:78` |
| sym-d1214ab49fdde708bfa8 | `lineage` | function | Returns the frame lineage carried by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:91` |
| sym-98d87216e9c161fd08bd | `lineage_failures_total` | function | Returns the lineage failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:350` |
| sym-5c690a8e2606d07e7136 | `load_native_extension_library` | function | Loads one trusted native dynamic library from an exact absolute path and imports its supported non-realtime source, operator, and endpoint registrations into this Session as one validated set. | `src/session/extensions/native_library.rs:29` |
| sym-f7144091d8bfca40c650 | `loss` | function | Returns the loss associated with `EdgeContract`. | `src/graph/ports.rs:349` |
| sym-e01c5600329c344bf7e6 | `major` | function | Returns the major associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:51` |
| sym-d0ce46423a79b4284e2b | `manifest` | function | Returns the manifest held by `Connector`. | `src/connector/mod.rs:119` |
| sym-17ad3cce17ae0a5d4250 | `manifest` | function | Returns the manifest held by `RegisteredConnector`. | `src/connector/mod.rs:136` |
| sym-b97c61fed1660410aef8 | `manifest` | function | Returns the manifest held by `SourceRegistry`. | `src/session/extensions/source.rs:291` |
| sym-d467250e43d30e9cf376 | `manifest_revision` | function | Returns the manifest revision held by `ConnectorManifest`. | `src/connector/manifest.rs:128` |
| sym-e68e640f091d011d8061 | `map_payload` | function | Transforms the payload held by `SignalEnvelope` while preserving envelope metadata. | `src/graph/signal/envelope.rs:45` |
| sym-8b50e50d6f02b38d6437 | `mark_discontinuity` | function | Marks the next value from `EndpointAudioReceiver` as discontinuous. | `src/endpoint/contract.rs:96` |
| sym-fe0871cf3d73d9d0e989 | `mark_discontinuity` | function | Marks the next value from `PlanEdgeReceiver` as discontinuous. | `src/runtime/audio/router.rs:630` |
| sym-8ff4ff54399e007d51f7 | `mark_discontinuity` | function | Marks the next value from `AudioInputBuffer` as discontinuous. | `src/session/extensions/audio_input/buffer.rs:46` |
| sym-c3a37e4901efc1525c4b | `mark_worker_failure` | function | Returns the mark worker failure held by `EndpointAudioReceiver`. | `src/endpoint/contract.rs:100` |
| sym-de48a84bcae85e82ba79 | `mark_worker_failure` | function | Returns the mark worker failure held by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:639` |
| sym-5e0b2d5ceaf474c219a7 | `matches` | function | Returns whether an input satisfies `SourceQuery`. | `src/capture/query.rs:22` |
| sym-978e99c20c1d0e860bd3 | `max_frame_bytes` | function | Returns the max frame bytes held by `SidecarProtocolLimits`. | `src/runtime/lifecycle/sidecar_protocol.rs:62` |
| sym-c6fa198f9907fdec7957 | `max_payload_bytes` | function | Returns the max payload bytes held by `EdgeContract`. | `src/graph/ports.rs:361` |
| sym-d080cac01208cf388fe8 | `maximum_buffered_audio_bytes` | function | Returns the maximum buffered audio bytes held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:344` |
| sym-df8acba32de33eb8a03f | `media` | function | Returns the media held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:48` |
| sym-a111f35155807eec9d8b | `media` | function | Returns the media held by `EndpointPortInput`. | `src/endpoint/contract.rs:211` |
| sym-59c84b5114d9ccab7207 | `media` | function | Returns the media held by `PortPrepareContext`. | `src/graph/node.rs:353` |
| sym-4b2290f3212fc6e167e0 | `media` | function | Returns the media held by `PortSpec`. | `src/graph/ports.rs:225` |
| sym-5ed77a87b9fecdd34b49 | `media` | function | Returns the media held by `EdgeContract`. | `src/graph/ports.rs:325` |
| sym-e7176215446668955472 | `message` | function | Returns the diagnostic message reported by `ConnectorConfigurationError`. | `src/connector/configuration.rs:623` |
| sym-778a0382ff61f0b25a61 | `message` | function | Returns the diagnostic message reported by `ConnectorError`. | `src/connector/error.rs:121` |
| sym-947e217bd6ae8fa4abb0 | `message` | function | Returns the diagnostic message reported by `EndpointFailure`. | `src/endpoint/runtime.rs:208` |
| sym-721db3dc79b2e5a79426 | `message` | function | Returns the diagnostic message reported by `SessionStartError`. | `src/lib.rs:961` |
| sym-dcfcca963568ac75044f | `message` | function | Returns the diagnostic message reported by `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:135` |
| sym-28ab778ccae2b9600465 | `metadata` | function | Returns the metadata held by `ConnectorAudioRecord`. | `src/connector/transport.rs:339` |
| sym-35144345756d03ecc4c7 | `metric_id` | function | Returns the metric identifier held by `RuntimePlan`. | `src/graph/plan.rs:145` |
| sym-58e27492a8bef2ad0845 | `metrics` | function | Convenience constructor for metrics ports. | `src/graph/signal/spec.rs:289` |
| sym-144f6292bc11b3dda96f | `metrics_snapshot` | function | Returns the metrics snapshot associated with `RunningSession`. | `src/lib.rs:824` |
| sym-5def61eddb0d041de241 | `metrics_snapshot` | function | Returns the metrics snapshot associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:117` |
| sym-04805bf8250099b87a8e | `microphone` | function | Creates `Source` for the selected microphone device. | `src/session/declaration/selector.rs:144` |
| sym-448810bf95d96f7b4ce5 | `microphone_default` | function | Creates `Source` for the host default microphone. | `src/session/declaration/selector.rs:148` |
| sym-2711e2b4707f92d51d9c | `minor` | function | Returns the minor associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:56` |
| sym-de5267f80dc8a9bb728c | `monotonic_timestamp_ns` | function | Returns the monotonic timestamp nanoseconds held by `SessionTimelineOrigin`. | `src/endpoint/runtime.rs:24` |
| sym-141e9c5891dcd0c4ff69 | `multiplicity` | function | Returns the multiplicity associated with `PortSpec`. | `src/graph/ports.rs:229` |
| sym-26594b415b62661d0b08 | `name` | function | Returns the name associated with `ConnectorConfigurationField`. | `src/connector/configuration.rs:206` |
| sym-6bbf728d87e74976df2a | `name` | function | Returns the name associated with `PortSpec`. | `src/graph/ports.rs:213` |
| sym-971a6c9d445eb7605d25 | `name` | function | Returns the name associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:63` |
| sym-5e37a61d22c00fae836f | `native` | function | Returns the native associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:40` |
| sym-13840ec974868b9fa41d | `native` | function | Creates the production host builder with the platform's native capture backend, leaving endpoint registration open to the owning application. | `src/session/lifecycle/host.rs:223` |
| sym-56dfaf3205b2a6004563 | `native_with_multistem_recording` | function | Builds the native Session host with one canonical multistem recorder. | `src/session/lifecycle/host.rs:48` |
| sym-6e55e01faf43e8a23cea | `needs_bridge_to` | function | Returns `true` if crossing from `self` to `other` requires a compiler-inserted Bridge. | `src/graph/partition.rs:71` |
| sym-336637cc5b0aa7486ec8 | `negotiate` | function | Negotiates the compatible media capabilities shared by `MediaCaps` and its peer. | `src/graph/ports.rs:124` |
| sym-3db5f4df5763985a9d69 | `new` | function | Creates a new `PksSessionStatus`. | `src/abi/session/abi.rs:69` |
| sym-c5d5c62df51e1409b550 | `new` | function | Creates a new `CapturePermissionLifecycle`. | `src/capture/authorization.rs:189` |
| sym-d4ae2b6e62b4da1a9cbc | `new` | function | Creates a new `CaptureLineageSeed`. | `src/capture/capture_owner.rs:31` |
| sym-607b63179a58247f45c4 | `new` | function | Creates a new `StableSourceId`. | `src/capture/identity.rs:33` |
| sym-63ee209fc6b8809d2745 | `new` | function | Creates a new `CaptureSampleTimeline`. | `src/capture/timeline.rs:52` |
| sym-c570bc1e1b1cb54d5277 | `new` | function | Mono decoder (48 kHz). Back-compatible default for the existing pipeline. | `src/codec/decoder.rs:39` |
| sym-69d564ec2595d96c9068 | `new` | function | Create a new encoder with default config (48 kHz, mono, Voip, 20 ms). | `src/codec/encoder.rs:168` |
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
| sym-0cf58a144f9a85982582 | `new` | function | Creates a new `EndpointGroupId`. | `src/endpoint/identity.rs:12` |
| sym-a1be858558e3a28f2626 | `new` | function | Creates a new `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:22` |
| sym-ea665e1f03f6777240b5 | `new` | function | Creates a new `EndpointPrepareContext`. | `src/endpoint/runtime.rs:108` |
| sym-39ed52964be43aa64e82 | `new` | function | Creates a new `EndpointFailure`. | `src/endpoint/runtime.rs:182` |
| sym-dd69a151b1b2a2989687 | `new` | function | Creates a new `SampleSpec`. | `src/frame/audio.rs:25` |
| sym-77b3385d98c4250014df | `new` | function | Creates a new `LineagedAudioFrame`. | `src/frame/audio.rs:272` |
| sym-9602da8c0f25fa84fac5 | `new` | function | Creates a new `ClockDomainId`. | `src/frame/identity.rs:32` |
| sym-44f3c327815dfe7bb5e6 | `new` | function | Creates a new `AudioBufferPool`. | `src/frame/pool.rs:39` |
| sym-4800fa93f446b821656f | `new` | function | Creates a new `RuntimePlanner`. | `src/graph/compile/plan.rs:14` |
| sym-8e635350ed4e8cc94836 | `new` | function | Creates a new `Compiler`. | `src/graph/compile/resolve.rs:449` |
| sym-501b7f4f5836ab935975 | `new` | function | Creates a new `Pipeline`. | `src/graph/dsl.rs:40` |
| sym-632f4284e84cd93b03eb | `new` | function | Creates a new `NodeConfig`. | `src/graph/node.rs:62` |
| sym-baccec434c0630b5b128 | `new` | function | Creates a new `NodeDescriptor`. | `src/graph/node.rs:176` |
| sym-cb61f3c847b318c0fb5a | `new` | function | Creates a new `PrepareContext`. | `src/graph/node.rs:271` |
| sym-84744a4dec72dad90121 | `new` | function | Creates a new `PortPrepareContext`. | `src/graph/node.rs:293` |
| sym-d89cdb51ac7756fa856d | `new` | function | Creates a new `OperatorId`. | `src/graph/operator.rs:19` |
| sym-a20e080faf9223fb614b | `new` | function | Creates a new `PortSpec`. | `src/graph/ports.rs:185` |
| sym-1c6dd1dfe38f33c22990 | `new` | function | Creates a new `NodeRegistry`. | `src/graph/registry.rs:73` |
| sym-836ce3931737b05f2d33 | `new` | function | Creates a new `SignalDerivation`. | `src/graph/signal/lineage.rs:107` |
| sym-7193b901d461c0e9ca43 | `new` | function | Creates a new `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:144` |
| sym-48818d161810b5399bc4 | `new` | function | Creates a new `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:29` |
| sym-ccffe09a19245796e996 | `new` | function | Creates a new `SignalId`. | `src/graph/signal/spec.rs:25` |
| sym-bac810f703773ab52c40 | `new` | function | Creates a new `SemanticRole`. | `src/graph/signal/spec.rs:60` |
| sym-a80eb1b0c92a1368eb47 | `new` | function | Creates a new `SchemaRef`. | `src/graph/signal/spec.rs:90` |
| sym-08fa3bcb2f21834f932b | `new` | function | Creates a new `SignalSpec`. | `src/graph/signal/spec.rs:226` |
| sym-4f8b51e2341047871846 | `new` | function | Creates a new `Session`. | `src/lib.rs:356` |
| sym-018a439dc3f2a9da4841 | `new` | function | Creates a new `StemLabel`. | `src/recording/config.rs:23` |
| sym-681108e3e5d1ba2c1c54 | `new` | function | Creates a new `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:56` |
| sym-21e2ca99159d1baa9353 | `new` | function | Creates a new `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:62` |
| sym-eb0fac256b17a095cd18 | `new` | function | Creates a new `PlanEdgeRouter`. | `src/runtime/audio/router.rs:680` |
| sym-15c9d2f4a353218916de | `new` | function | Creates a new `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:94` |
| sym-58471ad992ab2a02b94d | `new` | function | Creates a new `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:314` |
| sym-74526cc19c9485813592 | `new` | function | Creates a new `AsyncRuntimeHost`. | `src/runtime/lifecycle/async_host.rs:33` |
| sym-c4f65bbc78887a2b7f02 | `new` | function | Creates a new `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:82` |
| sym-04bd1b40282161c0a6dc | `new` | function | Creates a new `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:72` |
| sym-ca663a1fc727a4b84429 | `new` | function | Creates a new `BridgeSinkFactory`. | `src/runtime/nodes.rs:176` |
| sym-9f5ec8061e1c62168121 | `new` | function | Creates a new `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:264` |
| sym-568e7aff01f95eefc78b | `new` | function | Creates a new `SessionCompiler`. | `src/session/compile/mod.rs:77` |
| sym-610a485e4b039469096a | `new` | function | Creates a new `Operator`. | `src/session/declaration/draft.rs:300` |
| sym-b44c70061ea185f7b43b | `new` | function | Creates a new `Session`. | `src/session/declaration/draft.rs:333` |
| sym-334b36f26ab04df740af | `new` | function | Creates a new `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:33` |
| sym-33f9779ce68b1943b5cf | `new` | function | Creates a new `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:118` |
| sym-a3858386173ef9cbab20 | `new` | function | Creates a new `ProcessId`. | `src/session/declaration/selector.rs:9` |
| sym-8b6c6c3871e72ce2db3f | `new` | function | Creates a new `DeviceId`. | `src/session/declaration/selector.rs:22` |
| sym-2cdd691addb7b1064016 | `new` | function | Creates a new `SourceInstanceId`. | `src/session/declaration/spec.rs:17` |
| sym-18803c20de46b9fe6fba | `new` | function | Creates a new `OperatorInstanceId`. | `src/session/declaration/spec.rs:30` |
| sym-ea281c0561b7ce6c29c6 | `new` | function | Creates a new `SessionSpecVersion`. | `src/session/declaration/spec.rs:46` |
| sym-7594d036bc7fdb47da63 | `new` | function | Creates a new `TypedOperator`. | `src/session/declaration/typed_stream.rs:30` |
| sym-1a5414e2d06ef0cecc03 | `new` | function | Creates a new `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:29` |
| sym-813f11c48935912464ea | `new` | function | Creates a stable source implementation identity. | `src/session/extensions/source.rs:26` |
| sym-5b6510a0277fcf8dbe8d | `new` | function | Creates a new `SourceManifest`. | `src/session/extensions/source.rs:122` |
| sym-622465444bc2511fc9fa | `new` | function | Creates a new `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:43` |
| sym-e434d7e730cec18d2a0c | `new` | function | Creates a new `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:243` |
| sym-73e2ad6f978a471dbe7c | `new` | function | Creates a new `ClockCorrectionController`. | `src/timing/clock_correction.rs:13` |
| sym-2a3ac821bd95a8e595a5 | `new` | function | Creates a new `ClockDriftEstimator`. | `src/timing/clock_drift.rs:22` |
| sym-467efaaa4788a0962c0d | `new` | function | Creates a new `TimelineMapping`. | `src/timing/timeline_mapping.rs:8` |
| sym-d3bd67f80d05f51d19dc | `new_with_output_channels` | function | Creates `MixerSourceNode` with the supplied output channels. | `src/runtime/nodes.rs:280` |
| sym-68f41330fa1008ddb0d5 | `next` | function | Advances the local evidence epoch after an observed authorization change or an explicit source reopen. | `src/capture/authorization.rs:274` |
| sym-16beb1bd75823a8e4a9a | `next` | function | Returns the generation assigned after explicit rediscovery. | `src/capture/events.rs:18` |
| sym-41c19799e4db0d0dd702 | `node` | function | Returns the node held by `ConnectorManifest`. | `src/connector/manifest.rs:140` |
| sym-75ff7c3f6b094d8e0794 | `node` | function | Returns the node held by `GraphIr`. | `src/graph/ir.rs:47` |
| sym-b5113f12ad418b171a05 | `node` | function | Returns the node held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:188` |
| sym-d21d97866bc7d452a58d | `node` | function | Returns the node held by `GraphSpec`. | `src/graph/spec.rs:73` |
| sym-e895b862c39b54dd1556 | `node_configuration` | function | Returns the node configuration held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:150` |
| sym-59aa13fc702cd2e54b48 | `node_configuration` | function | Returns the node configuration held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:258` |
| sym-8da2711f4954700b3c0d | `node_count` | function | Returns the node count held by `GraphIr`. | `src/graph/ir.rs:40` |
| sym-9b737f5f193aeecd746b | `node_count` | function | Returns the node count held by `RuntimePlan`. | `src/graph/plan.rs:135` |
| sym-c3af1cdaa9a78f5da605 | `node_count` | function | Returns the node count held by `GraphSpec`. | `src/graph/spec.rs:65` |
| sym-8dc057e86680cbc4a958 | `node_count` | function | Returns the node count held by `CompiledSession`. | `src/session/compile/compiled.rs:47` |
| sym-a751620187add456badf | `node_type_id` | function | Returns the node type identifier held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:141` |
| sym-03f61e68cda696e9e6e8 | `node_type_id` | function | Returns the node type identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:179` |
| sym-50b92d4050c599fafa35 | `normalize_timestamp_ns` | function | Returns the normalize timestamp nanoseconds held by `TimelineMapping`. | `src/timing/timeline_mapping.rs:15` |
| sym-db8caf533c36e5677f4d | `normalized_total` | function | Returns the normalized total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:348` |
| sym-4b5b8d12d566894093ee | `observability` | function | Returns the observability associated with `EdgeContract`. | `src/graph/ports.rs:357` |
| sym-5f51988bb89a156b5ab1 | `observation` | function | Returns the current observation exposed by `RegisteredConnector`. | `src/connector/mod.rs:140` |
| sym-72186c7686c046cfb200 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventSender`. | `src/capture/events.rs:270` |
| sym-5b48ccb23dd8ea3456d6 | `observation_handle` | function | Returns a handle for reading observations from `SourceRuntimeEventReceiver`. | `src/capture/events.rs:321` |
| sym-84a783bb57fc8776c9fe | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameSender`. | `src/capture/frame_stream.rs:142` |
| sym-a64edc3f92284994a561 | `observation_handle` | function | Returns a handle for reading observations from `CapturedFrameStream`. | `src/capture/frame_stream.rs:183` |
| sym-f2db5c62a69fa044699c | `observation_handle` | function | Returns a handle for reading observations from `CaptureObservationCounters`. | `src/capture/observations.rs:107` |
| sym-a5cb1adefd7a956798a4 | `observation_handle` | function | Returns a handle for reading observations from `MacosInputSource`. | `src/capture/platform/macos/input.rs:231` |
| sym-75fe343f4d5a6f0542a0 | `observation_handle` | function | Returns a handle for reading observations from `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:267` |
| sym-91c2d86b10a9a63997b5 | `observation_handle` | function | Returns a handle for reading observations from `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:96` |
| sym-61e912b7cc5a34ad7d15 | `observation_handle` | function | Returns a read-only handle to this edge's authoritative live telemetry. | `src/runtime/audio/router.rs:526` |
| sym-c2e24bdeaa8112fdb1e9 | `observation_handle` | function | Returns a handle for reading observations from `PlanSourceSender`. | `src/runtime/audio/runner.rs:181` |
| sym-c9f6042eaf57175c5119 | `observation_receipt` | function | Returns the observation receipt associated with `CaptureOwner`. | `src/capture/capture_owner.rs:260` |
| sym-e02982d65399a150f192 | `observations` | function | Returns the observations exposed by `CaptureObservationReceipt`. | `src/capture/capture_owner.rs:174` |
| sym-78249af275b9cc106fe6 | `observations` | function | Returns the observations exposed by `CaptureOwner`. | `src/capture/capture_owner.rs:256` |
| sym-835351f51a8f903c484a | `observations` | function | Returns the observations exposed by `SourceRuntimeEventObservationHandle`. | `src/capture/events.rs:205` |
| sym-a5581c8cbad9390c13a5 | `observations` | function | Returns the observations exposed by `SourceRuntimeEventSender`. | `src/capture/events.rs:266` |
| sym-d54552db99b1952baa2e | `observations` | function | Returns the observations exposed by `SourceRuntimeEventReceiver`. | `src/capture/events.rs:317` |
| sym-5e1cf4492d30804e230b | `observations` | function | Returns the observations exposed by `CapturedFrameObservationHandle`. | `src/capture/frame_stream.rs:36` |
| sym-5b477e17989793fa8dc0 | `observations` | function | Returns the observations exposed by `CaptureObservationHandle`. | `src/capture/observations.rs:37` |
| sym-ba5eb2c43c17b7c5d1e9 | `observations` | function | Returns the observations exposed by `MacosInputSource`. | `src/capture/platform/macos/input.rs:227` |
| sym-c1049f62bd4f9fd89692 | `observations` | function | Returns the observations exposed by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:248` |
| sym-353c484ee921c68ef410 | `observations` | function | Returns the observations exposed by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:82` |
| sym-0952d710b28badc9600c | `observations` | function | Returns the observations exposed by `RegisteredConnector`. | `src/connector/mod.rs:153` |
| sym-ff77c27aa7377b2bf9b0 | `observations` | function | Snapshots the bounded edge counters for this endpoint input. | `src/endpoint/contract.rs:108` |
| sym-e7c6edd26ced8a2c02a5 | `observations` | function | Returns the observations exposed by `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:167` |
| sym-583d951b814628e2833f | `observations` | function | Returns the observations exposed by `MultistemRecording`. | `src/recording/writer.rs:250` |
| sym-907061b242fbf241320d | `observations` | function | Returns the observations exposed by `RealtimePlanExecutor`. | `src/runtime/audio/executor.rs:188` |
| sym-252bc04e689b888ace9b | `observations` | function | Returns a point-in-time snapshot of the edge's live observations. | `src/runtime/audio/router.rs:217` |
| sym-5b26179db3eb4f409e28 | `observations` | function | Returns the observations exposed by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:626` |
| sym-85045357c2c8bda7180f | `observations` | function | Returns the observations exposed by `PlanEdgeRouter`. | `src/runtime/audio/router.rs:852` |
| sym-2f6c167a8568aa579cd9 | `observations` | function | Returns the observations exposed by `PlanSourceObservationHandle`. | `src/runtime/audio/runner.rs:143` |
| sym-5ee03bf0844b79d1404b | `observations` | function | Returns the observations exposed by `PlanSourceSender`. | `src/runtime/audio/runner.rs:177` |
| sym-032d40321fea55178703 | `observations` | function | Returns the observations exposed by `PlanSourceInput`. | `src/runtime/audio/runner.rs:200` |
| sym-dc7f94d7dc77d5e39322 | `observations` | function | Returns the observations exposed by `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:261` |
| sym-94219cd0c936dc73bfab | `observations` | function | Returns the observations exposed by `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:921` |
| sym-24fc69e411a1cd29f7ed | `observations` | function | Returns the observations exposed by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:246` |
| sym-4119a81ca0dae8426522 | `observations` | function | Returns the observations exposed by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:131` |
| sym-2b071730ee8a80feda9f | `observations` | function | Returns the observations exposed by `SourceRuntime`. | `src/session/extensions/source.rs:579` |
| sym-2a9eacabe9b00d6c35d6 | `observations` | function | Returns the observations exposed by `SessionEventReceiver`. | `src/session/lifecycle/events.rs:517` |
| sym-cdccd0ea8e9123cb3513 | `observe` | function | Returns the current observation exposed by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:204` |
| sym-f72be4adaaf6e63df4cb | `observe` | function | Returns the current observation exposed by `SignalContinuityTracker`. | `src/graph/signal/continuity.rs:18` |
| sym-c0a379cb4e9dac135996 | `observe` | function | Returns the current observation exposed by `ClockDriftEstimator`. | `src/timing/clock_drift.rs:35` |
| sym-a3e36ca12a52a223a0c4 | `observe_callback_buffer` | function | Records an observation for callback buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:52` |
| sym-f7d3912ca925866be07c | `observe_complete_snapshot` | function | Records an observation for complete snapshot for `SourceLifecycleRegistry`. | `src/capture/lifecycle_registry.rs:36` |
| sym-8f8fbc2026eedef1dda1 | `observe_dispatch_queue_full` | function | Records an observation for dispatch queue full for `CaptureObservationCounters`. | `src/capture/observations.rs:70` |
| sym-022fd20e0b5e41fce56a | `observe_dispatch_queue_full_frames` | function | Records a known number of frames lost at a bounded native or Rust delivery edge. | `src/capture/observations.rs:76` |
| sym-9cbb51e6fbef2902b38d | `observe_enqueued_frame` | function | Records an observation for enqueued frame for `CaptureObservationCounters`. | `src/capture/observations.rs:58` |
| sym-b7beacb693d60aee3290 | `observe_oversized_buffer` | function | Records an observation for oversized buffer for `CaptureObservationCounters`. | `src/capture/observations.rs:89` |
| sym-450456c740c1b596179f | `observe_pool_exhaustion` | function | Records an observation for pool exhaustion for `CaptureObservationCounters`. | `src/capture/observations.rs:64` |
| sym-45976a1abbb2588ac007 | `observe_stream_error` | function | Records an observation for stream error for `CaptureObservationCounters`. | `src/capture/observations.rs:95` |
| sym-73a6a2afe3673c227258 | `observe_timestamp_epoch_clamp` | function | Records an observation for timestamp epoch clamp for `CaptureObservationCounters`. | `src/capture/observations.rs:101` |
| sym-d0eaceec9c05753154b5 | `observed` | function | Creates observed signal timing for `SignalTiming`. | `src/graph/signal/timing.rs:38` |
| sym-0d01bd0ea3304e47aa21 | `observed_timestamp_ns` | function | Returns the observed timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:75` |
| sym-4ffe4e9de671e20f756d | `ok` | function | Creates a successful status value for `PksSessionStatus`. | `src/abi/session/abi.rs:62` |
| sym-97dd85058ee8a8017a24 | `open` | function | Opens the resource represented by `PreparedCapture`. | `src/capture/capture_owner.rs:128` |
| sym-9fd0341145c82b3ee451 | `open` | function | Opens the resource represented by `CaptureDeliveryStartGateController`. | `src/capture/frame_stream.rs:77` |
| sym-2f821d7fbe0146feae40 | `open_metadata` | function | Returns the open metadata associated with `CaptureOwner`. | `src/capture/capture_owner.rs:242` |
| sym-5d7c7ac0e76c64bc9c46 | `operation` | function | Returns the operation associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:93` |
| sym-c00d6b09b0dba31eb841 | `operator` | function | Declares exactly one operator instance. Connect streams to named inputs and select named outputs through the returned Session-scoped handle. | `src/lib.rs:456` |
| sym-c4d88d283066544dddc0 | `operator` | function | Declares exactly one Session-owned operator instance. | `src/session/declaration/draft.rs:395` |
| sym-255df4616605df3ac4c5 | `operator` | function | Returns the operator associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:103` |
| sym-047deaec359a7c7f61d7 | `operator_count` | function | Returns the operator count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:99` |
| sym-e0aa058bb06b753d1501 | `operator_finalization_failures_total` | function | Returns the operator finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:338` |
| sym-a1a03603e13732dc10c5 | `operator_generation` | function | Returns the operator generation associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:150` |
| sym-2cc9104058399195bb19 | `operator_id` | function | Returns the operator identifier held by `ConnectorManifest`. | `src/connector/manifest.rs:132` |
| sym-0d93214a11f553678570 | `operator_id` | function | Returns the operator identifier held by `SignalDerivation`. | `src/graph/signal/lineage.rs:144` |
| sym-6de09da4d4990dde0757 | `operator_id` | function | Returns the operator identifier held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:176` |
| sym-ce9c62c0c50c9915b523 | `operator_id` | function | Returns the operator identifier held by `Operator`. | `src/session/declaration/draft.rs:307` |
| sym-ad007e7e221e1c5a1fc0 | `operator_id` | function | Returns the operator identifier held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:145` |
| sym-22354fa85ea55e542cdf | `operator_id` | function | Returns the operator identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:183` |
| sym-26ac05a27c8d705f6425 | `operator_id` | function | Returns the operator identifier held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:249` |
| sym-1f4297e1167322421139 | `operator_id` | function | Returns the operator identifier held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:71` |
| sym-fa7d1beeca646b5b0ccd | `operator_instance_id` | function | Returns the operator instance identifier held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1028` |
| sym-5ec037916bd759fd8575 | `operator_instance_id` | function | Returns the operator instance identifier held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:304` |
| sym-789e8cd7bcd8d10f1171 | `operator_mappings` | function | Returns the operator mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:56` |
| sym-7d07714354170f3b52aa | `operator_metrics` | function | Returns one finalizable observation handle per Session-owned operator instance, including exact per-input-port edge counters. | `src/lib.rs:832` |
| sym-cc59b91ee46c42a9c189 | `operator_metrics` | function | Returns the operator metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:205` |
| sym-dad9510c0489ee5ea571 | `operator_revision` | function | Returns the operator revision held by `SignalDerivation`. | `src/graph/signal/lineage.rs:147` |
| sym-f15ab11820908e6cc94d | `operators` | function | Returns the operators associated with `SessionSpec`. | `src/session/declaration/spec.rs:343` |
| sym-e13219d16a165056747a | `opus_config` | function | Returns the opus config held by `StreamProfile`. | `src/codec/profile.rs:73` |
| sym-1d7d0d9de993ec919a6e | `origin` | function | Returns the origin held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:84` |
| sym-5d1b31642248ba6c6040 | `origin` | function | Returns the origin held by `ConnectionSpec`. | `src/session/declaration/spec.rs:263` |
| sym-99b366ee077deb6f8637 | `out` | function | Selects a named output port from `NodeHandle`. | `src/graph/dsl.rs:18` |
| sym-1f270fe04adf3da9ced5 | `outcome` | function | Returns the outcome held by `SessionCancelResult`. | `src/lib.rs:1101` |
| sym-7daa5b580e4abd69d433 | `outcome` | function | Returns the outcome held by `SessionStopResult`. | `src/lib.rs:1121` |
| sym-dd2046e7e1bbf457c22a | `outcome` | function | Returns the outcome held by `SessionRecordingReceipt`. | `src/session/extensions/recording.rs:32` |
| sym-b197dbd4abb7a4d6524f | `outcome` | function | Returns the outcome held by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:243` |
| sym-7c043fb9c444ed2d25b4 | `outcome` | function | Returns the outcome held by `SessionTrace`. | `src/session/lifecycle/trace.rs:276` |
| sym-6c4a85df792990512f80 | `output` | function | Returns the output held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:744` |
| sym-429bc9596fb6a42ff5d9 | `output` | function | Returns the output held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:866` |
| sym-a51ba3c952b407ed044d | `output` | function | Returns the output held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1036` |
| sym-876aa14c13ad52ad6107 | `output` | function | Returns the output held by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:107` |
| sym-ea6df2e8b7c3afab51ed | `output` | function | Returns the output held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:56` |
| sym-9e88a9e1d4f9e52eb610 | `output` | function | Returns the output held by `SourceSessionContext`. | `src/session/extensions/source.rs:242` |
| sym-165350c7eb4d01df0d2a | `output_edge` | function | Returns the output edge associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:196` |
| sym-45a40b5ea1885461ee97 | `output_pool_exhaustions` | function | Returns the output pool exhaustions associated with `MixerTelemetry`. | `src/runtime/nodes.rs:263` |
| sym-5f8986723f56a351a506 | `output_port` | function | Returns the output port held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:947` |
| sym-bd16619807e465020349 | `output_port` | function | Returns the output port held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1032` |
| sym-09f6aefd2dd473790e00 | `output_port` | function | Returns the output port held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:141` |
| sym-70dbd7b16369627fa5e8 | `output_port` | function | Returns the output port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:79` |
| sym-9949de72230c160ad94c | `output_port` | function | Returns the output port held by `SourceManifest`. | `src/session/extensions/source.rs:217` |
| sym-a4a36c332782d868d32e | `output_ports` | function | Returns the output ports held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:231` |
| sym-3ff74d3a91ed55ee819b | `output_roles` | function | Returns the output roles associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:220` |
| sym-2d05b989e1f3feab47de | `output_root` | function | Returns the output root associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:65` |
| sym-e67bb7de8a87f69fe7d8 | `output_spec` | function | Returns the output spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:87` |
| sym-99af61c6fad5797e92de | `outputs` | function | Returns the outputs associated with `NodeDescriptor`. | `src/graph/node.rs:234` |
| sym-582f4c0d4006e5ae3813 | `outputs` | function | Returns the outputs associated with `AsyncOperatorPrepareContext`. | `src/graph/signal/preparation.rs:66` |
| sym-69d92265c4f8e7cda77b | `outputs` | function | Returns the outputs associated with `SourceManifest`. | `src/session/extensions/source.rs:170` |
| sym-fce088a006f3e399abdc | `overrun_count` | function | Returns the overrun count held by `BridgeSinkTelemetry`. | `src/runtime/nodes.rs:165` |
| sym-0f6c33d9765cf812be60 | `package_version` | function | Returns the package version held by `ConnectorManifest`. | `src/connector/manifest.rs:136` |
| sym-0c160b19fd4474a1371d | `partition` | function | Returns the partition associated with `RuntimePlan`. | `src/graph/plan.rs:140` |
| sym-55797113be143f702cbb | `path` | function | Returns the path associated with `NativeExtensionLibraryError`. | `src/native_extension/mod.rs:139` |
| sym-f090e15638035881e524 | `payload` | function | Returns the payload associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:62` |
| sym-89321d5d3ca7f9614d2d | `payload_size_bytes` | function | Returns the payload size bytes held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:66` |
| sym-31bbe14b5a178e65cbf5 | `pcm_source` | function | Declares the low-level bounded PCM source and returns its explicit Session handles and producer writer ownership. | `src/lib.rs:400` |
| sym-4e58cb471f6015a6e16c | `permission` | function | Returns the permission associated with `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:204` |
| sym-253ac881e05a7dfc0b4a | `permission_epoch` | function | Returns the permission epoch held by `CapturePermissionLifecycle`. | `src/capture/authorization.rs:200` |
| sym-7a99f9e74f8e8ab86c8f | `permission_epoch` | function | Returns the permission epoch held by `FrameLineage`. | `src/frame/lineage.rs:83` |
| sym-9812c802447ad91a03e1 | `plan` | function | Lower a verified IR into an execution-ready plan. | `src/graph/compile/plan.rs:24` |
| sym-4d15a0b12b1aeb6d598e | `plan_edge_observation_handle` | function | Plans edge observation handle for `EndpointAudioReceiver`. | `src/endpoint/contract.rs:117` |
| sym-651586cf43cd82600d3b | `planned_edge_count` | function | Returns the planned edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:57` |
| sym-9511cd04174451157d94 | `pocketstation::capture::capture_owner::join_capture_worker` | function | Joins one owned capture worker and preserves panic as a typed failure. | `src/capture/capture_owner.rs:334` |
| sym-0eb973c9ae1a8b49fb1e | `pocketstation::capture::capture_owner::prepare_capture` | function | Prepares a bounded capture owner without starting native delivery. | `src/capture/capture_owner.rs:298` |
| sym-e0ce04ad290fdad7a8b9 | `pocketstation::capture::capture_owner::prepare_capture_with_start_gate` | function | Prepares a bounded capture owner behind a caller-owned one-way start gate. | `src/capture/capture_owner.rs:306` |
| sym-098dd94d3d6620409e45 | `pocketstation::capture::events::publish_backend_failure` | function | Publishes one exact post-open backend failure without introducing another event queue or worker. | `src/capture/events.rs:280` |
| sym-c69563551656dae0f59c | `pocketstation::capture::events::source_runtime_event_channel` | function | Creates the bounded sender and receiver used for source runtime events. | `src/capture/events.rs:328` |
| sym-6f274ded6b73f261f576 | `pocketstation::capture::frame_stream::capture_delivery_start_gate` | function | Creates a closed Session-owned controller and callback-visible start gate. | `src/capture/frame_stream.rs:83` |
| sym-a3ad138af4976282293f | `pocketstation::capture::frame_stream::captured_frame_stream` | function | Wraps the supplied capture receiver as a stream of captured frames. | `src/capture/frame_stream.rs:191` |
| sym-babc87b1b6b9aeb16c0a | `pocketstation::capture::platform::macos::input::discover_input_sources_native` | function | Discovers microphone input sources through the native macOS backend. | `src/capture/platform/macos/input.rs:256` |
| sym-5bd4e29b7f4eba05db84 | `pocketstation::capture::platform::macos::macos_tap::discover_sources_native` | function | Enumerate all running processes that have audio output. Returns an empty `Vec` on macOS < 14.4 (public support floor) or on non-macOS platforms. | `src/capture/platform/macos/macos_tap.rs:87` |
| sym-a9490be832ccb13675e0 | `pocketstation::capture::platform::macos::macos_tap::tap_available` | function | Returns `true` when the CoreAudio process tap API is available. | `src/capture/platform/macos/macos_tap.rs:76` |
| sym-43501bc28a7080cc39cf | `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| sym-751f1bdabf888c4613df | `pocketstation::capture::query::discover_sources` | function | Discovers capture sources available from the local provider. | `src/capture/query.rs:85` |
| sym-fff0de4ef5a205420372 | `pocketstation::capture::query::resolve_query` | function | Resolves query for `query`. | `src/capture/query.rs:40` |
| sym-2400599b268a81f31496 | `pocketstation::capture::timeline::initialize_monotonic_timestamp_domain` | function | Initializes the process-wide capture timestamp domain from a setup thread. | `src/capture/timeline.rs:11` |
| sym-608f588747dc366db0ae | `pocketstation::capture::timeline::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain used by every capture adapter. The value is non-zero and comparable across PocketStation crates in the same process; it is never derived from a wall clock and cannot jump. | `src/capture/timeline.rs:18` |
| sym-f1cbe83f836ec650a735 | `pocketstation::conformance::observed_browser` | function | Declares and registers a deterministic native browser boundary used only by cross-language conformance harnesses. | `src/conformance.rs:335` |
| sym-6b1810eef20f60c09617 | `pocketstation::conformance::observed_connector` | function | Declares and registers a deterministic native connector used only by cross-language conformance harnesses. | `src/conformance.rs:274` |
| sym-819a694720009bdd6d06 | `pocketstation::conformance::run_extension_vector` | function | Executes the neutral typed Source -> `Stream<T>` -> Operator -> Endpoint vector through the canonical public Session. | `src/conformance.rs:1006` |
| sym-75f96b7e3b5fb8a80809 | `pocketstation::conformance::session` | function | Runs the conformance assertions for the Session contract. | `src/conformance.rs:198` |
| sym-357f4c2a90f63f1f1730 | `pocketstation::conformance::session_for_saturation` | function | Creates a finite fixture that produces enough frames to overflow a deliberately unconsumed canonical route. | `src/conformance.rs:204` |
| sym-6642041451c79afb6998 | `pocketstation::conformance::session_with_recording` | function | Creates the deterministic canonical-engine fixture with multistem recording. | `src/conformance.rs:209` |
| sym-e5e19d754c5ad58058fa | `pocketstation::conformance::session_with_recording_and_trace` | function | Creates the deterministic canonical-engine fixture with both aligned multistem recording and a bounded Session diagnostic trace. | `src/conformance.rs:231` |
| sym-6a6ff8b3e536c26a394b | `pocketstation::conformance::session_with_trace` | function | Creates the deterministic canonical-engine fixture with a bounded Session Session diagnostic trace recorder. | `src/conformance.rs:217` |
| sym-5e4327b53963a537d944 | `pocketstation::connector::sidecar::sidecar_connector_factory` | function | Creates a connector driver factory backed by the supplied sidecar process. | `src/connector/sidecar.rs:264` |
| sym-edd92efe7a54ca69ced8 | `pocketstation::graph::builtins::register_builtins` | function | Registers builtins for `builtins`. | `src/graph/builtins.rs:220` |
| sym-c84c6acdce6668ea5d9f | `pocketstation::microphone_permission_observation` | function | Reads the current microphone authorization state without prompting. | `src/lib.rs:55` |
| sym-2eb6b21ee19ecb2856b6 | `pocketstation::recording::error_code::recording_outcome_error_code` | function | Returns the recording outcome error code held by `error_code`. | `src/recording/error_code.rs:82` |
| sym-09c4a31af53a881d75c2 | `pocketstation::runtime::audio::runner::plan_source_channel` | function | Plans source channel for `runner`. | `src/runtime/audio/runner.rs:229` |
| sym-84b89e94dd3cc15a54f5 | `pocketstation::runtime::nodes::register_runtime_nodes` | function | Registers runtime nodes for `nodes`. | `src/runtime/nodes.rs:43` |
| sym-39ef5ea12a6d09945499 | `pocketstation::session::error_code::polled_audio_poll_error_code` | function | Returns the polled audio poll error code held by `error_code`. | `src/session/error_code.rs:255` |
| sym-9c7aed24d1b9cc8f1d9d | `pocketstation::session::error_code::session_declaration_error_code` | function | Returns the session declaration error code held by `error_code`. | `src/session/error_code.rs:195` |
| sym-60cff1b221f8ab3f6f87 | `pocketstation::session::error_code::session_start_failure_code` | function | Returns the session start failure code held by `error_code`. | `src/session/error_code.rs:225` |
| sym-60779fcdabce7a509a83 | `pocketstation::session::error_code::session_stop_failure_codes` | function | Returns every stable failure code carried by a Session stop result. | `src/session/error_code.rs:265` |
| sym-6d80dde20ce2a006601a | `pocketstation::session::extensions::builtins::register_session_graph_nodes` | function | Registers session graph nodes for `builtins`. | `src/session/extensions/builtins.rs:36` |
| sym-38553eb3f5fd53ccb7a7 | `pocketstation::session::lifecycle::running::start_prepared_session` | function | Starts prepared session for `running`. | `src/session/lifecycle/running.rs:615` |
| sym-e2db7459f61aae428d43 | `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | function | Starts prepared session cancellable for `running`. | `src/session/lifecycle/running.rs:631` |
| sym-aaef40a767909f538478 | `pocketstation::session::prepare::prepare_session_runtime` | function | Prepares session runtime for `prepare`. | `src/session/prepare/mod.rs:33` |
| sym-b4e20b5cb9711c373847 | `pocketstation::timing::monotonic_timestamp_ns` | function | Process-wide monotonic timestamp domain shared by capture, routing, and destination workers. | `src/timing/mod.rs:28` |
| sym-b4724928e196c76bee2b | `policy_epoch` | function | Returns the policy epoch associated with `SignalLineage`. | `src/graph/signal/lineage.rs:80` |
| sym-f2d2e0188b6edfd8b92c | `polled_audio` | function | Declares a bounded polled-audio endpoint on `Session`. | `src/lib.rs:497` |
| sym-7a78d9bb2e991fc348d6 | `polled_audio` | function | Declares a bounded polled-audio endpoint on `Session`. | `src/session/extensions/polled_audio.rs:14` |
| sym-9783d6b783873d785d90 | `polled_audio` | function | Declares a bounded polled-audio endpoint on `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:71` |
| sym-9da1c8304e4337da5515 | `polled_audio_receipt` | function | Returns the polled audio receipt associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:99` |
| sym-f9bc638db011084c320e | `polled_audio_receipts_total` | function | Returns the polled audio receipts total held by `SessionEngineHost`. | `src/session/lifecycle/host.rs:104` |
| sym-d61eec066d9ac19ebb30 | `pool_exhausted_total` | function | Returns the pool exhausted total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:360` |
| sym-fc8dc82bd71fc735df93 | `pool_slots` | function | Returns the pool slots associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:336` |
| sym-6fe2c67fee8fd8d4039e | `port_name` | function | Returns the port name held by `ConnectorAudioRecord`. | `src/connector/transport.rs:335` |
| sym-b5e5c0c3e0b0d5c896a4 | `port_name` | function | Returns the port name held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:40` |
| sym-ae54f2ebf47973451b7a | `port_name` | function | Returns the port name held by `EndpointPortInput`. | `src/endpoint/contract.rs:203` |
| sym-d34d02730f4091d78665 | `port_name` | function | Returns the port name held by `PortPrepareContext`. | `src/graph/node.rs:341` |
| sym-f8d30689c2c6840b3836 | `preparation_group` | function | Returns the preparation group associated with `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:39` |
| sym-b4e2dc46188095e6f2d6 | `preparation_group` | function | Returns the preparation group associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:82` |
| sym-9e73113df0713054f152 | `prepare` | function | Prepares resources required by `DesktopCaptureBackend`. | `src/capture/platform/macos/session_backend.rs:22` |
| sym-40b458a6269e8e0c2262 | `prepare` | function | Prepares resources required by `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:49` |
| sym-0403aa53b47c8ce6a126 | `prepare` | function | Prepares resources required by `PassthroughNode`. | `src/graph/builtins.rs:98` |
| sym-1555ba585793ec03dee4 | `prepare` | function | Prepares resources required by `GainNode`. | `src/graph/builtins.rs:154` |
| sym-8a35a9fe22759b2b9c89 | `prepare` | function | Prepares resources required by `MonoMixNode`. | `src/graph/builtins.rs:197` |
| sym-7348171d3ec34d64b215 | `prepare` | function | Prepares resources required by `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:99` |
| sym-e10a87f8168ef6956cba | `prepare` | function | Prepares resources required by `MixerSourceNode`. | `src/runtime/nodes.rs:432` |
| sym-e1c41c26941d3b956723 | `prepare` | function | Prepares resources required by `SourceRegistry`. | `src/session/extensions/source.rs:339` |
| sym-79539f26eb4055bb36aa | `prepare_and_spawn_from_plan_edge` | function | Prepares and spawn from plan edge for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:764` |
| sym-2fbf670e512c187c73d3 | `prepare_context` | function | Prepares context for `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:268` |
| sym-1d1df0a7341eaf2262b7 | `prepare_session` | function | Prepares session for `SourceRegistry`. | `src/session/extensions/source.rs:353` |
| sym-f2c2a0f54a85d4eebaf3 | `probe_interval` | function | Returns the probe interval held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:47` |
| sym-0f013d2539513ea3dc69 | `process` | function | Processes an input value through `SidecarConnectorDriverFactory`. | `src/connector/sidecar.rs:33` |
| sym-258a8ce93055beafae2a | `process` | function | Processes an input value through `PassthroughNode`. | `src/graph/builtins.rs:102` |
| sym-e0e41303eb7700e25d8b | `process` | function | Processes an input value through `GainNode`. | `src/graph/builtins.rs:158` |
| sym-959aabff687a70d5aaca | `process` | function | Processes an input value through `MonoMixNode`. | `src/graph/builtins.rs:201` |
| sym-c4cc2cba811b43f3cb33 | `process` | function | Processes an input value through `MixerSourceNode`. | `src/runtime/nodes.rs:441` |
| sym-0f5dd46a54fa11dd0327 | `process_id` | function | Returns the process identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:48` |
| sym-f287d61ad429cedaa2b1 | `process_instance` | function | Creates `ApplicationSelector` for one exact process instance. | `src/session/declaration/selector.rs:52` |
| sym-f30ce2949796d2affa10 | `process_ready` | function | Processes the ready inputs for `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:338` |
| sym-7d8fd92059fe75b9ce25 | `process_tree_scope` | function | Reports the native process boundary represented by this discovery result without making the CLI reconstruct a private capture mode. | `src/capture/identity.rs:140` |
| sym-f53955b6600b5a85ec04 | `process_tree_scope` | function | Reports the process boundary requested from the native backend. | `src/capture/selection.rs:55` |
| sym-5ba1b3010d14039efdc2 | `publish` | function | Publishes its owned operation for `TypedEdgeFanout`. | `src/runtime/signal/edge.rs:308` |
| sym-542f581ec3f3b1083410 | `query::SourceProvider::discover` | function | Discovers the resources visible to `SourceProvider`. | `src/capture/query.rs:49` |
| sym-996cf2843f20c56c51e1 | `queue_capacity_frames` | function | Returns the queue capacity frames held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:200` |
| sym-066b7df6db0d240ff02a | `queue_capacity_signals` | function | Returns the queue capacity signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:312` |
| sym-cabb29b1e5bccc4f7a60 | `queue_depth_signals` | function | Returns the queue depth signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:316` |
| sym-31bc9bc33102a7af8c19 | `queue_peak_signals` | function | Returns the queue peak signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:320` |
| sym-e0d3138e8702f4735a5a | `rank` | function | Priority rank for scheduling: lower = higher priority. | `src/graph/partition.rs:60` |
| sym-23421c43505fe755b0c7 | `rank` | function | Returns the rank associated with `EdgeObservabilityLevel`. | `src/graph/ports.rs:301` |
| sym-9101d8ee71370981db29 | `read` | function | Reads the persisted representation of `SessionTrace`. | `src/session/lifecycle/trace.rs:262` |
| sym-fb62d7a4401cec2a8906 | `readiness` | function | Returns the readiness held by `ConnectorManifest`. | `src/connector/manifest.rs:148` |
| sym-c04c490781b939e286b3 | `readiness_reason_code` | function | Returns the readiness reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:54` |
| sym-91cd07cd903200167e85 | `realtime_audio` | function | Generic realtime PCM edge. Concrete sample rate, frame size, and channel layout are negotiated from connected ports. | `src/graph/ports.rs:391` |
| sym-9656d0a8e46811ec82bc | `receipt` | function | Returns the receipt associated with `PolledAudioEndpoint`. | `src/endpoint/polled_audio.rs:30` |
| sym-2a1b0d259f87ca189edb | `receipt` | function | Returns the receipt associated with `SessionMultistemEndpointCoordinator`. | `src/recording/endpoint.rs:74` |
| sym-478c1c453e9ac39f6a42 | `receive_sidecar_signal` | function | Receives sidecar signal for `RunningSession`. | `src/lib.rs:860` |
| sym-574a15e57bd14dde4af7 | `receive_sidecar_signal` | function | Receives sidecar signal for `RunningSession`. | `src/session/lifecycle/running.rs:277` |
| sym-576cfa1deee2ac13a9b9 | `receive_signal` | function | Receives signal for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:307` |
| sym-f2c69629ba9e7c7539a8 | `receiver` | function | Returns the receiver held by `EndpointPortInput`. | `src/endpoint/contract.rs:223` |
| sym-4c973064cc494de91199 | `receiver_observations` | function | Returns the receiver observations held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:263` |
| sym-6f111b7c0456e49f65d7 | `record` | function | Attaches recording output to `StemHandle`. | `src/session/extensions/recording.rs:60` |
| sym-43d072a672244b26878c | `record` | function | Attaches recording output to `SourceOutputHandle`. | `src/session/extensions/recording.rs:79` |
| sym-be1a636293abde806859 | `record_discontinuity` | function | Records discontinuity for `ConnectorContext`. | `src/connector/worker/coordination.rs:126` |
| sym-1b530434b2a4e6bdbe95 | `record_failure` | function | Records failure for `ConnectorContext`. | `src/connector/worker/coordination.rs:134` |
| sym-deb447a53e3a1cf34a98 | `record_frame_delivered` | function | Records frame delivered for `ConnectorContext`. | `src/connector/worker/coordination.rs:118` |
| sym-b5fa937a483422151652 | `record_frame_dropped` | function | Records frame dropped for `ConnectorContext`. | `src/connector/worker/coordination.rs:122` |
| sym-19d177974704b689d231 | `record_frame_received` | function | Records frame received for `ConnectorContext`. | `src/connector/worker/coordination.rs:114` |
| sym-5543bb8d0f9cccb93092 | `record_retry` | function | Records retry for `ConnectorContext`. | `src/connector/worker/coordination.rs:130` |
| sym-7476bc553a07f57f3eec | `recording_outcome` | function | Returns the recording outcome held by `RunningSession`. | `src/lib.rs:810` |
| sym-9bfc48ad8cb792f0162e | `recording_receipt` | function | Returns the recording receipt associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:108` |
| sym-0fb4bec02bc59c3aa83a | `recording_receipts_total` | function | Returns the recording receipts total held by `SessionEngineHost`. | `src/session/lifecycle/host.rs:113` |
| sym-f84ec20dddf7dc6e8bac | `recording_root` | function | Configures the artifact root used by declared multistem recording routes. | `src/lib.rs:292` |
| sym-3bdc0bc33da4126bc45d | `records` | function | Returns the records associated with `SessionTrace`. | `src/session/lifecycle/trace.rs:272` |
| sym-5f1c1862b0746339d0c9 | `recovery` | function | Returns the recovery held by `ConnectorServiceStatus`. | `src/connector/status.rs:50` |
| sym-63ed6756a1423a186c9b | `recovery_reason_code` | function | Returns the recovery reason code held by `ConnectorServiceStatus`. | `src/connector/status.rs:62` |
| sym-adc952ba5c898d4b9b1a | `recv` | function | Receives the next value from `EndpointSignalReceiver`. | `src/endpoint/contract.rs:136` |
| sym-f8ef5ea4d8404ac5135f | `reenter_audio` | function | Re-enters this operator output into the Session's specialized audio lane. | `src/session/declaration/draft.rs:1087` |
| sym-919d7c5825d7c899beec | `register` | function | Registers a node definition with `NodeRegistry` while preserving unique identities. | `src/graph/registry.rs:77` |
| sym-2cb2f29f8a92545dc15d | `register` | function | Registers a node definition with `SourceRegistry` while preserving unique identities. | `src/session/extensions/source.rs:297` |
| sym-faa2afdc3209724248db | `register_async` | function | Registers async for `NodeRegistry`. | `src/graph/registry.rs:89` |
| sym-5a96cac8e52632c8717c | `register_async_operator` | function | Registers async operator for `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:114` |
| sym-74292491c573fb012c3d | `register_async_operator` | function | Registers async operator for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:305` |
| sym-2606d38a667e450049f4 | `register_audio_endpoint_driver` | function | Registers audio endpoint driver for `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:74` |
| sym-506baa4ec54cddbac0ba | `register_audio_endpoint_driver` | function | Registers one externally owned endpoint implementation with the canonical Session engine. | `src/session/lifecycle/host.rs:283` |
| sym-6a80178e232743ea77b1 | `register_browser_driver` | function | Registers the externally owned browser/remote transport implementation. | `src/lib.rs:539` |
| sym-3499d87b1f471c1e5679 | `register_connector` | function | Registers connector for `Session`. | `src/connector/mod.rs:204` |
| sym-90085a558d5ff691c2d7 | `register_connector_driver` | function | Registers the externally owned implementation for a declared connector. | `src/lib.rs:520` |
| sym-4d44ee21069d7426704e | `register_definition` | function | Registers definition for `NodeRegistry`. | `src/graph/registry.rs:112` |
| sym-04f336fd4ed689f6c47e | `register_endpoint` | function | Registers one externally owned endpoint as a single compiler/runtime extension. The endpoint definition and driver cannot be installed independently through this authority. | `src/lib.rs:571` |
| sym-fbf3df3b5f62054635df | `register_endpoint` | function | Atomically registers an endpoint's compiler contract and runtime driver. | `src/session/lifecycle/engine.rs:91` |
| sym-ba09d00e3c513d0f4383 | `register_endpoint` | function | Registers endpoint for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:294` |
| sym-346a5ca9a13b1c953686 | `register_multistem_recording` | function | Registers multistem recording for `SessionEngineBuilder`. | `src/session/extensions/recording.rs:38` |
| sym-0f812a4e0b294ae0e760 | `register_multistem_recording` | function | Registers multistem recording for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:333` |
| sym-157e0b5838f29a3b4a82 | `register_operator` | function | Registers operator for `Session`. | `src/lib.rs:486` |
| sym-74d8142a0e0ec886461d | `register_polled_audio_endpoint` | function | Registers polled audio endpoint for `SessionEngineBuilder`. | `src/session/extensions/polled_audio.rs:23` |
| sym-ba34d13d715059e3cf15 | `register_polled_audio_endpoint` | function | Registers polled audio endpoint for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:321` |
| sym-55a6ab93737ecb4a857e | `register_sidecar` | function | Registers one language-neutral sidecar under this Session's bounded process lifecycle. The child is spawned only during transactional start. | `src/lib.rs:474` |
| sym-1fbd9f01ba6296e0acf2 | `register_sidecar_process` | function | Retains one externally implemented sidecar under the canonical Session lifecycle. IDs are unique within the engine so observations and shutdown failures remain attributable without process-global state. | `src/session/lifecycle/engine.rs:125` |
| sym-8770b536d7f56b65d160 | `register_sidecar_process` | function | Registers sidecar process for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:313` |
| sym-23f5030f3b1baddf0044 | `register_source` | function | Retains an external source factory for this Session's canonical engine. | `src/lib.rs:461` |
| sym-442cfb4c5239aae3a809 | `register_source_factory` | function | Registers one externally implemented source contract by stable type ID. | `src/session/lifecycle/engine.rs:146` |
| sym-a6a91667156fb0e2227e | `registrations` | function | Returns the registrations associated with `NativeExtensionLibrary`. | `src/native_extension/mod.rs:72` |
| sym-870f7d2bc291a855e4a7 | `report_readiness_failure` | function | Returns the report readiness failure held by `ConnectorContext`. | `src/connector/worker/coordination.rs:97` |
| sym-c3518cf53f18f24be63b | `report_readiness_success` | function | Records a successful readiness probe for `ConnectorContext`. | `src/connector/worker/coordination.rs:80` |
| sym-549f138bef7f356d45e7 | `request` | function | Requests the state transition represented by `PlanRunnerCancellation`. | `src/runtime/audio/runner.rs:100` |
| sym-394a32ce61171c3b0df2 | `request` | function | Requests the state transition represented by `SessionStartCancellation`. | `src/session/lifecycle/start_contract.rs:103` |
| sym-edb622dbf724fc82a214 | `request_stop` | function | Requests a graceful stop from `MultistemRecording`. | `src/recording/writer.rs:272` |
| sym-a69045a3fb6aaed54980 | `required` | function | Returns the required held by `ConnectorRequirement`. | `src/connector/manifest.rs:65` |
| sym-03479efd2b40d0c96fec | `required` | function | Returns the required held by `PortSpec`. | `src/graph/ports.rs:233` |
| sym-3991bd370414c1a6bf57 | `requirement` | function | Returns the requirement held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:214` |
| sym-1578e0c96d33ae028176 | `requirements` | function | Returns the requirements held by `ConnectorManifest`. | `src/connector/manifest.rs:156` |
| sym-62c608a4c5be55750a40 | `requires_realtime_safety` | function | Returns `true` if the partition requires strict real-time safety. | `src/graph/partition.rs:55` |
| sym-9ff5d39a59fcef0a980f | `resolve` | function | Resolves `ConnectorConfigurationSchema` into its validated representation. | `src/connector/configuration.rs:259` |
| sym-c227984556b300940fc1 | `result` | function | Returns the result represented by `ConnectorRunOutcome`. | `src/connector/worker/mod.rs:59` |
| sym-fe9f27f4c3d118b2be48 | `result` | function | Returns the result represented by `MultistemRecordingReceipt`. | `src/recording/endpoint.rs:33` |
| sym-b033b0817db4909e396c | `retryability` | function | Returns the retryability associated with `ConnectorError`. | `src/connector/error.rs:117` |
| sym-97b6dcf0c3f93b8a5d14 | `retryability` | function | Returns the retryability associated with `EndpointFailure`. | `src/endpoint/runtime.rs:216` |
| sym-045acf7e73303738579f | `revision` | function | Returns the revision held by `ConnectorConfigurationSchema`. | `src/connector/configuration.rs:247` |
| sym-d2a03fe25ee3733863b5 | `revision` | function | Returns the revision held by `ConnectorServiceStatus`. | `src/connector/status.rs:66` |
| sym-f26a138e6883327137e1 | `revision` | function | Returns the revision held by `AsyncOperatorManifest`. | `src/graph/signal/operator.rs:180` |
| sym-76dfec156970b96b4baa | `revision` | function | Returns the revision held by `NativeExtensionRegistration`. | `src/native_extension/mod.rs:50` |
| sym-65167f9f6ef34f8ff38f | `revision` | function | Additive descriptor revision within the compatibility major encoded by the [`SourceTypeId`] suffix. A breaking source contract uses a new identifier ending in the next `vN`; it does not reuse this field. | `src/session/extensions/source.rs:149` |
| sym-b36ef5f001b9cab9bfd2 | `role` | function | Returns the role associated with `SignalSpec`. | `src/graph/signal/spec.rs:219` |
| sym-b08aa47df91410ac66d1 | `rollback_failures` | function | Returns the rollback failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:283` |
| sym-a477c396f1a80ca37031 | `rollback_failures` | function | Returns the rollback failures associated with `SessionStartFailure`. | `src/session/lifecycle/start_contract.rs:282` |
| sym-e22efd3d1ac881a5cfa6 | `rollback_failures_total` | function | Returns the rollback failures total held by `SessionStartError`. | `src/session/lifecycle/start_contract.rs:198` |
| sym-ec96f425cf103c87a799 | `route` | function | Returns the route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:95` |
| sym-e14e4ccc1f945efe7625 | `route_context` | function | Returns the route context associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:142` |
| sym-b084d64930dabbd75c77 | `route_count` | function | Returns the route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:91` |
| sym-aebbad9605773e0d6979 | `route_id` | function | Returns the route identifier held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:36` |
| sym-35d7bf42cf22453bf0ba | `route_id` | function | Returns the route identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:223` |
| sym-d97c1fc6d21b1b6ee568 | `route_id` | function | Returns the route identifier held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:80` |
| sym-c2da65fc0f9a3bff1326 | `route_id` | function | Returns the route identifier held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:146` |
| sym-8ebf756735bf2a581f22 | `route_id` | function | Returns the route identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:243` |
| sym-9711a06de2b1e2932a50 | `route_observations` | function | Returns the route observations held by `PreparedSession`. | `src/session/prepare/prepared.rs:61` |
| sym-5d166d61fb331a73ac95 | `runtime_events_total` | function | Returns the runtime events total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:358` |
| sym-29d0025b90c8d50f78a2 | `runtime_failures_total` | function | Returns the runtime failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:346` |
| sym-40c5a5d0ec258cb0c262 | `runtime_worker_panicked` | function | Returns whether runtime worker panicked is true for `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:342` |
| sym-776bcc49344645af6806 | `safety` | function | Returns the safety held by `NodeDescriptor`. | `src/graph/node.rs:242` |
| sym-f2a89d3128f130689581 | `safety` | function | Returns the safety held by `SourceManifest`. | `src/session/extensions/source.rs:178` |
| sym-abe2c1cc105369d74fe4 | `sample_capacity` | function | Returns the sample capacity held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:19` |
| sym-796ab11c7266286789ac | `sample_count` | function | Returns the sample count held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:23` |
| sym-033d971470da2af0a9d4 | `sample_format` | function | Returns the sample format associated with `EndpointAudioFrame`. | `src/endpoint/contract.rs:51` |
| sym-ecdca2eb323722da79ba | `sample_format` | function | Returns the sample format associated with `PlanEdgeFrame`. | `src/runtime/audio/router.rs:77` |
| sym-5d09422dd96053a2a69f | `sample_rate_hz` | function | Returns the sample rate hertz held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:43` |
| sym-f3d107283c5285a8d9e0 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:235` |
| sym-7bb8bdf536ae8c9f679d | `sample_rate_hz` | function | Returns the sample rate hertz held by `AudioFrame`. | `src/frame/audio.rs:126` |
| sym-780919ce9519b32a0016 | `sample_rate_hz` | function | Returns the sample rate hertz held by `SharedAudioFrame`. | `src/frame/audio.rs:196` |
| sym-54871e067d253d625643 | `sample_rate_hz` | function | Returns the sample rate hertz held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:63` |
| sym-acf20e35e55d599476f4 | `sample_spec` | function | Declares the exact canonical PCM format produced by the configured capture backends and consumed by compiled Session routes. | `src/lib.rs:300` |
| sym-7ca8e0a5f948499f2a15 | `sample_spec` | function | Returns the sample spec held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:59` |
| sym-548ea228160fc75d2bad | `samples` | function | Returns the audio samples held by `ConnectorAudioRecord`. | `src/connector/transport.rs:343` |
| sym-2f0d111a00ca5f86cf50 | `samples` | function | Returns the audio samples held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:55` |
| sym-f612888a97a6eb4f275c | `samples` | function | Returns the audio samples held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:243` |
| sym-d7128c49608f942689a5 | `samples` | function | Returns the audio samples held by `AudioFrame`. | `src/frame/audio.rs:146` |
| sym-55a9f79ef93ef938179a | `samples` | function | Returns the audio samples held by `SharedAudioFrame`. | `src/frame/audio.rs:216` |
| sym-d11b2cc16cb95f091f6e | `samples` | function | Returns the audio samples held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:84` |
| sym-fa9dfa9ff10b46911c0c | `samples` | function | Returns the audio samples held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:38` |
| sym-c39686ced9b7cca3273f | `samples_at_48k` | function | Returns the samples at 48k associated with `OpusFrameDuration`. | `src/codec/encoder.rs:15` |
| sym-086d969e6156417eb9f3 | `samples_mut` | function | Returns the samples mut held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:42` |
| sym-c1f16585dc839e939307 | `schema` | function | Returns the schema held by `SignalSpec`. | `src/graph/signal/spec.rs:223` |
| sym-83ca05faf24683720bbd | `selector_persistence_scope` | function | Reports how long this discovered selector can be reused without rediscovery. The capture owner remains authoritative for opening it. | `src/capture/identity.rs:114` |
| sym-f495eaea1846fa5eb990 | `selector_persistence_scope` | function | Describes how long the selector may be reused without rediscovery. | `src/capture/selection.rs:36` |
| sym-b257850baa8119ad494f | `send` | function | Sends a value through `AsyncOperatorInput`. | `src/runtime/signal/io.rs:39` |
| sym-7337ccc5428be3ae4613 | `send` | function | Sends a value through `StemHandle`. | `src/session/declaration/draft.rs:799` |
| sym-5f2c6c46e31f0ca32b0d | `send` | function | Sends a value through `SourceOutputHandle`. | `src/session/declaration/draft.rs:951` |
| sym-b95b72f1fe25957b56fc | `send` | function | Sends a value through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1069` |
| sym-814212b90982a385a66c | `send` | function | Sends a value through `Stream`. | `src/session/declaration/typed_stream.rs:151` |
| sym-5a7647d65531f376b26a | `send_audio` | function | Sends audio for `AsyncOperatorInput`. | `src/runtime/signal/io.rs:55` |
| sym-593385431638ac87ce8a | `send_to` | function | Connects this stream to one explicit endpoint input port. | `src/session/declaration/draft.rs:804` |
| sym-e11d665e8abe188bbd5a | `send_to` | function | Sends to for `SourceOutputHandle`. | `src/session/declaration/draft.rs:959` |
| sym-8c03ed804d9ab18e4b31 | `send_to` | function | Connects this derived output to one explicit endpoint input port. | `src/session/declaration/draft.rs:1074` |
| sym-5aaaace2d00507e01e3a | `sender_observations` | function | Returns the sender observations held by `PreparedSourceMapping`. | `src/session/prepare/mappings.rs:30` |
| sym-a28a5a383f020397e255 | `sequence_number` | function | Returns the sequence number held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:35` |
| sym-5c04463df13b6837b4e5 | `sequence_number` | function | Returns the sequence number held by `AudioFrame`. | `src/frame/audio.rs:142` |
| sym-a7e53c983feee0f91554 | `sequence_number` | function | Returns the sequence number held by `SharedAudioFrame`. | `src/frame/audio.rs:212` |
| sym-d9a573e9517b85c8a8f3 | `sequence_number` | function | Returns the sequence number held by `FrameLineage`. | `src/frame/lineage.rs:68` |
| sym-47790cb72d92e2c9a65d | `sequence_number` | function | Returns the sequence number held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:90` |
| sym-f61d79d29127e976d995 | `sequence_number` | function | Returns the sequence number held by `SignalLineage`. | `src/graph/signal/lineage.rs:71` |
| sym-84cef5b6533963f55094 | `sequence_number` | function | Returns the sequence number held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:49` |
| sym-480dc2e46b656bb6ea71 | `session_id` | function | Returns the session identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:38` |
| sym-85cdd76ca50eb5baf768 | `session_id` | function | Returns the session identifier held by `RegisteredConnector`. | `src/connector/mod.rs:132` |
| sym-6c7b927d2160ca21ee08 | `session_id` | function | Returns the session identifier held by `EndpointPrepareContext`. | `src/endpoint/runtime.rs:125` |
| sym-90a7dbdce4ff0ac99abe | `session_id` | function | Returns the session identifier held by `FrameLineage`. | `src/frame/lineage.rs:56` |
| sym-620cbbbe1801a55c105f | `session_id` | function | Returns the session identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:59` |
| sym-8ba0649d1f8f05e07392 | `session_id` | function | Returns the session identifier held by `RunningSession`. | `src/lib.rs:798` |
| sym-d9172d9eb89023b5eaa3 | `session_id` | function | Returns the session identifier held by `CompiledSession`. | `src/session/compile/compiled.rs:22` |
| sym-638164b8bc8e589a0acf | `session_id` | function | Returns the session identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:599` |
| sym-db64d4be2edd0760db32 | `session_id` | function | Returns the session identifier held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:721` |
| sym-dc18d2b9349bd24dee39 | `session_id` | function | Returns the session identifier held by `StemHandle`. | `src/session/declaration/draft.rs:791` |
| sym-39047daaf6df32ac7ef7 | `session_id` | function | Returns the session identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:854` |
| sym-4c3a7f8121fd5760f037 | `session_id` | function | Returns the session identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:931` |
| sym-cf199e334715b14b6c69 | `session_id` | function | Returns the session identifier held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1024` |
| sym-885a1679a5ba31e9b5aa | `session_id` | function | Returns the session identifier held by `SessionSpec`. | `src/session/declaration/spec.rs:319` |
| sym-dce847e6607c647c0806 | `session_id` | function | Returns the session identifier held by `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:267` |
| sym-19af4a4c70c51ed1b83e | `session_id` | function | Returns the session identifier held by `SessionEvent`. | `src/session/lifecycle/events.rs:318` |
| sym-388955d6ff8377b092a4 | `session_id` | function | Returns the session identifier held by `RunningSession`. | `src/session/lifecycle/running.rs:197` |
| sym-47353463b40769aeb567 | `session_id` | function | Returns the session identifier held by `SessionTrace`. | `src/session/lifecycle/trace.rs:268` |
| sym-4d981a1abebbc1cd34ce | `session_id` | function | Returns the session identifier held by `PreparedSession`. | `src/session/prepare/prepared.rs:31` |
| sym-3ea0cb1192354749380e | `session_timeline_origin` | function | Returns the session timeline origin associated with `EndpointPrepareContext`. | `src/endpoint/runtime.rs:146` |
| sym-e60011769c414b552d0d | `session_timestamp_ns` | function | Returns the session timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:79` |
| sym-559f88de06bf0479f50d | `session_trace` | function | Enables the bounded Session Session trace recorder. | `src/lib.rs:326` |
| sym-ce9d7f400de076e026f4 | `session_trace_outcome` | function | Returns the session trace outcome held by `RunningSession`. | `src/lib.rs:878` |
| sym-77bd6344321dc6e986d7 | `set_application_backend` | function | Sets the application backend used by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:261` |
| sym-8774dae4ad64c18d818f | `set_bitrate_kbps` | function | Update the live encoder bitrate. Called by CODEC_HINT handler (AUDIO-021). `kbps` = 0 switches to Opus auto (VBR). Safe to call mid-stream. | `src/codec/encoder.rs:280` |
| sym-fb5a243d1ff52e6d66fb | `set_complexity` | function | Set encoder complexity (0 = fastest, 10 = highest quality). | `src/codec/encoder.rs:274` |
| sym-63c54342848999d62803 | `set_connected` | function | Sets the connected used by `ConnectorContext`. | `src/connector/worker/coordination.rs:74` |
| sym-cf48b27584a245401247 | `set_degraded` | function | Sets the degraded used by `ConnectorContext`. | `src/connector/worker/coordination.rs:56` |
| sym-740bbd73e15643480eb9 | `set_healthy` | function | Sets the healthy used by `ConnectorContext`. | `src/connector/worker/coordination.rs:62` |
| sym-a0b96cd19c19a474a51e | `set_microphone_backend` | function | Sets the microphone backend used by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:269` |
| sym-2b8d78ff36bb186d1d53 | `set_not_ready` | function | Sets the not ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:50` |
| sym-5319ff2f9f98c58f7c49 | `set_ready` | function | Sets the ready used by `ConnectorContext`. | `src/connector/worker/coordination.rs:44` |
| sym-2f689a4c0a801493252f | `set_reconnecting` | function | Sets the reconnecting used by `ConnectorContext`. | `src/connector/worker/coordination.rs:68` |
| sym-02c41f0e2a234c86c784 | `set_session_trace` | function | Sets the session trace used by `SessionEngineBuilder`. | `src/session/lifecycle/engine.rs:66` |
| sym-73b19d574cef5d4dd538 | `shared_audio_rejected_total` | function | Returns the shared audio rejected total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:356` |
| sym-3fe113f7e869d83b241a | `shared_ref_count` | function | Returns the shared ref count held by `AudioBufferPool`. | `src/frame/pool.rs:102` |
| sym-d7e58eb464e00e23004c | `shared_ref_count` | function | Returns the shared ref count held by `SharedAudioBufferHandle`. | `src/frame/pool.rs:315` |
| sym-6446238cf23d54b235da | `shutdown` | function | Shuts down `AsyncRuntimeHost` according to its lifecycle contract. | `src/runtime/lifecycle/async_host.rs:91` |
| sym-4a0b2aae18dbc0be8d91 | `shutdown_mode` | function | Returns the shutdown mode held by `ConnectorContext`. | `src/connector/worker/coordination.rs:32` |
| sym-901266c8f4f31ab58f4f | `sidecar` | function | Builds an outbound Connector backed by one bounded sidecar process. | `src/connector/mod.rs:112` |
| sym-15ff5bfbb1415e9a942b | `sidecar_metrics` | function | Returns the sidecar metrics held by `RunningSession`. | `src/lib.rs:841` |
| sym-09bc8d8ec558db9440a4 | `sidecar_metrics` | function | Returns the sidecar metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:239` |
| sym-57abbe9a4e4f19ceae2b | `signal` | function | Returns the signal held by `EndpointRouteContext`. | `src/endpoint/runtime.rs:73` |
| sym-eaec6bfdad1634d716cf | `signal` | function | Returns the signal held by `PortPrepareContext`. | `src/graph/node.rs:349` |
| sym-90d7d530db87c660af44 | `signal` | function | Returns the signal held by `PortSpec`. | `src/graph/ports.rs:221` |
| sym-c8ef6715767732639ce8 | `signal_spec` | function | Returns the signal spec held by `ExtensionSignal`. | `src/conformance.rs:1184` |
| sym-63a584618047103b17fb | `signal_spec` | function | Returns the signal spec held by `ConnectorInputDescriptor`. | `src/connector/worker/driver.rs:44` |
| sym-cb7407e154a82f3e89b3 | `signal_spec` | function | Returns the signal spec held by `EndpointPortInput`. | `src/endpoint/contract.rs:207` |
| sym-4e7e8a1fa845699a0f26 | `signal_spec` | function | Returns the signal spec held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:70` |
| sym-707b29fcd7425f754fa3 | `signal_spec` | function | Returns the signal spec held by `Stream`. | `src/session/declaration/typed_stream.rs:128` |
| sym-c64c86324f00ea62e711 | `signals_dropped_total` | function | Returns the signals dropped total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:332` |
| sym-d6fa4c580c00b1bd0e0e | `signals_enqueued_total` | function | Returns the signals enqueued total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:324` |
| sym-e199425d4e27dd96c9c6 | `signals_received_total` | function | Returns the signals received total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:328` |
| sym-3e13e9e872b74e22420a | `size_bytes` | function | Owned media bytes represented by this payload. Envelope metadata and queue slot storage are fixed-size and accounted separately by the edge. | `src/graph/signal/payload.rs:37` |
| sym-0f15aeec2635e7c05d0c | `slot_count` | function | Returns the slot count held by `AudioBufferPool`. | `src/frame/pool.rs:65` |
| sym-384658cb5081274530d3 | `slot_size` | function | Returns the slot size associated with `AudioBufferPool`. | `src/frame/pool.rs:62` |
| sym-2063e5c7bdc649c7cbd5 | `snapshot` | function | Returns a point-in-time snapshot of `CaptureObservationCounters`. | `src/capture/observations.rs:113` |
| sym-73ace5aff979ab1a8403 | `snapshot` | function | Returns a point-in-time snapshot of `ConnectorObservationHandle`. | `src/connector/observations.rs:53` |
| sym-5978b98570fa78720e42 | `snapshot` | function | Returns a point-in-time snapshot of `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:52` |
| sym-ae863e70faf14262d756 | `snapshot` | function | Returns a point-in-time snapshot of `SourceRuntimeObservationHandle`. | `src/session/extensions/source.rs:412` |
| sym-4ab916f93bbc4bf11a2b | `snapshot` | function | Returns a point-in-time snapshot of `ClockDriftEstimator`. | `src/timing/clock_drift.rs:66` |
| sym-67c45b3e2d80f4b93897 | `source` | function | Declares one instance of an open external source type. | `src/lib.rs:390` |
| sym-85a7007148f652539d83 | `source` | function | Declares one externally implemented source instance. | `src/session/declaration/draft.rs:367` |
| sym-46f355b26a71c5fef035 | `source` | function | Returns the source held by `StemSpec`. | `src/session/declaration/spec.rs:155` |
| sym-049a9e0e3849865eb813 | `source` | function | Returns the source held by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:103` |
| sym-3c79ed6e9c21192fcddc | `source` | function | Returns the source held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:52` |
| sym-42e9ab27532e9ca32ac5 | `source` | function | Returns the source held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:79` |
| sym-35e1f838e3bbf5d0bdfa | `source` | function | Returns the source held by `SessionStartFailure`. | `src/session/lifecycle/start_contract.rs:302` |
| sym-a713c8fdd9070f6e69ac | `source_count` | function | Returns the source count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:75` |
| sym-4a5e3c8e1de56da7a85c | `source_declarations` | function | Returns the source declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:32` |
| sym-4d8a644603f74098d4c2 | `source_failures` | function | Returns the source failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:275` |
| sym-a7abe406c11df0c70626 | `source_generation` | function | Returns the source generation associated with `FrameLineage`. | `src/frame/lineage.rs:77` |
| sym-14f98766e0b55e2653b5 | `source_generation` | function | Returns the source generation associated with `SignalLineage`. | `src/graph/signal/lineage.rs:74` |
| sym-b1421c39551045540b2c | `source_id` | function | Derives the immutable captured-frame identity for this resolved source. | `src/capture/identity.rs:46` |
| sym-32d2a7ed59af69cfb712 | `source_id` | function | Returns the source identifier held by `MacosInputSource`. | `src/capture/platform/macos/input.rs:223` |
| sym-4ec6139dfc44bad9c5ba | `source_id` | function | Returns the source identifier held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:255` |
| sym-67d23693651f132a604e | `source_id` | function | Returns the source identifier held by `DesktopCaptureSource`. | `src/capture/platform/macos/mod.rs:89` |
| sym-4ef49f400751fadaa88b | `source_id` | function | Returns the source identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:27` |
| sym-413007befbdea96468e9 | `source_id` | function | Returns the source identifier held by `AudioFrame`. | `src/frame/audio.rs:122` |
| sym-109944a5099c0b998d76 | `source_id` | function | Returns the source identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:192` |
| sym-90d41b0a81b73aa89b65 | `source_id` | function | Returns the source identifier held by `FrameLineage`. | `src/frame/lineage.rs:59` |
| sym-710524a75f88e53a3c41 | `source_id` | function | Returns the source identifier held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:100` |
| sym-43488b47bd041888b96a | `source_id` | function | Returns the source identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:65` |
| sym-b7cc2b54a01a8c5180c8 | `source_id` | function | Returns the source identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:42` |
| sym-d0abd5830de6dd7bf919 | `source_id` | function | Returns the source identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:862` |
| sym-1b195f1205e0848de35b | `source_id` | function | Returns the source identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:939` |
| sym-cf40309b8b9f180dd6f2 | `source_id` | function | Returns the source identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:80` |
| sym-af9104d2940128c5d8ba | `source_input_count` | function | Returns the source input count held by `PreparedSession`. | `src/session/prepare/prepared.rs:46` |
| sym-ec8404adbd8581e015f3 | `source_instance_id` | function | Returns the source instance identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:935` |
| sym-5b8087d86cb0c3bee6d8 | `source_instance_id` | function | Returns the source instance identifier held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:137` |
| sym-ae48308f843b4d3b29b1 | `source_instances` | function | Returns the source instances associated with `SessionSpec`. | `src/session/declaration/spec.rs:327` |
| sym-97469b7639e84edcb915 | `source_manifest` | function | Returns the validated manifest currently registered for `source_type_id`. | `src/session/lifecycle/engine.rs:171` |
| sym-de67f5f2145766105936 | `source_manifest` | function | Returns the validated source manifest retained by this engine. | `src/session/lifecycle/engine.rs:217` |
| sym-8a02c114e6461c1c19ca | `source_mappings` | function | Returns the source mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:41` |
| sym-3959f42c65362a5408e5 | `source_node_id` | function | Returns the source node identifier held by `PlanSourceInput`. | `src/runtime/audio/runner.rs:196` |
| sym-1e39f60c2fc9cef89efe | `source_observations` | function | Returns the source observations held by `RealtimePlanRunner`. | `src/runtime/audio/runner.rs:349` |
| sym-f9040c21898b424ca678 | `source_outputs` | function | Returns the source outputs held by `SessionSpec`. | `src/session/declaration/spec.rs:331` |
| sym-1519e2ec06a2a7558e9b | `source_send_rejections_total` | function | Returns the source send rejections total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:354` |
| sym-e84693afeae9bdc19d62 | `source_timestamp_ns` | function | Returns the source timestamp nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:71` |
| sym-b9057a6848d5d290196d | `source_to_receive_latency` | function | Returns the source to receive latency associated with `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:223` |
| sym-ee7b8dbe789e0c7b0ebc | `source_type_id` | function | Returns the source type identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:84` |
| sym-0bba47a4fa19b4036db3 | `source_type_id` | function | Returns the source type identifier held by `SourceManifest`. | `src/session/extensions/source.rs:142` |
| sym-5a06df3383e632a7cbf8 | `spawn` | function | Spawns its owned operation for `GeneratedAudioBridge`. | `src/runtime/bridge/audio.rs:131` |
| sym-4eafd5e007133d46792e | `spawn` | function | Spawns its owned operation for `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:174` |
| sym-e15df2672ade067b190c | `spawn` | function | Spawns its owned operation for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:536` |
| sym-835034b86e14d509e436 | `spawn` | function | Spawns its owned operation for `SourceRegistry`. | `src/session/extensions/source.rs:328` |
| sym-d38da82f30374fd95821 | `spawn` | function | Spawns its owned operation for `SourceRuntime`. | `src/session/extensions/source.rs:565` |
| sym-25dc31c898dc120339e8 | `spawn_composed` | function | Spawns composed for `AsyncOperatorWorker`. | `src/runtime/signal/operator.rs:604` |
| sym-110df16dbd7916614c33 | `spawn_with_context` | function | Starts a directly-fed worker with an already negotiated signal-shaped prepare context. Session-owned graph execution uses the compiled-edge path; this entry point exists for external harnesses that negotiate the boundary before constructing a full Session. | `src/runtime/signal/operator.rs:563` |
| sym-7a45cca7a39083387a5d | `spec` | function | Returns the spec associated with `Pipeline`. | `src/graph/dsl.rs:86` |
| sym-f81f3095d83de27e6dd8 | `spec` | function | Returns the spec associated with `CompiledSession`. | `src/session/compile/compiled.rs:27` |
| sym-9752961e5ddb081c1daa | `spec` | function | Returns the spec associated with `PreparedSession`. | `src/session/prepare/prepared.rs:36` |
| sym-0f5477669ed919659664 | `stable_id` | function | Returns the stable identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:59` |
| sym-a2f7e8afbcf6f5834845 | `stage` | function | Returns the stage held by `ConnectorError`. | `src/connector/error.rs:113` |
| sym-5c1728703e1eb4513514 | `stage` | function | Returns the stage held by `EndpointFailure`. | `src/endpoint/runtime.rs:204` |
| sym-128a50f0e7a242e466b7 | `stage` | function | Returns the stage held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:154` |
| sym-99bd0539fc39d430d2af | `stage` | function | Returns the stage held by `SessionRollbackFailure`. | `src/session/lifecycle/events.rs:175` |
| sym-bc255400a33914df5553 | `stage` | function | Returns the stage held by `SessionFinalizationFailure`. | `src/session/lifecycle/events.rs:199` |
| sym-64c701081481a05faa2e | `start` | function | Starts the lifecycle represented by `Session`. | `src/lib.rs:617` |
| sym-674309c9f0fcf3562c76 | `start` | function | Starts the lifecycle represented by `PreparedSourceRuntime`. | `src/session/extensions/source.rs:505` |
| sym-29ed2d94dd71d31d29af | `start` | function | Starts the lifecycle represented by `SessionEngine`. | `src/session/lifecycle/engine.rs:284` |
| sym-f8141b7f5ce4d444fd2e | `start` | function | Starts the lifecycle represented by `SessionEngineHost`. | `src/session/lifecycle/host.rs:60` |
| sym-a142a5c4552258956794 | `start` | function | Starts the lifecycle represented by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:160` |
| sym-5c5f75faf8eeea15b2d2 | `start_cancellable` | function | Starts cancellable for `Session`. | `src/lib.rs:621` |
| sym-8133e38497b5704e0230 | `start_compiled` | function | Starts compiled for `SessionEngine`. | `src/session/lifecycle/engine.rs:234` |
| sym-5562de7d77650f1b76dd | `start_compiled` | function | Starts compiled for `SessionEngineHost`. | `src/session/lifecycle/host.rs:71` |
| sym-99660ab97803eb41799a | `start_compiled_cancellable` | function | Starts compiled cancellable for `SessionEngine`. | `src/session/lifecycle/engine.rs:258` |
| sym-199f34b43bc98924e5c9 | `start_compiled_cancellable` | function | Starts compiled cancellable for `SessionEngineHost`. | `src/session/lifecycle/host.rs:84` |
| sym-7a8283c512cbf4e0aaca | `start_failure` | function | Starts failure for `SessionEngineStartError`. | `src/session/lifecycle/engine.rs:329` |
| sym-3157359ad4c3cebcd1b3 | `startup_timeout` | function | Returns the startup timeout held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:43` |
| sym-445061b1cdeabb0d84fe | `state` | function | Returns the state associated with `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:257` |
| sym-13d24c01034610de423f | `state` | function | Returns the state associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:271` |
| sym-faaa010a8fce3934ca85 | `stats` | function | Returns the current statistics for `CapturedFrameSender`. | `src/capture/frame_stream.rs:138` |
| sym-d03ce71523419d47f749 | `stats` | function | Returns the current statistics for `CapturedFrameStream`. | `src/capture/frame_stream.rs:179` |
| sym-f9518dff6232229a6843 | `stem_id` | function | Returns the stem identifier held by `CaptureLineageSeed`. | `src/capture/capture_owner.rs:42` |
| sym-7da7bec585f5e361fef0 | `stem_id` | function | Returns the stem identifier held by `FrameLineage`. | `src/frame/lineage.rs:62` |
| sym-a327e3f0d06bdc1b7cb3 | `stem_id` | function | Returns the stem identifier held by `SessionSourceFailure`. | `src/session/lifecycle/events.rs:114` |
| sym-b8daa1768b70f889d513 | `stem_id` | function | Returns the stem identifier held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:308` |
| sym-f2f68006652b0408ae51 | `stem_id` | function | Returns the stem identifier held by `PreparedSourceMapping`. | `src/session/prepare/mappings.rs:25` |
| sym-e83524184b8468e1f086 | `stem_id` | function | Returns the stem identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:248` |
| sym-2a471b85614175cf398a | `stems` | function | Returns the stems associated with `SessionSpec`. | `src/session/declaration/spec.rs:323` |
| sym-64020a60cae03227b5c5 | `stereo_broadcast` | function | 20 ms stereo audio transport profile with an explicit bitrate. | `src/codec/encoder.rs:116` |
| sym-146b6565bd66f5ada332 | `stop` | function | Stops `RunningSession` and returns its terminal result. | `src/lib.rs:886` |
| sym-9bbf71ffd69edc57692f | `stop` | function | Stops `RunningSession` and returns its terminal result. | `src/session/lifecycle/running.rs:406` |
| sym-3f7f81492dc47ddf5f22 | `stop_and_join` | function | Stops `CaptureOwner`, joins its worker, and returns the terminal result. | `src/capture/capture_owner.rs:264` |
| sym-bc33a88ba937e5600efe | `stop_and_join` | function | Stops `MacosInputSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/input.rs:235` |
| sym-7c11be2e65aaa2fdfdfa | `stop_and_join` | function | Stops `SystemLoopbackSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/loopback.rs:274` |
| sym-0e5c4a2c142daf106c45 | `stop_and_join` | function | Stops `DesktopCaptureSource`, joins its worker, and returns the terminal result. | `src/capture/platform/macos/mod.rs:103` |
| sym-40ec179be76127e8dcb3 | `stream_id` | function | Returns the stream identifier held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:31` |
| sym-83eabc1a32583616286a | `stream_id` | function | Returns the stream identifier held by `PolledAudioFrame`. | `src/endpoint/polled_audio_driver.rs:227` |
| sym-1a7c40cfd0e0592759b0 | `stream_id` | function | Returns the stream identifier held by `AudioFrame`. | `src/frame/audio.rs:118` |
| sym-95f96b2f2639f19c6149 | `stream_id` | function | Returns the stream identifier held by `SharedAudioFrame`. | `src/frame/audio.rs:188` |
| sym-4b63e7102cfdd7d4aa31 | `stream_id` | function | Returns the stream identifier held by `SignalLineage`. | `src/graph/signal/lineage.rs:62` |
| sym-81b88379be3692cd9338 | `stream_id` | function | Returns the stream identifier held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:35` |
| sym-d94d109d7ca7448fac40 | `stream_id` | function | Returns the stream identifier held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:943` |
| sym-c7467927049ca2a7d49c | `stream_id` | function | Returns the stream identifier held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:145` |
| sym-5a43501bd451af89b2a4 | `success` | function | Returns whether `ConnectorRunOutcome` completed successfully. | `src/connector/worker/mod.rs:51` |
| sym-59b3a78e38624726ca27 | `success_threshold` | function | Returns the success threshold held by `ConnectorReadinessPolicy`. | `src/connector/readiness.rs:51` |
| sym-99ae82bf9dcffcfb428d | `supports` | function | Returns whether supports is true for `SignalPayload`. | `src/graph/signal/payload.rs:17` |
| sym-791a8c729906d51aff5b | `supports_signal` | function | Returns whether supports signal applies to `MediaCaps`. | `src/graph/ports.rs:142` |
| sym-9d40cdc6e238ae01433f | `syntax_version` | function | Returns the syntax version held by `OperatorId`. | `src/graph/operator.rs:35` |
| sym-288593518ef94b3e6b4a | `take_event_receiver` | function | Takes event receiver for `RunningSession`. | `src/session/lifecycle/running.rs:201` |
| sym-4659bc18e2ec63cdbb93 | `take_event_receiver` | function | Takes event receiver for `SessionStartFailure`. | `src/session/lifecycle/start_contract.rs:286` |
| sym-3664c3784dfc92f111c8 | `target` | function | Returns the target associated with `ConnectionSpec`. | `src/session/declaration/spec.rs:267` |
| sym-b413552451c02630cb6b | `text` | function | Convenience constructor for text ports. | `src/graph/signal/spec.rs:279` |
| sym-f7dd4da9046c620dde25 | `through` | function | Routes the current stream through a declared operator using `StemHandle`. | `src/session/declaration/draft.rs:823` |
| sym-969f80a1b7838447bf99 | `through` | function | Routes the current stream through a declared operator using `SourceOutputHandle`. | `src/session/declaration/draft.rs:975` |
| sym-a9d9998fe022fafa82d6 | `through` | function | Routes the current stream through a declared operator using `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1055` |
| sym-1c25d2bbc72173bb69fe | `through` | function | Routes the current stream through a declared operator using `Stream`. | `src/session/declaration/typed_stream.rs:132` |
| sym-c966078a27a3045dcfab | `through_ports` | function | Returns the through ports held by `StemHandle`. | `src/session/declaration/draft.rs:827` |
| sym-5f19d6d99c6223820052 | `through_ports` | function | Returns the through ports held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:979` |
| sym-9dfedbd21d41cce18579 | `through_ports` | function | Returns the through ports held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1059` |
| sym-191791f5a9e29a495b2d | `tick` | function | Applies one measured clock offset to `ClockCorrectionController` and returns the bounded correction. | `src/timing/clock_correction.rs:23` |
| sym-b6b4a844db15ffe22dd2 | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:87` |
| sym-af2b07027a70cb678472 | `timestamp_end_ns` | function | Returns the timestamp end nanoseconds held by `SignalTiming`. | `src/graph/signal/timing.rs:65` |
| sym-d4bec64a2b4084678d0a | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `EndpointAudioFrame`. | `src/endpoint/contract.rs:39` |
| sym-a29004a6bd0080e13a04 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `AudioFrame`. | `src/frame/audio.rs:138` |
| sym-26218b0d80b65da873c1 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SharedAudioFrame`. | `src/frame/audio.rs:208` |
| sym-c119046c1ba5cd1f6ab7 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `SignalEnvelope`. | `src/graph/signal/envelope.rs:110` |
| sym-ef971d108c38935f8e73 | `timestamp_ns` | function | Returns the timestamp nanoseconds held by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:56` |
| sym-64a5722d27d4c081535f | `timestamp_start_ns` | function | Returns the timestamp start nanoseconds held by `FrameLineage`. | `src/frame/lineage.rs:71` |
| sym-374b52ec82d70bd29706 | `timing` | function | Returns the timing associated with `SignalEnvelope`. | `src/graph/signal/envelope.rs:74` |
| sym-23cd1cb19f75e8bd9259 | `to` | function | Returns the destination owned by `PlanEdgeReceiver`. | `src/runtime/audio/router.rs:517` |
| sym-c3b32da3a661cb4eee80 | `topo_order` | function | Returns the topo order associated with `GraphIr`. | `src/graph/ir.rs:51` |
| sym-0dcd8f08977269d04741 | `total_bytes` | function | Returns the total bytes held by `EdgeBufferPlan`. | `src/graph/plan.rs:44` |
| sym-e91d118dbce1fdbf9b18 | `try_acquire` | function | Attempts to acquire through `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:107` |
| sym-e6d409232a31e0b82a02 | `try_acquire` | function | Acquires one preallocated buffer owned by this input. | `src/session/extensions/audio_input/mod.rs:112` |
| sym-ef7332c6f9f647fe5aec | `try_clone` | function | Attempts to clone through `SharedAudioFrame`. | `src/frame/audio.rs:220` |
| sym-a47007a0acae1169e418 | `try_clone` | function | Attempts to clone through `SharedLineagedAudioFrame`. | `src/frame/audio.rs:312` |
| sym-bf3505e13225bc33563e | `try_clone` | function | Attempts to clone through `SharedAudioBufferHandle`. | `src/frame/pool.rs:304` |
| sym-c87a1b1d86e245f5d21b | `try_copy_from_slice` | function | Copies samples into this fixed-capacity slot without panicking. | `src/frame/pool.rs:240` |
| sym-0cc4a5fc5d81a2e8058a | `try_copy_from_slice` | function | Attempts to copy from slice through `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:34` |
| sym-a8af356094c5c830f747 | `try_from` | function | Attempts to from through `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:25` |
| sym-3061c1a5028d4292b75d | `try_new` | function | Creates a new `ConnectorAudioRecord` after validating its inputs. | `src/connector/transport.rs:300` |
| sym-a3ed90fa20535487bd72 | `try_new` | function | Creates a new `AudioFrame` after validating its inputs. | `src/frame/audio.rs:61` |
| sym-103b407fd5cd4c1ca485 | `try_new` | function | Creates a new `FrameLineage` after validating its inputs. | `src/frame/lineage.rs:21` |
| sym-6c4621ccecc3cd2bf2fa | `try_new` | function | Creates a new `SignalLineage` after validating its inputs. | `src/graph/signal/lineage.rs:21` |
| sym-f63ba85284550d4f57f1 | `try_new` | function | Creates a new `SignalTiming` after validating its inputs. | `src/graph/signal/timing.rs:14` |
| sym-7096dce6e4b8356e1b35 | `try_next` | function | Attempts to next through `CapturedFrameStream`. | `src/capture/frame_stream.rs:160` |
| sym-760838edced2ebbd0123 | `try_next_lineaged_frame` | function | Attempts to next lineaged frame through `CaptureOwner`. | `src/capture/capture_owner.rs:213` |
| sym-892b0eb29c32bd2e629a | `try_poll` | function | Attempts to poll through `PolledAudioReceipt`. | `src/endpoint/polled_audio_driver.rs:110` |
| sym-5e1737b9c872e1e5b5c6 | `try_poll_audio` | function | Attempts to poll audio through `RunningSession`. | `src/lib.rs:802` |
| sym-a445cacf8fba42ce4cde | `try_receive_sidecar_signal` | function | Attempts to receive sidecar signal through `RunningSession`. | `src/lib.rs:853` |
| sym-222a4c2e3ed198888240 | `try_receive_sidecar_signal` | function | Attempts to receive sidecar signal through `RunningSession`. | `src/session/lifecycle/running.rs:266` |
| sym-50471b407d97976ae0c3 | `try_receive_signal` | function | Attempts to receive signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:299` |
| sym-1eb7daad98e7e6a2fcdb | `try_recv` | function | Attempts to receive the next value from `SourceRuntimeEventReceiver` without waiting. | `src/capture/events.rs:304` |
| sym-8ad0fb8f103b4dfa1c9d | `try_recv` | function | Attempts to receive the next value from `EndpointAudioReceiver` without waiting. | `src/endpoint/contract.rs:86` |
| sym-21ca6203bd69dfc90d24 | `try_recv` | function | Attempts to receive the next value from `EndpointSignalReceiver` without waiting. | `src/endpoint/contract.rs:132` |
| sym-bac8dedd23fdc447cb38 | `try_recv` | function | Pops one queued frame before sampling the canonical process clock. | `src/runtime/audio/router.rs:545` |
| sym-636fc69959f88e5eed7c | `try_recv` | function | Attempts to receive the next value from `SessionEventReceiver` without waiting. | `src/session/lifecycle/events.rs:506` |
| sym-f36db133109e542f6b7b | `try_recv_event` | function | Attempts to recv event through `RunningSession`. | `src/lib.rs:816` |
| sym-47daeaf065cae2bd061e | `try_recv_for_testing` | function | Attempts to recv for testing through `PlanSourceInput`. | `src/runtime/audio/runner.rs:213` |
| sym-43780d528cfa4194d637 | `try_recv_runtime_event` | function | Attempts to recv runtime event through `CaptureOwner`. | `src/capture/capture_owner.rs:205` |
| sym-07196501368e626e171a | `try_send` | function | Publishes from a capture worker without blocking. When the bounded control channel is full, the newest event is dropped and counted. | `src/capture/events.rs:232` |
| sym-9fb62ce79920ca104da3 | `try_send` | function | Attempts to send a value through `CapturedFrameSender` without waiting for capacity. | `src/capture/frame_stream.rs:109` |
| sym-90aa02ae9f2c137483c3 | `try_send` | function | Attempts to send a value through `PlanSourceSender` without waiting for capacity. | `src/runtime/audio/runner.rs:149` |
| sym-ed55cef307d2732ff00d | `try_send` | function | Attempts to send a value through `AudioInputWriter` without waiting for capacity. | `src/session/extensions/audio_input/buffer.rs:147` |
| sym-df23e272edd796294462 | `try_send` | function | Submits one previously acquired buffer without blocking. | `src/session/extensions/audio_input/mod.rs:122` |
| sym-96891e49ef1c038c36fe | `try_send_sidecar_signal` | function | Attempts to send sidecar signal through `RunningSession`. | `src/lib.rs:845` |
| sym-6b38441c0c7c3671f993 | `try_send_sidecar_signal` | function | Attempts to send sidecar signal through `RunningSession`. | `src/session/lifecycle/running.rs:254` |
| sym-358bf685d21afad964ea | `try_send_signal` | function | Attempts to send signal through `SidecarHost`. | `src/runtime/lifecycle/sidecar_host.rs:265` |
| sym-20dbac7745fd63cf7ef5 | `try_set_len` | function | Changes the visible sample length without panicking. | `src/frame/pool.rs:228` |
| sym-f18ac4d14ef285ce8275 | `try_set_sample_count` | function | Attempts to set sample count through `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:27` |
| sym-2f1a30074ed9996c2a46 | `try_write` | function | Attempts to write through `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:135` |
| sym-5195f06439254b84644f | `try_write` | function | Writes one complete interleaved frame without blocking. | `src/session/extensions/audio_input/mod.rs:117` |
| sym-76b01e53be77e448ec97 | `type_id` | function | Returns the type identifier held by `NodeDescriptor`. | `src/graph/node.rs:222` |
| sym-01f8a81e39a7b2ee2fdb | `type_ids` | function | Returns the type identifiers held by `NodeRegistry`. | `src/graph/registry.rs:179` |
| sym-85ee7c5f479a400c6639 | `type_str` | function | Returns the type str associated with `ResolvedNode`. | `src/graph/ir.rs:19` |
| sym-2a8626d1442fe79ec687 | `typed_edge` | function | Returns the typed edge associated with `RuntimePlan`. | `src/graph/plan.rs:152` |
| sym-ef0d835f0a5d199e2f26 | `underrun_count` | function | Returns the underrun count held by `SystemOutputTelemetry`. | `src/runtime/nodes.rs:61` |
| sym-339c8509b7a1e07e55ad | `untracked` | function | Creates an envelope for data that has not yet entered a source-aware Session. Session sources must attach lineage before routing it. | `src/graph/signal/envelope.rs:17` |
| sym-9ed9b9e32359e86016cf | `upstream_lineage` | function | Returns the upstream lineage held by `SignalDerivation`. | `src/graph/signal/lineage.rs:138` |
| sym-66b72d654eedc1e47b15 | `upstream_timing` | function | Returns the upstream timing associated with `SignalDerivation`. | `src/graph/signal/lineage.rs:141` |
| sym-fec48c2902cf14237c10 | `validate` | function | Validates `ConnectorManifest` against its declared contract. | `src/connector/manifest.rs:160` |
| sym-fa0b35e1926f0614794b | `validate` | function | Validates `SignalEnvelope` against its declared contract. | `src/graph/signal/envelope.rs:117` |
| sym-64bc9b02618ad57280b7 | `validate` | function | Validates `AsyncOperatorManifest` against its declared contract. | `src/graph/signal/operator.rs:238` |
| sym-060ff447535f4d8819a5 | `validate` | function | Validates `SignalSpec` against its declared contract. | `src/graph/signal/spec.rs:328` |
| sym-db3c89f31b476b7c37c7 | `validate` | function | Validates `GeneratedAudioBridgeSpec` against its declared contract. | `src/runtime/bridge/audio.rs:31` |
| sym-8f150f1ab19afbab4458 | `validate` | function | Validates `SidecarMessage` against its declared contract. | `src/runtime/lifecycle/sidecar_protocol.rs:195` |
| sym-d2e472b4502bce94a562 | `validate` | function | Validates `SessionSpec` against its declared contract. | `src/session/declaration/spec.rs:351` |
| sym-b473504e17e352bb7ca6 | `validate` | function | Validates `SourceManifest` against its declared contract. | `src/session/extensions/source.rs:182` |
| sym-edfdd3e6d8304b97a496 | `validate` | function | Validates `SessionTrace` against its declared contract. | `src/session/lifecycle/trace.rs:280` |
| sym-715a37328717b030183b | `validate_config` | function | Validates config for `PassthroughFactory`. | `src/graph/builtins.rs:82` |
| sym-a03fc8ba878c311a2716 | `validate_config` | function | Validates config for `GainFactory`. | `src/graph/builtins.rs:122` |
| sym-317697f91f22bb20b85c | `validate_config` | function | Validates config for `MonoMixFactory`. | `src/graph/builtins.rs:181` |
| sym-f5bd3a902608d86dc6e7 | `validate_config` | function | Validates config for `NodeDefinitionRef`. | `src/graph/registry.rs:47` |
| sym-5e7fd9aee5e3e0cc65ee | `validate_config` | function | Validates config for `SystemOutputSourceFactory`. | `src/runtime/nodes.rs:99` |
| sym-75396090cb01c107cf75 | `validate_config` | function | Validates config for `BridgeSinkFactory`. | `src/runtime/nodes.rs:203` |
| sym-1b6a74280750bad774d0 | `validate_config` | function | Validates config for `SourceRegistry`. | `src/session/extensions/source.rs:313` |
| sym-227e98c682197657ad15 | `validate_frame_sample_count` | function | Validate an interleaved frame length without reading its samples. | `src/codec/encoder.rs:210` |
| sym-dc2ca6e0e3c516ff2f3b | `value` | function | Returns the value held by `SourceInstanceId`. | `src/session/declaration/spec.rs:21` |
| sym-612091b32802c2041171 | `value` | function | Returns the value held by `OperatorInstanceId`. | `src/session/declaration/spec.rs:34` |
| sym-94b5238b1063adc98ddc | `value_kind` | function | Returns the value kind held by `ConnectorConfigurationField`. | `src/connector/configuration.rs:210` |
| sym-0a6fdc0e23446a9a9c1d | `version` | function | Returns the version associated with `SessionSpec`. | `src/session/declaration/spec.rs:315` |
| sym-86242b33bac542fa93ba | `visited` | function | Returns the visited held by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:146` |
| sym-a1257ac50f1e954932ce | `voice_broadcast` | function | Standard 20 ms mono voice transport profile with in-band FEC. | `src/codec/encoder.rs:108` |
| sym-8fc0f5e0f6045de84cce | `wait_for_stop` | function | Waits until a stop request is visible to `ConnectorContext`. | `src/connector/worker/coordination.rs:40` |
| sym-6b1fb9ef0dc70530b16e | `wait_ready` | function | Waits for ready for `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:72` |
| sym-c36dfbd15a22abb906dd | `wait_terminal` | function | Waits for terminal for `AsyncOperatorObservationHandle`. | `src/runtime/signal/observations.rs:86` |
| sym-6ef496a1b40f7398314c | `wire_id` | function | Stable language-neutral identifier for the fundamental wire class. Semantic role and schema remain separate fields. | `src/graph/signal/spec.rs:236` |
| sym-a745759ed1a6838f5b8b | `with` | function | Returns `ConnectorConfiguration` with the supplied entry applied. | `src/connector/configuration.rs:121` |
| sym-8ae3534590506ed86fb5 | `with` | function | Returns `NodeConfig` with the supplied entry applied. | `src/graph/node.rs:66` |
| sym-5109a2150b7d577aa0bb | `with` | function | Returns `EndpointConfiguration` with the supplied entry applied. | `src/session/declaration/endpoint.rs:37` |
| sym-627db520310059eca7ec | `with_backpressure` | function | Sets the backpressure on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:370` |
| sym-afd5dee85ea66002a474 | `with_capability` | function | Sets the capability on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:113` |
| sym-d746482c86b5a710d833 | `with_capture_backends` | function | Creates the standard Session host builder with caller-owned capture backends. This is the reuse seam for CLIs, tests, and platform adapters that decorate native capture without rebuilding Session semantics. | `src/session/lifecycle/host.rs:200` |
| sym-a15f96724e9ad314cc50 | `with_channels` | function | Decoder for an explicit channel layout and a maximum 20 ms packet. | `src/codec/decoder.rs:44` |
| sym-1593a3a186b9c9d20418 | `with_configuration` | function | Sets the configuration on `EndpointDescriptor` and returns the updated value. | `src/session/declaration/endpoint.rs:127` |
| sym-8942f83b86e8489b8716 | `with_constraint` | function | Sets the constraint on `ConnectorConfigurationField` and returns the updated value. | `src/connector/configuration.rs:195` |
| sym-da6d4ffe77b7e932d2d7 | `with_copy_policy` | function | Sets the copy policy on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:375` |
| sym-687cece7a09ec0fb114d | `with_derivation` | function | Sets the derivation on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:57` |
| sym-8a30324e0b3660d9594b | `with_driver` | function | Builds a connector whose bounded receiver loop is owned by Core. | `src/connector/mod.rs:88` |
| sym-26d27b862403373c9c03 | `with_duration_ns` | function | Sets the duration nanoseconds on `SignalTiming` and returns the updated value. | `src/graph/signal/timing.rs:47` |
| sym-16e1073102da090bb83a | `with_external_details` | function | Attaches stable external failure details without changing Endpoint's provider-neutral lifecycle authority. | `src/endpoint/runtime.rs:194` |
| sym-02f8227d01af37900fe0 | `with_input_edge` | function | Declares the bounded delivery policy for routes entering this endpoint. | `src/session/declaration/endpoint.rs:136` |
| sym-323eff431ca9de287a74 | `with_jitter_budget_ms` | function | Sets the jitter budget milliseconds on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:380` |
| sym-70767f46b65e3b7ee5a7 | `with_lineage` | function | Sets the lineage on `SignalEnvelope` and returns the updated value. | `src/graph/signal/envelope.rs:51` |
| sym-70f9f3ea7a5631909337 | `with_max_frame_duration` | function | Decoder with an explicit maximum packet duration. | `src/codec/decoder.rs:53` |
| sym-c943f133ee9a45f550df | `with_max_payload_bytes` | function | Sets the max payload bytes on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:385` |
| sym-66545fc4ae5804efe857 | `with_media` | function | Sets the media on `EdgeContract` and returns the updated value. | `src/graph/ports.rs:365` |
| sym-037be4868ac4e8b742fe | `with_requirement` | function | Sets the requirement on `ConnectorManifest` and returns the updated value. | `src/connector/manifest.rs:119` |
| sym-6e2035240b0754c5d84d | `with_role` | function | Attach a semantic role annotation. | `src/graph/signal/spec.rs:309` |
| sym-d19dc48d1ebe26b02c2e | `with_schema` | function | Attach a schema reference. | `src/graph/signal/spec.rs:315` |
| sym-2dd0dc96d616dce7df18 | `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/graph/node.rs:81` |
| sym-894dd01b22b9142da7db | `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/session/declaration/endpoint.rs:49` |
| sym-fff13687c25d7ca92c1a | `worker_mappings` | function | Returns the worker mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:51` |
| sym-2bf0176c4c44ed570eef | `writer` | function | Returns the writer held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:60` |
| sym-e0a6e3861170bd631e34 | `writer_mut` | function | Returns the writer mut held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:64` |
| sym-3e387f2a2fdf47f09b7d | `audio` | module | Types and operations for audio. | `src/frame/audio.rs:1` |
| sym-d316ebdca2236094d1c8 | `audio` | module | Allocation-free realtime audio execution lane. | `src/runtime/audio/mod.rs:1` |
| sym-4da030ade0b9f5f20efb | `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| sym-2e9979a7622aa06e45c4 | `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| sym-1be476d99603380afb2a | `lifecycle` | module | Non-realtime runtime ownership and process-protocol lifecycle. | `src/runtime/lifecycle/mod.rs:1` |
| sym-9fa6d8a8cc9db89e2fb3 | `lineage` | module | Compact source-aware lineage carried on realtime audio frames. | `src/frame/lineage.rs:1` |
| sym-9a05e045aaa6851693fc | `pocketstation` | module | # PocketStation | `src/lib.rs:1` |
| sym-db747a3f71f6ea35f826 | `pocketstation::codec` | module | Real Opus encode, decode, and packet-loss concealment primitives. | `src/codec/mod.rs:1` |
| sym-3e1ef87e2e37dd6b659f | `pocketstation::conformance` | module | Deterministic canonical-engine fixture for external conformance harnesses. | `src/conformance.rs:1` |
| sym-d23ab2d2747aa3aa43ab | `pocketstation::connector` | module | Types and operations for connector. | `src/connector/mod.rs:1` |
| sym-2fc137327957b96be890 | `pocketstation::graph` | module | Stable signal, port, capability, partition, and extension contracts. | `src/graph/mod.rs:1` |
| sym-08dc481b9163099b3f1e | `pocketstation::runtime::nodes` | module | First-party CLI realtime nodes retained behind `internal-testing`. | `src/runtime/nodes.rs:1` |
| sym-2d75d52c1a60afe46936 | `pocketstation::timing` | module | Timing primitives owned by the PocketStation runtime. | `src/timing/mod.rs:1` |
| sym-30a0f1ed866f7e53eb12 | `pool` | module | Fixed-capacity realtime audio storage and ownership handles. | `src/frame/pool.rs:1` |
| sym-aa210816831d394482ce | `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| sym-8e4bf95b7cdd0b95ce18 | `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| sym-21f51a6de16d9bebcbfb | `signal` | module | Bounded asynchronous signal execution lane. | `src/runtime/signal/mod.rs:1` |
| sym-6cca65de0b51d3ab858e | `pocketstation::RunningSession` | struct | Owns a started Session together with event, polling, recording, trace, and stop resources. | `src/lib.rs:785` |
| sym-1eddbf258be2238a3272 | `pocketstation::Session` | struct | Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it. | `src/lib.rs:232` |
| sym-ac43eaf41f9ecddbe4ed | `pocketstation::SessionBuilder` | struct | Setup-time configuration for the public Rust Session. | `src/lib.rs:271` |
| sym-bb4066a950774df10eb8 | `pocketstation::SessionCancelResult` | struct | Reports the structured session cancel result. | `src/lib.rs:1091` |
| sym-44d006ed2b57b8b0ad28 | `pocketstation::SessionStartError` | struct | Stable façade error for Session startup. | `src/lib.rs:940` |
| sym-f09b9f5cd0aef6c19080 | `pocketstation::SessionStopResult` | struct | Reports the structured session stop result. | `src/lib.rs:1111` |
| sym-9bd58b6b5a2c501b0ef4 | `pocketstation::abi::executable_extension::PksExtensionCallbacks` | struct | Defines the optional function table through which a native extension prepares, runs, stops, and releases instances. | `src/abi/executable_extension.rs:91` |
| sym-1f6f74d8e3a6c6a532c1 | `pocketstation::abi::executable_extension::PksExtensionLibrary` | struct | Owns a loaded native-extension library and the registrations imported from its validated descriptor. | `src/abi/executable_extension.rs:123` |
| sym-618a77e04b1403f70a09 | `pocketstation::abi::executable_extension::PksExtensionPipelineDeclaration` | struct | Describes the extension pipeline declaration contract. | `src/abi/executable_extension.rs:168` |
| sym-2ceb679c2593bc6ff675 | `pocketstation::abi::executable_extension::PksExtensionSignalBuffer` | struct | Provides bounded extension-owned storage for a signal returned through the native ABI. | `src/abi/executable_extension.rs:153` |
| sym-860049bbb07b04ff2964 | `pocketstation::abi::executable_extension::PksExtensionSignalView` | struct | Borrows one signal payload and metadata for delivery into a native-extension callback. | `src/abi/executable_extension.rs:138` |
| sym-105d0da91c28e68138d0 | `pocketstation::abi::extension::PksExtensionAbiVersion` | struct | Carries the major and minor native-extension ABI versions checked during loading. | `src/abi/extension.rs:14` |
| sym-92f78c6ba703b0ab2f66 | `pocketstation::abi::extension::PksExtensionDescriptor` | struct | Describes the extension descriptor contract. | `src/abi/extension.rs:47` |
| sym-f579e6b2c4f5264875fe | `pocketstation::abi::extension::PksExtensionPort` | struct | Describes one native-extension port across the C ABI, including direction and signal metadata. | `src/abi/extension.rs:60` |
| sym-fc0b6bbde02125f27278 | `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| sym-8e9872a8d6c4dec6ba0f | `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Borrows a UTF-8 byte range across the C Session ABI as a pointer and length. | `src/abi/session/abi.rs:101` |
| sym-8fcc600b77a4781feb1f | `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| sym-bda1f6e65224cdffaaba | `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| sym-dff2ac7d1dec9ec6fe1a | `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| sym-714e41ee82c6f1511e60 | `pocketstation::capture::authorization::PermissionEpoch` | struct | Identifies the permission-observation generation attached to captured lineage. | `src/capture/authorization.rs:267` |
| sym-fccbb77f706d93ad2d08 | `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| sym-78605017adba4fbddc4a | `pocketstation::capture::capture_owner::CaptureLineageSeed` | struct | Stable session and stem identity assigned before an exact source is opened. | `src/capture/capture_owner.rs:25` |
| sym-3c0c69c620f6dd6deda0 | `pocketstation::capture::capture_owner::CaptureObservationReceipt` | struct | Retains the identity and observation access returned for capture observation. | `src/capture/capture_owner.rs:167` |
| sym-6ae72c6f6afc98127bca | `pocketstation::capture::capture_owner::CaptureOpenMetadata` | struct | Authoritative lineage state established only after native capture opens. | `src/capture/capture_owner.rs:49` |
| sym-a35ed17eb84ee0dd6c5b | `pocketstation::capture::capture_owner::CaptureOwner` | struct | RAII owner for native capture, its bounded frame stream, and runtime events. | `src/capture/capture_owner.rs:194` |
| sym-4a8719ab63c1ef34fe48 | `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| sym-4c867c58f7305d568f6f | `pocketstation::capture::capture_owner::CapturePrepareRequest` | struct | Setup-time request for one bounded callback-oriented capture owner. | `src/capture/capture_owner.rs:61` |
| sym-9c2561aa4757d483954b | `pocketstation::capture::capture_owner::CaptureStopOutcome` | struct | Final observations returned only after backend stop and join complete. | `src/capture/capture_owner.rs:185` |
| sym-ffb3c52b846161e5c59a | `pocketstation::capture::capture_owner::PreparedCapture` | struct | Prepared capture plus its preallocated delivery endpoints. | `src/capture/capture_owner.rs:119` |
| sym-a79b62eb7cf5031444c1 | `pocketstation::capture::events::CaptureRuntimeFailure` | struct | Reports a capture runtime failure. | `src/capture/events.rs:47` |
| sym-fdac50d90e108fc47f99 | `pocketstation::capture::events::SourceGeneration` | struct | Identifies one appearance generation of a capture source across loss and reappearance. | `src/capture/events.rs:12` |
| sym-b1b9f5177636a52d174c | `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | Owns bounded access to source runtime event observation. | `src/capture/events.rs:200` |
| sym-c79856166f9cb5662eac | `pocketstation::capture::events::SourceRuntimeEventObservations` | struct | Reports the source runtime event observations collected at an observation boundary. | `src/capture/events.rs:111` |
| sym-820c883f3a87ad3206b6 | `pocketstation::capture::events::SourceRuntimeEventReceiver` | struct | Receives source runtime event values across its declared ownership boundary. | `src/capture/events.rs:298` |
| sym-18ae885cdb68fbf9848d | `pocketstation::capture::events::SourceRuntimeEventSender` | struct | Sends source runtime event values across its declared ownership boundary. | `src/capture/events.rs:224` |
| sym-0607c39706b2cfbfb184 | `pocketstation::capture::frame_stream::CaptureDeliveryStartGate` | struct | Read-only one-way start barrier checked by capture delivery callbacks. | `src/capture/frame_stream.rs:54` |
| sym-ba87f1a978171c732fee | `pocketstation::capture::frame_stream::CaptureDeliveryStartGateController` | struct | Session-owned authority that opens one capture delivery start gate. | `src/capture/frame_stream.rs:72` |
| sym-4e0d846572401687d759 | `pocketstation::capture::frame_stream::CapturedFrameObservationHandle` | struct | Owns bounded access to captured frame observation. | `src/capture/frame_stream.rs:31` |
| sym-a36525bd590a744112f0 | `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| sym-db323b040cf0438fd279 | `pocketstation::capture::frame_stream::CapturedFrameStream` | struct | Non-blocking consumer for captured `AudioFrame`s. | `src/capture/frame_stream.rs:154` |
| sym-68f1c481c04b6f6aea69 | `pocketstation::capture::frame_stream::CapturedFrameStreamStats` | struct | Reports the captured frame stream stats collected at an observation boundary. | `src/capture/frame_stream.rs:17` |
| sym-6c2c56e62ec561ad6b99 | `pocketstation::capture::identity::CaptureSource` | struct | Owns production of capture values and its lifecycle state. | `src/capture/identity.rs:82` |
| sym-a29db69c4e99d0c44e31 | `pocketstation::capture::identity::StableSourceId` | struct | Uniquely identifies stable source within its PocketStation ownership scope. | `src/capture/identity.rs:26` |
| sym-de769116ed5e8392b258 | `pocketstation::capture::lifecycle_registry::SourceLifecycleRegistry` | struct | Assigns source generations across complete discovery snapshots. | `src/capture/lifecycle_registry.rs:31` |
| sym-66b06d778e4b81bce7de | `pocketstation::capture::observations::CaptureObservationCounters` | struct | Setup-time cloneable handle; every observation is one relaxed atomic operation and remains allocation-free, lock-free, and log-free. | `src/capture/observations.rs:46` |
| sym-715462cb7417f2b40112 | `pocketstation::capture::observations::CaptureObservationHandle` | struct | Owns bounded access to capture observation. | `src/capture/observations.rs:32` |
| sym-1da6281c321b80d52835 | `pocketstation::capture::observations::CaptureObservations` | struct | Reports the capture observations collected at an observation boundary. | `src/capture/observations.rs:8` |
| sym-1e7720cfde78f1f8b2fe | `pocketstation::capture::platform::macos::DesktopCaptureSource` | struct | Owns production of desktop capture values and its lifecycle state. | `src/capture/platform/macos/mod.rs:33` |
| sym-ff885ee5cdaf51d69b81 | `pocketstation::capture::platform::macos::input::MacosInputSource` | struct | Owns production of macos input values and its lifecycle state. | `src/capture/platform/macos/input.rs:68` |
| sym-9a418761411cb1aa102e | `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| sym-941215720358956ba0e2 | `pocketstation::capture::platform::macos::session_backend::DesktopCaptureBackend` | struct | macOS adapter from the platform-neutral Session capture contract to the existing CoreAudio/input RAII owner. | `src/capture/platform/macos/session_backend.rs:11` |
| sym-d56cebd8ad916b797d1f | `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| sym-0ed3e83b4ab45dd014cb | `pocketstation::capture::timeline::CaptureSampleTimeline` | struct | Source-time clock for capture streams whose media cadence is defined by the number of sample frames produced by the device. | `src/capture/timeline.rs:31` |
| sym-036dc1f9db8126134b2a | `pocketstation::codec::decoder::OpusDecoder` | struct | Real Opus decoder wrapping libopus via the `opus` crate. | `src/codec/decoder.rs:15` |
| sym-da3f913eba209c5fe886 | `pocketstation::codec::encoder::OpusConfig` | struct | Explicit configuration for an Opus encoder instance. | `src/codec/encoder.rs:72` |
| sym-7c89d9af3a6611d2d853 | `pocketstation::codec::encoder::OpusEncoder` | struct | Real Opus encoder wrapping libopus via the `opus` crate. | `src/codec/encoder.rs:157` |
| sym-a2eaab909c0d16144ca7 | `pocketstation::conformance::ExtensionConformanceReport` | struct | Language-neutral outcome returned by the W20 fixture. | `src/conformance.rs:573` |
| sym-8114425d7c7dcbe196b9 | `pocketstation::conformance::ExtensionSignal` | struct | Owns one signal payload used by the native-extension conformance fixtures. | `src/conformance.rs:1181` |
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
| sym-267e3ff822fee19f1ecd | `pocketstation::endpoint::contract::EndpointAudioFrame` | struct | Read-only audio frame delivered to an external endpoint. | `src/endpoint/contract.rs:18` |
| sym-477cdcbb1df07b07f639 | `pocketstation::endpoint::contract::EndpointAudioReceiver` | struct | Exclusive consumer for one bounded realtime-audio endpoint edge. | `src/endpoint/contract.rs:68` |
| sym-382e9fb3581d16a20ccc | `pocketstation::endpoint::contract::EndpointPortInput` | struct | Carries typed input for endpoint port. | `src/endpoint/contract.rs:153` |
| sym-627c94191cfef9a46728 | `pocketstation::endpoint::contract::EndpointSignalReceiver` | struct | Exclusive consumer for one bounded asynchronous signal endpoint edge. | `src/endpoint/contract.rs:123` |
| sym-5f2952d6944973cc1c88 | `pocketstation::endpoint::identity::EndpointGroupId` | struct | Explicit Session-scoped grouping key for endpoints that share one lifecycle. | `src/endpoint/identity.rs:9` |
| sym-fbcb718abdda5a26510c | `pocketstation::endpoint::polled_audio::PolledAudioEndpoint` | struct | Safe composition owner for application-polled audio. | `src/endpoint/polled_audio.rs:16` |
| sym-d40c1e0748d919e3234f | `pocketstation::endpoint::polled_audio_driver::PolledAudioBatchLease` | struct | Owns bounded access to polled audio batch. | `src/endpoint/polled_audio_driver.rs:172` |
| sym-a80ddf76df17c927b68e | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfig` | struct | Configures polled audio endpoint behavior at its owning API boundary. | `src/endpoint/polled_audio_driver.rs:23` |
| sym-434dbbbc531976bf9796 | `pocketstation::endpoint::polled_audio_driver::PolledAudioFrame` | struct | Carries one polled audio payload together with its declared metadata. | `src/endpoint/polled_audio_driver.rs:210` |
| sym-9b68fb7793859b5af3b3 | `pocketstation::endpoint::polled_audio_driver::PolledAudioObservations` | struct | Reports the polled audio observations collected at an observation boundary. | `src/endpoint/polled_audio_driver.rs:56` |
| sym-ffbe18ac1475853536f7 | `pocketstation::endpoint::polled_audio_driver::PolledAudioReceipt` | struct | Retains the identity and observation access returned for polled audio. | `src/endpoint/polled_audio_driver.rs:105` |
| sym-1acd6566beddec2b1ad1 | `pocketstation::endpoint::registry::EndpointDriverRegistry` | struct | Indexes registered endpoint driver implementations by their stable identities. | `src/endpoint/registry.rs:54` |
| sym-f4b8e6b72352b88ed6ad | `pocketstation::endpoint::runtime::EndpointCancellationOutcome` | struct | Reports the structured endpoint cancellation outcome. | `src/endpoint/runtime.rs:289` |
| sym-80686ff8b5ac0125a741 | `pocketstation::endpoint::runtime::EndpointDriverFinalization` | struct | Reports an endpoint driver's terminal observations and any finalization failure. | `src/endpoint/runtime.rs:295` |
| sym-707209feeb55089f10f8 | `pocketstation::endpoint::runtime::EndpointDriverObservations` | struct | Reports the endpoint driver observations collected at an observation boundary. | `src/endpoint/runtime.rs:228` |
| sym-76f91d197c51f7d96af2 | `pocketstation::endpoint::runtime::EndpointFailure` | struct | Reports a endpoint failure. | `src/endpoint/runtime.rs:174` |
| sym-c22f8a7564d7a5b121a2 | `pocketstation::endpoint::runtime::EndpointFinalizationOutcome` | struct | Reports the structured endpoint finalization outcome. | `src/endpoint/runtime.rs:301` |
| sym-eef74ac3cc89e0fc9134 | `pocketstation::endpoint::runtime::EndpointPrepareContext` | struct | Carries the inputs and runtime context required to endpoint prepare. | `src/endpoint/runtime.rs:98` |
| sym-f4ca7bd5d5eea77667f1 | `pocketstation::endpoint::runtime::EndpointRouteContext` | struct | Typed Session route identity supplied to every endpoint input. | `src/endpoint/runtime.rs:44` |
| sym-d7394b735bd27ea67b1e | `pocketstation::endpoint::runtime::EndpointStartFailure` | struct | Reports a endpoint start failure. | `src/endpoint/runtime.rs:443` |
| sym-dd6611b464415b86d8bf | `pocketstation::endpoint::runtime::EndpointStartGate` | struct | Read-only one-way start barrier shared by endpoint drivers in one startup. | `src/endpoint/runtime.rs:371` |
| sym-8246b27fde41c7114b61 | `pocketstation::endpoint::runtime::PreparedEndpoint` | struct | Owns endpoint resources after preparation and before its runtime driver starts. | `src/endpoint/runtime.rs:405` |
| sym-0199ce6a3f14bc6410ee | `pocketstation::endpoint::runtime::RunningEndpoint` | struct | Owns a started endpoint driver until shutdown and finalization complete. | `src/endpoint/runtime.rs:481` |
| sym-9a2d09d1aa729ae9ad87 | `pocketstation::endpoint::runtime::SessionTimelineOrigin` | struct | One Session-owned anchor in PocketStation's monotonic nanosecond clock. | `src/endpoint/runtime.rs:13` |
| sym-fc1f92d68a20921380f3 | `pocketstation::frame::audio::AudioFrame` | struct | Carries one audio payload together with its declared metadata. | `src/frame/audio.rs:39` |
| sym-e84366f3ca396d712e65 | `pocketstation::frame::audio::LineagedAudioFrame` | struct | An exclusive audio frame and the immutable lineage snapshot captured before the frame crosses a bounded edge. | `src/frame/audio.rs:266` |
| sym-1745667d6843cfaaf2e6 | `pocketstation::frame::audio::SampleSpec` | struct | Configures sample behavior at its owning API boundary. | `src/frame/audio.rs:18` |
| sym-55e4e4b2bf2b792d424b | `pocketstation::frame::audio::SharedAudioFrame` | struct | Carries one shared audio payload together with its declared metadata. | `src/frame/audio.rs:176` |
| sym-ec68dca3da1e27df59ad | `pocketstation::frame::audio::SharedLineagedAudioFrame` | struct | Carries one shared lineaged audio payload together with its declared metadata. | `src/frame/audio.rs:298` |
| sym-cf299e85b856722d5c82 | `pocketstation::frame::identity::ClockDomainId` | struct | Uniquely identifies clock domain within its PocketStation ownership scope. | `src/frame/identity.rs:29` |
| sym-54e80634d3c8f1d68c51 | `pocketstation::frame::identity::ConnectorId` | struct | Uniquely identifies connector within its PocketStation ownership scope. | `src/frame/identity.rs:25` |
| sym-329abdbf68985263c68e | `pocketstation::frame::identity::EndpointId` | struct | Uniquely identifies endpoint within its PocketStation ownership scope. | `src/frame/identity.rs:24` |
| sym-081d3a4abc3e6c70e88d | `pocketstation::frame::identity::RouteId` | struct | Uniquely identifies route within its PocketStation ownership scope. | `src/frame/identity.rs:26` |
| sym-e0967f4574129b9a2554 | `pocketstation::frame::identity::SessionId` | struct | Uniquely identifies session within its PocketStation ownership scope. | `src/frame/identity.rs:22` |
| sym-d50d47dff9e2a7898f88 | `pocketstation::frame::identity::SourceId` | struct | Uniquely identifies source within its PocketStation ownership scope. | `src/frame/identity.rs:21` |
| sym-383b7bf043152137963b | `pocketstation::frame::identity::StemId` | struct | Uniquely identifies stem within its PocketStation ownership scope. | `src/frame/identity.rs:23` |
| sym-bc74b3d384bea7406755 | `pocketstation::frame::identity::StreamId` | struct | Uniquely identifies stream within its PocketStation ownership scope. | `src/frame/identity.rs:20` |
| sym-c34282f9a743c50c5b56 | `pocketstation::frame::lineage::FrameLineage` | struct | Preserves source, stream, sequence, clock, generation, and discontinuity identity for an audio frame. | `src/frame/lineage.rs:6` |
| sym-de7b5a299f41a9ec535b | `pocketstation::frame::pool::AudioBufferHandle` | struct | Owns bounded access to audio buffer. | `src/frame/pool.rs:198` |
| sym-3537e580cc2bbebe2007 | `pocketstation::frame::pool::AudioBufferPool` | struct | Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame. | `src/frame/pool.rs:24` |
| sym-d167211f444b21d144e8 | `pocketstation::frame::pool::SharedAudioBufferHandle` | struct | Owns bounded access to shared audio buffer. | `src/frame/pool.rs:281` |
| sym-e9b4ed91b9dd9e9693d5 | `pocketstation::graph::builtins::GainFactory` | struct | Constructs gain implementations from validated declarations. | `src/graph/builtins.rs:107` |
| sym-9138021fd69def65c431 | `pocketstation::graph::builtins::GainNode` | struct | Executes the graph-node behavior defined for gain. | `src/graph/builtins.rs:149` |
| sym-abf1724908e924cd339e | `pocketstation::graph::builtins::MonoMixFactory` | struct | Constructs mono mix implementations from validated declarations. | `src/graph/builtins.rs:166` |
| sym-1e8022df7b2bdd8194c2 | `pocketstation::graph::builtins::MonoMixNode` | struct | Executes the graph-node behavior defined for mono mix. | `src/graph/builtins.rs:194` |
| sym-16369a8aa1320fa11c38 | `pocketstation::graph::builtins::PassthroughFactory` | struct | Constructs passthrough implementations from validated declarations. | `src/graph/builtins.rs:67` |
| sym-95378ef44adc52fb1604 | `pocketstation::graph::builtins::PassthroughNode` | struct | Executes the graph-node behavior defined for passthrough. | `src/graph/builtins.rs:95` |
| sym-8d7d9b1faf10a2b82106 | `pocketstation::graph::compile::plan::RuntimePlanner` | struct | Validates the graph and produces the bounded runtime execution and memory plan. | `src/graph/compile/plan.rs:11` |
| sym-68c17b53e51b9a38892c | `pocketstation::graph::compile::resolve::Compiler` | struct | Runs the ordered graph-validation passes that resolve a graph specification into executable IR. | `src/graph/compile/resolve.rs:444` |
| sym-5fb1a08175aae75072c8 | `pocketstation::graph::dsl::NodeHandle` | struct | Owns bounded access to node. | `src/graph/dsl.rs:10` |
| sym-e7e10f26cc71831ff9f3 | `pocketstation::graph::dsl::Pipeline` | struct | Builds typed operator connections on a Session while preserving port and signal contracts. | `src/graph/dsl.rs:33` |
| sym-4a7a64b246002ea2c449 | `pocketstation::graph::ir::GraphIr` | struct | Stores the resolved nodes, edges, and topological order used by runtime planning. | `src/graph/ir.rs:32` |
| sym-3e3fc8f1dee97a455d8d | `pocketstation::graph::ir::ResolvedEdge` | struct | Binds one compiled graph edge to its resolved source, destination, and contract. | `src/graph/ir.rs:25` |
| sym-760020cb75191c019ca7 | `pocketstation::graph::ir::ResolvedNode` | struct | Executes the graph-node behavior defined for resolved. | `src/graph/ir.rs:10` |
| sym-702e8e4842c3c0cdc9b8 | `pocketstation::graph::node::NodeConfig` | struct | Configures node behavior at its owning API boundary. | `src/graph/node.rs:43` |
| sym-3cb0290118edc5d0de76 | `pocketstation::graph::node::NodeDescriptor` | struct | Describes the node descriptor contract. | `src/graph/node.rs:165` |
| sym-e6fbf54fe5d552b87771 | `pocketstation::graph::node::NodeTypeId` | struct | Uniquely identifies node type within its PocketStation ownership scope. | `src/graph/node.rs:13` |
| sym-ee14bc2e1d88ef685c4c | `pocketstation::graph::node::PortPrepareContext` | struct | Exact graph-owned contract for one prepared node port. | `src/graph/node.rs:282` |
| sym-af4fe91c32a35b9d7323 | `pocketstation::graph::node::PrepareContext` | struct | Carries the inputs and runtime context required to prepare. | `src/graph/node.rs:266` |
| sym-7ed29d02c45d61e7ca26 | `pocketstation::graph::operator::OperatorId` | struct | Open identifier for a registered graph operator implementation. | `src/graph/operator.rs:16` |
| sym-0e261352558bec60d885 | `pocketstation::graph::plan::EdgeBufferPlan` | struct | Records the compiled execution and resource plan for edge buffer. | `src/graph/plan.rs:36` |
| sym-85c056af48f9331d937e | `pocketstation::graph::plan::EdgeMetricId` | struct | Uniquely identifies edge metric within its PocketStation ownership scope. | `src/graph/plan.rs:33` |
| sym-9e676868be4fa0973067 | `pocketstation::graph::plan::FanInGroup` | struct | Groups the compiled edges mixed into one input port. | `src/graph/plan.rs:90` |
| sym-7ccfac0e4f930c514c30 | `pocketstation::graph::plan::FanOutGroup` | struct | Groups the compiled edges that share one output port as their origin. | `src/graph/plan.rs:84` |
| sym-e6b149b05732b507e868 | `pocketstation::graph::plan::MemoryPlan` | struct | Records the compiled execution and resource plan for memory. | `src/graph/plan.rs:64` |
| sym-1f65b825d2418bede2a4 | `pocketstation::graph::plan::PartitionGroup` | struct | A group of nodes assigned to the same execution partition in a compiled plan. | `src/graph/plan.rs:78` |
| sym-2a050143cba82c51fb88 | `pocketstation::graph::plan::RuntimePlan` | struct | Records the compiled execution and resource plan for runtime. | `src/graph/plan.rs:121` |
| sym-dc60628cb45b96719653 | `pocketstation::graph::plan::SourceOutputPlan` | struct | One connected output of a graph root that runtime preparation must feed. | `src/graph/plan.rs:113` |
| sym-785c56af44cc90993880 | `pocketstation::graph::plan::TypedEdgePlan` | struct | Records the compiled execution and resource plan for typed edge. | `src/graph/plan.rs:96` |
| sym-e3b8fb2e4969b955d24b | `pocketstation::graph::ports::AudioCaps` | struct | Declares the sample formats, channel layouts, and rates accepted by an audio port. | `src/graph/ports.rs:48` |
| sym-d67e44734c7dbe7b92f4 | `pocketstation::graph::ports::EdgeContract` | struct | Declares the validated constraints applied to edge. | `src/graph/ports.rs:311` |
| sym-d8649d52f1eedef21143 | `pocketstation::graph::ports::PortSpec` | struct | Configures port behavior at its owning API boundary. | `src/graph/ports.rs:175` |
| sym-43cc78bc376e36d1d4cb | `pocketstation::graph::registry::NodeRegistry` | struct | Indexes registered node implementations by their stable identities. | `src/graph/registry.rs:67` |
| sym-fb50305c1d7896ca2eea | `pocketstation::graph::signal::continuity::SignalContinuityObservation` | struct | Reports sequence or timestamp continuity observed for one signal stream. | `src/graph/signal/continuity.rs:6` |
| sym-f9d38b8d55d785f4fd14 | `pocketstation::graph::signal::continuity::SignalContinuityTracker` | struct | Tracks sequence and timing progress so discontinuities remain observable. | `src/graph/signal/continuity.rs:13` |
| sym-37459f723646a1086699 | `pocketstation::graph::signal::envelope::SignalEnvelope` | struct | Carries a typed signal payload together with timing, lineage, continuity, and terminal metadata. | `src/graph/signal/envelope.rs:6` |
| sym-4ad350aa3e6504103d90 | `pocketstation::graph::signal::lineage::SignalDerivation` | struct | Source-independent record of the signal consumed by an operator. | `src/graph/signal/lineage.rs:97` |
| sym-2d7c967df1ae78ef6c7e | `pocketstation::graph::signal::lineage::SignalLineage` | struct | Preserves source, stream, generation, discontinuity, and policy identity across signal processing. | `src/graph/signal/lineage.rs:8` |
| sym-8a75c57c79e9dbb9b93c | `pocketstation::graph::signal::operator::AsyncOperatorManifest` | struct | Describes the async operator manifest contract. | `src/graph/signal/operator.rs:127` |
| sym-00cc38841823e922c202 | `pocketstation::graph::signal::operator::OperatorDeadlinePolicy` | struct | Configures operator deadline behavior at its owning API boundary. | `src/graph/signal/operator.rs:52` |
| sym-711fcad62c474199fb3c | `pocketstation::graph::signal::operator::OperatorOutputRolePolicy` | struct | Configures operator output role behavior at its owning API boundary. | `src/graph/signal/operator.rs:69` |
| sym-710a3c56223b20241720 | `pocketstation::graph::signal::operator::OperatorPermissionPolicy` | struct | Configures operator permission behavior at its owning API boundary. | `src/graph/signal/operator.rs:46` |
| sym-560e212809ec344b2752 | `pocketstation::graph::signal::preparation::AsyncOperatorPrepareContext` | struct | Complete graph-owned preparation contract for one asynchronous Operator. | `src/graph/signal/preparation.rs:22` |
| sym-5d7929c6fd69ca7cd1cc | `pocketstation::graph::signal::spec::SchemaRef` | struct | Reference to an external schema document. | `src/graph/signal/spec.rs:87` |
| sym-d674d8fe8e1244f2acd5 | `pocketstation::graph::signal::spec::SemanticRole` | struct | Semantic role annotation on a port. | `src/graph/signal/spec.rs:57` |
| sym-0314405456bf175ba218 | `pocketstation::graph::signal::spec::SignalId` | struct | Opaque identifier for a custom signal type. | `src/graph/signal/spec.rs:22` |
| sym-8da0af19f03d9e3acb21 | `pocketstation::graph::signal::spec::SignalSpec` | struct | Full signal contract for a single port. | `src/graph/signal/spec.rs:205` |
| sym-77d7538603d53c1db9e3 | `pocketstation::graph::signal::timing::SignalTiming` | struct | Carries a signal timestamp, clock domain, and timing semantics without rewriting source lineage. | `src/graph/signal/timing.rs:6` |
| sym-1c59dc8283e62cd7a51a | `pocketstation::graph::spec::EdgeId` | struct | Uniquely identifies edge within its PocketStation ownership scope. | `src/graph/spec.rs:22` |
| sym-01f0c56bd84e4302f09a | `pocketstation::graph::spec::EdgeSpec` | struct | Configures edge behavior at its owning API boundary. | `src/graph/spec.rs:50` |
| sym-145206287fc5511da2ca | `pocketstation::graph::spec::GraphSpec` | struct | Configures graph behavior at its owning API boundary. | `src/graph/spec.rs:58` |
| sym-e14c57c74bfe721843c3 | `pocketstation::graph::spec::InputPortRef` | struct | Names an operator or endpoint input port used as the target of a graph connection. | `src/graph/spec.rs:37` |
| sym-a6060603ece5242a6a69 | `pocketstation::graph::spec::NodeId` | struct | Uniquely identifies node within its PocketStation ownership scope. | `src/graph/spec.rs:8` |
| sym-3834dc5d94dd3feafd00 | `pocketstation::graph::spec::NodeSpec` | struct | Configures node behavior at its owning API boundary. | `src/graph/spec.rs:43` |
| sym-63017f9de8da0d03ad5e | `pocketstation::graph::spec::OutputPortRef` | struct | Names an operator output port used as the origin of a graph connection. | `src/graph/spec.rs:31` |
| sym-daadf87bca13446b6fe2 | `pocketstation::native_extension::NativeExtensionLibrary` | struct | Immutable receipt for registrations imported into one Session. Executable code ownership remains internal to the registered factories and drivers. | `src/native_extension/mod.rs:62` |
| sym-f5c3060c94d92cd71443 | `pocketstation::native_extension::NativeExtensionLibraryError` | struct | Reports a native extension library error. | `src/native_extension/mod.rs:124` |
| sym-5dd141e95bf3c2701bdf | `pocketstation::native_extension::NativeExtensionRegistration` | struct | Identifies one node registration imported transactionally from a native extension. | `src/native_extension/mod.rs:34` |
| sym-310a3085646b09a76102 | `pocketstation::recording::config::RecorderStemConfig` | struct | Configures recorder stem behavior at its owning API boundary. | `src/recording/config.rs:55` |
| sym-7a5d9b850a0a307684ee | `pocketstation::recording::config::StemLabel` | struct | Stores the validated human-readable label used for one recording stem. | `src/recording/config.rs:20` |
| sym-7f9542d3376d16437ab7 | `pocketstation::recording::endpoint::MultistemRecordingReceipt` | struct | Retains the identity and observation access returned for multistem recording. | `src/recording/endpoint.rs:28` |
| sym-23af3968ee72443beb9e | `pocketstation::recording::endpoint::SessionMultistemEndpointCoordinator` | struct | Canonical Session-owned multistem recorder declaration. | `src/recording/endpoint.rs:49` |
| sym-8395072f714378c544cb | `pocketstation::recording::writer::DiscontinuityRecord` | struct | Records one immutable discontinuity observation. | `src/recording/writer.rs:92` |
| sym-bcc0c641f0c70c19d325 | `pocketstation::recording::writer::MultistemRecording` | struct | Owns the per-stem recording workers and coordinates their terminal finalization outcome. | `src/recording/writer.rs:138` |
| sym-712535870fc618757b72 | `pocketstation::recording::writer::RecordingObservations` | struct | Reports the recording observations collected at an observation boundary. | `src/recording/writer.rs:130` |
| sym-bf5ce1b3444891dee634 | `pocketstation::recording::writer::RecordingOutcome` | struct | Reports the structured recording outcome. | `src/recording/writer.rs:111` |
| sym-94317d1483a96e604808 | `pocketstation::recording::writer::RecordingStemOutcome` | struct | Reports the structured recording stem outcome. | `src/recording/writer.rs:120` |
| sym-999e5225deb92676fab9 | `pocketstation::runtime::audio::executor::PlanExecutionSummary` | struct | Reports the counters and terminal facts collected for plan execution. | `src/runtime/audio/executor.rs:37` |
| sym-12c9dcbeb63ae553697d | `pocketstation::runtime::audio::executor::RealtimePlanExecutor` | struct | Executes realtime plan according to its compiled plan and cancellation contract. | `src/runtime/audio/executor.rs:54` |
| sym-85ed70417f4479b8653a | `pocketstation::runtime::audio::router::DispatchSummary` | struct | Reports the counters and terminal facts collected for dispatch. | `src/runtime/audio/router.rs:667` |
| sym-5ea2d61ac8d5c673c22a | `pocketstation::runtime::audio::router::EdgeObservations` | struct | Reports the edge observations collected at an observation boundary. | `src/runtime/audio/router.rs:122` |
| sym-ba8a781437dfc25bb2a3 | `pocketstation::runtime::audio::router::PlanEdgeObservationHandle` | struct | Cloneable read-only access to one plan edge's authoritative live telemetry. | `src/runtime/audio/router.rs:211` |
| sym-84a3d7f08e769240b8df | `pocketstation::runtime::audio::router::PlanEdgeReceiver` | struct | Receives plan edge values across its declared ownership boundary. | `src/runtime/audio/router.rs:488` |
| sym-812845848a90a3ff1c95 | `pocketstation::runtime::audio::router::PlanEdgeRouter` | struct | Routes plan edge according to the compiled edge contracts. | `src/runtime/audio/router.rs:675` |
| sym-60530e211a28aa76f701 | `pocketstation::runtime::audio::runner::PlanRunnerCancellation` | struct | Shares a lock-free cancellation flag between the Session owner and the realtime plan runner. | `src/runtime/audio/runner.rs:89` |
| sym-d4f96dfbab67ca6d1216 | `pocketstation::runtime::audio::runner::PlanRunnerFinishSummary` | struct | Reports the counters and terminal facts collected for plan runner finish. | `src/runtime/audio/runner.rs:298` |
| sym-1ded08bd8c65cfa44bf8 | `pocketstation::runtime::audio::runner::PlanRunnerStepSummary` | struct | Reports the counters and terminal facts collected for plan runner step. | `src/runtime/audio/runner.rs:270` |
| sym-c73d020407481f9efb27 | `pocketstation::runtime::audio::runner::PlanSourceInput` | struct | Carries typed input for plan source. | `src/runtime/audio/runner.rs:188` |
| sym-dae899aaf0c5cef7c0f5 | `pocketstation::runtime::audio::runner::PlanSourceInputObservations` | struct | Reports the plan source input observations collected at an observation boundary. | `src/runtime/audio/runner.rs:22` |
| sym-e6ff4c4b5f87f0b09a0d | `pocketstation::runtime::audio::runner::PlanSourceObservationHandle` | struct | Owns bounded access to plan source observation. | `src/runtime/audio/runner.rs:138` |
| sym-bfc959b8328fdda70b43 | `pocketstation::runtime::audio::runner::PlanSourceSender` | struct | Sends plan source values across its declared ownership boundary. | `src/runtime/audio/runner.rs:131` |
| sym-96fd49ef30d198f18d09 | `pocketstation::runtime::audio::runner::RealtimePlanRunner` | struct | Executes realtime plan according to its compiled plan and cancellation contract. | `src/runtime/audio/runner.rs:305` |
| sym-04b9b36542cd1ad434e3 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridge` | struct | Transfers generated audio across the bounded runtime boundary it owns. | `src/runtime/bridge/audio.rs:123` |
| sym-e182a317efb0cc92b5fa | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeSpec` | struct | Configures generated audio bridge behavior at its owning API boundary. | `src/runtime/bridge/audio.rs:19` |
| sym-693e742a932cc425aa5e | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHost` | struct | Session-owned async executor for connector and derived-endpoint lifecycle. | `src/runtime/lifecycle/async_host.rs:26` |
| sym-fcbe913cd3ea765735f8 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarDeadlines` | struct | Sets finite startup, I/O, shutdown, and reap deadlines for a sidecar process. | `src/runtime/lifecycle/sidecar_host.rs:54` |
| sym-80e32f1f181d73c9528a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHost` | struct | Owns the resources and lifecycle for sidecar. | `src/runtime/lifecycle/sidecar_host.rs:157` |
| sym-64b0da52e710cf5e9e7f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostObservations` | struct | Reports the sidecar host observations collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:109` |
| sym-c2c74acc46582b437cc5 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostSnapshot` | struct | Reports the sidecar host snapshot collected at an observation boundary. | `src/runtime/lifecycle/sidecar_host.rs:133` |
| sym-ae5681d427e2460827a8 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarProcessSpec` | struct | Configures sidecar process behavior at its owning API boundary. | `src/runtime/lifecycle/sidecar_host.rs:71` |
| sym-72e7f904ad10284bea77 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessage` | struct | Carries one typed control or signal message across the sidecar protocol. | `src/runtime/lifecycle/sidecar_protocol.rs:73` |
| sym-0e71be2989315373af71 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolLimits` | struct | Sets the maximum sidecar message and buffered-byte sizes enforced by protocol I/O. | `src/runtime/lifecycle/sidecar_protocol.rs:43` |
| sym-ceb3a7366dc723836d62 | `pocketstation::runtime::nodes::BridgeSinkFactory` | struct | Constructs bridge sink implementations from validated declarations. | `src/runtime/nodes.rs:170` |
| sym-5837fc53ff8656dcc0e1 | `pocketstation::runtime::nodes::BridgeSinkTelemetry` | struct | Reports the counters and terminal facts collected for bridge sink. | `src/runtime/nodes.rs:156` |
| sym-6ea036855374732d5982 | `pocketstation::runtime::nodes::MixerSourceNode` | struct | Executes the graph-node behavior defined for mixer source. | `src/runtime/nodes.rs:268` |
| sym-8f3a2ff7ff34b38a6974 | `pocketstation::runtime::nodes::MixerTelemetry` | struct | Reports the counters and terminal facts collected for mixer. | `src/runtime/nodes.rs:246` |
| sym-16cef5cbc791f7bc71da | `pocketstation::runtime::nodes::SystemOutputSourceFactory` | struct | Constructs system output source implementations from validated declarations. | `src/runtime/nodes.rs:66` |
| sym-2d93d5b64b1ecb3c420c | `pocketstation::runtime::nodes::SystemOutputTelemetry` | struct | Reports the counters and terminal facts collected for system output. | `src/runtime/nodes.rs:48` |
| sym-435da8b9f25b9a3af8f0 | `pocketstation::runtime::signal::edge::SignalEdgeObservationHandle` | struct | Owns bounded access to signal edge observation. | `src/runtime/signal/edge.rs:49` |
| sym-128c118d3e5e40a57214 | `pocketstation::runtime::signal::edge::SignalEdgeObservations` | struct | Reports the signal edge observations collected at an observation boundary. | `src/runtime/signal/edge.rs:31` |
| sym-f5747ff63e42fa3fab3f | `pocketstation::runtime::signal::edge::SignalEdgeReceiver` | struct | Receives signal edge values across its declared ownership boundary. | `src/runtime/signal/edge.rs:204` |
| sym-f187b0898dc003b414fe | `pocketstation::runtime::signal::edge::SignalEdgeSendError` | struct | Reports a signal edge send error. | `src/runtime/signal/edge.rs:118` |
| sym-24f3dbc05bc9502a77e7 | `pocketstation::runtime::signal::edge::TypedEdgeBranchSpec` | struct | Configures typed edge branch behavior at its owning API boundary. | `src/runtime/signal/edge.rs:248` |
| sym-610a8bb218db6be8e3b9 | `pocketstation::runtime::signal::edge::TypedEdgeFanout` | struct | Publishes one immutable signal envelope to the bounded branches of a compiled fan-out edge. | `src/runtime/signal/edge.rs:259` |
| sym-c4718c28c4b923a144dc | `pocketstation::runtime::signal::edge::TypedEdgePublishReport` | struct | Reports how many fan-out branches accepted or dropped one published signal. | `src/runtime/signal/edge.rs:380` |
| sym-59c6c36255a5a389a368 | `pocketstation::runtime::signal::io::AsyncOperatorInput` | struct | Carries typed input for async operator. | `src/runtime/signal/io.rs:23` |
| sym-efcca93ced1b383b475c | `pocketstation::runtime::signal::io::AsyncOperatorInputAccessError` | struct | Reports a async operator input access error. | `src/runtime/signal/io.rs:31` |
| sym-d631b28c8bcea8cac29c | `pocketstation::runtime::signal::io::AsyncOperatorNamedOutput` | struct | Carries typed output from async operator named. | `src/runtime/signal/io.rs:95` |
| sym-1d6ed06e77ed6e50087c | `pocketstation::runtime::signal::io::AsyncOperatorNamedOutputBranchSpec` | struct | Configures async operator named output branch behavior at its owning API boundary. | `src/runtime/signal/io.rs:90` |
| sym-3706d6c06565e6eb46d5 | `pocketstation::runtime::signal::io::AsyncOperatorTypedInput` | struct | Carries typed input for async operator typed. | `src/runtime/signal/io.rs:79` |
| sym-e2d307781675a9d68440 | `pocketstation::runtime::signal::observations::AsyncOperatorObservationHandle` | struct | Owns bounded access to async operator observation. | `src/runtime/signal/observations.rs:47` |
| sym-8073d2cb37343fedccf4 | `pocketstation::runtime::signal::observations::AsyncOperatorObservations` | struct | Reports the async operator observations collected at an observation boundary. | `src/runtime/signal/observations.rs:29` |
| sym-655d9e7aec9441daa9bd | `pocketstation::runtime::signal::operator::AsyncOperatorWorker` | struct | Owns the asynchronous operator task, typed I/O, cancellation, and terminal join result. | `src/runtime/signal/operator.rs:250` |
| sym-1b97d8f8094f57114645 | `pocketstation::runtime::signal::operator::CompiledOperatorInputContract` | struct | Declares the validated constraints applied to compiled operator input. | `src/runtime/signal/operator.rs:103` |
| sym-37b95dd73c3048fe3bf7 | `pocketstation::session::compile::SessionCompiler` | struct | Compiles an immutable Session declaration into a validated graph and runtime plan. | `src/session/compile/mod.rs:41` |
| sym-6b4a95539434bb61f8b8 | `pocketstation::session::compile::compiled::CompiledSession` | struct | Owns the validated Session specification and declarations produced by compilation. | `src/session/compile/compiled.rs:13` |
| sym-0e8124f36bd5de49baea | `pocketstation::session::declaration::draft::DerivedStreamHandle` | struct | Owns bounded access to derived stream. | `src/session/declaration/draft.rs:839` |
| sym-d71bbbef55511fef690b | `pocketstation::session::declaration::draft::EndpointHandle` | struct | Owns bounded access to endpoint. | `src/session/declaration/draft.rs:592` |
| sym-4f41f9353d26059245b0 | `pocketstation::session::declaration::draft::Operator` | struct | Declares one operator instance, including its stable operator identity and validated node configuration. | `src/session/declaration/draft.rs:294` |
| sym-cd704da54c98d47c3b56 | `pocketstation::session::declaration::draft::OperatorInputHandle` | struct | Owns bounded access to operator input. | `src/session/declaration/draft.rs:713` |
| sym-49c9f3b283a84be7c72f | `pocketstation::session::declaration::draft::OperatorInstanceHandle` | struct | Owns bounded access to operator instance. | `src/session/declaration/draft.rs:706` |
| sym-77564129aca897aa0d3e | `pocketstation::session::declaration::draft::Session` | struct | Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it. | `src/session/declaration/draft.rs:328` |
| sym-bfa9c0d19ac86d266475 | `pocketstation::session::declaration::draft::SourceInstanceHandle` | struct | Owns bounded access to source instance. | `src/session/declaration/draft.rs:846` |
| sym-e9b2e197157acfe903c2 | `pocketstation::session::declaration::draft::SourceOutputHandle` | struct | Owns bounded access to source output. | `src/session/declaration/draft.rs:922` |
| sym-43021652e2609f6eaf55 | `pocketstation::session::declaration::draft::StemHandle` | struct | Owns bounded access to stem. | `src/session/declaration/draft.rs:700` |
| sym-55f570999391b00c6afe | `pocketstation::session::declaration::endpoint::EndpointConfiguration` | struct | Configures endpoint behavior at its owning API boundary. | `src/session/declaration/endpoint.rs:14` |
| sym-8ab59e971379f8759b12 | `pocketstation::session::declaration::endpoint::EndpointDescriptor` | struct | Describes the endpoint descriptor contract. | `src/session/declaration/endpoint.rs:110` |
| sym-ced7a171e9aad0904992 | `pocketstation::session::declaration::selector::DeviceId` | struct | Uniquely identifies device within its PocketStation ownership scope. | `src/session/declaration/selector.rs:19` |
| sym-9d8a954acd3075896e45 | `pocketstation::session::declaration::selector::ProcessId` | struct | Uniquely identifies process within its PocketStation ownership scope. | `src/session/declaration/selector.rs:6` |
| sym-37210a7077c28712842d | `pocketstation::session::declaration::spec::ConnectionSpec` | struct | The single Session connection declaration used for every stream origin and every operator/endpoint destination. | `src/session/declaration/spec.rs:238` |
| sym-baa92f25178821207385 | `pocketstation::session::declaration::spec::EndpointSpec` | struct | Configures endpoint behavior at its owning API boundary. | `src/session/declaration/spec.rs:161` |
| sym-4430bef5c0978362e42b | `pocketstation::session::declaration::spec::GeneratedAudioIngressSpec` | struct | One operator PCM output that re-enters the specialized Session audio lane. | `src/session/declaration/spec.rs:106` |
| sym-98fc00e7a6bc86add3bd | `pocketstation::session::declaration::spec::OperatorInstanceId` | struct | Uniquely identifies operator instance within its PocketStation ownership scope. | `src/session/declaration/spec.rs:27` |
| sym-383d6bd8b74ab6e780f2 | `pocketstation::session::declaration::spec::OperatorInstanceSpec` | struct | Configures operator instance behavior at its owning API boundary. | `src/session/declaration/spec.rs:197` |
| sym-1ecb310b7568e8d03e4b | `pocketstation::session::declaration::spec::SessionSpec` | struct | Configures session behavior at its owning API boundary. | `src/session/declaration/spec.rs:277` |
| sym-943de9d96c5d263c454b | `pocketstation::session::declaration::spec::SessionSpecVersion` | struct | Identifies the major and minor version of the immutable Session declaration schema. | `src/session/declaration/spec.rs:40` |
| sym-92b2cd72ce843ed7ecc2 | `pocketstation::session::declaration::spec::SourceInstanceId` | struct | Uniquely identifies source instance within its PocketStation ownership scope. | `src/session/declaration/spec.rs:14` |
| sym-09958a46c2082d9d9795 | `pocketstation::session::declaration::spec::SourceInstanceSpec` | struct | Configures source instance behavior at its owning API boundary. | `src/session/declaration/spec.rs:68` |
| sym-20a9b6bc1e7f57ba96dc | `pocketstation::session::declaration::spec::SourceOutputSpec` | struct | Configures source output behavior at its owning API boundary. | `src/session/declaration/spec.rs:94` |
| sym-24dbc709b5286e3aaf7d | `pocketstation::session::declaration::spec::StemSpec` | struct | Configures stem behavior at its owning API boundary. | `src/session/declaration/spec.rs:62` |
| sym-3a87dfd94517526532e2 | `pocketstation::session::declaration::typed_stream::Stream` | struct | Typed Rust declaration façade compiled into stable dynamic signal, schema, port, and edge contracts. This wrapper carries no frames and is not a generic runtime queue. | `src/session/declaration/typed_stream.rs:96` |
| sym-ebafdf4d5cd8b9d2ee70 | `pocketstation::session::declaration::typed_stream::TypedOperator` | struct | Binds an operator declaration to its typed input and output ports so graph connections preserve signal specifications. | `src/session/declaration/typed_stream.rs:20` |
| sym-0982052a3a888324fd5c | `pocketstation::session::extensions::audio_input::AudioInput` | struct | Intent-first façade for feeding audio already owned by the embedding application into a Session. | `src/session/extensions/audio_input/mod.rs:94` |
| sym-72b172a3dce4db303731 | `pocketstation::session::extensions::audio_input::AudioInputConfig` | struct | Configures audio input behavior at its owning API boundary. | `src/session/extensions/audio_input/mod.rs:22` |
| sym-dbac5243766c09a7b9bd | `pocketstation::session::extensions::audio_input::buffer::AudioInputBuffer` | struct | Leases bounded PCM storage from an external-audio input until the caller submits or releases it. | `src/session/extensions/audio_input/buffer.rs:11` |
| sym-3d4e420aec42d84d078a | `pocketstation::session::extensions::audio_input::buffer::AudioInputObservations` | struct | Reports the audio input observations collected at an observation boundary. | `src/session/extensions/audio_input/buffer.rs:72` |
| sym-6b363e100c83b5cb5cd8 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` | struct | Reports a audio input write error. | `src/session/extensions/audio_input/buffer.rs:305` |
| sym-ca735f7e2f4cd44a9991 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriter` | struct | Sends audio input values across its declared ownership boundary. | `src/session/extensions/audio_input/buffer.rs:91` |
| sym-1c97de4848d355fd0eb2 | `pocketstation::session::extensions::audio_input::source::PcmSource` | struct | Low-level PCM source ownership for integrations that separately retain the Session handles and producer writer. | `src/session/extensions/audio_input/source.rs:33` |
| sym-364c8cf04eaf39b47761 | `pocketstation::session::extensions::recording::SessionRecordingReceipt` | struct | Retains the identity and observation access returned for session recording. | `src/session/extensions/recording.rs:27` |
| sym-b3b37c8991fad13557c7 | `pocketstation::session::extensions::source::PreparedSourceRuntime` | struct | Fully validated source resources which have not started producing signals. | `src/session/extensions/source.rs:437` |
| sym-b39fcdd2c9daf9c470a3 | `pocketstation::session::extensions::source::SourceCancellation` | struct | Exposes the cancellation state observed by a running external source driver. | `src/session/extensions/source.rs:250` |
| sym-b669908a1775b2286107 | `pocketstation::session::extensions::source::SourceConfiguration` | struct | Configures source behavior at its owning API boundary. | `src/session/extensions/source.rs:87` |
| sym-95c27ed5187844b659c1 | `pocketstation::session::extensions::source::SourceEmission` | struct | Carries one external-source emission with its output-port identity and signal envelope. | `src/session/extensions/source.rs:261` |
| sym-ca376e24878c7407d467 | `pocketstation::session::extensions::source::SourceManifest` | struct | Describes the source manifest contract. | `src/session/extensions/source.rs:112` |
| sym-26a9699930a68beb65f7 | `pocketstation::session::extensions::source::SourceOutputBranchSpec` | struct | Configures source output branch behavior at its owning API boundary. | `src/session/extensions/source.rs:370` |
| sym-c3c081e895400edef948 | `pocketstation::session::extensions::source::SourceOutputIdentity` | struct | Identifies one declared source output by source type, output port, and stream identity. | `src/session/extensions/source.rs:229` |
| sym-47bee885ecb5eb06a617 | `pocketstation::session::extensions::source::SourceOutputReceiver` | struct | Receives source output values across its declared ownership boundary. | `src/session/extensions/source.rs:375` |
| sym-0e76136dc2a0ff3998e9 | `pocketstation::session::extensions::source::SourcePrepareContext` | struct | Carries the inputs and runtime context required to source prepare. | `src/session/extensions/source.rs:223` |
| sym-a53a66f784db816ce220 | `pocketstation::session::extensions::source::SourceRegistry` | struct | Indexes registered source implementations by their stable identities. | `src/session/extensions/source.rs:286` |
| sym-67669087b0d5cee0081e | `pocketstation::session::extensions::source::SourceRuntime` | struct | Owns an external source driver's cancellation handle, observations, and terminal worker join. | `src/session/extensions/source.rs:427` |
| sym-a80689c392316efca609 | `pocketstation::session::extensions::source::SourceRuntimeObservationHandle` | struct | Owns bounded access to source runtime observation. | `src/session/extensions/source.rs:407` |
| sym-8fce4cdf757d97952751 | `pocketstation::session::extensions::source::SourceRuntimeObservations` | struct | Reports the source runtime observations collected at an observation boundary. | `src/session/extensions/source.rs:394` |
| sym-92a7b58d4c8dcb88b0cc | `pocketstation::session::extensions::source::SourceSessionContext` | struct | Carries the inputs and runtime context required to source session. | `src/session/extensions/source.rs:235` |
| sym-fd5bee9370534038e978 | `pocketstation::session::extensions::source::SourceTypeId` | struct | Uniquely identifies source type within its PocketStation ownership scope. | `src/session/extensions/source.rs:17` |
| sym-fd9f74be79f1974c2a5c | `pocketstation::session::lifecycle::engine::SessionEngine` | struct | Canonical production composition path for one safe Rust Session engine. | `src/session/lifecycle/engine.rs:202` |
| sym-966a5c41955c404f896a | `pocketstation::session::lifecycle::engine::SessionEngineBuilder` | struct | Setup-time builder for one canonical Session composition environment. | `src/session/lifecycle/engine.rs:30` |
| sym-41dbd64e604d85c982b9 | `pocketstation::session::lifecycle::events::SessionControlFailure` | struct | Typed control-plane failure without exposing an implementation error type. | `src/session/lifecycle/events.rs:70` |
| sym-3e6b033f8fce6a171342 | `pocketstation::session::lifecycle::events::SessionEndpointFailure` | struct | Endpoint failure associated with one stable route and endpoint. | `src/session/lifecycle/events.rs:125` |
| sym-2aef1c990e2284b493d2 | `pocketstation::session::lifecycle::events::SessionEvent` | struct | Event emitted by the session lifecycle authority. | `src/session/lifecycle/events.rs:308` |
| sym-5bb661e537cf32ee660d | `pocketstation::session::lifecycle::events::SessionEventReceiver` | struct | Sole consumer for a session's bounded control-event queue. | `src/session/lifecycle/events.rs:500` |
| sym-248f0f8282f56d2fca16 | `pocketstation::session::lifecycle::events::SessionFinalizationFailure` | struct | Failure observed while finalizing a stopping session. | `src/session/lifecycle/events.rs:186` |
| sym-379436c6a5925f2948f8 | `pocketstation::session::lifecycle::events::SessionRollbackFailure` | struct | Failure observed while rolling back a partial session start. | `src/session/lifecycle/events.rs:165` |
| sym-5cf033f0d633f2b1cede | `pocketstation::session::lifecycle::events::SessionSourceFailure` | struct | Source failure associated with one stable session stem. | `src/session/lifecycle/events.rs:104` |
| sym-b3bccc2e4f52a51dd776 | `pocketstation::session::lifecycle::events::SessionTerminalOutcome` | struct | Complete terminal result. Failure categories remain separate for diagnosis. | `src/session/lifecycle/events.rs:217` |
| sym-bbadd875b1ef9a118c7f | `pocketstation::session::lifecycle::host::NativeSessionEngineHostOptions` | struct | Configures native session engine host behavior at its owning API boundary. | `src/session/lifecycle/host.rs:164` |
| sym-227e371ddba6ac90f3cc | `pocketstation::session::lifecycle::host::SessionEngineHost` | struct | Safe host-owned Session environment for foreign-language adapters. | `src/session/lifecycle/host.rs:31` |
| sym-f9f0b91c1949ac76a7d4 | `pocketstation::session::lifecycle::host::SessionEngineHostBuilder` | struct | Setup-time owner for the canonical Session host. | `src/session/lifecycle/host.rs:188` |
| sym-149559a32ccbfb47a5f6 | `pocketstation::session::lifecycle::observations::SessionAudioReentryMetrics` | struct | Exact boundedness and lifecycle accounting for one operator PCM output re-entering the Session audio lane. | `src/session/lifecycle/observations.rs:253` |
| sym-bff940477f4afadaf3a9 | `pocketstation::session::lifecycle::observations::SessionDerivedRouteMetrics` | struct | Reports the session derived route metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:431` |
| sym-adf85226a354ceb9fc4c | `pocketstation::session::lifecycle::observations::SessionEventQueueObservations` | struct | Point-in-time observations for a session's bounded control-event queue. | `src/session/lifecycle/observations.rs:17` |
| sym-0614a0b7c7b7b1ed9d46 | `pocketstation::session::lifecycle::observations::SessionExternalSourceMetrics` | struct | Reports the session external source metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:124` |
| sym-2e5cb39db451a3f2ce64 | `pocketstation::session::lifecycle::observations::SessionMetricsSnapshot` | struct | Authoritative point-in-time observations for the current Session boundary. | `src/session/lifecycle/observations.rs:36` |
| sym-73f440df4c771b863e60 | `pocketstation::session::lifecycle::observations::SessionOperatorInputMetrics` | struct | Reports the session operator input metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:242` |
| sym-1e0767b7ab652909fe45 | `pocketstation::session::lifecycle::observations::SessionOperatorMetrics` | struct | Reports the session operator metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:382` |
| sym-6348e7cc3443e6188038 | `pocketstation::session::lifecycle::observations::SessionRouteDropObservations` | struct | Explicit numerator, denominator, interval, and typed reasons for one route. | `src/session/lifecycle/observations.rs:157` |
| sym-f480dbf9cd3220385148 | `pocketstation::session::lifecycle::observations::SessionRouteLatencyObservations` | struct | Common-clock source timestamp to route-receive latency in nanoseconds. | `src/session/lifecycle/observations.rs:182` |
| sym-2487163135b10a9af4fc | `pocketstation::session::lifecycle::observations::SessionRouteMetrics` | struct | Reports the session route metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:139` |
| sym-9d7a90784619b2b2d8e6 | `pocketstation::session::lifecycle::observations::SessionSidecarMetrics` | struct | Exact bounded-queue and process-lifecycle accounting for one Session-owned language-neutral sidecar. | `src/session/lifecycle/observations.rs:133` |
| sym-67f0ff15120c91e5aa37 | `pocketstation::session::lifecycle::observations::SessionSourceMetrics` | struct | Reports the session source metrics collected at an observation boundary. | `src/session/lifecycle/observations.rs:117` |
| sym-c7e7a644228d03a12afc | `pocketstation::session::lifecycle::running::RunningSession` | struct | Owns a started Session together with event, polling, recording, trace, and stop resources. | `src/session/lifecycle/running.rs:173` |
| sym-68aedad4c3704116a4c4 | `pocketstation::session::lifecycle::start_contract::CaptureBackendSet` | struct | Supplies the application and microphone capture backends used while preparing a Session. | `src/session/lifecycle/start_contract.rs:17` |
| sym-9f419e06ca0a934a4dc0 | `pocketstation::session::lifecycle::start_contract::SessionStartCancellation` | struct | Thread-safe cancellation request for a Session that has not reached `Running` yet. | `src/session/lifecycle/start_contract.rs:98` |
| sym-6a84ee52966ff9444afa | `pocketstation::session::lifecycle::start_contract::SessionStartFailure` | struct | Reports a session start failure. | `src/session/lifecycle/start_contract.rs:263` |
| sym-6286efff5338f801fb1f | `pocketstation::session::lifecycle::start_contract::SessionStartOptions` | struct | Configures session start behavior at its owning API boundary. | `src/session/lifecycle/start_contract.rs:23` |
| sym-77c6d656210a25e6d1e1 | `pocketstation::session::lifecycle::start_contract::SessionStopOutcome` | struct | Reports the structured session stop outcome. | `src/session/lifecycle/start_contract.rs:308` |
| sym-71bbef638fc1229ce3b7 | `pocketstation::session::lifecycle::trace::SessionTrace` | struct | Contains the ordered lifecycle records read from a Session trace artifact. | `src/session/lifecycle/trace.rs:255` |
| sym-ef541f59f627f87e1306 | `pocketstation::session::lifecycle::trace::SessionTraceRecord` | struct | Records one immutable session trace observation. | `src/session/lifecycle/trace.rs:55` |
| sym-83aef9211aa72a000709 | `pocketstation::session::lifecycle::trace::SessionTraceRecorder` | struct | Collects ordered lifecycle records and writes the trace artifact during Session finalization. | `src/session/lifecycle/trace.rs:152` |
| sym-e4d5f3c25b8a17109c87 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderHandle` | struct | Owns bounded access to session trace recorder. | `src/session/lifecycle/trace.rs:108` |
| sym-ed13f313496039988171 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderOutcome` | struct | Reports the structured session trace recorder outcome. | `src/session/lifecycle/trace.rs:70` |
| sym-c7863eeebbc04918f570 | `pocketstation::session::lifecycle::trace::SessionTraceTerminal` | struct | Records the terminal Session disposition and component failures stored in a trace. | `src/session/lifecycle/trace.rs:339` |
| sym-826f8497cd27cf20f8b0 | `pocketstation::session::lifecycle::trace::SessionTraceValidation` | struct | Reports the validated identity and record count of a parsed Session trace. | `src/session/lifecycle/trace.rs:348` |
| sym-87003232f55fde7822ed | `pocketstation::session::prepare::mappings::PreparedOperatorMapping` | struct | Correlates the prepared identities and runtime resources for prepared operator. | `src/session/prepare/mappings.rs:160` |
| sym-ea5f4c5474ca3d763495 | `pocketstation::session::prepare::mappings::PreparedSignalRouteMapping` | struct | Correlates the prepared identities and runtime resources for prepared signal route. | `src/session/prepare/mappings.rs:131` |
| sym-a85b1f3786cee9752986 | `pocketstation::session::prepare::mappings::PreparedSourceMapping` | struct | Correlates the prepared identities and runtime resources for prepared source. | `src/session/prepare/mappings.rs:18` |
| sym-82315d869217276dc46e | `pocketstation::session::prepare::mappings::PreparedWorkerMapping` | struct | Correlates the prepared identities and runtime resources for prepared worker. | `src/session/prepare/mappings.rs:35` |
| sym-fec7fd168a3b483edf7f | `pocketstation::session::prepare::prepared::PreparedSession` | struct | Setup-time ownership for one compiled Session. | `src/session/prepare/prepared.rs:18` |
| sym-bd7efbabb0ea9370bb5c | `pocketstation::timing::clock_correction::ClockCorrectionController` | struct | Applies bounded proportional corrections from measured clock offsets without changing lineage. | `src/timing/clock_correction.rs:4` |
| sym-35fb3cce537ca1a2170d | `pocketstation::timing::clock_drift::ClockDriftEstimator` | struct | Estimates source-clock drift from accumulated source and Session timing observations. | `src/timing/clock_drift.rs:10` |
| sym-9313cfa91ee45d8cb05e | `pocketstation::timing::clock_drift::ClockDriftSnapshot` | struct | Reports the clock drift snapshot collected at an observation boundary. | `src/timing/clock_drift.rs:4` |
| sym-9fcb4fc3036c0fad6c1c | `pocketstation::timing::timeline_mapping::TimelineMapping` | struct | Correlates the prepared identities and runtime resources for timeline. | `src/timing/timeline_mapping.rs:2` |
| sym-d76d36195f7b36192f09 | `ApplicationSelector::ProcessInstance::process_id` | struct_field | Identifies the process identifier recorded by `ProcessInstance`. | `src/session/declaration/selector.rs:36` |
| sym-aa840cb4272ecb2fdecb | `ApplicationSelector::ProcessInstance::stable_id` | struct_field | Identifies the stable identifier recorded by `ProcessInstance`. | `src/session/declaration/selector.rs:37` |
| sym-c55e055e8748124a0884 | `AsyncOperatorNamedOutput::output_port` | struct_field | Stores the output port used by `AsyncOperatorNamedOutput`. | `src/runtime/signal/io.rs:96` |
| sym-1e1febd632d779661a28 | `AsyncOperatorNamedOutput::receiver` | struct_field | Stores the receiver used by `AsyncOperatorNamedOutput`. | `src/runtime/signal/io.rs:97` |
| sym-fe8f504ed2cfb3aa18fa | `AsyncOperatorNamedOutputBranchSpec::branch` | struct_field | Stores the branch used by `AsyncOperatorNamedOutputBranchSpec`. | `src/runtime/signal/io.rs:92` |
| sym-359a6d6acecf7b04a155 | `AsyncOperatorNamedOutputBranchSpec::output_port` | struct_field | Stores the output port used by `AsyncOperatorNamedOutputBranchSpec`. | `src/runtime/signal/io.rs:91` |
| sym-20ee8a38d9beb0b10d0b | `AsyncOperatorObservations::cancellation_total` | struct_field | Counts the total number of cancellation observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:39` |
| sym-b78bea654cbcb4564e0a | `AsyncOperatorObservations::graceful_finish_total` | struct_field | Counts the total number of graceful finish observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:40` |
| sym-988682d4c1a4423ec888 | `AsyncOperatorObservations::idle_poll_total` | struct_field | Counts the total number of idle poll observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:41` |
| sym-1d200f034b81b0b17a51 | `AsyncOperatorObservations::input_attempted_total` | struct_field | Counts the total number of input attempted observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:30` |
| sym-ba6adc21a44dea8587b3 | `AsyncOperatorObservations::input_dropped_total` | struct_field | Counts the total number of input dropped observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:31` |
| sym-82c230d8410fc36cad1a | `AsyncOperatorObservations::joined` | struct_field | Stores the joined used by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:43` |
| sym-a4d98d98b89bf4437b11 | `AsyncOperatorObservations::output_dropped_total` | struct_field | Counts the total number of output dropped observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:34` |
| sym-6fded292257bad1c68d1 | `AsyncOperatorObservations::output_emitted_total` | struct_field | Counts the total number of output emitted observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:33` |
| sym-0835b2a0744961720fe8 | `AsyncOperatorObservations::output_nonterminal_total` | struct_field | Counts the total number of output nonterminal observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:35` |
| sym-a03be0454e50e5443c4a | `AsyncOperatorObservations::output_terminal_total` | struct_field | Counts the total number of output terminal observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:36` |
| sym-43f856743e3fe174b1c0 | `AsyncOperatorObservations::process_failure_total` | struct_field | Counts the total number of process failure observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:37` |
| sym-981cb528bbf8136e4698 | `AsyncOperatorObservations::processed_total` | struct_field | Counts the total number of processed observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:32` |
| sym-fc0b58c59b286007cf74 | `AsyncOperatorObservations::ready` | struct_field | Indicates whether ready applies to `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:42` |
| sym-4726283f05a225ceca82 | `AsyncOperatorObservations::timeout_total` | struct_field | Counts the total number of timeout observed by `AsyncOperatorObservations`. | `src/runtime/signal/observations.rs:38` |
| sym-75b600156196bee0976f | `AsyncOperatorTypedInput::capacity_signals` | struct_field | Sets the capacity signals available to `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:86` |
| sym-0fdf6971244779fff5f4 | `AsyncOperatorTypedInput::edge_contract` | struct_field | Stores the edge contract used by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:85` |
| sym-1857e0edc1cd21965d72 | `AsyncOperatorTypedInput::edge_id` | struct_field | Identifies the edge identifier recorded by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:82` |
| sym-d839613ffc05bf0f66f3 | `AsyncOperatorTypedInput::media` | struct_field | Stores the media used by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:84` |
| sym-c99b24c05b19e4a14b8c | `AsyncOperatorTypedInput::port_name` | struct_field | Stores the port name used by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:80` |
| sym-31c3709e220d82e5e22e | `AsyncOperatorTypedInput::receiver` | struct_field | Stores the receiver used by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:81` |
| sym-a36ca34d60777b7ed1ba | `AsyncOperatorTypedInput::signal_spec` | struct_field | Stores the signal spec used by `AsyncOperatorTypedInput`. | `src/runtime/signal/io.rs:83` |
| sym-002021d5d6ab029a6b8b | `AsyncOperatorWorkerError::CancelTimeout::timeout_ms` | struct_field | Stores the timeout value for `CancelTimeout`, in milliseconds. | `src/runtime/signal/error.rs:22` |
| sym-1f676abb4f853d304fff | `AsyncOperatorWorkerError::CloseTimeout::timeout_ms` | struct_field | Stores the timeout value for `CloseTimeout`, in milliseconds. | `src/runtime/signal/error.rs:18` |
| sym-a944f1d48c11b86dc505 | `AsyncOperatorWorkerError::InvalidPlanInput::kind` | struct_field | Stores the kind used by `InvalidPlanInput`. | `src/runtime/signal/error.rs:50` |
| sym-491d4760a9c3c0d84529 | `AsyncOperatorWorkerError::OutputPayloadTooLarge::branch_index` | struct_field | Stores the branch index used by `OutputPayloadTooLarge`. | `src/runtime/signal/error.rs:43` |
| sym-2a8d5f3e09cd5f36ac4e | `AsyncOperatorWorkerError::OutputPayloadTooLarge::max_payload_bytes` | struct_field | Limits payload storage for `OutputPayloadTooLarge`, in bytes. | `src/runtime/signal/error.rs:45` |
| sym-851c39c4cde54614c4d1 | `AsyncOperatorWorkerError::OutputPayloadTooLarge::payload_bytes` | struct_field | Stores the payload size for `OutputPayloadTooLarge`, in bytes. | `src/runtime/signal/error.rs:44` |
| sym-475743b47ed09c78a814 | `AsyncOperatorWorkerError::PrepareTimeout::timeout_ms` | struct_field | Stores the timeout value for `PrepareTimeout`, in milliseconds. | `src/runtime/signal/error.rs:10` |
| sym-3dbd3b5ab6191f4f2cd4 | `AsyncOperatorWorkerError::TerminalOutputDropped::branch_index` | struct_field | Stores the branch index used by `TerminalOutputDropped`. | `src/runtime/signal/error.rs:38` |
| sym-f5d2cc3020d960f09600 | `AsyncOperatorWorkerError::Timeout::timeout_ms` | struct_field | Stores the timeout value for `Timeout`, in milliseconds. | `src/runtime/signal/error.rs:14` |
| sym-d4f522a8f56c36abf79d | `AsyncOperatorWorkerError::UnknownInputPort::port_name` | struct_field | Stores the port name used by `UnknownInputPort`. | `src/runtime/signal/error.rs:34` |
| sym-616b72c151c8a0166c19 | `AsyncRuntimeHostError::HostTimeout::timeout_ms` | struct_field | Stores the timeout value for `HostTimeout`, in milliseconds. | `src/runtime/lifecycle/async_host.rs:18` |
| sym-0bf4096c2da0e2f24c2e | `AudioCaps::channel_layout` | struct_field | Stores the channel layout used by `AudioCaps`. | `src/graph/ports.rs:51` |
| sym-b994975eb47488bcff05 | `AudioCaps::format` | struct_field | Stores the format used by `AudioCaps`. | `src/graph/ports.rs:52` |
| sym-fb237ba82003c93b965b | `AudioCaps::frame_samples` | struct_field | Stores the frame samples used by `AudioCaps`. | `src/graph/ports.rs:50` |
| sym-fbfa557a47d336f0071f | `AudioCaps::sample_rate_hz` | struct_field | Stores the sample rate value for `AudioCaps`, in hertz. | `src/graph/ports.rs:49` |
| sym-e18e55a8a4fef6a6fa53 | `AudioInputBufferError::WrongFrameLength::actual_samples` | struct_field | Stores the actual samples used by `WrongFrameLength`. | `src/session/extensions/audio_input/buffer.rs:291` |
| sym-69c4992aa33dc377ec40 | `AudioInputBufferError::WrongFrameLength::expected_samples` | struct_field | Stores the expected samples used by `WrongFrameLength`. | `src/session/extensions/audio_input/buffer.rs:290` |
| sym-44f193b87c5ede60e9ce | `AudioInputObservations::accepted_total` | struct_field | Counts the total number of accepted observed by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:76` |
| sym-8038fc0867db99457ae4 | `AudioInputObservations::available_buffers` | struct_field | Stores the available buffers used by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:75` |
| sym-e9d8ca88fd3775157580 | `AudioInputObservations::buffer_slots` | struct_field | Stores the buffer slots used by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:74` |
| sym-9e200166f71a9c0c38a9 | `AudioInputObservations::cancelled` | struct_field | Stores the cancelled used by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:79` |
| sym-7544c5917befa6070a6c | `AudioInputObservations::capacity_frames` | struct_field | Sets the capacity frames available to `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:73` |
| sym-d859c159d81a71225e10 | `AudioInputObservations::closed` | struct_field | Stores the closed used by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:80` |
| sym-b8c6c431e97733f68f67 | `AudioInputObservations::full_total` | struct_field | Counts the total number of full observed by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:77` |
| sym-dcd33504cd9d46947f67 | `AudioInputObservations::invalid_total` | struct_field | Counts the total number of invalid observed by `AudioInputObservations`. | `src/session/extensions/audio_input/buffer.rs:78` |
| sym-c6230ec63d40a0522ffd | `CaptureBackendSet::application` | struct_field | Stores the application used by `CaptureBackendSet`. | `src/session/lifecycle/start_contract.rs:18` |
| sym-7300fa63a391b46f9a7b | `CaptureBackendSet::microphone` | struct_field | Stores the microphone used by `CaptureBackendSet`. | `src/session/lifecycle/start_contract.rs:19` |
| sym-38c79b01d994239517d7 | `CaptureDelivery::frame_sender` | struct_field | Stores the frame sender used by `CaptureDelivery`. | `src/capture/capture_owner.rs:74` |
| sym-184f1403da6183166817 | `CaptureDelivery::runtime_event_sender` | struct_field | Stores the runtime event sender used by `CaptureDelivery`. | `src/capture/capture_owner.rs:75` |
| sym-cfcf6d455b4ac6269f0a | `CaptureObservations::callback_buffers_total` | struct_field | Counts the total number of callback buffers observed by `CaptureObservations`. | `src/capture/observations.rs:9` |
| sym-558b4639e8c15e8c4933 | `CaptureObservations::dispatch_queue_full_total` | struct_field | Counts the total number of dispatch queue full observed by `CaptureObservations`. | `src/capture/observations.rs:12` |
| sym-f1354342b21d545e43d0 | `CaptureObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `CaptureObservations`. | `src/capture/observations.rs:10` |
| sym-f2cd3e04115cceb22798 | `CaptureObservations::invalid_buffer_total` | struct_field | Counts the total number of invalid buffer observed by `CaptureObservations`. | `src/capture/observations.rs:13` |
| sym-b558b3b06363e5fb40e4 | `CaptureObservations::oversized_buffer_total` | struct_field | Counts the total number of oversized buffer observed by `CaptureObservations`. | `src/capture/observations.rs:14` |
| sym-4ca717256cb392393317 | `CaptureObservations::pool_exhausted_total` | struct_field | Counts the total number of pool exhausted observed by `CaptureObservations`. | `src/capture/observations.rs:11` |
| sym-41f6c642b6797e94e80e | `CaptureObservations::stream_errors_total` | struct_field | Counts the total number of stream errors observed by `CaptureObservations`. | `src/capture/observations.rs:15` |
| sym-b6eaec6997de39decfff | `CaptureObservations::timestamp_epoch_clamps_total` | struct_field | Counts the total number of timestamp epoch clamps observed by `CaptureObservations`. | `src/capture/observations.rs:16` |
| sym-e84bcedb6149e2c78a6f | `CaptureOpenMetadata::clock_id` | struct_field | Identifies the clock identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:53` |
| sym-fc7fa7010a37b17da2c5 | `CaptureOpenMetadata::discontinuity_epoch` | struct_field | Stores the discontinuity epoch used by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:55` |
| sym-52b2fc6425ad3e13ebb2 | `CaptureOpenMetadata::permission_epoch` | struct_field | Stores the permission epoch used by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:56` |
| sym-098564a8d55bed3f48d3 | `CaptureOpenMetadata::session_id` | struct_field | Identifies the session identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:50` |
| sym-ac5bd4262b98592d78f5 | `CaptureOpenMetadata::source_generation` | struct_field | Stores the source generation used by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:54` |
| sym-e7013d71bbec92f27427 | `CaptureOpenMetadata::source_id` | struct_field | Identifies the source identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:51` |
| sym-6ac703dc19190afa5202 | `CaptureOpenMetadata::stem_id` | struct_field | Identifies the stem identifier recorded by `CaptureOpenMetadata`. | `src/capture/capture_owner.rs:52` |
| sym-375e0cbb3cc9eacb7aa1 | `CaptureOwnerObservations::backend` | struct_field | Stores the backend used by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:161` |
| sym-c507584e4bbc449174f2 | `CaptureOwnerObservations::frame_stream` | struct_field | Stores the frame stream used by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:162` |
| sym-87dcecd21143dc6e9434 | `CaptureOwnerObservations::runtime_events` | struct_field | Stores the runtime events used by `CaptureOwnerObservations`. | `src/capture/capture_owner.rs:163` |
| sym-778180323cfdf5e12eb8 | `CapturePrepareRequest::frame_capacity_frames` | struct_field | Sets the frame capacity frames available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:64` |
| sym-7207623794ba4a77f51d | `CapturePrepareRequest::lineage_seed` | struct_field | Stores the lineage seed used by `CapturePrepareRequest`. | `src/capture/capture_owner.rs:63` |
| sym-ed8aef993492af565c3e | `CapturePrepareRequest::mode` | struct_field | Stores the mode used by `CapturePrepareRequest`. | `src/capture/capture_owner.rs:62` |
| sym-3fe909146d9788d27014 | `CapturePrepareRequest::runtime_event_capacity_events` | struct_field | Sets the runtime event capacity events available to `CapturePrepareRequest`. | `src/capture/capture_owner.rs:65` |
| sym-c147acda6c2632026425 | `CaptureSampleTimelineError::SourcePositionMovedBackward::expected_at_least` | struct_field | Stores the expected at least used by `SourcePositionMovedBackward`. | `src/capture/timeline.rs:45` |
| sym-75b22b8047340c14d50f | `CaptureSampleTimelineError::SourcePositionMovedBackward::observed` | struct_field | Stores the observed used by `SourcePositionMovedBackward`. | `src/capture/timeline.rs:46` |
| sym-2ebcb5bf416a4de25c48 | `CaptureStopOutcome::observations` | struct_field | Carries the observations collected for `CaptureStopOutcome`. | `src/capture/capture_owner.rs:186` |
| sym-f7ddbb59cf345aefc3d4 | `CapturedFrameStreamStats::delivered_frames` | struct_field | Stores the delivered frames used by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:18` |
| sym-599900d91119a9fa314a | `CapturedFrameStreamStats::dropped_newest_frames` | struct_field | Stores the dropped newest frames used by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:19` |
| sym-bba867916e478ded5b1a | `CapturedFrameStreamStats::frames_discarded_before_start_total` | struct_field | Counts the total number of frames discarded before start observed by `CapturedFrameStreamStats`. | `src/capture/frame_stream.rs:20` |
| sym-ad76e0a22e34ff04feb5 | `ClockDriftSnapshot::accumulated_error_ns` | struct_field | Stores the accumulated error value for `ClockDriftSnapshot`, in nanoseconds. | `src/timing/clock_drift.rs:6` |
| sym-75e5bfb2fadc319c5ebd | `ClockDriftSnapshot::drift_ppm` | struct_field | Stores the drift ppm used by `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:5` |
| sym-01ef1ec0849935f2decc | `ClockDriftSnapshot::observed_samples_count` | struct_field | Stores the number of observed samples represented by `ClockDriftSnapshot`. | `src/timing/clock_drift.rs:7` |
| sym-17d65c8a8a1977f3b560 | `CompileError::AdapterUnavailable::edge` | struct_field | Stores the edge used by `AdapterUnavailable`. | `src/graph/compile/resolve.rs:62` |
| sym-87d2e89076fea45ac980 | `CompileError::AdapterUnavailable::type_id` | struct_field | Identifies the type identifier recorded by `AdapterUnavailable`. | `src/graph/compile/resolve.rs:62` |
| sym-957d01aa76ae1dbc8dbf | `CompileError::ClockDomainMismatch::expected` | struct_field | Records the value expected by `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:41` |
| sym-9e5a4d8ee011fcfe9ead | `CompileError::ClockDomainMismatch::found` | struct_field | Stores the found used by `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:42` |
| sym-97dcea461c1d677c632d | `CompileError::ClockDomainMismatch::node` | struct_field | Stores the node used by `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:39` |
| sym-7a1a72e3f32f75031cb0 | `CompileError::ClockDomainMismatch::port` | struct_field | Stores the port used by `ClockDomainMismatch`. | `src/graph/compile/resolve.rs:40` |
| sym-0efed89ffabb66f537c7 | `CompileError::InvalidConfig::reason` | struct_field | Carries the reason reported by `InvalidConfig`. | `src/graph/compile/resolve.rs:30` |
| sym-679199634db2ac07a1ad | `CompileError::InvalidConfig::type_id` | struct_field | Identifies the type identifier recorded by `InvalidConfig`. | `src/graph/compile/resolve.rs:30` |
| sym-776b3a07bc0feba89a8b | `CompileError::InvalidRealtimeEdge::edge` | struct_field | Stores the edge used by `InvalidRealtimeEdge`. | `src/graph/compile/resolve.rs:58` |
| sym-00b117c0699146d230b4 | `CompileError::InvalidRealtimeEdge::reason` | struct_field | Carries the reason reported by `InvalidRealtimeEdge`. | `src/graph/compile/resolve.rs:58` |
| sym-252334ba36621a737d42 | `CompileError::InvalidSafetyContract::execution` | struct_field | Stores the execution used by `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:54` |
| sym-cf749f4111d98777545c | `CompileError::InvalidSafetyContract::node` | struct_field | Stores the node used by `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:52` |
| sym-7373df3306f0145e3c03 | `CompileError::InvalidSafetyContract::safety` | struct_field | Stores the safety used by `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:55` |
| sym-855a832042462300d344 | `CompileError::InvalidSafetyContract::type_id` | struct_field | Identifies the type identifier recorded by `InvalidSafetyContract`. | `src/graph/compile/resolve.rs:53` |
| sym-1937170c9358d6ccd873 | `CompileError::MediaMismatch::edge` | struct_field | Stores the edge used by `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |
| sym-aa103a116bab0badcce0 | `CompileError::MediaMismatch::from` | struct_field | Identifies the origin represented by `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |
| sym-1ec471ad790ecaa35423 | `CompileError::MediaMismatch::to` | struct_field | Identifies the destination represented by `MediaMismatch`. | `src/graph/compile/resolve.rs:45` |
| sym-58b915c071f8e9418298 | `CompileError::SignalMismatch::edge` | struct_field | Stores the edge used by `SignalMismatch`. | `src/graph/compile/resolve.rs:47` |
| sym-2158e816e72aeb34a28e | `CompileError::SignalMismatch::from` | struct_field | Identifies the origin represented by `SignalMismatch`. | `src/graph/compile/resolve.rs:47` |
| sym-92a501cdd98cd64424c7 | `CompileError::SignalMismatch::to` | struct_field | Identifies the destination represented by `SignalMismatch`. | `src/graph/compile/resolve.rs:47` |
| sym-41ed3da9e535b59d59ab | `CompileError::UnknownPort::node` | struct_field | Stores the node used by `UnknownPort`. | `src/graph/compile/resolve.rs:34` |
| sym-4baae97b4cc64e5171e7 | `CompileError::UnknownPort::port` | struct_field | Stores the port used by `UnknownPort`. | `src/graph/compile/resolve.rs:34` |
| sym-525de1b186a601dea2c1 | `CompileError::WrongPortDirection::node` | struct_field | Stores the node used by `WrongPortDirection`. | `src/graph/compile/resolve.rs:36` |
| sym-21746a5eda214476b994 | `CompileError::WrongPortDirection::port` | struct_field | Stores the port used by `WrongPortDirection`. | `src/graph/compile/resolve.rs:36` |
| sym-a4fc7fd67b0604e89a07 | `CompiledOperatorInputContract::capacity_signals` | struct_field | Sets the capacity signals available to `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:113` |
| sym-265c4cf8f1a23281ca28 | `CompiledOperatorInputContract::edge_contract` | struct_field | Stores the edge contract used by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:112` |
| sym-0226af788b38b4d1f286 | `CompiledOperatorInputContract::edge_id` | struct_field | Identifies the edge identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:104` |
| sym-eca89bff0bacb134b7a6 | `CompiledOperatorInputContract::input_port` | struct_field | Stores the input port used by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:109` |
| sym-6be1b98b288a4547ea12 | `CompiledOperatorInputContract::media` | struct_field | Stores the media used by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:111` |
| sym-606affe9a4a183648b4b | `CompiledOperatorInputContract::operator_node` | struct_field | Stores the operator node used by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:105` |
| sym-409c435f8fcbe87f61d8 | `CompiledOperatorInputContract::session_id` | struct_field | Identifies the session identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:106` |
| sym-3f470a4afe4784955311 | `CompiledOperatorInputContract::signal_spec` | struct_field | Stores the signal spec used by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:110` |
| sym-1d4cdef6a63d9597dc4f | `CompiledOperatorInputContract::source_id` | struct_field | Identifies the source identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:108` |
| sym-8ac0877877df37098656 | `CompiledOperatorInputContract::stem_id` | struct_field | Identifies the stem identifier recorded by `CompiledOperatorInputContract`. | `src/runtime/signal/operator.rs:107` |
| sym-84325425539d1b03d7a2 | `ConfigError::Invalid::key` | struct_field | Stores the key used by `Invalid`. | `src/graph/node.rs:145` |
| sym-d19fa19571ea246805f6 | `ConfigError::Invalid::reason` | struct_field | Carries the reason reported by `Invalid`. | `src/graph/node.rs:145` |
| sym-70725d4e3509bdea4387 | `ConnectionTarget::EndpointInput::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `EndpointInput`. | `src/session/declaration/spec.rs:230` |
| sym-8e2adbf282afd7153ca6 | `ConnectionTarget::EndpointInput::input_port` | struct_field | Stores the input port used by `EndpointInput`. | `src/session/declaration/spec.rs:231` |
| sym-3a98adff543be885924b | `ConnectionTarget::OperatorInput::input_port` | struct_field | Stores the input port used by `OperatorInput`. | `src/session/declaration/spec.rs:227` |
| sym-e8a956c559467bcaf24c | `ConnectionTarget::OperatorInput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorInput`. | `src/session/declaration/spec.rs:226` |
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
| sym-86d1d823e3794685596a | `DiscontinuityRecord::kind` | struct_field | Stores the kind used by `DiscontinuityRecord`. | `src/recording/writer.rs:95` |
| sym-2441ca89ecc458e9d840 | `DiscontinuityRecord::label` | struct_field | Stores the label used by `DiscontinuityRecord`. | `src/recording/writer.rs:94` |
| sym-5910cad8a108299c9553 | `DiscontinuityRecord::sequence_end` | struct_field | Stores the sequence end used by `DiscontinuityRecord`. | `src/recording/writer.rs:99` |
| sym-83781326ab920ef86604 | `DiscontinuityRecord::sequence_start` | struct_field | Stores the sequence start used by `DiscontinuityRecord`. | `src/recording/writer.rs:98` |
| sym-e1d1fb413497c6b7bc3f | `DiscontinuityRecord::stem_id` | struct_field | Identifies the stem identifier recorded by `DiscontinuityRecord`. | `src/recording/writer.rs:93` |
| sym-b900e3a681279e2edfbf | `DiscontinuityRecord::timestamp_end_ns` | struct_field | Stores the timestamp end value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:97` |
| sym-ae88bc73ec6d0e0cd105 | `DiscontinuityRecord::timestamp_start_ns` | struct_field | Stores the timestamp start value for `DiscontinuityRecord`, in nanoseconds. | `src/recording/writer.rs:96` |
| sym-41ef44f3ff8a327ca76c | `DispatchSummary::attempted_edges` | struct_field | Stores the attempted edges used by `DispatchSummary`. | `src/runtime/audio/router.rs:668` |
| sym-72a244e3d3ac5baea18f | `DispatchSummary::copy_pool_exhausted_edges` | struct_field | Stores the copy pool exhausted edges used by `DispatchSummary`. | `src/runtime/audio/router.rs:671` |
| sym-789bce62caa0c32945e6 | `DispatchSummary::dropped_edges` | struct_field | Stores the dropped edges used by `DispatchSummary`. | `src/runtime/audio/router.rs:670` |
| sym-45cb531bddfeb0b2c492 | `DispatchSummary::enqueued_edges` | struct_field | Stores the enqueued edges used by `DispatchSummary`. | `src/runtime/audio/router.rs:669` |
| sym-7c320fdb64a92450bbc0 | `DispatchSummary::freeze_failed_edges` | struct_field | Stores the freeze failed edges used by `DispatchSummary`. | `src/runtime/audio/router.rs:672` |
| sym-9ea69a9bc92f5e066f2a | `EdgeBufferPlan::bytes_per_frame` | struct_field | Stores the bytes per frame used by `EdgeBufferPlan`. | `src/graph/plan.rs:39` |
| sym-9fb179b4c7d60c76847f | `EdgeBufferPlan::capacity_frames` | struct_field | Sets the capacity frames available to `EdgeBufferPlan`. | `src/graph/plan.rs:38` |
| sym-71679349bbba7b07a540 | `EdgeBufferPlan::copy_policy` | struct_field | Stores the copy policy used by `EdgeBufferPlan`. | `src/graph/plan.rs:40` |
| sym-843a36a23a1732b0a201 | `EdgeBufferPlan::edge` | struct_field | Stores the edge used by `EdgeBufferPlan`. | `src/graph/plan.rs:37` |
| sym-47e3623d1b415a621c8a | `EdgeObservations::branch_pool_exhausted_drops_total` | struct_field | Counts the total number of branch pool exhausted drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:133` |
| sym-4ac16fd7e3b377d6e903 | `EdgeObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:136` |
| sym-b47c06287ad7a6211138 | `EdgeObservations::enqueue_to_receive_invalid_order_total` | struct_field | Counts the total number of enqueue to receive invalid order observed by `EdgeObservations`. | `src/runtime/audio/router.rs:143` |
| sym-5288c918c735cc9a1216 | `EdgeObservations::enqueue_to_receive_max_ns` | struct_field | Stores the enqueue to receive max value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:147` |
| sym-fbff4d9ceedfbca014fc | `EdgeObservations::enqueue_to_receive_p50_ns` | struct_field | Stores the enqueue to receive p50 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:144` |
| sym-0c3b83773f2038669c3f | `EdgeObservations::enqueue_to_receive_p95_ns` | struct_field | Stores the enqueue to receive p95 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:145` |
| sym-ac86547b5cd230a13eb9 | `EdgeObservations::enqueue_to_receive_p99_ns` | struct_field | Stores the enqueue to receive p99 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:146` |
| sym-bc7c6973361acb364962 | `EdgeObservations::enqueue_to_receive_samples_total` | struct_field | Counts the total number of enqueue to receive samples observed by `EdgeObservations`. | `src/runtime/audio/router.rs:142` |
| sym-bb1458c9aeba20eb433e | `EdgeObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `EdgeObservations`. | `src/runtime/audio/router.rs:127` |
| sym-ad173646ae53ead24336 | `EdgeObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `EdgeObservations`. | `src/runtime/audio/router.rs:128` |
| sym-6b5ca163d11e59f3053c | `EdgeObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `EdgeObservations`. | `src/runtime/audio/router.rs:126` |
| sym-caec7847ca03009cbf10 | `EdgeObservations::freeze_failed_drops_total` | struct_field | Counts the total number of freeze failed drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:135` |
| sym-d7bc1f92ddc22b55bc56 | `EdgeObservations::invalid_copy_policy_drops_total` | struct_field | Counts the total number of invalid copy policy drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:134` |
| sym-f426082b2bad7d00e2ff | `EdgeObservations::lineage_epoch_discontinuities_total` | struct_field | Counts the total number of lineage epoch discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:140` |
| sym-40201ed8d8a2aea5168e | `EdgeObservations::manually_reported_discontinuities_total` | struct_field | Counts the total number of manually reported discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:141` |
| sym-1cc760ad907e29424ed0 | `EdgeObservations::overruns_total` | struct_field | Counts the total number of overruns observed by `EdgeObservations`. | `src/runtime/audio/router.rs:129` |
| sym-601eebcf6ea33827e01f | `EdgeObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `EdgeObservations`. | `src/runtime/audio/router.rs:123` |
| sym-85bc41a8ada6d9672643 | `EdgeObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `EdgeObservations`. | `src/runtime/audio/router.rs:124` |
| sym-1119173d07b2e892ee38 | `EdgeObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:131` |
| sym-0852008e26815f2fde88 | `EdgeObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `EdgeObservations`. | `src/runtime/audio/router.rs:125` |
| sym-70cef97a269924759a43 | `EdgeObservations::receiver_unavailable_drops_total` | struct_field | Counts the total number of receiver unavailable drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:130` |
| sym-5050e66e625c418af9a1 | `EdgeObservations::sequence_discontinuities_total` | struct_field | Counts the total number of sequence discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:138` |
| sym-a103ab8553b6c5d3c5f2 | `EdgeObservations::shared_reference_exhausted_drops_total` | struct_field | Counts the total number of shared reference exhausted drops observed by `EdgeObservations`. | `src/runtime/audio/router.rs:132` |
| sym-a335fc3cf257d5899527 | `EdgeObservations::shutdown_discarded_total` | struct_field | Counts the total number of shutdown discarded observed by `EdgeObservations`. | `src/runtime/audio/router.rs:156` |
| sym-f73f04221210656735f8 | `EdgeObservations::source_identity_discontinuities_total` | struct_field | Counts the total number of source identity discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:137` |
| sym-19c5913f4ba22b5ae32f | `EdgeObservations::source_timestamp_to_receive_future_total` | struct_field | Counts the total number of source timestamp to receive future observed by `EdgeObservations`. | `src/runtime/audio/router.rs:150` |
| sym-6cb391b8a4fad0ba4453 | `EdgeObservations::source_timestamp_to_receive_max_ns` | struct_field | Stores the source timestamp to receive max value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:154` |
| sym-f5472b4821fa709ee930 | `EdgeObservations::source_timestamp_to_receive_missing_total` | struct_field | Counts the total number of source timestamp to receive missing observed by `EdgeObservations`. | `src/runtime/audio/router.rs:149` |
| sym-a75ab8add3e46c145cc4 | `EdgeObservations::source_timestamp_to_receive_p50_ns` | struct_field | Stores the source timestamp to receive p50 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:151` |
| sym-10e8b329422f80e8a165 | `EdgeObservations::source_timestamp_to_receive_p95_ns` | struct_field | Stores the source timestamp to receive p95 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:152` |
| sym-86c08adf73a55dedf9fc | `EdgeObservations::source_timestamp_to_receive_p99_ns` | struct_field | Stores the source timestamp to receive p99 value for `EdgeObservations`, in nanoseconds. | `src/runtime/audio/router.rs:153` |
| sym-e4990c2cfb74c8b108ec | `EdgeObservations::source_timestamp_to_receive_samples_total` | struct_field | Counts the total number of source timestamp to receive samples observed by `EdgeObservations`. | `src/runtime/audio/router.rs:148` |
| sym-3d176e807ef44a736ef8 | `EdgeObservations::timestamp_discontinuities_total` | struct_field | Counts the total number of timestamp discontinuities observed by `EdgeObservations`. | `src/runtime/audio/router.rs:139` |
| sym-f1a278e06f836d00af1e | `EdgeObservations::worker_failures_total` | struct_field | Counts the total number of worker failures observed by `EdgeObservations`. | `src/runtime/audio/router.rs:155` |
| sym-43a31e1ee06a6e0705d0 | `EdgeSpec::from` | struct_field | Identifies the origin represented by `EdgeSpec`. | `src/graph/spec.rs:52` |
| sym-6b633ac3c7f6c13e0b42 | `EdgeSpec::id` | struct_field | Identifies the id recorded by `EdgeSpec`. | `src/graph/spec.rs:51` |
| sym-cf6401ef9ba128da1b43 | `EdgeSpec::requested` | struct_field | Stores the requested used by `EdgeSpec`. | `src/graph/spec.rs:54` |
| sym-f1bd2483ddf7594d5190 | `EdgeSpec::to` | struct_field | Identifies the destination represented by `EdgeSpec`. | `src/graph/spec.rs:53` |
| sym-9eafa855d0adb4757b7b | `EndpointCancellationOutcome::observations` | struct_field | Carries the observations collected for `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:290` |
| sym-b54f840f9c32f7ba9fc2 | `EndpointCancellationOutcome::result` | struct_field | Stores the result used by `EndpointCancellationOutcome`. | `src/endpoint/runtime.rs:291` |
| sym-cc3e93fe46141b5cadd6 | `EndpointDriverFinalization::observations` | struct_field | Carries the observations collected for `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:296` |
| sym-ae3c0e1cee9ba34d9710 | `EndpointDriverFinalization::result` | struct_field | Stores the result used by `EndpointDriverFinalization`. | `src/endpoint/runtime.rs:297` |
| sym-f18f449a15a26fbedd15 | `EndpointDriverObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:232` |
| sym-3e8d7dc3ceadead997e3 | `EndpointDriverObservations::failures_total` | struct_field | Counts the total number of failures observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:233` |
| sym-93f41b49fc7737f0321b | `EndpointDriverObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:230` |
| sym-18239882504feb615200 | `EndpointDriverObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:231` |
| sym-e6ef748486e51d8018eb | `EndpointDriverObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `EndpointDriverObservations`. | `src/endpoint/runtime.rs:229` |
| sym-67da87ab6ac014eea575 | `EndpointDriverRegistryError::Duplicate::node_type_id` | struct_field | Identifies the node type identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:26` |
| sym-bb4e15a5c3c9d2517beb | `EndpointDriverRegistryError::Duplicate::operator_id` | struct_field | Identifies the operator identifier recorded by `Duplicate`. | `src/endpoint/registry.rs:25` |
| sym-2a1cc821592407f99fe0 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::operator_id` | struct_field | Identifies the operator identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:32` |
| sym-bb09ca0c12744042898c | `EndpointDriverRegistryError::OperatorNodeTypeConflict::registered_node_type_id` | struct_field | Identifies the registered node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:33` |
| sym-98e8637e19152ed24ef6 | `EndpointDriverRegistryError::OperatorNodeTypeConflict::requested_node_type_id` | struct_field | Identifies the requested node type identifier recorded by `OperatorNodeTypeConflict`. | `src/endpoint/registry.rs:34` |
| sym-9862e96f4633eaf76a35 | `EndpointExtensionRegistrationError::ConflictingDefinition::node_type_id` | struct_field | Identifies the node type identifier recorded by `ConflictingDefinition`. | `src/session/lifecycle/engine.rs:311` |
| sym-897fec9d31bb50be35c5 | `EndpointFinalizationOutcome::join_finalize_result` | struct_field | Stores the join finalize result used by `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:304` |
| sym-cd9d7d3c0023b209e9a4 | `EndpointFinalizationOutcome::observations` | struct_field | Carries the observations collected for `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:302` |
| sym-d76d3bb026b7263f74a1 | `EndpointFinalizationOutcome::request_stop_result` | struct_field | Stores the request stop result used by `EndpointFinalizationOutcome`. | `src/endpoint/runtime.rs:303` |
| sym-b315d98af68d13d169b2 | `EndpointInputOrigin::Source::audio_stem_id` | struct_field | Identifies the audio stem identifier recorded by `Source`. | `src/endpoint/runtime.rs:38` |
| sym-536711f3c786eb7932b4 | `EndpointInputOrigin::Source::source_id` | struct_field | Identifies the source identifier recorded by `Source`. | `src/endpoint/runtime.rs:36` |
| sym-5f1e40074f4ad284fe18 | `EndpointInputOrigin::Source::stream_id` | struct_field | Identifies the stream identifier recorded by `Source`. | `src/endpoint/runtime.rs:37` |
| sym-d630f30f329338ce4a5c | `EndpointPrepareError::NotRegistered::node_type_id` | struct_field | Identifies the node type identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:47` |
| sym-099bd3e73344dda7dd3e | `EndpointPrepareError::NotRegistered::operator_id` | struct_field | Identifies the operator identifier recorded by `NotRegistered`. | `src/endpoint/registry.rs:46` |
| sym-df6e5a09858ffdcc4441 | `EndpointReceiver::Audio::receiver` | struct_field | Stores the receiver used by `Audio`. | `src/endpoint/contract.rs:147` |
| sym-3ce0fa40a2cba322e866 | `EndpointReceiver::Audio::sample_spec` | struct_field | Stores the sample spec used by `Audio`. | `src/endpoint/contract.rs:148` |
| sym-942d13e2b3b6d30af0b6 | `FanInGroup::into` | struct_field | Stores the into used by `FanInGroup`. | `src/graph/plan.rs:91` |
| sym-389442ec3eaa502b667b | `FanInGroup::sources` | struct_field | Stores the sources used by `FanInGroup`. | `src/graph/plan.rs:92` |
| sym-cb1b8fe4463dac8baeae | `FanOutGroup::from` | struct_field | Identifies the origin represented by `FanOutGroup`. | `src/graph/plan.rs:85` |
| sym-47edd5cd98814ff2943e | `FanOutGroup::targets` | struct_field | Stores the targets used by `FanOutGroup`. | `src/graph/plan.rs:86` |
| sym-1f3d24ffd4f2c8ba539f | `GeneratedAudioBridgeSpec::clock_id` | struct_field | Identifies the clock identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:24` |
| sym-10ff3aa0094634e1266a | `GeneratedAudioBridgeSpec::pool_slots` | struct_field | Stores the pool slots used by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:27` |
| sym-40243f21cb7873eb594a | `GeneratedAudioBridgeSpec::sample_spec` | struct_field | Stores the sample spec used by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:25` |
| sym-7ae09e214d850afdf3ad | `GeneratedAudioBridgeSpec::samples_per_frame` | struct_field | Stores the samples per frame used by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:26` |
| sym-471e7987356a65af83ea | `GeneratedAudioBridgeSpec::session_id` | struct_field | Identifies the session identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:20` |
| sym-f9548a86b068876b48f0 | `GeneratedAudioBridgeSpec::source_id` | struct_field | Identifies the source identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:23` |
| sym-e530e14c92446441ef96 | `GeneratedAudioBridgeSpec::stem_id` | struct_field | Identifies the stem identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:21` |
| sym-cd2379392ad1162663ab | `GeneratedAudioBridgeSpec::stream_id` | struct_field | Identifies the stream identifier recorded by `GeneratedAudioBridgeSpec`. | `src/runtime/bridge/audio.rs:22` |
| sym-8d918cba76f2bac963e3 | `GraphIr::edges` | struct_field | Stores the edges used by `GraphIr`. | `src/graph/ir.rs:34` |
| sym-4122f5dd06beaffbca77 | `GraphIr::nodes` | struct_field | Stores the nodes used by `GraphIr`. | `src/graph/ir.rs:33` |
| sym-cf85ca9ac0c1987ffe11 | `GraphIr::topo_order` | struct_field | Stores the topo order used by `GraphIr`. | `src/graph/ir.rs:35` |
| sym-630fe74f11dd1ff77eb0 | `GraphSpec::edges` | struct_field | Stores the edges used by `GraphSpec`. | `src/graph/spec.rs:60` |
| sym-bbaa9928b84ae6b393ae | `GraphSpec::nodes` | struct_field | Stores the nodes used by `GraphSpec`. | `src/graph/spec.rs:59` |
| sym-2e16f081e17dfc80a429 | `InputPortRef::node` | struct_field | Stores the node used by `InputPortRef`. | `src/graph/spec.rs:38` |
| sym-ae5e084b69767f0211df | `InputPortRef::port` | struct_field | Stores the port used by `InputPortRef`. | `src/graph/spec.rs:39` |
| sym-131b9888ec322ea27158 | `MemoryPlan::branch_copy_pool_bytes` | struct_field | Stores the branch copy pool size for `MemoryPlan`, in bytes. | `src/graph/plan.rs:66` |
| sym-d2232b7e64dac12dea83 | `MemoryPlan::edge_buffers` | struct_field | Stores the edge buffers used by `MemoryPlan`. | `src/graph/plan.rs:67` |
| sym-fab6024f9d0090769e81 | `MemoryPlan::realtime_pool_bytes` | struct_field | Stores the realtime pool size for `MemoryPlan`, in bytes. | `src/graph/plan.rs:65` |
| sym-a58fb860fb50ac3f0905 | `NativeSessionEngineHostOptions::polled_audio_endpoint` | struct_field | Stores the polled audio endpoint used by `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:168` |
| sym-d7c726aa64031950d79b | `NativeSessionEngineHostOptions::sample_spec` | struct_field | Stores the sample spec used by `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:165` |
| sym-9168a6bf58c568312956 | `NativeSessionEngineHostOptions::source_queue_capacity_frames` | struct_field | Sets the source queue capacity frames available to `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:166` |
| sym-cb581b1ab2e6f290bfee | `NativeSessionEngineHostOptions::start_options` | struct_field | Stores the start options used by `NativeSessionEngineHostOptions`. | `src/session/lifecycle/host.rs:167` |
| sym-f85ae1852c8d3d784a9d | `NodeError::ExternalBoundaryExecution::node_type_id` | struct_field | Identifies the node type identifier recorded by `ExternalBoundaryExecution`. | `src/graph/node.rs:159` |
| sym-8f952e6d830eb28e48a9 | `NodeError::ProcessTimeout::timeout_ms` | struct_field | Stores the timeout value for `ProcessTimeout`, in milliseconds. | `src/graph/node.rs:155` |
| sym-27e74626136711d6b8c8 | `NodeRegistrationError::DuplicateNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `DuplicateNodeType`. | `src/graph/registry.rs:61` |
| sym-e14aa88d15bebcd09773 | `NodeRegistrationError::DuplicateOperatorId::operator_id` | struct_field | Identifies the operator identifier recorded by `DuplicateOperatorId`. | `src/graph/registry.rs:63` |
| sym-253f53d4a21365943c97 | `NodeSpec::config` | struct_field | Stores the config used by `NodeSpec`. | `src/graph/spec.rs:46` |
| sym-c49759b896a1f249a135 | `NodeSpec::id` | struct_field | Identifies the id recorded by `NodeSpec`. | `src/graph/spec.rs:44` |
| sym-ee51850deb3aa7809088 | `NodeSpec::type_id` | struct_field | Identifies the type identifier recorded by `NodeSpec`. | `src/graph/spec.rs:45` |
| sym-708647bb2c7c15399945 | `OperatorDeadlinePolicy::process_timeout_ms` | struct_field | Stores the process timeout value for `OperatorDeadlinePolicy`, in milliseconds. | `src/graph/signal/operator.rs:53` |
| sym-6d16ee49b9c74b65b4ab | `OperatorOutputRolePolicy::allowed` | struct_field | Stores the allowed used by `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:70` |
| sym-6b95877721f5da36d7c5 | `OperatorOutputRolePolicy::terminal` | struct_field | Indicates whether terminal applies to `OperatorOutputRolePolicy`. | `src/graph/signal/operator.rs:71` |
| sym-291d3a1228f2ef2b38e7 | `OperatorPermissionPolicy::filesystem_allowed` | struct_field | Stores the filesystem allowed used by `OperatorPermissionPolicy`. | `src/graph/signal/operator.rs:48` |
| sym-2dbcca788b24d1a47895 | `OperatorPermissionPolicy::network_allowed` | struct_field | Stores the network allowed used by `OperatorPermissionPolicy`. | `src/graph/signal/operator.rs:47` |
| sym-c4bc476bb4fb56c40af9 | `OpusConfig::application` | struct_field | Selects the Opus application mode used when the encoder is created. | `src/codec/encoder.rs:80` |
| sym-2b5616507ada775062c4 | `OpusConfig::bitrate_kbps` | struct_field | Target bitrate in kbps. None = Opus auto (variable bitrate). | `src/codec/encoder.rs:82` |
| sym-007e3405aec6a783cc0d | `OpusConfig::channels` | struct_field | Selects the mono or stereo channel layout accepted by the encoder. | `src/codec/encoder.rs:76` |
| sym-2d0fcaa9179bea6d3de5 | `OpusConfig::complexity` | struct_field | Encoder complexity 0–10. Higher = better quality, more CPU. | `src/codec/encoder.rs:84` |
| sym-4484c0aeca53126e0cb2 | `OpusConfig::dtx` | struct_field | Discontinuous transmission (silence suppression). | `src/codec/encoder.rs:86` |
| sym-3bb363207da0a142620f | `OpusConfig::fec` | struct_field | In-band forward error correction. | `src/codec/encoder.rs:88` |
| sym-191188211296f8867449 | `OpusConfig::frame_duration` | struct_field | Frame duration. Default: 20 ms (AUDIO-012). | `src/codec/encoder.rs:78` |
| sym-dd0722b130cdb77f7f4e | `OpusConfig::sample_rate` | struct_field | Sample rate. Opus only supports 48 kHz internally. | `src/codec/encoder.rs:74` |
| sym-923be3ef9dae457fab1d | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::maximum_samples_per_channel` | struct_field | Stores the maximum samples per channel used by `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:31` |
| sym-243218e6112065479597 | `OpusDecodeError::FrameDurationExceedsConfiguredMaximum::requested_samples_per_channel` | struct_field | Stores the requested samples per channel used by `FrameDurationExceedsConfiguredMaximum`. | `src/codec/decoder.rs:30` |
| sym-e32f4a59510868afd2bc | `OpusEncodeError::InvalidFrameSampleCount::channels` | struct_field | Stores the channels used by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:137` |
| sym-67adeb362ec785bcc0fe | `OpusEncodeError::InvalidFrameSampleCount::expected_sample_count` | struct_field | Stores the number of expected sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:138` |
| sym-ff5649d385c39e7d37d4 | `OpusEncodeError::InvalidFrameSampleCount::sample_count` | struct_field | Stores the number of sample represented by `InvalidFrameSampleCount`. | `src/codec/encoder.rs:136` |
| sym-f606aaed26e625461190 | `OutputPortRef::node` | struct_field | Stores the node used by `OutputPortRef`. | `src/graph/spec.rs:32` |
| sym-294eb0404bf911668b2f | `OutputPortRef::port` | struct_field | Stores the port used by `OutputPortRef`. | `src/graph/spec.rs:33` |
| sym-6d4eb4ebaedc826e8df6 | `PartitionGroup::execution` | struct_field | Stores the execution used by `PartitionGroup`. | `src/graph/plan.rs:79` |
| sym-1af7e87d7bdae905b7f5 | `PartitionGroup::nodes` | struct_field | Stores the nodes used by `PartitionGroup`. | `src/graph/plan.rs:80` |
| sym-0db6c4c8e3c8eef8794b | `PksExtensionAbiVersion::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:16` |
| sym-61bf3eecc5a6498ce0a5 | `PksExtensionAbiVersion::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionAbiVersion`. | `src/abi/extension.rs:17` |
| sym-359a061e0f765240b0e7 | `PksExtensionAbiVersion::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionAbiVersion` ABI structure. | `src/abi/extension.rs:15` |
| sym-8bc7e5531d5f4ebeabda | `PksExtensionCallbacks::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:93` |
| sym-dab1881821172b856539 | `PksExtensionCallbacks::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:94` |
| sym-599b2a991f18d418e6df | `PksExtensionCallbacks::create` | struct_field | Provides the create callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:99` |
| sym-98126a76f7c0767dbb2f | `PksExtensionCallbacks::destroy_instance` | struct_field | Provides the destroy instance callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:106` |
| sym-3266527728cbcf1d1263 | `PksExtensionCallbacks::destroy_registration` | struct_field | Provides the destroy registration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:107` |
| sym-7df0c15a0078af5652ee | `PksExtensionCallbacks::endpoint_consume` | struct_field | Provides the endpoint consume callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:103` |
| sym-a1e986f527710e8625ba | `PksExtensionCallbacks::finish` | struct_field | Provides the finish callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:105` |
| sym-f4c8933bb9ce216ef72f | `PksExtensionCallbacks::max_payload_bytes` | struct_field | Limits payload storage for `PksExtensionCallbacks`, in bytes. | `src/abi/executable_extension.rs:96` |
| sym-522407c189871214eb21 | `PksExtensionCallbacks::operator_process` | struct_field | Provides the operator process callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:102` |
| sym-5d77bd53a7d9d8b2d701 | `PksExtensionCallbacks::prepare` | struct_field | Provides the prepare callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:100` |
| sym-4d09a07c66dbbf9811f4 | `PksExtensionCallbacks::registration_context` | struct_field | Carries the opaque registration context used by `PksExtensionCallbacks` callbacks. | `src/abi/executable_extension.rs:95` |
| sym-47be468b1863e10030a3 | `PksExtensionCallbacks::request_stop` | struct_field | Provides the request stop callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:104` |
| sym-53f881a9867fa7f1f1aa | `PksExtensionCallbacks::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:97` |
| sym-30f44a2d9d92969320be | `PksExtensionCallbacks::source_next` | struct_field | Provides the source next callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:101` |
| sym-3ae4ce3f92f139cdd659 | `PksExtensionCallbacks::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionCallbacks` ABI structure. | `src/abi/executable_extension.rs:92` |
| sym-eb0dca1e517ca0c1c0e2 | `PksExtensionCallbacks::validate_configuration` | struct_field | Provides the validate configuration callback used by `PksExtensionCallbacks`. | `src/abi/executable_extension.rs:98` |
| sym-abcd0237ad46888633d5 | `PksExtensionDescriptor::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:49` |
| sym-5ca838c1da4cb525fa85 | `PksExtensionDescriptor::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionDescriptor`. | `src/abi/extension.rs:50` |
| sym-0f4d4d22a0e6fd2eb32e | `PksExtensionDescriptor::extension_id` | struct_field | Identifies the extension identifier recorded by `PksExtensionDescriptor`. | `src/abi/extension.rs:55` |
| sym-1e35a852cf8422cfda75 | `PksExtensionDescriptor::generation` | struct_field | Stores the generation used by `PksExtensionDescriptor`. | `src/abi/extension.rs:53` |
| sym-89d0ad00d57c7921b628 | `PksExtensionDescriptor::kind` | struct_field | Stores the kind used by `PksExtensionDescriptor`. | `src/abi/extension.rs:51` |
| sym-f46920a3fa7f30ff7673 | `PksExtensionDescriptor::port_count` | struct_field | Stores the number of port represented by `PksExtensionDescriptor`. | `src/abi/extension.rs:54` |
| sym-a14b3b8e2f94ee7318d3 | `PksExtensionDescriptor::revision` | struct_field | Stores the revision used by `PksExtensionDescriptor`. | `src/abi/extension.rs:52` |
| sym-99130f9c9a586b1285aa | `PksExtensionDescriptor::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionDescriptor` ABI structure. | `src/abi/extension.rs:48` |
| sym-cb5ea79192e835be63b4 | `PksExtensionLibrary::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:125` |
| sym-0ca96d62385da73c2546 | `PksExtensionLibrary::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:126` |
| sym-0260646c2de1572ba96f | `PksExtensionLibrary::acquire_registration` | struct_field | Provides the acquire registration callback used by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:130` |
| sym-0d7fd33f9f0952bf4ee9 | `PksExtensionLibrary::library_context` | struct_field | Carries the opaque library context used by `PksExtensionLibrary` callbacks. | `src/abi/executable_extension.rs:129` |
| sym-302714882ef5b2a2fa83 | `PksExtensionLibrary::registration_count` | struct_field | Stores the number of registration represented by `PksExtensionLibrary`. | `src/abi/executable_extension.rs:127` |
| sym-09da8e8904c29d2d2f56 | `PksExtensionLibrary::reserved` | struct_field | Reserves storage for forward-compatible evolution of `PksExtensionLibrary`. | `src/abi/executable_extension.rs:128` |
| sym-e50c6c86107f9d04ccfe | `PksExtensionLibrary::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionLibrary` ABI structure. | `src/abi/executable_extension.rs:124` |
| sym-e197462f13f33ef433f9 | `PksExtensionPipelineDeclaration::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:170` |
| sym-d744ad468d2792262a95 | `PksExtensionPipelineDeclaration::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:171` |
| sym-e72c8374c009be1e17cc | `PksExtensionPipelineDeclaration::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:177` |
| sym-c8d2d4bb6fadfa41d72e | `PksExtensionPipelineDeclaration::endpoint_input_port` | struct_field | Stores the endpoint input port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:178` |
| sym-7239e71c128d9ccb3a98 | `PksExtensionPipelineDeclaration::operator_id` | struct_field | Identifies the operator identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:174` |
| sym-08ab9b48097e42ccb58f | `PksExtensionPipelineDeclaration::operator_input_port` | struct_field | Stores the operator input port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:175` |
| sym-d47b28e1665ddbfc4c37 | `PksExtensionPipelineDeclaration::operator_output_port` | struct_field | Stores the operator output port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:176` |
| sym-77247e2a7fc20076a61f | `PksExtensionPipelineDeclaration::source_id` | struct_field | Identifies the source identifier recorded by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:172` |
| sym-3bf095f4da2d36534f8d | `PksExtensionPipelineDeclaration::source_output_port` | struct_field | Stores the source output port used by `PksExtensionPipelineDeclaration`. | `src/abi/executable_extension.rs:173` |
| sym-95b0b3761e220a8fd8bb | `PksExtensionPipelineDeclaration::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPipelineDeclaration` ABI structure. | `src/abi/executable_extension.rs:169` |
| sym-1ede5196a6baf13504c8 | `PksExtensionPort::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:62` |
| sym-92de720178a3677aaa75 | `PksExtensionPort::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionPort`. | `src/abi/extension.rs:63` |
| sym-3430f45987e8b747886a | `PksExtensionPort::direction` | struct_field | Stores the direction used by `PksExtensionPort`. | `src/abi/extension.rs:64` |
| sym-24332ac9dcfae7bc9f70 | `PksExtensionPort::name` | struct_field | Stores the name used by `PksExtensionPort`. | `src/abi/extension.rs:66` |
| sym-29f827a3d1ea5505235d | `PksExtensionPort::required` | struct_field | Indicates whether required applies to `PksExtensionPort`. | `src/abi/extension.rs:65` |
| sym-f497b7f5351c84b5f18c | `PksExtensionPort::schema` | struct_field | Stores the schema used by `PksExtensionPort`. | `src/abi/extension.rs:69` |
| sym-b9da34912ba076cf18d0 | `PksExtensionPort::semantic_role` | struct_field | Stores the semantic role used by `PksExtensionPort`. | `src/abi/extension.rs:68` |
| sym-a3ebc882dc49ed1311dd | `PksExtensionPort::signal_id` | struct_field | Identifies the signal identifier recorded by `PksExtensionPort`. | `src/abi/extension.rs:67` |
| sym-9c4cd90eef0d3e802c7f | `PksExtensionPort::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionPort` ABI structure. | `src/abi/extension.rs:61` |
| sym-a95b6ef65a7d569748f5 | `PksExtensionSignalBuffer::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:155` |
| sym-8fa2405a53f09a512414 | `PksExtensionSignalBuffer::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:156` |
| sym-2a42363a1803788c54f0 | `PksExtensionSignalBuffer::capacity_bytes` | struct_field | Stores the capacity size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:158` |
| sym-f47b06e230874e8f392e | `PksExtensionSignalBuffer::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:157` |
| sym-eda5ddd827a09334004e | `PksExtensionSignalBuffer::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:163` |
| sym-e23e3cff54d1066910c6 | `PksExtensionSignalBuffer::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalBuffer`. | `src/abi/executable_extension.rs:160` |
| sym-3f7d813fdaa6e1bbce85 | `PksExtensionSignalBuffer::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalBuffer`, in bytes. | `src/abi/executable_extension.rs:159` |
| sym-c97d55f25ab252aee445 | `PksExtensionSignalBuffer::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:161` |
| sym-d5374acccd0afe623ad9 | `PksExtensionSignalBuffer::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalBuffer`, in nanoseconds. | `src/abi/executable_extension.rs:162` |
| sym-cd59cbb262c2aa7c160f | `PksExtensionSignalBuffer::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalBuffer` ABI structure. | `src/abi/executable_extension.rs:154` |
| sym-7c18c4e3d29a42664727 | `PksExtensionSignalView::abi_major` | struct_field | Stores the major ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:140` |
| sym-a22b5837d46373c3682a | `PksExtensionSignalView::abi_minor` | struct_field | Stores the minor ABI version expected by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:141` |
| sym-0a5eaaf109f4a21fc1b3 | `PksExtensionSignalView::data` | struct_field | Carries the data owned or referenced by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:142` |
| sym-fcb856959047ed19b3be | `PksExtensionSignalView::duration_ns` | struct_field | Stores the duration value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:147` |
| sym-d292a82a33e80eaaa9f8 | `PksExtensionSignalView::flags` | struct_field | Carries the bit flags defined by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:144` |
| sym-c1c8d06838fb0e9fce04 | `PksExtensionSignalView::len_bytes` | struct_field | Stores the len size for `PksExtensionSignalView`, in bytes. | `src/abi/executable_extension.rs:143` |
| sym-ac6eeb8886eb4844b4f0 | `PksExtensionSignalView::observed_timestamp_ns` | struct_field | Stores the observed timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:145` |
| sym-ae98ce87b33de0a77c79 | `PksExtensionSignalView::sequence_number` | struct_field | Stores the sequence number used by `PksExtensionSignalView`. | `src/abi/executable_extension.rs:148` |
| sym-e24bb7bc7b74e43341b6 | `PksExtensionSignalView::source_timestamp_ns` | struct_field | Stores the source timestamp value for `PksExtensionSignalView`, in nanoseconds. | `src/abi/executable_extension.rs:146` |
| sym-8013fce198748680db57 | `PksExtensionSignalView::struct_size_bytes` | struct_field | Stores the byte size of the `PksExtensionSignalView` ABI structure. | `src/abi/executable_extension.rs:139` |
| sym-3aeea7e8779fe554c1dd | `PksSessionStatus::code` | struct_field | Stores the code used by `PksSessionStatus`. | `src/abi/session/abi.rs:57` |
| sym-a964cbfac0b18ce4d0a8 | `PksSessionStatus::detail` | struct_field | Stores the detail used by `PksSessionStatus`. | `src/abi/session/abi.rs:58` |
| sym-4850a01a3818712b3bc8 | `PksSessionUtf8::data` | struct_field | Carries the data owned or referenced by `PksSessionUtf8`. | `src/abi/session/abi.rs:102` |
| sym-f1d5e74b271b98a97960 | `PksSessionUtf8::len_bytes` | struct_field | Stores the len size for `PksSessionUtf8`, in bytes. | `src/abi/session/abi.rs:103` |
| sym-8d74553c30756ec314b7 | `PlanError::FanInOnSinglePort::node` | struct_field | Stores the node used by `FanInOnSinglePort`. | `src/graph/plan.rs:23` |
| sym-cc71ed110282f0771c10 | `PlanError::FanInOnSinglePort::port` | struct_field | Stores the port used by `FanInOnSinglePort`. | `src/graph/plan.rs:23` |
| sym-941b99fe190f7e2df514 | `PlanError::MissingEdgeContract::edge` | struct_field | Stores the edge used by `MissingEdgeContract`. | `src/graph/plan.rs:27` |
| sym-3db297d63e39bf5830c9 | `PlanError::MissingOutputSignal::edge` | struct_field | Stores the edge used by `MissingOutputSignal`. | `src/graph/plan.rs:29` |
| sym-fd7854830f4f7951c19a | `PlanError::MoveExclusiveFanOut::node` | struct_field | Stores the node used by `MoveExclusiveFanOut`. | `src/graph/plan.rs:25` |
| sym-e250a705cb5bd6cfc20e | `PlanError::MoveExclusiveFanOut::port` | struct_field | Stores the port used by `MoveExclusiveFanOut`. | `src/graph/plan.rs:25` |
| sym-275f7c1e91232399e8bd | `PlanExecutionSummary::edges_attempted` | struct_field | Stores the edges attempted used by `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:39` |
| sym-97ae66ef846dc552877f | `PlanExecutionSummary::edges_dropped` | struct_field | Stores the edges dropped used by `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:41` |
| sym-a267d7b850b3166c7fde | `PlanExecutionSummary::edges_enqueued` | struct_field | Stores the edges enqueued used by `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:40` |
| sym-1d67a9e3e03dd6b8db3f | `PlanExecutionSummary::nodes_executed` | struct_field | Stores the nodes executed used by `PlanExecutionSummary`. | `src/runtime/audio/executor.rs:38` |
| sym-7e659ada14ea7cb3e3df | `PlanRouterError::InvalidFrameBytes::bytes_per_frame` | struct_field | Stores the bytes per frame used by `InvalidFrameBytes`. | `src/runtime/audio/router.rs:25` |
| sym-b26952d4e9ec79f5ebbf | `PlanRouterError::InvalidFrameBytes::edge_id` | struct_field | Identifies the edge identifier recorded by `InvalidFrameBytes`. | `src/runtime/audio/router.rs:24` |
| sym-f33931deefed32253677 | `PlanRouterError::MissingMemoryPlan::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingMemoryPlan`. | `src/runtime/audio/router.rs:19` |
| sym-0c55e34161931a26e5b3 | `PlanRouterError::ZeroCapacity::edge_id` | struct_field | Identifies the edge identifier recorded by `ZeroCapacity`. | `src/runtime/audio/router.rs:21` |
| sym-b73ef914e458f346c574 | `PlanRunnerError::DuplicateSource::source_node_id` | struct_field | Identifies the source node identifier recorded by `DuplicateSource`. | `src/runtime/audio/runner.rs:260` |
| sym-6a811a6f1a8fdf37a184 | `PlanRunnerError::ZeroSourceCapacity::source_node_id` | struct_field | Identifies the source node identifier recorded by `ZeroSourceCapacity`. | `src/runtime/audio/runner.rs:258` |
| sym-f84fd3f4a4bf1badb1f2 | `PlanRunnerFinishSummary::drain_budget_exhausted` | struct_field | Stores the drain budget exhausted used by `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:301` |
| sym-5078383450bb56808138 | `PlanRunnerFinishSummary::execution` | struct_field | Stores the execution used by `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:302` |
| sym-a6c7c332b756f218b49a | `PlanRunnerFinishSummary::source_frames_discarded_total` | struct_field | Counts the total number of source frames discarded observed by `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:300` |
| sym-3920d1632a41fd485db0 | `PlanRunnerFinishSummary::source_frames_processed_total` | struct_field | Counts the total number of source frames processed observed by `PlanRunnerFinishSummary`. | `src/runtime/audio/runner.rs:299` |
| sym-efa60cb5966217baf916 | `PlanRunnerStepSummary::execution` | struct_field | Stores the execution used by `PlanRunnerStepSummary`. | `src/runtime/audio/runner.rs:272` |
| sym-b227ea3ddc3a14c67e10 | `PlanRunnerStepSummary::source_frames_processed_total` | struct_field | Counts the total number of source frames processed observed by `PlanRunnerStepSummary`. | `src/runtime/audio/runner.rs:271` |
| sym-94e657bd0dbc9fda6c7a | `PlanSourceInputObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:27` |
| sym-192dee08ac689ce24bd9 | `PlanSourceInputObservations::frames_discarded_total` | struct_field | Counts the total number of frames discarded observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:30` |
| sym-fd3ee0a3dcf011bbe059 | `PlanSourceInputObservations::frames_enqueued_total` | struct_field | Counts the total number of frames enqueued observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:26` |
| sym-e27c655c950b4569e255 | `PlanSourceInputObservations::frames_rejected_cancelled_total` | struct_field | Counts the total number of frames rejected cancelled observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:29` |
| sym-4bb6462c104a47a85d30 | `PlanSourceInputObservations::frames_rejected_full_total` | struct_field | Counts the total number of frames rejected full observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:28` |
| sym-e09247184685c97af3cb | `PlanSourceInputObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:23` |
| sym-65891132c39f25c95570 | `PlanSourceInputObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:24` |
| sym-6272ec6423d93d0c373a | `PlanSourceInputObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `PlanSourceInputObservations`. | `src/runtime/audio/runner.rs:25` |
| sym-6cd8927e42b28f15eb81 | `PlanSourceSendOutcome::Rejected::error` | struct_field | Stores the error used by `Rejected`. | `src/runtime/audio/runner.rs:126` |
| sym-b262fd6c160bc1aa6f07 | `PlanSourceSendOutcome::Rejected::frame` | struct_field | Stores the frame used by `Rejected`. | `src/runtime/audio/runner.rs:127` |
| sym-21adcffcd2a34aa678ab | `PolledAudioEndpointConfig::max_batch_frames` | struct_field | Stores the max batch frames used by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:25` |
| sym-5120060d28c6de64186d | `PolledAudioEndpointConfig::max_outstanding_leases` | struct_field | Stores the max outstanding leases used by `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:26` |
| sym-736e0245e5e31232c39f | `PolledAudioEndpointConfig::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioEndpointConfig`. | `src/endpoint/polled_audio_driver.rs:24` |
| sym-50983d32097e8f36d929 | `PolledAudioObservations::batches_polled_total` | struct_field | Counts the total number of batches polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:69` |
| sym-5cbaf29414a52c644cec | `PolledAudioObservations::frames_delivered_total` | struct_field | Counts the total number of frames delivered observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:63` |
| sym-cb13211dc1085ed0e1c3 | `PolledAudioObservations::frames_polled_total` | struct_field | Counts the total number of frames polled observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:70` |
| sym-bb3bc37eb133eaa9f31c | `PolledAudioObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:62` |
| sym-b210aac23de5cf323bd9 | `PolledAudioObservations::invalid_ownership_drops_total` | struct_field | Counts the total number of invalid ownership drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:65` |
| sym-4bfd19b0b4c430a49114 | `PolledAudioObservations::lease_capacity_count` | struct_field | Sets the lease capacity count available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:66` |
| sym-eda9d5d63eb6e6be3b64 | `PolledAudioObservations::lease_exhausted_total` | struct_field | Counts the total number of lease exhausted observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:68` |
| sym-77b965237b400a2c6fd2 | `PolledAudioObservations::outstanding_leases` | struct_field | Stores the outstanding leases used by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:67` |
| sym-fb89adcd966167edc480 | `PolledAudioObservations::queue_capacity_frames` | struct_field | Sets the queue capacity frames available to `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:58` |
| sym-8cee7b51a4d6045b835b | `PolledAudioObservations::queue_depth_frames` | struct_field | Reports the queue depth frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:59` |
| sym-fcd3825abc7b443ccb32 | `PolledAudioObservations::queue_depth_invariant_failures_total` | struct_field | Counts the total number of queue depth invariant failures observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:61` |
| sym-4d3e7691869e45928567 | `PolledAudioObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:64` |
| sym-0322f69305ad1efb7f89 | `PolledAudioObservations::queue_peak_frames` | struct_field | Reports the queue peak frames observed by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:60` |
| sym-66d30bfd0461d14c1860 | `PolledAudioObservations::registered_endpoints` | struct_field | Stores the registered endpoints used by `PolledAudioObservations`. | `src/endpoint/polled_audio_driver.rs:57` |
| sym-9fdb5c3a8f1cd3b525e8 | `PrepareContext::sample_spec` | struct_field | Stores the sample spec used by `PrepareContext`. | `src/graph/node.rs:267` |
| sym-71238b467d4d0940767d | `RecorderError::FrameSpecMismatch::actual_channels` | struct_field | Stores the actual channels used by `FrameSpecMismatch`. | `src/recording/writer.rs:61` |
| sym-8294d3e3948681723ffd | `RecorderError::FrameSpecMismatch::actual_rate_hz` | struct_field | Stores the actual rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:60` |
| sym-ae395f7b6665c7ad0257 | `RecorderError::FrameSpecMismatch::expected_channels` | struct_field | Stores the expected channels used by `FrameSpecMismatch`. | `src/recording/writer.rs:63` |
| sym-8d51670c5d0ce778b89b | `RecorderError::FrameSpecMismatch::expected_rate_hz` | struct_field | Stores the expected rate value for `FrameSpecMismatch`, in hertz. | `src/recording/writer.rs:62` |
| sym-ecf0a3a88b242ac63017 | `RecorderError::FrameSpecMismatch::label` | struct_field | Stores the label used by `FrameSpecMismatch`. | `src/recording/writer.rs:59` |
| sym-652d1d091b04748211e7 | `RecorderError::GapTooLarge::duration_ns` | struct_field | Stores the duration value for `GapTooLarge`, in nanoseconds. | `src/recording/writer.rs:70` |
| sym-8c3d161113387de15e72 | `RecorderError::GapTooLarge::label` | struct_field | Stores the label used by `GapTooLarge`. | `src/recording/writer.rs:70` |
| sym-b219d7b9e5d6723afd96 | `RecorderError::InvalidSampleSpec::channels` | struct_field | Stores the channels used by `InvalidSampleSpec`. | `src/recording/writer.rs:42` |
| sym-d9e82eb856f533090b38 | `RecorderError::InvalidSampleSpec::label` | struct_field | Stores the label used by `InvalidSampleSpec`. | `src/recording/writer.rs:40` |
| sym-6ea612a44f559762056e | `RecorderError::InvalidSampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `InvalidSampleSpec`, in hertz. | `src/recording/writer.rs:41` |
| sym-50eef94c282d64b04916 | `RecorderError::LineageMismatch::actual` | struct_field | Records the value observed by `LineageMismatch`. | `src/recording/writer.rs:54` |
| sym-5bc3401f723c24ad75e4 | `RecorderError::LineageMismatch::expected` | struct_field | Records the value expected by `LineageMismatch`. | `src/recording/writer.rs:55` |
| sym-ebbac0ff38977c987aac | `RecorderError::LineageMismatch::field` | struct_field | Stores the field used by `LineageMismatch`. | `src/recording/writer.rs:53` |
| sym-4ef03f5878022534ba51 | `RecorderError::LineageMismatch::label` | struct_field | Stores the label used by `LineageMismatch`. | `src/recording/writer.rs:52` |
| sym-1864bc6e2d75f2f14118 | `RecorderError::SessionMismatch::actual` | struct_field | Records the value observed by `SessionMismatch`. | `src/recording/writer.rs:33` |
| sym-2bf6b3f917bc84462f34 | `RecorderError::SessionMismatch::expected` | struct_field | Records the value expected by `SessionMismatch`. | `src/recording/writer.rs:34` |
| sym-b189c2bd0a53defdea5e | `RecorderError::SessionMismatch::label` | struct_field | Stores the label used by `SessionMismatch`. | `src/recording/writer.rs:32` |
| sym-f3b4fc16ca189f99b9f0 | `RecorderError::SourceMismatch::actual` | struct_field | Records the value observed by `SourceMismatch`. | `src/recording/writer.rs:47` |
| sym-e85ede34f6f81087152c | `RecorderError::SourceMismatch::expected` | struct_field | Records the value expected by `SourceMismatch`. | `src/recording/writer.rs:48` |
| sym-49e2f3a0387d6e36a214 | `RecorderError::SourceMismatch::label` | struct_field | Stores the label used by `SourceMismatch`. | `src/recording/writer.rs:46` |
| sym-0986cab597e6de1b7695 | `RecorderStemConfig::channels` | struct_field | Stores the channels used by `RecorderStemConfig`. | `src/recording/config.rs:66` |
| sym-8d49b77e7fdf693eacbc | `RecorderStemConfig::clock_id` | struct_field | Identifies the clock identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:59` |
| sym-92e037e3886b75006889 | `RecorderStemConfig::label` | struct_field | Stores the label used by `RecorderStemConfig`. | `src/recording/config.rs:64` |
| sym-2eafceaeacba85623d18 | `RecorderStemConfig::permission` | struct_field | Stores the permission used by `RecorderStemConfig`. | `src/recording/config.rs:63` |
| sym-5386f08f66607517e9cd | `RecorderStemConfig::permission_epoch` | struct_field | Stores the permission epoch used by `RecorderStemConfig`. | `src/recording/config.rs:61` |
| sym-38ae5238c40c96a92225 | `RecorderStemConfig::permission_scope` | struct_field | Stores the permission scope used by `RecorderStemConfig`. | `src/recording/config.rs:62` |
| sym-78bdd138f370820cdff4 | `RecorderStemConfig::sample_rate_hz` | struct_field | Stores the sample rate value for `RecorderStemConfig`, in hertz. | `src/recording/config.rs:65` |
| sym-2561bafb64813167480d | `RecorderStemConfig::session_id` | struct_field | Identifies the session identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:56` |
| sym-7f976c36294e47df090e | `RecorderStemConfig::source_generation` | struct_field | Stores the source generation used by `RecorderStemConfig`. | `src/recording/config.rs:60` |
| sym-648645677ef7c62f35e2 | `RecorderStemConfig::source_id` | struct_field | Identifies the source identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:57` |
| sym-6fceafe803bbefd0086c | `RecorderStemConfig::stem_id` | struct_field | Identifies the stem identifier recorded by `RecorderStemConfig`. | `src/recording/config.rs:58` |
| sym-c1936056dc0a991fd665 | `RecorderStemConfig::timeline_mapping` | struct_field | Stores the timeline mapping used by `RecorderStemConfig`. | `src/recording/config.rs:67` |
| sym-907a92060e1b3d3f0117 | `RecordingObservations::discontinuities_total` | struct_field | Counts the total number of discontinuities observed by `RecordingObservations`. | `src/recording/writer.rs:134` |
| sym-dff1691ebf43fa4208f5 | `RecordingObservations::failures_total` | struct_field | Counts the total number of failures observed by `RecordingObservations`. | `src/recording/writer.rs:135` |
| sym-9403802bd3b2aa8d8a29 | `RecordingObservations::frames_received_total` | struct_field | Counts the total number of frames received observed by `RecordingObservations`. | `src/recording/writer.rs:131` |
| sym-8277db04333ba81da2a5 | `RecordingObservations::frames_rejected_total` | struct_field | Counts the total number of frames rejected observed by `RecordingObservations`. | `src/recording/writer.rs:133` |
| sym-1cf065a263b2c545a9d8 | `RecordingObservations::frames_written_total` | struct_field | Counts the total number of frames written observed by `RecordingObservations`. | `src/recording/writer.rs:132` |
| sym-cce6423e7357e2b8f22c | `RecordingOutcome::completed_stems` | struct_field | Stores the completed stems used by `RecordingOutcome`. | `src/recording/writer.rs:114` |
| sym-9e70d3f3fdc9c1516e2a | `RecordingOutcome::failed_stems` | struct_field | Stores the failed stems used by `RecordingOutcome`. | `src/recording/writer.rs:115` |
| sym-6aa351dac5c1cb34b677 | `RecordingOutcome::session_dir` | struct_field | Stores the session dir used by `RecordingOutcome`. | `src/recording/writer.rs:112` |
| sym-044c131802ff86e1be01 | `RecordingOutcome::state` | struct_field | Stores the state used by `RecordingOutcome`. | `src/recording/writer.rs:113` |
| sym-5ad93b4b3edb53d5b6a5 | `RecordingOutcome::stems` | struct_field | Stores the stems used by `RecordingOutcome`. | `src/recording/writer.rs:116` |
| sym-5b6c7f6a0aa1e99c38d9 | `RecordingStemOutcome::edge_observations` | struct_field | Stores the edge observations used by `RecordingStemOutcome`. | `src/recording/writer.rs:126` |
| sym-2d5e4745b4e61b79fecb | `RecordingStemOutcome::error` | struct_field | Stores the error used by `RecordingStemOutcome`. | `src/recording/writer.rs:125` |
| sym-f25ff7308e04715af922 | `RecordingStemOutcome::gap_ranges` | struct_field | Stores the gap ranges used by `RecordingStemOutcome`. | `src/recording/writer.rs:124` |
| sym-a8bad434fe08af8bbc6b | `RecordingStemOutcome::label` | struct_field | Stores the label used by `RecordingStemOutcome`. | `src/recording/writer.rs:121` |
| sym-9c5a22fd95dc6959fd3d | `RecordingStemOutcome::stale_frames` | struct_field | Stores the stale frames used by `RecordingStemOutcome`. | `src/recording/writer.rs:123` |
| sym-c350fe48abaced46c0d5 | `RecordingStemOutcome::written_frames` | struct_field | Stores the written frames used by `RecordingStemOutcome`. | `src/recording/writer.rs:122` |
| sym-d46ac345f55f292412c0 | `ResolvedEdge::contract` | struct_field | Stores the contract used by `ResolvedEdge`. | `src/graph/ir.rs:28` |
| sym-473a49f06ce03948140f | `ResolvedEdge::media` | struct_field | Stores the media used by `ResolvedEdge`. | `src/graph/ir.rs:27` |
| sym-2b80e9dec54a1563e525 | `ResolvedEdge::spec` | struct_field | Stores the spec used by `ResolvedEdge`. | `src/graph/ir.rs:26` |
| sym-ba4ef1668d332081e0ed | `ResolvedNode::descriptor` | struct_field | Stores the descriptor used by `ResolvedNode`. | `src/graph/ir.rs:12` |
| sym-f2efb1a101abea0813b4 | `ResolvedNode::spec` | struct_field | Stores the spec used by `ResolvedNode`. | `src/graph/ir.rs:11` |
| sym-ac7e3644a59a6cf9a091 | `RuntimePlan::edge_count` | struct_field | Stores the number of edge represented by `RuntimePlan`. | `src/graph/plan.rs:130` |
| sym-a6e60b8992f1b13e6962 | `RuntimePlan::edge_metrics` | struct_field | Stores the edge metrics used by `RuntimePlan`. | `src/graph/plan.rs:125` |
| sym-d31c532865cdad386ef4 | `RuntimePlan::fan_in` | struct_field | Stores the fan in used by `RuntimePlan`. | `src/graph/plan.rs:127` |
| sym-97489216fc67a438d521 | `RuntimePlan::fan_out` | struct_field | Stores the fan out used by `RuntimePlan`. | `src/graph/plan.rs:126` |
| sym-60ceea6484b4cf818f66 | `RuntimePlan::memory_plan` | struct_field | Stores the memory plan used by `RuntimePlan`. | `src/graph/plan.rs:124` |
| sym-f530f587c1df945eb680 | `RuntimePlan::node_order` | struct_field | Stores the node order used by `RuntimePlan`. | `src/graph/plan.rs:122` |
| sym-da039552805d56b8934d | `RuntimePlan::partitions` | struct_field | Stores the partitions used by `RuntimePlan`. | `src/graph/plan.rs:123` |
| sym-7072cf5bc4fcf62b4458 | `RuntimePlan::source_outputs` | struct_field | Stores the source outputs used by `RuntimePlan`. | `src/graph/plan.rs:129` |
| sym-bbc88c2787183ce07ae6 | `RuntimePlan::typed_edges` | struct_field | Stores the typed edges used by `RuntimePlan`. | `src/graph/plan.rs:128` |
| sym-6247202f5d12c86eb2fc | `SessionCompileError::AmbiguousEndpointInput::input_ports_total` | struct_field | Counts the total number of input ports observed by `AmbiguousEndpointInput`. | `src/session/compile/error.rs:29` |
| sym-d1153524d593d6083fc8 | `SessionCompileError::AmbiguousEndpointInput::node_type_id` | struct_field | Identifies the node type identifier recorded by `AmbiguousEndpointInput`. | `src/session/compile/error.rs:28` |
| sym-b5c858a89510f27e59a6 | `SessionCompileError::AmbiguousOperatorPort::direction` | struct_field | Stores the direction used by `AmbiguousOperatorPort`. | `src/session/compile/error.rs:34` |
| sym-0bcfb2d30c65b5255cf1 | `SessionCompileError::AmbiguousOperatorPort::operator_id` | struct_field | Identifies the operator identifier recorded by `AmbiguousOperatorPort`. | `src/session/compile/error.rs:33` |
| sym-3f6b595824c353ff95a5 | `SessionCompileError::AudioBridgeOutputNotExclusive::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `AudioBridgeOutputNotExclusive`. | `src/session/compile/error.rs:58` |
| sym-20df7b517194d23485a1 | `SessionCompileError::AudioBridgeOutputNotExclusive::output_port` | struct_field | Stores the output port used by `AudioBridgeOutputNotExclusive`. | `src/session/compile/error.rs:59` |
| sym-0d2bc57b14ddf3c30414 | `SessionCompileError::DuplicateOperatorInputConnection::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `DuplicateOperatorInputConnection`. | `src/session/compile/error.rs:63` |
| sym-32da6dd8850bc7bb7398 | `SessionCompileError::DuplicateOperatorInputConnection::port_name` | struct_field | Stores the port name used by `DuplicateOperatorInputConnection`. | `src/session/compile/error.rs:64` |
| sym-7829214eb203fbd1153b | `SessionCompileError::InvalidAudioBridgeOutput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `InvalidAudioBridgeOutput`. | `src/session/compile/error.rs:51` |
| sym-247a81e6a8c0332ed7e5 | `SessionCompileError::InvalidAudioBridgeOutput::output_port` | struct_field | Stores the output port used by `InvalidAudioBridgeOutput`. | `src/session/compile/error.rs:52` |
| sym-78eb38543d185da918e1 | `SessionCompileError::InvalidExternalSourceConfiguration::reason` | struct_field | Carries the reason reported by `InvalidExternalSourceConfiguration`. | `src/session/compile/error.rs:80` |
| sym-7a69b3360dc25bf6b470 | `SessionCompileError::InvalidExternalSourceConfiguration::source_type_id` | struct_field | Identifies the source type identifier recorded by `InvalidExternalSourceConfiguration`. | `src/session/compile/error.rs:79` |
| sym-e48a952b999913db17b0 | `SessionCompileError::MissingRequiredOperatorInput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `MissingRequiredOperatorInput`. | `src/session/compile/error.rs:44` |
| sym-6e8a535bbd1767e00b99 | `SessionCompileError::MissingRequiredOperatorInput::port_name` | struct_field | Stores the port name used by `MissingRequiredOperatorInput`. | `src/session/compile/error.rs:45` |
| sym-08c39cfbfe31f70787c2 | `SessionCompileError::OperatorNodeTypeMismatch::declared_node_type_id` | struct_field | Identifies the declared node type identifier recorded by `OperatorNodeTypeMismatch`. | `src/session/compile/error.rs:18` |
| sym-40dcd95d439a8a2c1b45 | `SessionCompileError::OperatorNodeTypeMismatch::operator_id` | struct_field | Identifies the operator identifier recorded by `OperatorNodeTypeMismatch`. | `src/session/compile/error.rs:16` |
| sym-0368a22e5a363f35100e | `SessionCompileError::OperatorNodeTypeMismatch::registered_node_type_id` | struct_field | Identifies the registered node type identifier recorded by `OperatorNodeTypeMismatch`. | `src/session/compile/error.rs:17` |
| sym-04c1e6a4e4bbb07c1a87 | `SessionCompileError::UnknownAsyncOperator::operator_id` | struct_field | Identifies the operator identifier recorded by `UnknownAsyncOperator`. | `src/session/compile/error.rs:21` |
| sym-f4dec4834d1603b9847b | `SessionCompileError::UnknownEndpointInputPort::node_type_id` | struct_field | Identifies the node type identifier recorded by `UnknownEndpointInputPort`. | `src/session/compile/error.rs:84` |
| sym-9cffd6bd5e0ba636d6ae | `SessionCompileError::UnknownEndpointInputPort::port_name` | struct_field | Stores the port name used by `UnknownEndpointInputPort`. | `src/session/compile/error.rs:85` |
| sym-08b7bb37bdabdbdb90b4 | `SessionCompileError::UnknownEndpointNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `UnknownEndpointNodeType`. | `src/session/compile/error.rs:23` |
| sym-ed372723df3478bf0d73 | `SessionCompileError::UnknownExternalSource::source_type_id` | struct_field | Identifies the source type identifier recorded by `UnknownExternalSource`. | `src/session/compile/error.rs:69` |
| sym-7a4d3e32aed5fb0fc4fb | `SessionCompileError::UnknownExternalSourceOutput::output_port` | struct_field | Stores the output port used by `UnknownExternalSourceOutput`. | `src/session/compile/error.rs:75` |
| sym-1c23f6a6cef5b4504ef8 | `SessionCompileError::UnknownExternalSourceOutput::source_type_id` | struct_field | Identifies the source type identifier recorded by `UnknownExternalSourceOutput`. | `src/session/compile/error.rs:74` |
| sym-f1c8d2edd2768de7eb0c | `SessionCompileError::UnknownOperator::operator_id` | struct_field | Identifies the operator identifier recorded by `UnknownOperator`. | `src/session/compile/error.rs:11` |
| sym-d0c975b99e590a4665f3 | `SessionCompileError::UnknownOperatorPort::direction` | struct_field | Stores the direction used by `UnknownOperatorPort`. | `src/session/compile/error.rs:39` |
| sym-fa54dd4a7574bb72ba86 | `SessionCompileError::UnknownOperatorPort::operator_id` | struct_field | Identifies the operator identifier recorded by `UnknownOperatorPort`. | `src/session/compile/error.rs:38` |
| sym-ddbc224f14159296a686 | `SessionCompileError::UnknownOperatorPort::port_name` | struct_field | Stores the port name used by `UnknownOperatorPort`. | `src/session/compile/error.rs:40` |
| sym-a83280ac6e9620e2f04f | `SessionCompileError::UnknownSourceNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `UnknownSourceNodeType`. | `src/session/compile/error.rs:67` |
| sym-63bdf8d3f793e1f41985 | `SessionComponentId::Endpoint::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `Endpoint`. | `src/session/lifecycle/events.rs:57` |
| sym-bbb47cc6faa7ac476a84 | `SessionComponentId::Endpoint::route_id` | struct_field | Identifies the route identifier recorded by `Endpoint`. | `src/session/lifecycle/events.rs:56` |
| sym-c9aecb792ce9a46362ab | `SessionComponentId::Operator::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `Operator`. | `src/session/lifecycle/events.rs:60` |
| sym-53349c360fde5d0c9410 | `SessionComponentId::Sidecar::sidecar_id` | struct_field | Identifies the sidecar identifier recorded by `Sidecar`. | `src/session/lifecycle/events.rs:63` |
| sym-ffc2df5803ee1fa40bdb | `SessionComponentId::Source::stem_id` | struct_field | Identifies the stem identifier recorded by `Source`. | `src/session/lifecycle/events.rs:53` |
| sym-c306e548ec6c34fcd723 | `SessionDerivedRouteMetrics::endpoint` | struct_field | Stores the endpoint used by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:435` |
| sym-2bc0be56938505c9c80e | `SessionDerivedRouteMetrics::endpoint_finalization_failures_total` | struct_field | Counts the total number of endpoint finalization failures observed by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:437` |
| sym-742186122a3bdb053ea1 | `SessionDerivedRouteMetrics::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:433` |
| sym-bcedcc8bf0cb6efaa0c6 | `SessionDerivedRouteMetrics::endpoint_observation_stage` | struct_field | Stores the endpoint observation stage used by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:436` |
| sym-363c6e95c69a08049429 | `SessionDerivedRouteMetrics::output` | struct_field | Carries the output produced by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:434` |
| sym-9febd1735a84d2b13480 | `SessionDerivedRouteMetrics::route_id` | struct_field | Identifies the route identifier recorded by `SessionDerivedRouteMetrics`. | `src/session/lifecycle/observations.rs:432` |
| sym-0b05a27ff71a975dfe2b | `SessionEngineBuildError::DuplicateSidecarId::sidecar_id` | struct_field | Identifies the sidecar identifier recorded by `DuplicateSidecarId`. | `src/session/lifecycle/engine.rs:301` |
| sym-de490d72160496aaf36c | `SessionEngineBuildError::InvalidConfiguration::reason` | struct_field | Carries the reason reported by `InvalidConfiguration`. | `src/session/lifecycle/engine.rs:299` |
| sym-290bddfb9b9e2b853d23 | `SessionError::DraftFrozen::session_id` | struct_field | Identifies the session identifier recorded by `DraftFrozen`. | `src/session/error.rs:36` |
| sym-c2a6196300ad89a87496 | `SessionError::ForeignEndpoint::actual` | struct_field | Records the value observed by `ForeignEndpoint`. | `src/session/error.rs:33` |
| sym-4013bfb7566f84109d0b | `SessionError::ForeignEndpoint::expected` | struct_field | Records the value expected by `ForeignEndpoint`. | `src/session/error.rs:32` |
| sym-10a7fef50a6e233bc8c4 | `SessionError::InvalidEndpoint::reason` | struct_field | Carries the reason reported by `InvalidEndpoint`. | `src/session/error.rs:25` |
| sym-2bc67fe8f3cebd865828 | `SessionError::InvalidOperator::reason` | struct_field | Carries the reason reported by `InvalidOperator`. | `src/session/error.rs:27` |
| sym-5a1170a4e8e51cb3f199 | `SessionError::InvalidRoute::reason` | struct_field | Carries the reason reported by `InvalidRoute`. | `src/session/error.rs:29` |
| sym-1d0d8a35b0998bb49565 | `SessionError::InvalidSelector::reason` | struct_field | Carries the reason reported by `InvalidSelector`. | `src/session/error.rs:23` |
| sym-862025c641086415bd6c | `SessionError::NoRoutes::stem_id` | struct_field | Identifies the stem identifier recorded by `NoRoutes`. | `src/session/error.rs:10` |
| sym-5a7002f0a439ed08de6b | `SessionError::NoSourceOutputRoutes::output_port` | struct_field | Stores the output port used by `NoSourceOutputRoutes`. | `src/session/error.rs:20` |
| sym-92cb8bb584ef48c3d917 | `SessionError::NoSourceOutputRoutes::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `NoSourceOutputRoutes`. | `src/session/error.rs:19` |
| sym-04459642a7150b9b7d05 | `SessionError::NoSourceOutputs::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `NoSourceOutputs`. | `src/session/error.rs:13` |
| sym-f55f7c7053069d65d57a | `SessionError::OperatorHasNoDestination::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorHasNoDestination`. | `src/session/error.rs:64` |
| sym-eff690ade503b9ffb50e | `SessionError::UnknownEndpoint::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `UnknownEndpoint`. | `src/session/error.rs:44` |
| sym-01d4c00db428df9d0e32 | `SessionError::UnknownOperatorInstance::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `UnknownOperatorInstance`. | `src/session/error.rs:60` |
| sym-7c014262f478c99d05aa | `SessionError::UnknownSourceInstance::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `UnknownSourceInstance`. | `src/session/error.rs:49` |
| sym-3d72751eb512039e4f26 | `SessionError::UnknownSourceOutput::output_port` | struct_field | Stores the output port used by `UnknownSourceOutput`. | `src/session/error.rs:56` |
| sym-f724226f095dd1426f74 | `SessionError::UnknownSourceOutput::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `UnknownSourceOutput`. | `src/session/error.rs:55` |
| sym-2bb14de3a3cc7323c504 | `SessionError::UnknownStem::stem_id` | struct_field | Identifies the stem identifier recorded by `UnknownStem`. | `src/session/error.rs:46` |
| sym-e50d966f5afa81119731 | `SessionError::UnsupportedVersion::major` | struct_field | Stores the major used by `UnsupportedVersion`. | `src/session/error.rs:42` |
| sym-16eec3bd8126603e5780 | `SessionError::UnsupportedVersion::minor` | struct_field | Stores the minor used by `UnsupportedVersion`. | `src/session/error.rs:42` |
| sym-560687ae834652bf016d | `SessionEventQueueObservations::capacity_event_count` | struct_field | Sets the capacity event count available to `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:18` |
| sym-b0dbdb9e13d44f74ed58 | `SessionEventQueueObservations::depth_events` | struct_field | Reports the depth events observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:21` |
| sym-53979dff764d6c456d1e | `SessionEventQueueObservations::depth_owned_bytes` | struct_field | Stores the depth owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:22` |
| sym-25f092b7093ea0f2932e | `SessionEventQueueObservations::events_dropped_oversized_total` | struct_field | Counts the total number of events dropped oversized observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:27` |
| sym-10b2b910d4cac553d53f | `SessionEventQueueObservations::events_dropped_total` | struct_field | Counts the total number of events dropped observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:26` |
| sym-67faf46ee640f6bbd7a6 | `SessionEventQueueObservations::events_enqueued_total` | struct_field | Counts the total number of events enqueued observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:25` |
| sym-15629c31fcfd372ee5f7 | `SessionEventQueueObservations::maximum_buffered_owned_bytes` | struct_field | Stores the maximum buffered owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:20` |
| sym-d4c57b3648e879a97173 | `SessionEventQueueObservations::maximum_event_owned_bytes` | struct_field | Stores the maximum event owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:19` |
| sym-3c7a423603f5063a5691 | `SessionEventQueueObservations::peak_depth_event_count` | struct_field | Reports the peak depth event count observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:23` |
| sym-61210733447617b3f895 | `SessionEventQueueObservations::peak_depth_owned_bytes` | struct_field | Stores the peak depth owned size for `SessionEventQueueObservations`, in bytes. | `src/session/lifecycle/observations.rs:24` |
| sym-958462b6eb49add214c0 | `SessionEventQueueObservations::receiver_closed_total` | struct_field | Counts the total number of receiver closed observed by `SessionEventQueueObservations`. | `src/session/lifecycle/observations.rs:28` |
| sym-cd14c6e080704bf008a8 | `SessionExternalSourceMetrics::runtime` | struct_field | Stores the runtime used by `SessionExternalSourceMetrics`. | `src/session/lifecycle/observations.rs:127` |
| sym-517a83a5703329abaccd | `SessionExternalSourceMetrics::source_id` | struct_field | Identifies the source identifier recorded by `SessionExternalSourceMetrics`. | `src/session/lifecycle/observations.rs:126` |
| sym-fdbaf2758ec2d7b8fe44 | `SessionExternalSourceMetrics::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `SessionExternalSourceMetrics`. | `src/session/lifecycle/observations.rs:125` |
| sym-9eb7104391088dcf454f | `SessionGraphRegistrationError::DuplicateNodeType::node_type_id` | struct_field | Identifies the node type identifier recorded by `DuplicateNodeType`. | `src/session/extensions/builtins.rs:32` |
| sym-1a6fdaa538d377d089dd | `SessionOperatorInputMetrics::edge` | struct_field | Stores the edge used by `SessionOperatorInputMetrics`. | `src/session/lifecycle/observations.rs:244` |
| sym-eb4e4c99c0963e5a3396 | `SessionOperatorInputMetrics::port_name` | struct_field | Stores the port name used by `SessionOperatorInputMetrics`. | `src/session/lifecycle/observations.rs:243` |
| sym-791c514329b809695502 | `SessionOperatorMetrics::finalization_failures_total` | struct_field | Counts the total number of finalization failures observed by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:394` |
| sym-eceb95720c2ead20785e | `SessionOperatorMetrics::input_edge` | struct_field | Sole counter authority for input delivered by the compiled Session plan. | `src/session/lifecycle/observations.rs:389` |
| sym-0f033f1a2dbac10257b4 | `SessionOperatorMetrics::input_ports` | struct_field | Exact per-port input accounting. `input_edge` is the compatibility aggregate across this slice. | `src/session/lifecycle/observations.rs:392` |
| sym-912735085791df328fba | `SessionOperatorMetrics::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:383` |
| sym-15859553403e9df7eb1a | `SessionOperatorMetrics::worker` | struct_field | Stores the worker used by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:393` |
| sym-174f0b815d7898ec2905 | `SessionPrepareError::DuplicateOperatorInput::node_id` | struct_field | Identifies the node identifier recorded by `DuplicateOperatorInput`. | `src/session/prepare/error.rs:76` |
| sym-8a6652fb8a33713e6e54 | `SessionPrepareError::DuplicateSignalRoute::route_id` | struct_field | Identifies the route identifier recorded by `DuplicateSignalRoute`. | `src/session/prepare/error.rs:80` |
| sym-96ee69ae31663ca9c659 | `SessionPrepareError::DuplicateSourceNode::stem_id` | struct_field | Identifies the stem identifier recorded by `DuplicateSourceNode`. | `src/session/prepare/error.rs:17` |
| sym-cbfb9dbe69d388a70090 | `SessionPrepareError::DuplicateWorkerRoute::route_id` | struct_field | Identifies the route identifier recorded by `DuplicateWorkerRoute`. | `src/session/prepare/error.rs:53` |
| sym-b28aba84850d90821f2e | `SessionPrepareError::IncompatibleNodeBinding::node_id` | struct_field | Identifies the node identifier recorded by `IncompatibleNodeBinding`. | `src/session/prepare/error.rs:74` |
| sym-58a68787879fc6b95924 | `SessionPrepareError::InvalidExternalAudioMedia::output_port` | struct_field | Stores the output port used by `InvalidExternalAudioMedia`. | `src/session/prepare/error.rs:28` |
| sym-468cfa014fa2afe5822b | `SessionPrepareError::InvalidExternalAudioMedia::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `InvalidExternalAudioMedia`. | `src/session/prepare/error.rs:27` |
| sym-223a0b9a12735347649a | `SessionPrepareError::InvalidGeneratedAudioMedia::stem_id` | struct_field | Identifies the stem identifier recorded by `InvalidGeneratedAudioMedia`. | `src/session/prepare/error.rs:33` |
| sym-ab16bbf77f176c10a604 | `SessionPrepareError::InvalidOperatorInputPort::edge_id` | struct_field | Identifies the edge identifier recorded by `InvalidOperatorInputPort`. | `src/session/prepare/error.rs:49` |
| sym-a7e49a5892981cee475a | `SessionPrepareError::InvalidOperatorInputPort::port_name` | struct_field | Stores the port name used by `InvalidOperatorInputPort`. | `src/session/prepare/error.rs:49` |
| sym-853720ea29ecb4ef17aa | `SessionPrepareError::MissingAsyncOperatorFactory::node_id` | struct_field | Identifies the node identifier recorded by `MissingAsyncOperatorFactory`. | `src/session/prepare/error.rs:68` |
| sym-8269d52b3b937dffbca5 | `SessionPrepareError::MissingExternalAudioIngress::output_port` | struct_field | Stores the output port used by `MissingExternalAudioIngress`. | `src/session/prepare/error.rs:21` |
| sym-9ee43deee7eb0ff8b61d | `SessionPrepareError::MissingExternalAudioIngress::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `MissingExternalAudioIngress`. | `src/session/prepare/error.rs:20` |
| sym-112655a651d418b33477 | `SessionPrepareError::MissingExternalSourceDefinition::source_type_id` | struct_field | Identifies the source type identifier recorded by `MissingExternalSourceDefinition`. | `src/session/prepare/error.rs:24` |
| sym-19b6f8bb209f8e136fea | `SessionPrepareError::MissingExternalSourceRouteEdge::route_id` | struct_field | Identifies the route identifier recorded by `MissingExternalSourceRouteEdge`. | `src/session/prepare/error.rs:37` |
| sym-305c5ffb28ce05d9449c | `SessionPrepareError::MissingGeneratedAudioBridge::stem_id` | struct_field | Identifies the stem identifier recorded by `MissingGeneratedAudioBridge`. | `src/session/prepare/error.rs:35` |
| sym-5777526d4287551dfc61 | `SessionPrepareError::MissingGeneratedAudioIngress::stem_id` | struct_field | Identifies the stem identifier recorded by `MissingGeneratedAudioIngress`. | `src/session/prepare/error.rs:31` |
| sym-5398106ed41955bdc95e | `SessionPrepareError::MissingNodeBinding::node_id` | struct_field | Identifies the node identifier recorded by `MissingNodeBinding`. | `src/session/prepare/error.rs:72` |
| sym-1030933b4f9bade26d3b | `SessionPrepareError::MissingOperatorSignalInput::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingOperatorSignalInput`. | `src/session/prepare/error.rs:78` |
| sym-386881cafee8bc063be8 | `SessionPrepareError::MissingSourceNode::stem_id` | struct_field | Identifies the stem identifier recorded by `MissingSourceNode`. | `src/session/prepare/error.rs:15` |
| sym-efcae39fec2f5c96a077 | `SessionPrepareError::MissingTypedEdgePlan::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingTypedEdgePlan`. | `src/session/prepare/error.rs:66` |
| sym-d619bbd2a2e21b924996 | `SessionPrepareError::MissingWorkerCapacity::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerCapacity`. | `src/session/prepare/error.rs:45` |
| sym-99036c0540925d0151be | `SessionPrepareError::MissingWorkerEdge::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerEdge`. | `src/session/prepare/error.rs:41` |
| sym-c9361048537fab448b9a | `SessionPrepareError::MissingWorkerEdgeContract::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerEdgeContract`. | `src/session/prepare/error.rs:43` |
| sym-0e64c13c0e0a392c9249 | `SessionPrepareError::MissingWorkerSampleSpec::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerSampleSpec`. | `src/session/prepare/error.rs:47` |
| sym-b5f32d7270d2cee86862 | `SessionPrepareError::MissingWorkerTarget::edge_id` | struct_field | Identifies the edge identifier recorded by `MissingWorkerTarget`. | `src/session/prepare/error.rs:39` |
| sym-66a78728845eb5b855c0 | `SessionPrepareError::OperatorDeclarationMismatch::node_id` | struct_field | Identifies the node identifier recorded by `OperatorDeclarationMismatch`. | `src/session/prepare/error.rs:70` |
| sym-296be5584a48ab656e3c | `SessionPrepareError::SignalRouteMismatch::edge_id` | struct_field | Identifies the edge identifier recorded by `SignalRouteMismatch`. | `src/session/prepare/error.rs:82` |
| sym-5e172b253225f9dfda25 | `SessionPrepareError::SignalRouteMismatch::route_id` | struct_field | Identifies the route identifier recorded by `SignalRouteMismatch`. | `src/session/prepare/error.rs:82` |
| sym-ec43af387d5156b2db4d | `SessionPrepareError::UnknownWorkerRoute::edge_id` | struct_field | Identifies the edge identifier recorded by `UnknownWorkerRoute`. | `src/session/prepare/error.rs:51` |
| sym-0dfcf7d1de8fa61da4f8 | `SessionPrepareError::UnknownWorkerRoute::route_id` | struct_field | Identifies the route identifier recorded by `UnknownWorkerRoute`. | `src/session/prepare/error.rs:51` |
| sym-02e1ff82282b325e12af | `SessionPrepareError::WorkerRouteMismatch::actual_endpoint_id` | struct_field | Identifies the actual endpoint identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:63` |
| sym-637166a4b8a3e15b169d | `SessionPrepareError::WorkerRouteMismatch::actual_stem_id` | struct_field | Identifies the actual stem identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:61` |
| sym-df736e511f1b8204bb63 | `SessionPrepareError::WorkerRouteMismatch::edge_id` | struct_field | Identifies the edge identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:58` |
| sym-c29140a788353a2a38a9 | `SessionPrepareError::WorkerRouteMismatch::expected_endpoint_id` | struct_field | Identifies the expected endpoint identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:62` |
| sym-e20e71ee6e04cfb0a21e | `SessionPrepareError::WorkerRouteMismatch::expected_stem_id` | struct_field | Identifies the expected stem identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:60` |
| sym-4209d1644b4564693eb0 | `SessionPrepareError::WorkerRouteMismatch::route_id` | struct_field | Identifies the route identifier recorded by `WorkerRouteMismatch`. | `src/session/prepare/error.rs:59` |
| sym-c91fb42f9adeec80122a | `SessionPrepareError::WorkerTopologyMismatch::actual` | struct_field | Records the value observed by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:88` |
| sym-20c143189259cc7d669b | `SessionPrepareError::WorkerTopologyMismatch::actual_operator_inputs` | struct_field | Stores the actual operator inputs used by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:90` |
| sym-7dffab65befd7dc2926a | `SessionPrepareError::WorkerTopologyMismatch::actual_signal_endpoints` | struct_field | Stores the actual signal endpoints used by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:92` |
| sym-47e72ea70b0d3f398350 | `SessionPrepareError::WorkerTopologyMismatch::expected` | struct_field | Records the value expected by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:87` |
| sym-c36657d5a2a651ebba2f | `SessionPrepareError::WorkerTopologyMismatch::expected_operator_inputs` | struct_field | Stores the expected operator inputs used by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:89` |
| sym-278a4fe42def139b3de0 | `SessionPrepareError::WorkerTopologyMismatch::expected_signal_endpoints` | struct_field | Stores the expected signal endpoints used by `WorkerTopologyMismatch`. | `src/session/prepare/error.rs:91` |
| sym-4c334962fbabec17e2cf | `SessionRouteDropObservations::branch_pool_exhausted_drops_total` | struct_field | Counts the total number of branch pool exhausted drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:165` |
| sym-5d53fe5d2bd767a8c1f5 | `SessionRouteDropObservations::frames_attempted_total` | struct_field | Counts the total number of frames attempted observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:161` |
| sym-72c8f7ec4fc09c4f15b2 | `SessionRouteDropObservations::frames_dropped_total` | struct_field | Counts the total number of frames dropped observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:160` |
| sym-e5fc91bac8cfeb52ce89 | `SessionRouteDropObservations::freeze_failed_drops_total` | struct_field | Counts the total number of freeze failed drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:167` |
| sym-2f19fced1ec70d0b0058 | `SessionRouteDropObservations::interval` | struct_field | Stores the interval used by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:159` |
| sym-067687ed79c27b3e97b1 | `SessionRouteDropObservations::invalid_copy_policy_drops_total` | struct_field | Counts the total number of invalid copy policy drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:166` |
| sym-bd7981ee576254501dce | `SessionRouteDropObservations::queue_full_drops_total` | struct_field | Counts the total number of queue full drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:163` |
| sym-1c83cd916b4d7756141b | `SessionRouteDropObservations::receiver_unavailable_drops_total` | struct_field | Counts the total number of receiver unavailable drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:162` |
| sym-f613289465e4b59a0249 | `SessionRouteDropObservations::route_id` | struct_field | Identifies the route identifier recorded by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:158` |
| sym-b5ec814a47c38a47aa98 | `SessionRouteDropObservations::shared_reference_exhausted_drops_total` | struct_field | Counts the total number of shared reference exhausted drops observed by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:164` |
| sym-7fcaa4340ced7dd56eb4 | `SessionRouteLatencyObservations::boundary` | struct_field | Stores the boundary used by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:184` |
| sym-b79cb6ec6e92f9570586 | `SessionRouteLatencyObservations::future_timestamp_total` | struct_field | Counts the total number of future timestamp observed by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:188` |
| sym-7877009e8a4a2b19b06c | `SessionRouteLatencyObservations::max_ns` | struct_field | Stores the max value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:192` |
| sym-a6bfc5fdb9f4d19f05bd | `SessionRouteLatencyObservations::missing_or_incompatible_clock_total` | struct_field | Counts the total number of missing or incompatible clock observed by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:187` |
| sym-eadc9cdf2bdcf67ec2b6 | `SessionRouteLatencyObservations::p50_ns` | struct_field | Stores the p50 value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:189` |
| sym-2768c1d6e5365fdefa65 | `SessionRouteLatencyObservations::p95_ns` | struct_field | Stores the p95 value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:190` |
| sym-8f725f055f9fd4e18dd0 | `SessionRouteLatencyObservations::p99_ns` | struct_field | Stores the p99 value for `SessionRouteLatencyObservations`, in nanoseconds. | `src/session/lifecycle/observations.rs:191` |
| sym-f5a609d054ac649e8314 | `SessionRouteLatencyObservations::route_id` | struct_field | Identifies the route identifier recorded by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:183` |
| sym-2cd1585535befc18c0ed | `SessionRouteLatencyObservations::samples_total` | struct_field | Counts the total number of samples observed by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:186` |
| sym-8d2982684eea83cdb9bd | `SessionRouteLatencyObservations::unit` | struct_field | Stores the unit used by `SessionRouteLatencyObservations`. | `src/session/lifecycle/observations.rs:185` |
| sym-fe7ee1ff590e4bd7e07c | `SessionRouteMetrics::edge` | struct_field | Stores the edge used by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:142` |
| sym-643c5eaa65ea04058544 | `SessionRouteMetrics::endpoint` | struct_field | Stores the endpoint used by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:143` |
| sym-a68ffe5ab32f83feffad | `SessionRouteMetrics::endpoint_finalization_failures_total` | struct_field | Counts the total number of endpoint finalization failures observed by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:145` |
| sym-f135adee8d14c5fac5c1 | `SessionRouteMetrics::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:141` |
| sym-cdb6c22458497e2e6c5d | `SessionRouteMetrics::endpoint_observation_stage` | struct_field | Stores the endpoint observation stage used by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:144` |
| sym-6e390bdedc8ea76ad304 | `SessionRouteMetrics::route_id` | struct_field | Identifies the route identifier recorded by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:140` |
| sym-c934f60e06c1c0648f05 | `SessionSidecarMetrics::host` | struct_field | Stores the host used by `SessionSidecarMetrics`. | `src/session/lifecycle/observations.rs:135` |
| sym-e9400790c987c2d68198 | `SessionSidecarMetrics::sidecar_id` | struct_field | Identifies the sidecar identifier recorded by `SessionSidecarMetrics`. | `src/session/lifecycle/observations.rs:134` |
| sym-ccb4010992034a222f32 | `SessionSourceMetrics::capture` | struct_field | Stores the capture used by `SessionSourceMetrics`. | `src/session/lifecycle/observations.rs:119` |
| sym-e228d46fc7f836f267bf | `SessionSourceMetrics::ingress` | struct_field | Stores the ingress used by `SessionSourceMetrics`. | `src/session/lifecycle/observations.rs:120` |
| sym-798d49f118b82c9cfb24 | `SessionSourceMetrics::stem_id` | struct_field | Identifies the stem identifier recorded by `SessionSourceMetrics`. | `src/session/lifecycle/observations.rs:118` |
| sym-0b94dbf9abde75b26716 | `SessionStartError::Cancelled::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `Cancelled`. | `src/session/lifecycle/start_contract.rs:194` |
| sym-91b325289aba8e7aa949 | `SessionStartError::CaptureOpen::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `CaptureOpen`. | `src/session/lifecycle/start_contract.rs:169` |
| sym-ed3cd1176f4e79325368 | `SessionStartError::CaptureOpen::source` | struct_field | Carries the source selected for `CaptureOpen`. | `src/session/lifecycle/start_contract.rs:168` |
| sym-945fd316ca5804e08fc3 | `SessionStartError::CaptureOpen::stem_id` | struct_field | Identifies the stem identifier recorded by `CaptureOpen`. | `src/session/lifecycle/start_contract.rs:166` |
| sym-96348f2a40f6ab09344c | `SessionStartError::CapturePrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `CapturePrepare`. | `src/session/lifecycle/start_contract.rs:162` |
| sym-4c4cec2ffba8bd0fa2b9 | `SessionStartError::CapturePrepare::source` | struct_field | Carries the source selected for `CapturePrepare`. | `src/session/lifecycle/start_contract.rs:161` |
| sym-f7d4f4f06bbfcd5a77a7 | `SessionStartError::CapturePrepare::stem_id` | struct_field | Identifies the stem identifier recorded by `CapturePrepare`. | `src/session/lifecycle/start_contract.rs:159` |
| sym-4196140e38a2274b7c98 | `SessionStartError::EndpointPrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `EndpointPrepare`. | `src/session/lifecycle/start_contract.rs:155` |
| sym-54973c883b3285e10222 | `SessionStartError::EndpointPrepare::source` | struct_field | Carries the source selected for `EndpointPrepare`. | `src/session/lifecycle/start_contract.rs:154` |
| sym-12718a91477673020e3c | `SessionStartError::EndpointStart::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `EndpointStart`. | `src/session/lifecycle/start_contract.rs:175` |
| sym-575e81ff4fcef2c247e1 | `SessionStartError::EndpointStart::source` | struct_field | Carries the source selected for `EndpointStart`. | `src/session/lifecycle/start_contract.rs:174` |
| sym-ca237e2183027d1f79f3 | `SessionStartError::ExternalAudioBridge::message` | struct_field | Carries the diagnostic message reported by `ExternalAudioBridge`. | `src/session/lifecycle/start_contract.rs:125` |
| sym-ad6852bea494bb40945a | `SessionStartError::ExternalAudioBridge::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `ExternalAudioBridge`. | `src/session/lifecycle/start_contract.rs:126` |
| sym-354e7d2ca0330bc7deb9 | `SessionStartError::ExternalSourcePrepare::message` | struct_field | Carries the diagnostic message reported by `ExternalSourcePrepare`. | `src/session/lifecycle/start_contract.rs:120` |
| sym-5d0c75d5035e497434d8 | `SessionStartError::ExternalSourcePrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `ExternalSourcePrepare`. | `src/session/lifecycle/start_contract.rs:121` |
| sym-c1d16a529c672ae97b18 | `SessionStartError::ExternalSourceStart::message` | struct_field | Carries the diagnostic message reported by `ExternalSourceStart`. | `src/session/lifecycle/start_contract.rs:135` |
| sym-6f362400457e84a855a9 | `SessionStartError::ExternalSourceStart::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `ExternalSourceStart`. | `src/session/lifecycle/start_contract.rs:136` |
| sym-071a8938e70563283430 | `SessionStartError::GeneratedAudioBridge::message` | struct_field | Carries the diagnostic message reported by `GeneratedAudioBridge`. | `src/session/lifecycle/start_contract.rs:130` |
| sym-62d0fc0cfedae9717962 | `SessionStartError::GeneratedAudioBridge::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `GeneratedAudioBridge`. | `src/session/lifecycle/start_contract.rs:131` |
| sym-f1513eeb93f0a30a8c4e | `SessionStartError::InvalidOptions::reason` | struct_field | Carries the reason reported by `InvalidOptions`. | `src/session/lifecycle/start_contract.rs:115` |
| sym-9d34f6be7a5d8cedf290 | `SessionStartError::MissingEndpointDeclaration::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `MissingEndpointDeclaration`. | `src/session/lifecycle/start_contract.rs:150` |
| sym-291f0d8f4b99218adb3e | `SessionStartError::OperatorPrepare::message` | struct_field | Carries the diagnostic message reported by `OperatorPrepare`. | `src/session/lifecycle/start_contract.rs:146` |
| sym-30d2d6595c5990082bc2 | `SessionStartError::OperatorPrepare::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorPrepare`. | `src/session/lifecycle/start_contract.rs:145` |
| sym-373d41c929aab6033425 | `SessionStartError::OperatorPrepare::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `OperatorPrepare`. | `src/session/lifecycle/start_contract.rs:147` |
| sym-54de3cfe9b7215043614 | `SessionStartError::OperatorRuntimeHost::message` | struct_field | Carries the diagnostic message reported by `OperatorRuntimeHost`. | `src/session/lifecycle/start_contract.rs:140` |
| sym-4048000c3db91f3d8261 | `SessionStartError::OperatorRuntimeHost::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `OperatorRuntimeHost`. | `src/session/lifecycle/start_contract.rs:141` |
| sym-63940834b72833d342aa | `SessionStartError::RuntimeRunner::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `RuntimeRunner`. | `src/session/lifecycle/start_contract.rs:181` |
| sym-1262abaf53c554f8d8bb | `SessionStartError::RuntimeRunner::source` | struct_field | Carries the source selected for `RuntimeRunner`. | `src/session/lifecycle/start_contract.rs:180` |
| sym-3cf0505bdaf6c938770c | `SessionStartError::RuntimeWorkerReady::message` | struct_field | Carries the diagnostic message reported by `RuntimeWorkerReady`. | `src/session/lifecycle/start_contract.rs:190` |
| sym-94241a5bd882e8aa2339 | `SessionStartError::RuntimeWorkerReady::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `RuntimeWorkerReady`. | `src/session/lifecycle/start_contract.rs:191` |
| sym-55937c1649fa34b337c4 | `SessionStartError::RuntimeWorkerSpawn::message` | struct_field | Carries the diagnostic message reported by `RuntimeWorkerSpawn`. | `src/session/lifecycle/start_contract.rs:185` |
| sym-281a8ed0ed95601da855 | `SessionStartError::RuntimeWorkerSpawn::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `RuntimeWorkerSpawn`. | `src/session/lifecycle/start_contract.rs:186` |
| sym-aeeb994175d367984c22 | `SessionStartOptions::capture_frame_capacity_frames` | struct_field | Sets the capture frame capacity frames available to `SessionStartOptions`. | `src/session/lifecycle/start_contract.rs:24` |
| sym-0291bbd86d83e3919cb4 | `SessionStartOptions::capture_runtime_event_capacity_events` | struct_field | Sets the capture runtime event capacity events available to `SessionStartOptions`. | `src/session/lifecycle/start_contract.rs:25` |
| sym-b536e344a41a120a4e63 | `SessionStartOptions::runtime_idle_poll_ms` | struct_field | Stores the runtime idle poll value for `SessionStartOptions`, in milliseconds. | `src/session/lifecycle/start_contract.rs:27` |
| sym-7fcb122e98fd4e13994e | `SessionStartOptions::runtime_ready_timeout_ms` | struct_field | Stores the runtime ready timeout value for `SessionStartOptions`, in milliseconds. | `src/session/lifecycle/start_contract.rs:28` |
| sym-f57c72c9ebf663e4fd46 | `SessionStartOptions::runtime_work_budget_frames` | struct_field | Stores the runtime work budget frames used by `SessionStartOptions`. | `src/session/lifecycle/start_contract.rs:26` |
| sym-0dded7e43cba10d3391b | `SessionStartOptions::session_event_capacity_events` | struct_field | Sets the session event capacity events available to `SessionStartOptions`. | `src/session/lifecycle/start_contract.rs:29` |
| sym-12636597bd2238d64b07 | `SessionTraceRecord::kind` | struct_field | Stores the kind used by `SessionTraceRecord`. | `src/session/lifecycle/trace.rs:59` |
| sym-106b06b7f5923d3452c8 | `SessionTraceRecord::observed_at_ns` | struct_field | Stores the observed at value for `SessionTraceRecord`, in nanoseconds. | `src/session/lifecycle/trace.rs:57` |
| sym-f35ebc6c8b22aaebd825 | `SessionTraceRecord::sequence_index` | struct_field | Stores the sequence index used by `SessionTraceRecord`. | `src/session/lifecycle/trace.rs:56` |
| sym-57d73cd3e6cacda51a29 | `SessionTraceRecord::session_id` | struct_field | Identifies the session identifier recorded by `SessionTraceRecord`. | `src/session/lifecycle/trace.rs:58` |
| sym-6b265b7f4f5e804bfa59 | `SessionTraceRecordKind::EndpointFailure::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `EndpointFailure`. | `src/session/lifecycle/trace.rs:36` |
| sym-62d35b66407474fdcdcd | `SessionTraceRecordKind::EndpointFailure::route_id` | struct_field | Identifies the route identifier recorded by `EndpointFailure`. | `src/session/lifecycle/trace.rs:35` |
| sym-90386f590b72dcba4f3a | `SessionTraceRecordKind::EndpointFailure::stage_code` | struct_field | Stores the stage code used by `EndpointFailure`. | `src/session/lifecycle/trace.rs:37` |
| sym-dab23572cfc0fd2f72f7 | `SessionTraceRecordKind::FinalizationFailure::stage` | struct_field | Stores the stage used by `FinalizationFailure`. | `src/session/lifecycle/trace.rs:43` |
| sym-19f761b326abd1357823 | `SessionTraceRecordKind::Lifecycle::state` | struct_field | Stores the state used by `Lifecycle`. | `src/session/lifecycle/trace.rs:29` |
| sym-94fe693bb0be943cf406 | `SessionTraceRecordKind::RollbackFailure::stage` | struct_field | Stores the stage used by `RollbackFailure`. | `src/session/lifecycle/trace.rs:40` |
| sym-02755f8dace60a4065e3 | `SessionTraceRecordKind::SourceFailure::stem_id` | struct_field | Identifies the stem identifier recorded by `SourceFailure`. | `src/session/lifecycle/trace.rs:32` |
| sym-7bc6b9fd9c427f985a1b | `SessionTraceRecordKind::Terminal::endpoint_failures_total` | struct_field | Counts the total number of endpoint failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:48` |
| sym-ec40a963d72e60609074 | `SessionTraceRecordKind::Terminal::finalization_failures_total` | struct_field | Counts the total number of finalization failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:50` |
| sym-bfc34497c3f6d270029e | `SessionTraceRecordKind::Terminal::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:49` |
| sym-ca28700b0098c37cddd2 | `SessionTraceRecordKind::Terminal::source_failures_total` | struct_field | Counts the total number of source failures observed by `Terminal`. | `src/session/lifecycle/trace.rs:47` |
| sym-68c0d29f5c0c03966482 | `SessionTraceRecordKind::Terminal::state` | struct_field | Stores the state used by `Terminal`. | `src/session/lifecycle/trace.rs:46` |
| sym-488ca83fed6437f4248d | `SessionTraceRecorderOutcome::path` | struct_field | Stores the path used by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:71` |
| sym-2ae640d8931c35412da2 | `SessionTraceRecorderOutcome::records_attempted_total` | struct_field | Counts the total number of records attempted observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:72` |
| sym-c8092dec35b3d1e3d10c | `SessionTraceRecorderOutcome::records_dropped_total` | struct_field | Counts the total number of records dropped observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:74` |
| sym-58bc8fd25890a06421fa | `SessionTraceRecorderOutcome::records_enqueued_total` | struct_field | Counts the total number of records enqueued observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:73` |
| sym-a02ad0025f7ecb7c595e | `SessionTraceRecorderOutcome::records_written_total` | struct_field | Counts the total number of records written observed by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:75` |
| sym-8d72d7dad18016f8fe21 | `SessionTraceRecorderOutcome::rolling_hash` | struct_field | Stores the rolling hash used by `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:76` |
| sym-1025b6581f9ea0fe70d6 | `SessionTraceRecorderStartError::OutputExists::path` | struct_field | Stores the path used by `OutputExists`. | `src/session/lifecycle/trace.rs:92` |
| sym-66e220be1888025605da | `SessionTraceTerminal::endpoint_failures_total` | struct_field | Counts the total number of endpoint failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:342` |
| sym-fd94e938204daba5d97f | `SessionTraceTerminal::finalization_failures_total` | struct_field | Counts the total number of finalization failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:344` |
| sym-11d0b062945a04162037 | `SessionTraceTerminal::rollback_failures_total` | struct_field | Counts the total number of rollback failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:343` |
| sym-686dd72d723822eebb6e | `SessionTraceTerminal::source_failures_total` | struct_field | Counts the total number of source failures observed by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:341` |
| sym-77cf046df52f7b36343b | `SessionTraceTerminal::state` | struct_field | Stores the state used by `SessionTraceTerminal`. | `src/session/lifecycle/trace.rs:340` |
| sym-a1551c33137c9170329b | `SessionTraceValidation::lifecycle` | struct_field | Stores the lifecycle used by `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:350` |
| sym-e53aab684f034343a3d4 | `SessionTraceValidation::records_validated_total` | struct_field | Counts the total number of records validated observed by `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:352` |
| sym-f973bfdba58b92f89c0c | `SessionTraceValidation::session_id` | struct_field | Identifies the session identifier recorded by `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:349` |
| sym-d7e71df16d48b06b58a2 | `SessionTraceValidation::terminal` | struct_field | Indicates whether terminal applies to `SessionTraceValidation`. | `src/session/lifecycle/trace.rs:351` |
| sym-82465092056037d534d0 | `SidecarDeadlines::processing` | struct_field | Stores the processing used by `SidecarDeadlines`. | `src/runtime/lifecycle/sidecar_host.rs:56` |
| sym-9baa092e7585c826ed37 | `SidecarDeadlines::ready` | struct_field | Indicates whether ready applies to `SidecarDeadlines`. | `src/runtime/lifecycle/sidecar_host.rs:55` |
| sym-f4ac2134d8d400b3d175 | `SidecarDeadlines::shutdown` | struct_field | Stores the shutdown used by `SidecarDeadlines`. | `src/runtime/lifecycle/sidecar_host.rs:57` |
| sym-0dd5c844b1de85d140c1 | `SidecarHostError::InvalidState::actual` | struct_field | Records the value observed by `InvalidState`. | `src/runtime/lifecycle/sidecar_host.rs:721` |
| sym-b926cc699c8a5d4c33b3 | `SidecarHostError::InvalidState::expected` | struct_field | Records the value expected by `InvalidState`. | `src/runtime/lifecycle/sidecar_host.rs:720` |
| sym-4f4695544342a9199bd8 | `SidecarHostError::UnexpectedMessage::actual` | struct_field | Records the value observed by `UnexpectedMessage`. | `src/runtime/lifecycle/sidecar_host.rs:712` |
| sym-dad77b4ed4fde0ca9469 | `SidecarHostError::UnexpectedMessage::expected` | struct_field | Records the value expected by `UnexpectedMessage`. | `src/runtime/lifecycle/sidecar_host.rs:711` |
| sym-37f499f81b50c8a37bb1 | `SidecarHostSnapshot::data_dropped_total` | struct_field | Counts the total number of data dropped observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:138` |
| sym-2edd4e04ce5d29fda794 | `SidecarHostSnapshot::data_enqueued_total` | struct_field | Counts the total number of data enqueued observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:136` |
| sym-e1187b3508ec1c4d05a2 | `SidecarHostSnapshot::data_received_total` | struct_field | Counts the total number of data received observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:137` |
| sym-c2762c18b8e5ce0222e7 | `SidecarHostSnapshot::forced_kills_total` | struct_field | Counts the total number of forced kills observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:141` |
| sym-88e42c51d957e9047f2e | `SidecarHostSnapshot::protocol_failures_total` | struct_field | Counts the total number of protocol failures observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:139` |
| sym-651b82bd5e5b101d81f7 | `SidecarHostSnapshot::reaps_total` | struct_field | Counts the total number of reaps observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:142` |
| sym-b42e422e882b65212ea8 | `SidecarHostSnapshot::state` | struct_field | Stores the state used by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:134` |
| sym-b2f609e324c4f990f9c6 | `SidecarHostSnapshot::state_transitions` | struct_field | Stores the state transitions used by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:135` |
| sym-19d45a3c12dfca9f2a77 | `SidecarHostSnapshot::timeouts_total` | struct_field | Counts the total number of timeouts observed by `SidecarHostSnapshot`. | `src/runtime/lifecycle/sidecar_host.rs:140` |
| sym-95f558d27ff12fced9dd | `SidecarMessage::kind` | struct_field | Stores the kind used by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:74` |
| sym-c70db04d6f020d40435f | `SidecarMessage::payload` | struct_field | Stores the payload used by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:82` |
| sym-585bcd46fcd5f6277992 | `SidecarMessage::role` | struct_field | Stores the role used by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:80` |
| sym-e2bf5a8c573baaa42fac | `SidecarMessage::schema` | struct_field | Stores the schema used by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:81` |
| sym-52600cd45af11c89339d | `SidecarMessage::sequence_number` | struct_field | Stores the sequence number used by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:77` |
| sym-be1d56d720f86e86c1da | `SidecarMessage::signal_id` | struct_field | Identifies the signal identifier recorded by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:79` |
| sym-a445d50a72680566c0d8 | `SidecarMessage::stream_id` | struct_field | Identifies the stream identifier recorded by `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:76` |
| sym-06a3f5808f2c8f012d31 | `SidecarMessage::terminal` | struct_field | Indicates whether terminal applies to `SidecarMessage`. | `src/runtime/lifecycle/sidecar_protocol.rs:75` |
| sym-3400986c4d4782e2ace8 | `SidecarMessage::timestamp_ns` | struct_field | Stores the timestamp value for `SidecarMessage`, in nanoseconds. | `src/runtime/lifecycle/sidecar_protocol.rs:78` |
| sym-e41681826e5965dfe2f9 | `SidecarProcessSpec::arguments` | struct_field | Stores the arguments used by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:74` |
| sym-0c6fbb365c428e081459 | `SidecarProcessSpec::configuration` | struct_field | Stores the configuration used by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:75` |
| sym-86fac552cd898d89c1f3 | `SidecarProcessSpec::data_capacity_messages` | struct_field | Sets the data capacity messages available to `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:76` |
| sym-bf17bf9613e72d992956 | `SidecarProcessSpec::deadlines` | struct_field | Stores the deadlines used by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:78` |
| sym-78dcffbfa3bdf0b4f088 | `SidecarProcessSpec::id` | struct_field | Identifies the id recorded by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:72` |
| sym-b0d95000d9037052a849 | `SidecarProcessSpec::program` | struct_field | Stores the program used by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:73` |
| sym-d39d0375b29e29f262ff | `SidecarProcessSpec::protocol_limits` | struct_field | Stores the protocol limits used by `SidecarProcessSpec`. | `src/runtime/lifecycle/sidecar_host.rs:77` |
| sym-91c5c2013bd3a2df6cb1 | `SidecarProtocolError::FieldTooLarge::actual` | struct_field | Records the value observed by `FieldTooLarge`. | `src/runtime/lifecycle/sidecar_protocol.rs:314` |
| sym-39c78b7883b0908ec30a | `SidecarProtocolError::FieldTooLarge::field` | struct_field | Stores the field used by `FieldTooLarge`. | `src/runtime/lifecycle/sidecar_protocol.rs:313` |
| sym-b5a1d8fc39f5f4273270 | `SidecarProtocolError::FieldTooLarge::maximum` | struct_field | Sets the inclusive maximum accepted by `FieldTooLarge`. | `src/runtime/lifecycle/sidecar_protocol.rs:315` |
| sym-a20a04f852bd39ce8935 | `SidecarProtocolLimits::max_payload_bytes` | struct_field | Limits payload storage for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:47` |
| sym-f8ddf7920e493c69b106 | `SidecarProtocolLimits::max_role_bytes` | struct_field | Stores the max role size for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:45` |
| sym-31a0d322b6302092f21a | `SidecarProtocolLimits::max_schema_bytes` | struct_field | Stores the max schema size for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:46` |
| sym-c81c7e6f05aa7e42d5a4 | `SidecarProtocolLimits::max_signal_id_bytes` | struct_field | Stores the max signal id size for `SidecarProtocolLimits`, in bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:44` |
| sym-952984de250bd8be0ff1 | `SignalContinuityObservation::discontinuity_observed` | struct_field | Stores the discontinuity observed used by `SignalContinuityObservation`. | `src/graph/signal/continuity.rs:7` |
| sym-4bf70913d8e60972e8c6 | `SignalContinuityObservation::policy_changed` | struct_field | Stores the policy changed used by `SignalContinuityObservation`. | `src/graph/signal/continuity.rs:9` |
| sym-f6b3ec07d6ffa4275185 | `SignalContinuityObservation::source_recovered` | struct_field | Stores the source recovered used by `SignalContinuityObservation`. | `src/graph/signal/continuity.rs:8` |
| sym-bb15d6dd98851f65fe99 | `SignalEdgeObservations::capacity_signals` | struct_field | Sets the capacity signals available to `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:32` |
| sym-8616760f7d5213e6b68f | `SignalEdgeObservations::delivered_total` | struct_field | Compatibility alias for `enqueued_total`. | `src/runtime/signal/edge.rs:44` |
| sym-6354a1195da9f288b0ba | `SignalEdgeObservations::depth_signals` | struct_field | Reports the depth signals observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:35` |
| sym-233c198bfd15af460d91 | `SignalEdgeObservations::dropped_total` | struct_field | Counts the total number of dropped observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:45` |
| sym-de2ba230d2a8e19a1191 | `SignalEdgeObservations::enqueued_total` | struct_field | Counts the total number of enqueued observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:37` |
| sym-0395b42230075a0d0522 | `SignalEdgeObservations::max_payload_bytes` | struct_field | Limits payload storage for `SignalEdgeObservations`, in bytes. | `src/runtime/signal/edge.rs:33` |
| sym-51c71c3db2a457b74b69 | `SignalEdgeObservations::maximum_buffered_payload_bytes` | struct_field | Stores the maximum buffered payload size for `SignalEdgeObservations`, in bytes. | `src/runtime/signal/edge.rs:34` |
| sym-35b6accb84c98ea5c4d4 | `SignalEdgeObservations::peak_depth_signals` | struct_field | Reports the peak depth signals observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:36` |
| sym-d35f85ce4558ab271b11 | `SignalEdgeObservations::received_total` | struct_field | Counts the total number of received observed by `SignalEdgeObservations`. | `src/runtime/signal/edge.rs:38` |
| sym-4677b8ccd08fe1e0802d | `SourceEmission::envelope` | struct_field | Stores the envelope used by `SourceEmission`. | `src/session/extensions/source.rs:263` |
| sym-7c1ca3cc9f4542169d07 | `SourceEmission::output_port` | struct_field | Stores the output port used by `SourceEmission`. | `src/session/extensions/source.rs:262` |
| sym-d165b5f072f52dc9ba36 | `SourceEmission::terminal` | struct_field | Indicates whether terminal applies to `SourceEmission`. | `src/session/extensions/source.rs:264` |
| sym-ff90286391079df16de5 | `SourceOutputBranchSpec::branch` | struct_field | Stores the branch used by `SourceOutputBranchSpec`. | `src/session/extensions/source.rs:372` |
| sym-07749ed3d12dd53e6d83 | `SourceOutputBranchSpec::output_port` | struct_field | Stores the output port used by `SourceOutputBranchSpec`. | `src/session/extensions/source.rs:371` |
| sym-3e793eea7d76f6ccc083 | `SourceOutputIdentity::output_port` | struct_field | Stores the output port used by `SourceOutputIdentity`. | `src/session/extensions/source.rs:230` |
| sym-a7c40d62c1f4844938b1 | `SourceOutputIdentity::stream_id` | struct_field | Identifies the stream identifier recorded by `SourceOutputIdentity`. | `src/session/extensions/source.rs:231` |
| sym-90c685c3f816486585db | `SourceOutputPlan::branch_edges` | struct_field | Stores the branch edges used by `SourceOutputPlan`. | `src/graph/plan.rs:117` |
| sym-d32cc1f6aed57a611c4c | `SourceOutputPlan::from` | struct_field | Identifies the origin represented by `SourceOutputPlan`. | `src/graph/plan.rs:114` |
| sym-e2cbbf8e8ed29c6cf02b | `SourceOutputPlan::media` | struct_field | Stores the media used by `SourceOutputPlan`. | `src/graph/plan.rs:116` |
| sym-905780073cf5cc8617f5 | `SourceOutputPlan::signal` | struct_field | Stores the signal used by `SourceOutputPlan`. | `src/graph/plan.rs:115` |
| sym-693f067aa1f855b1e725 | `SourceOutputReceiver::output_port` | struct_field | Stores the output port used by `SourceOutputReceiver`. | `src/session/extensions/source.rs:376` |
| sym-a4c76b62c7675da684b9 | `SourceOutputReceiver::receiver` | struct_field | Stores the receiver used by `SourceOutputReceiver`. | `src/session/extensions/source.rs:377` |
| sym-7a06bcb6f36f63949444 | `SourcePrepareContext::manifest` | struct_field | Stores the manifest used by `SourcePrepareContext`. | `src/session/extensions/source.rs:224` |
| sym-45b25f70d9cd4df163b7 | `SourcePrepareContext::session` | struct_field | Stores the session used by `SourcePrepareContext`. | `src/session/extensions/source.rs:225` |
| sym-08275246f43d0e942da5 | `SourceRuntimeObservations::cancellation_total` | struct_field | Counts the total number of cancellation observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:398` |
| sym-2eed15e5bb52cc3d577e | `SourceRuntimeObservations::discontinuity_total` | struct_field | Counts the total number of discontinuity observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:399` |
| sym-a52e038a03e2e0b2e232 | `SourceRuntimeObservations::dropped_total` | struct_field | Counts the total number of dropped observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:396` |
| sym-da8d920bd6969c805953 | `SourceRuntimeObservations::emitted_total` | struct_field | Counts the total number of emitted observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:395` |
| sym-c3e3d1e656d3fe0eeba5 | `SourceRuntimeObservations::failure_total` | struct_field | Counts the total number of failure observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:397` |
| sym-d0ba543701ea47e7729c | `SourceRuntimeObservations::joined` | struct_field | Stores the joined used by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:403` |
| sym-2a09d28ee63febc4d30f | `SourceRuntimeObservations::policy_change_total` | struct_field | Counts the total number of policy change observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:401` |
| sym-19b439a7974a132024ef | `SourceRuntimeObservations::ready` | struct_field | Indicates whether ready applies to `SourceRuntimeObservations`. | `src/session/extensions/source.rs:402` |
| sym-35f9b3a71ae163cadd27 | `SourceRuntimeObservations::recovery_total` | struct_field | Counts the total number of recovery observed by `SourceRuntimeObservations`. | `src/session/extensions/source.rs:400` |
| sym-0ebdfe57de385db6c15d | `SourceSessionContext::outputs` | struct_field | Stores the outputs used by `SourceSessionContext`. | `src/session/extensions/source.rs:238` |
| sym-26c282c098dc40d49bcd | `SourceSessionContext::session_id` | struct_field | Identifies the session identifier recorded by `SourceSessionContext`. | `src/session/extensions/source.rs:236` |
| sym-7549d7037ea50ec24f61 | `SourceSessionContext::source_id` | struct_field | Identifies the source identifier recorded by `SourceSessionContext`. | `src/session/extensions/source.rs:237` |
| sym-3ff54bfcb70c6b95cad9 | `SourceTypeIdError::TooLong::actual_bytes` | struct_field | Stores the actual size for `TooLong`, in bytes. | `src/session/extensions/source.rs:75` |
| sym-af70b487e23492530611 | `SourceTypeIdError::TooLong::maximum_bytes` | struct_field | Stores the maximum size for `TooLong`, in bytes. | `src/session/extensions/source.rs:76` |
| sym-ef156c00381b95373579 | `StreamOrigin::OperatorOutput::operator_instance_id` | struct_field | Identifies the operator instance identifier recorded by `OperatorOutput`. | `src/session/declaration/spec.rs:217` |
| sym-a5a80a32f41c25bc4e2c | `StreamOrigin::OperatorOutput::output_port` | struct_field | Stores the output port used by `OperatorOutput`. | `src/session/declaration/spec.rs:218` |
| sym-693cf5284ddc22b54198 | `StreamOrigin::SourceOutput::output_port` | struct_field | Stores the output port used by `SourceOutput`. | `src/session/declaration/spec.rs:212` |
| sym-da0a112fe0706ec14425 | `StreamOrigin::SourceOutput::source_id` | struct_field | Identifies the source identifier recorded by `SourceOutput`. | `src/session/declaration/spec.rs:214` |
| sym-77eb656113d5e320e9c4 | `StreamOrigin::SourceOutput::source_instance_id` | struct_field | Identifies the source instance identifier recorded by `SourceOutput`. | `src/session/declaration/spec.rs:211` |
| sym-b5b934efd43145b8f5af | `StreamOrigin::SourceOutput::stream_id` | struct_field | Identifies the stream identifier recorded by `SourceOutput`. | `src/session/declaration/spec.rs:213` |
| sym-3da084bf16fb7475c2eb | `TimelineMapping::session_origin_ns` | struct_field | Stores the session origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:4` |
| sym-b074cd38dd530f8d721c | `TimelineMapping::source_origin_ns` | struct_field | Stores the source origin value for `TimelineMapping`, in nanoseconds. | `src/timing/timeline_mapping.rs:3` |
| sym-ce5f87d4cfafc0551dfd | `TypedEdgeBranchSpec::capacity_signals` | struct_field | Sets the capacity signals available to `TypedEdgeBranchSpec`. | `src/runtime/signal/edge.rs:249` |
| sym-523c94242a83f36e10ed | `TypedEdgeBranchSpec::edge_contract` | struct_field | Stores the edge contract used by `TypedEdgeBranchSpec`. | `src/runtime/signal/edge.rs:250` |
| sym-a99cecd43a5e42f5de8d | `TypedEdgeBuildError::CapacityTooLarge::capacity_signals` | struct_field | Sets the capacity signals available to `CapacityTooLarge`. | `src/runtime/signal/edge.rs:393` |
| sym-a4764e45cc06ea8c00bb | `TypedEdgeBuildError::CapacityTooLarge::maximum` | struct_field | Sets the inclusive maximum accepted by `CapacityTooLarge`. | `src/runtime/signal/edge.rs:394` |
| sym-00860353bb232516f37f | `TypedEdgeBuildError::PayloadLimitTooLarge::max_payload_bytes` | struct_field | Limits payload storage for `PayloadLimitTooLarge`, in bytes. | `src/runtime/signal/edge.rs:402` |
| sym-3d74e7380a254c7b0c77 | `TypedEdgeBuildError::PayloadLimitTooLarge::maximum` | struct_field | Sets the inclusive maximum accepted by `PayloadLimitTooLarge`. | `src/runtime/signal/edge.rs:403` |
| sym-3821ffd6df8543b2351e | `TypedEdgePlan::capacity_signals` | struct_field | Sets the capacity signals available to `TypedEdgePlan`. | `src/graph/plan.rs:103` |
| sym-534981f360a3c8bcc55e | `TypedEdgePlan::contract` | struct_field | Stores the contract used by `TypedEdgePlan`. | `src/graph/plan.rs:102` |
| sym-5a33e79e495a3df07e88 | `TypedEdgePlan::edge` | struct_field | Stores the edge used by `TypedEdgePlan`. | `src/graph/plan.rs:97` |
| sym-b1d46d436c3f90759d59 | `TypedEdgePlan::from` | struct_field | Identifies the origin represented by `TypedEdgePlan`. | `src/graph/plan.rs:98` |
| sym-dd9686b987f57d35b8e4 | `TypedEdgePlan::media` | struct_field | Stores the media used by `TypedEdgePlan`. | `src/graph/plan.rs:101` |
| sym-a0c064e5616aaf861797 | `TypedEdgePlan::metric_id` | struct_field | Identifies the metric identifier recorded by `TypedEdgePlan`. | `src/graph/plan.rs:104` |
| sym-c098967c5ed3ad5d77b5 | `TypedEdgePlan::signal` | struct_field | Stores the signal used by `TypedEdgePlan`. | `src/graph/plan.rs:100` |
| sym-4e8cda6f6c079f44c1f1 | `TypedEdgePlan::to` | struct_field | Identifies the destination represented by `TypedEdgePlan`. | `src/graph/plan.rs:99` |
| sym-a74ca20e0d1a597d130c | `TypedEdgePublishError::PayloadTooLarge::branch_index` | struct_field | Stores the branch index used by `PayloadTooLarge`. | `src/runtime/signal/edge.rs:417` |
| sym-b7d9f031c91db48d4f0c | `TypedEdgePublishError::PayloadTooLarge::max_payload_bytes` | struct_field | Limits payload storage for `PayloadTooLarge`, in bytes. | `src/runtime/signal/edge.rs:419` |
| sym-ece64088eb1894b9e7b4 | `TypedEdgePublishError::PayloadTooLarge::payload_bytes` | struct_field | Stores the payload size for `PayloadTooLarge`, in bytes. | `src/runtime/signal/edge.rs:418` |
| sym-82b00c0c15cf11b2b136 | `TypedEdgePublishError::RequiredBranchFull::branch_index` | struct_field | Stores the branch index used by `RequiredBranchFull`. | `src/runtime/signal/edge.rs:422` |
| sym-b7114f685be5e3dbd26c | `TypedEdgePublishReport::delivered_total` | struct_field | Counts the total number of delivered observed by `TypedEdgePublishReport`. | `src/runtime/signal/edge.rs:381` |
| sym-02ede0d39f6b3831cf2f | `TypedEdgePublishReport::dropped_total` | struct_field | Counts the total number of dropped observed by `TypedEdgePublishReport`. | `src/runtime/signal/edge.rs:382` |
| sym-24f46b076e9df29d20fc | `TypedStreamError::AmbiguousPort::direction` | struct_field | Stores the direction used by `AmbiguousPort`. | `src/session/declaration/typed_stream.rs:203` |
| sym-d90d4f5494a94f1e6c47 | `TypedStreamError::InputSignalMismatch::port` | struct_field | Stores the port used by `InputSignalMismatch`. | `src/session/declaration/typed_stream.rs:205` |
| sym-814c9f5b7c0a64458ded | `TypedStreamError::MissingPort::direction` | struct_field | Stores the direction used by `MissingPort`. | `src/session/declaration/typed_stream.rs:201` |
| sym-3e8a192854f496551ae6 | `TypedStreamError::OperatorIdentityMismatch::declaration` | struct_field | Stores the declaration used by `OperatorIdentityMismatch`. | `src/session/declaration/typed_stream.rs:192` |
| sym-5846c54b340d7c6c5c46 | `TypedStreamError::OperatorIdentityMismatch::manifest` | struct_field | Stores the manifest used by `OperatorIdentityMismatch`. | `src/session/declaration/typed_stream.rs:193` |
| sym-89d47b18e31b608fb3a5 | `TypedStreamError::OutputSignalMismatch::port` | struct_field | Stores the port used by `OutputSignalMismatch`. | `src/session/declaration/typed_stream.rs:207` |
| sym-1dc082a5d2496f5c81f4 | `TypedStreamError::UnknownPort::direction` | struct_field | Stores the direction used by `UnknownPort`. | `src/session/declaration/typed_stream.rs:197` |
| sym-12e8e2c492bdf77fb906 | `TypedStreamError::UnknownPort::port` | struct_field | Stores the port used by `UnknownPort`. | `src/session/declaration/typed_stream.rs:198` |
| sym-63d4caa3f8249ce4806e | `audio::AudioFrameBuildError::MisalignedSamples::channels` | struct_field | Stores the channels used by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-020cbffec15678965496 | `audio::AudioFrameBuildError::MisalignedSamples::samples` | struct_field | Stores the samples used by `MisalignedSamples`. | `src/frame/audio.rs:57` |
| sym-4f16ce993cc48329e3f7 | `audio::SampleSpec::channels` | struct_field | Stores the channels used by `SampleSpec`. | `src/frame/audio.rs:20` |
| sym-52d5039021f6cd50053c | `audio::SampleSpec::format` | struct_field | Stores the format used by `SampleSpec`. | `src/frame/audio.rs:21` |
| sym-059bce41080500fbc90a | `audio::SampleSpec::sample_rate_hz` | struct_field | Stores the sample rate value for `SampleSpec`, in hertz. | `src/frame/audio.rs:19` |
| sym-7664c4456b7b644bd5cb | `authorization::CaptureAuthorizationSnapshot::application_policy` | struct_field | Stores the application policy used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:20` |
| sym-637ec0825fe64893013b | `authorization::CaptureAuthorizationSnapshot::capability` | struct_field | Stores the capability used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:18` |
| sym-43c8c0739b846d1f9934 | `authorization::CaptureAuthorizationSnapshot::capture_scope` | struct_field | Stores the capture scope used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:22` |
| sym-bfb980861013d712937b | `authorization::CaptureAuthorizationSnapshot::identity_strength` | struct_field | Stores the identity strength used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:23` |
| sym-073af6c2662e8cb30e5d | `authorization::CaptureAuthorizationSnapshot::observed_at_ns` | struct_field | Stores the observed at value for `CaptureAuthorizationSnapshot`, in nanoseconds. | `src/capture/authorization.rs:25` |
| sym-74695c5f870e4a669f17 | `authorization::CaptureAuthorizationSnapshot::open_outcome` | struct_field | Stores the open outcome used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:26` |
| sym-78476799989c93fd00f8 | `authorization::CaptureAuthorizationSnapshot::os_permission` | struct_field | Stores the os permission used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:19` |
| sym-da440180eacfdaa511cb | `authorization::CaptureAuthorizationSnapshot::permission_epoch` | struct_field | Stores the permission epoch used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:24` |
| sym-3188d8fad2d775859a43 | `authorization::CaptureAuthorizationSnapshot::session_grant` | struct_field | Stores the session grant used by `CaptureAuthorizationSnapshot`. | `src/capture/authorization.rs:21` |
| sym-37fdbe707be788a82fb5 | `authorization::CaptureError::BackendSetupRequired::action` | struct_field | Stores the action used by `BackendSetupRequired`. | `src/capture/authorization.rs:298` |
| sym-6900d16093aa9e0be4b3 | `authorization::CaptureError::BackendSetupRequired::backend` | struct_field | Stores the backend used by `BackendSetupRequired`. | `src/capture/authorization.rs:297` |
| sym-c2ed0449b0dea0b67ecb | `authorization::CaptureError::BackendStatus::operation` | struct_field | Stores the operation used by `BackendStatus`. | `src/capture/authorization.rs:304` |
| sym-bf639884f816cbf95df4 | `authorization::CaptureError::BackendStatus::status_code` | struct_field | Stores the status code used by `BackendStatus`. | `src/capture/authorization.rs:305` |
| sym-b40f7f41ec9bc37c240e | `authorization::CaptureError::CaptureWorkerPanicked::worker` | struct_field | Stores the worker used by `CaptureWorkerPanicked`. | `src/capture/authorization.rs:316` |
| sym-0b436b7ab46d1c6d4e5f | `authorization::CaptureError::PermissionDenied::operation` | struct_field | Stores the operation used by `PermissionDenied`. | `src/capture/authorization.rs:301` |
| sym-35d71fd94c50c32dfab7 | `authorization::CaptureError::SourceUnavailable::stable_key` | struct_field | Stores the stable key used by `SourceUnavailable`. | `src/capture/authorization.rs:308` |
| sym-3dccdf6602fe6deb6c67 | `authorization::CapturePermissionTransition::current` | struct_field | Stores the current used by `CapturePermissionTransition`. | `src/capture/authorization.rs:171` |
| sym-260e3b0756d352d4cdde | `authorization::CapturePermissionTransition::kind` | struct_field | Stores the kind used by `CapturePermissionTransition`. | `src/capture/authorization.rs:169` |
| sym-a53df0e8bcb46547e36a | `authorization::CapturePermissionTransition::permission_epoch` | struct_field | Stores the permission epoch used by `CapturePermissionTransition`. | `src/capture/authorization.rs:172` |
| sym-747552518d78c8ec1099 | `authorization::CapturePermissionTransition::previous` | struct_field | Stores the previous used by `CapturePermissionTransition`. | `src/capture/authorization.rs:170` |
| sym-6e5e1ff83c5743c0996e | `authorization::CaptureScope::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/authorization.rs:249` |
| sym-5ed06979ef16447a6b13 | `authorization::CaptureScope::ExactInputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactInputDevice`. | `src/capture/authorization.rs:250` |
| sym-6d84e3217f4d63c8514b | `authorization::CaptureScope::ExactOutputDevice::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactOutputDevice`. | `src/capture/authorization.rs:251` |
| sym-7b35e6776a9ad0f6857e | `events::CaptureRuntimeFailure::error_class` | struct_field | Stores the error class used by `CaptureRuntimeFailure`. | `src/capture/events.rs:49` |
| sym-3807587d486e070e9441 | `events::CaptureRuntimeFailure::operation` | struct_field | Stores the operation used by `CaptureRuntimeFailure`. | `src/capture/events.rs:48` |
| sym-bfe85cfae39c03c55b2e | `events::CaptureRuntimeFailureClass::BackendClass::class` | struct_field | Stores the class used by `BackendClass`. | `src/capture/events.rs:43` |
| sym-dfb974cf26237ca42c98 | `events::CaptureRuntimeFailureClass::PlatformStatus::status_code` | struct_field | Stores the status code used by `PlatformStatus`. | `src/capture/events.rs:42` |
| sym-0f09d21fcfb9896181e4 | `events::SourceRuntimeEvent::BackendFailure::failure` | struct_field | Carries the failure reported by `BackendFailure`. | `src/capture/events.rs:63` |
| sym-8cc396e9c576990b1626 | `events::SourceRuntimeEvent::BackendFailure::generation` | struct_field | Stores the generation used by `BackendFailure`. | `src/capture/events.rs:62` |
| sym-5e338da4ad5b5a28605e | `events::SourceRuntimeEvent::BackendFailure::stable_id` | struct_field | Identifies the stable identifier recorded by `BackendFailure`. | `src/capture/events.rs:61` |
| sym-29a0a855d7399dcfa721 | `events::SourceRuntimeEvent::SourceUnavailable::failure` | struct_field | Carries the failure reported by `SourceUnavailable`. | `src/capture/events.rs:58` |
| sym-930c8762a56effaecc0d | `events::SourceRuntimeEvent::SourceUnavailable::generation` | struct_field | Stores the generation used by `SourceUnavailable`. | `src/capture/events.rs:56` |
| sym-b6254ed8c91ae7794dca | `events::SourceRuntimeEvent::SourceUnavailable::recovery_requirement` | struct_field | Stores the recovery requirement used by `SourceUnavailable`. | `src/capture/events.rs:57` |
| sym-700e9374d1d49f5cd2a5 | `events::SourceRuntimeEvent::SourceUnavailable::stable_id` | struct_field | Identifies the stable identifier recorded by `SourceUnavailable`. | `src/capture/events.rs:55` |
| sym-e9e12b180de73f91fada | `events::SourceRuntimeEventObservations::capacity_event_count` | struct_field | Sets the capacity event count available to `SourceRuntimeEventObservations`. | `src/capture/events.rs:112` |
| sym-576f1fa5808bb74f0e50 | `events::SourceRuntimeEventObservations::depth_events` | struct_field | Reports the depth events observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:115` |
| sym-97221a2cf4ff68787c57 | `events::SourceRuntimeEventObservations::depth_owned_bytes` | struct_field | Stores the depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:116` |
| sym-e911cc094999d6f2cce8 | `events::SourceRuntimeEventObservations::events_dropped_oversized_total` | struct_field | Counts the total number of events dropped oversized observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:120` |
| sym-83e9ab76972024638989 | `events::SourceRuntimeEventObservations::events_dropped_total` | struct_field | Counts the total number of events dropped observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:119` |
| sym-a69a3772f376c36ea735 | `events::SourceRuntimeEventObservations::events_enqueued_total` | struct_field | Counts the total number of events enqueued observed by `SourceRuntimeEventObservations`. | `src/capture/events.rs:118` |
| sym-5e2b5f1b156347230739 | `events::SourceRuntimeEventObservations::maximum_buffered_owned_bytes` | struct_field | Stores the maximum buffered owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:114` |
| sym-e9a15487b85ce9efc151 | `events::SourceRuntimeEventObservations::maximum_event_owned_bytes` | struct_field | Stores the maximum event owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:113` |
| sym-4466dd4912bee96392a9 | `events::SourceRuntimeEventObservations::peak_depth_owned_bytes` | struct_field | Stores the peak depth owned size for `SourceRuntimeEventObservations`, in bytes. | `src/capture/events.rs:117` |
| sym-e4030a6410f700cc5dcc | `identity::CaptureSource::app_id` | struct_field | Identifies the app identifier recorded by `CaptureSource`. | `src/capture/identity.rs:86` |
| sym-b739ff2f382d7dcbf9c8 | `identity::CaptureSource::channels` | struct_field | Stores the channels used by `CaptureSource`. | `src/capture/identity.rs:90` |
| sym-d36900aa9174e272ab96 | `identity::CaptureSource::device_uid` | struct_field | Stores the device uid used by `CaptureSource`. | `src/capture/identity.rs:87` |
| sym-7b5c76d328c34c5a816d | `identity::CaptureSource::name` | struct_field | Stores the name used by `CaptureSource`. | `src/capture/identity.rs:84` |
| sym-d40befabb7e0de351b5a | `identity::CaptureSource::process_id` | struct_field | Identifies the process identifier recorded by `CaptureSource`. | `src/capture/identity.rs:85` |
| sym-af5d68e009d0a63951c9 | `identity::CaptureSource::sample_rate_hz` | struct_field | Stores the sample rate value for `CaptureSource`, in hertz. | `src/capture/identity.rs:89` |
| sym-a738e42405aa73b5ce30 | `identity::CaptureSource::stable_id` | struct_field | Identifies the stable identifier recorded by `CaptureSource`. | `src/capture/identity.rs:83` |
| sym-4831e34c203fe51d8896 | `identity::CaptureSource::state` | struct_field | Stores the state used by `CaptureSource`. | `src/capture/identity.rs:88` |
| sym-3e2e7e8983dd10ab814d | `identity::StableSourceId::kind` | struct_field | Stores the kind used by `StableSourceId`. | `src/capture/identity.rs:28` |
| sym-4abf90caaf0d81f1b139 | `identity::StableSourceId::platform` | struct_field | Stores the platform used by `StableSourceId`. | `src/capture/identity.rs:27` |
| sym-ba9aa6872ba7a3de4ecf | `identity::StableSourceId::stable_key` | struct_field | Stores the stable key used by `StableSourceId`. | `src/capture/identity.rs:29` |
| sym-456d36fc5ce16183d24f | `lifecycle_registry::SourceGenerationTransition::Disappeared::generation` | struct_field | Stores the generation used by `Disappeared`. | `src/capture/lifecycle_registry.rs:11` |
| sym-b726b1261e3ddf381f57 | `lifecycle_registry::SourceGenerationTransition::Disappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Disappeared`. | `src/capture/lifecycle_registry.rs:10` |
| sym-9b6d08a34d1db71df987 | `lifecycle_registry::SourceGenerationTransition::Reappeared::generation` | struct_field | Stores the generation used by `Reappeared`. | `src/capture/lifecycle_registry.rs:16` |
| sym-4024e6be8248dd03cd51 | `lifecycle_registry::SourceGenerationTransition::Reappeared::previous_generation` | struct_field | Stores the previous generation used by `Reappeared`. | `src/capture/lifecycle_registry.rs:15` |
| sym-db0383bcd447d6a31389 | `lifecycle_registry::SourceGenerationTransition::Reappeared::stable_id` | struct_field | Identifies the stable identifier recorded by `Reappeared`. | `src/capture/lifecycle_registry.rs:14` |
| sym-b653deb71c75103c93e5 | `pocketstation::SessionEndpointError::DuplicateNodeTypeId::node_type_id` | struct_field | Identifies the node type identifier recorded by `DuplicateNodeTypeId`. | `src/lib.rs:1041` |
| sym-093a2d281e2036cf8341 | `pocketstation::SessionEndpointError::DuplicateOperatorId::operator_id` | struct_field | Identifies the operator identifier recorded by `DuplicateOperatorId`. | `src/lib.rs:1039` |
| sym-4a91a5c1bfc409fc5bd5 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_finalized_total` | struct_field | Counts the total number of endpoint finalized observed by `ExtensionConformanceReport`. | `src/conformance.rs:594` |
| sym-bd6866238e0189356f15 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_id` | struct_field | Identifies the endpoint identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:579` |
| sym-327996ec50a22c56e255 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_prepared_total` | struct_field | Counts the total number of endpoint prepared observed by `ExtensionConformanceReport`. | `src/conformance.rs:591` |
| sym-5d9ea197178ea39ee5b2 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_received_total` | struct_field | Counts the total number of endpoint received observed by `ExtensionConformanceReport`. | `src/conformance.rs:592` |
| sym-5e4ad34b0bac81288868 | `pocketstation::conformance::ExtensionConformanceReport::endpoint_stopped_total` | struct_field | Counts the total number of endpoint stopped observed by `ExtensionConformanceReport`. | `src/conformance.rs:593` |
| sym-57d9a2011b5951969682 | `pocketstation::conformance::ExtensionConformanceReport::failure_requested` | struct_field | Stores the failure requested used by `ExtensionConformanceReport`. | `src/conformance.rs:582` |
| sym-a4427a3826d9634b89b1 | `pocketstation::conformance::ExtensionConformanceReport::input_payload` | struct_field | Stores the input payload used by `ExtensionConformanceReport`. | `src/conformance.rs:580` |
| sym-cebe956585f443578318 | `pocketstation::conformance::ExtensionConformanceReport::lifecycle_event_total` | struct_field | Counts the total number of lifecycle event observed by `ExtensionConformanceReport`. | `src/conformance.rs:595` |
| sym-dd6c5e8a6a0fbb6d1868 | `pocketstation::conformance::ExtensionConformanceReport::maximum_buffered_payload_bytes` | struct_field | Stores the maximum buffered payload size for `ExtensionConformanceReport`, in bytes. | `src/conformance.rs:602` |
| sym-7b44fa3de3ea58f8b1cc | `pocketstation::conformance::ExtensionConformanceReport::operator_closed_total` | struct_field | Counts the total number of operator closed observed by `ExtensionConformanceReport`. | `src/conformance.rs:590` |
| sym-d242074c0cd6803dd85d | `pocketstation::conformance::ExtensionConformanceReport::operator_failure_total` | struct_field | Counts the total number of operator failure observed by `ExtensionConformanceReport`. | `src/conformance.rs:589` |
| sym-707ac517053b18af0c2d | `pocketstation::conformance::ExtensionConformanceReport::operator_id` | struct_field | Identifies the operator identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:578` |
| sym-046c8a2a8f1faf755151 | `pocketstation::conformance::ExtensionConformanceReport::operator_output_total` | struct_field | Counts the total number of operator output observed by `ExtensionConformanceReport`. | `src/conformance.rs:588` |
| sym-0384e48e95aa77134e55 | `pocketstation::conformance::ExtensionConformanceReport::operator_prepared_total` | struct_field | Counts the total number of operator prepared observed by `ExtensionConformanceReport`. | `src/conformance.rs:586` |
| sym-a5eb8f2e0026f6a9f20c | `pocketstation::conformance::ExtensionConformanceReport::operator_processed_total` | struct_field | Counts the total number of operator processed observed by `ExtensionConformanceReport`. | `src/conformance.rs:587` |
| sym-ab37b6c196b3fe791de4 | `pocketstation::conformance::ExtensionConformanceReport::output_payload` | struct_field | Stores the output payload used by `ExtensionConformanceReport`. | `src/conformance.rs:581` |
| sym-211e19d2cfac816deb0b | `pocketstation::conformance::ExtensionConformanceReport::queue_capacity_signals` | struct_field | Sets the queue capacity signals available to `ExtensionConformanceReport`. | `src/conformance.rs:597` |
| sym-b668261d30024c0021cf | `pocketstation::conformance::ExtensionConformanceReport::queue_peak_signals` | struct_field | Reports the queue peak signals observed by `ExtensionConformanceReport`. | `src/conformance.rs:598` |
| sym-764c86937d09c7784524 | `pocketstation::conformance::ExtensionConformanceReport::role_id` | struct_field | Identifies the role identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:576` |
| sym-a007acc788ecf3978109 | `pocketstation::conformance::ExtensionConformanceReport::route_capacity_signals` | struct_field | Sets the route capacity signals available to `ExtensionConformanceReport`. | `src/conformance.rs:599` |
| sym-a1c066e5a4dba617caa7 | `pocketstation::conformance::ExtensionConformanceReport::route_delivered_total` | struct_field | Counts the total number of route delivered observed by `ExtensionConformanceReport`. | `src/conformance.rs:601` |
| sym-bd4012ef8c1f00c3635b | `pocketstation::conformance::ExtensionConformanceReport::route_peak_signals` | struct_field | Reports the route peak signals observed by `ExtensionConformanceReport`. | `src/conformance.rs:600` |
| sym-2adc1ae4b2b799b23ee9 | `pocketstation::conformance::ExtensionConformanceReport::schema_id` | struct_field | Identifies the schema identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:575` |
| sym-818af23b3935b4850ca8 | `pocketstation::conformance::ExtensionConformanceReport::signal_id` | struct_field | Identifies the signal identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:574` |
| sym-b6e1791a8960d218898f | `pocketstation::conformance::ExtensionConformanceReport::source_closed_total` | struct_field | Counts the total number of source closed observed by `ExtensionConformanceReport`. | `src/conformance.rs:585` |
| sym-e6340236d88bc0ef7194 | `pocketstation::conformance::ExtensionConformanceReport::source_emitted_total` | struct_field | Counts the total number of source emitted observed by `ExtensionConformanceReport`. | `src/conformance.rs:584` |
| sym-01071c492d7e56c1b15d | `pocketstation::conformance::ExtensionConformanceReport::source_prepared_total` | struct_field | Counts the total number of source prepared observed by `ExtensionConformanceReport`. | `src/conformance.rs:583` |
| sym-74990ab7812c12aede20 | `pocketstation::conformance::ExtensionConformanceReport::source_type_id` | struct_field | Identifies the source type identifier recorded by `ExtensionConformanceReport`. | `src/conformance.rs:577` |
| sym-caa55da2ac5b06e62cf7 | `pocketstation::conformance::ExtensionConformanceReport::stop_success` | struct_field | Stores the stop success used by `ExtensionConformanceReport`. | `src/conformance.rs:603` |
| sym-3d7ff581ab6ed1a98162 | `pocketstation::conformance::ExtensionConformanceReport::terminal_event_total` | struct_field | Counts the total number of terminal event observed by `ExtensionConformanceReport`. | `src/conformance.rs:596` |
| sym-d1153d617a98481c966e | `pocketstation::connector::ConnectorDeclarationError::WrongSession::registered` | struct_field | Stores the registered used by `WrongSession`. | `src/connector/mod.rs:236` |
| sym-3f52f0a98a39a82cb3cc | `pocketstation::connector::ConnectorDeclarationError::WrongSession::requested` | struct_field | Stores the requested used by `WrongSession`. | `src/connector/mod.rs:237` |
| sym-cfef2464d1289822c2ef | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::registered` | struct_field | Stores the registered used by `WrongSession`. | `src/connector/mod.rs:249` |
| sym-c61cdedce41edfe48070 | `pocketstation::connector::ConnectorObservationLookupError::WrongSession::requested` | struct_field | Stores the requested used by `WrongSession`. | `src/connector/mod.rs:250` |
| sym-6f802083c5c1d7266efd | `pool::AudioBufferWriteError::CapacityExceeded::capacity_samples` | struct_field | Sets the capacity samples available to `CapacityExceeded`. | `src/frame/pool.rs:20` |
| sym-15ef9e6300ed6340bc47 | `pool::AudioBufferWriteError::CapacityExceeded::requested_samples` | struct_field | Stores the requested samples used by `CapacityExceeded`. | `src/frame/pool.rs:19` |
| sym-84282b1095338b4ef024 | `selection::CaptureMode::ExactApplication::process_id` | struct_field | Identifies the process identifier recorded by `ExactApplication`. | `src/capture/selection.rs:22` |
| sym-f8d6a4d0f6384da1cf41 | `selection::CaptureMode::ExactApplication::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplication`. | `src/capture/selection.rs:23` |
| sym-33273e59149260413aee | `selection::CaptureMode::ExactApplicationStable::stable_id` | struct_field | Identifies the stable identifier recorded by `ExactApplicationStable`. | `src/capture/selection.rs:26` |
| sym-bc4d349ab8b98a15a530 | `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| sym-8f6c56609c608d400c5f | `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| sym-daefb89a465f85894b9d | `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| sym-0b8e49c6c2e292c1b1a7 | `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| sym-e38e2d3daefdbafe1737 | `pocketstation::connector::worker::ConnectorFactory` | trait | Implement this trait to provide connector behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:17` |
| sym-b8944599561449d3bff7 | `pocketstation::connector::worker::ConnectorWorker` | trait | Implement this trait to provide connector worker behavior to PocketStation; its methods define the preparation and runtime contract. | `src/connector/worker/mod.rs:32` |
| sym-0f52b43dab2320808210 | `pocketstation::connector::worker::driver::ConnectorDriver` | trait | Provider-specific behavior executed on Core's bounded connector worker. | `src/connector/worker/driver.rs:92` |
| sym-22009b8aa13b514c452d | `pocketstation::connector::worker::driver::ConnectorDriverFactory` | trait | Prepares provider state while Core retains receiver and lifecycle authority. | `src/connector/worker/driver.rs:123` |
| sym-24a418620ed384787b21 | `pocketstation::endpoint::contract::EndpointDriverFactory` | trait | Implement this trait to provide endpoint behavior to PocketStation; its methods define the preparation and runtime contract. | `src/endpoint/contract.rs:232` |
| sym-b1b5bdddc63ee20f95f0 | `pocketstation::endpoint::runtime::PreparedEndpointDriver` | trait | Prepared endpoint resources that have not started consuming their edge. | `src/endpoint/runtime.rs:318` |
| sym-5693e1fc212774b1d075 | `pocketstation::endpoint::runtime::RunningEndpointDriver` | trait | Active endpoint resources owned until finalization. | `src/endpoint/runtime.rs:336` |
| sym-ec0840ecdb4585f0f6fe | `pocketstation::graph::registry::NodeDefinition` | trait | Implement this trait to provide node definition behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/registry.rs:21` |
| sym-1b8125a05c92b5ecb289 | `pocketstation::graph::registry::NodeFactory` | trait | Implement this trait to provide node behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/registry.rs:11` |
| sym-b4ae814495049f35b5b0 | `pocketstation::graph::runtime_node::RuntimeNode` | trait | Realtime invariant: for nodes whose ExecutionClass::is_realtime is true, process() must stay alloc-free, lock-free, log-free, and blocking-free (LAW 15). All working state is sized once in prepare() and reused for the lifetime of the node. | `src/graph/runtime_node.rs:7` |
| sym-2ad36efe33e7d9573345 | `pocketstation::graph::signal::operator::AsyncNode` | trait | Async operator contract for model, connector, transport, and control-plane work. | `src/graph/signal/operator.rs:13` |
| sym-79b260f9a429d6aa12da | `pocketstation::graph::signal::operator::AsyncOperatorFactory` | trait | Implement this trait to provide async operator behavior to PocketStation; its methods define the preparation and runtime contract. | `src/graph/signal/operator.rs:368` |
| sym-13738e14e563aed506b3 | `pocketstation::session::declaration::typed_stream::StreamSignal` | trait | Compile-time marker supplied by an SDK or external package. | `src/session/declaration/typed_stream.rs:15` |
| sym-ff941939086b7b0aa8b2 | `pocketstation::session::extensions::source::SourceDriver` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/session/extensions/source.rs:267` |
| sym-e7822b7ea5e6006285d0 | `pocketstation::session::extensions::source::SourceFactory` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/session/extensions/source.rs:276` |
| sym-929589cb52631dd270e1 | `pocketstation::abi::executable_extension::PksExtensionAcquireRegistrationCallback` | type_alias | Defines the optional C callback used to acquire an extension registration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:110` |
| sym-8eadd98bd426760ed3bc | `pocketstation::abi::executable_extension::PksExtensionCreateCallback` | type_alias | Defines the optional C callback used to create an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:56` |
| sym-72a0d1a013e62c74c736 | `pocketstation::abi::executable_extension::PksExtensionDestroyCallback` | type_alias | Defines the optional C callback used to destroy extension-owned context; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:87` |
| sym-058bc6398ba9559930f5 | `pocketstation::abi::executable_extension::PksExtensionEndpointConsumeCallback` | type_alias | Defines the optional C callback used to consume an endpoint input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:77` |
| sym-40dddd7eddd2e900d81d | `pocketstation::abi::executable_extension::PksExtensionFinishCallback` | type_alias | Defines the optional C callback used to finish extension work; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:85` |
| sym-040e44a7efec0d272a56 | `pocketstation::abi::executable_extension::PksExtensionLibraryEntrypoint` | type_alias | Names the extension library entrypoint type used by the public API. | `src/abi/executable_extension.rs:133` |
| sym-cab0e0649d56d2d4021e | `pocketstation::abi::executable_extension::PksExtensionOperatorProcessCallback` | type_alias | Defines the optional C callback used to process an operator input; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:70` |
| sym-51603d93967850b8bcbe | `pocketstation::abi::executable_extension::PksExtensionPrepareCallback` | type_alias | Defines the optional C callback used to prepare an extension instance; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:48` |
| sym-d346b403b0052ab732f7 | `pocketstation::abi::executable_extension::PksExtensionSourceNextCallback` | type_alias | Defines the optional C callback used to produce the next source signal; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:63` |
| sym-5d1b8c85f5778fb1f9f5 | `pocketstation::abi::executable_extension::PksExtensionStopCallback` | type_alias | Defines the optional C callback used to request an extension instance to stop; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:83` |
| sym-28fc61ff23de4de04295 | `pocketstation::abi::executable_extension::PksExtensionValidateConfigurationCallback` | type_alias | Defines the optional C callback used to validate extension configuration; pointer validity and ownership follow the extension ABI contract. | `src/abi/executable_extension.rs:50` |
| sym-72117563a0dcd6169ca7 | `pocketstation::graph::signal::preparation::AsyncNodeFuture` | type_alias | Names the future returned by async node operations. | `src/graph/signal/preparation.rs:9` |
| sym-387aed1ad8aa81026025 | `pocketstation::graph::signal::preparation::AsyncOperatorEdgePrepareContext` | type_alias | Exact bounded graph edge supplied to an asynchronous Operator at prepare time. | `src/graph/signal/preparation.rs:18` |
| sym-d9af76a5d5a89764ab51 | `pocketstation::runtime::signal::edge::TypedEdgeObservationHandle` | type_alias | Names the typed edge observation handle type used by the public API. | `src/runtime/signal/edge.rs:245` |
| sym-41a71e2e1fcdc2d3a41f | `pocketstation::runtime::signal::edge::TypedEdgeObservations` | type_alias | Names the typed edge observations type used by the public API. | `src/runtime/signal/edge.rs:244` |
| sym-5675a7987e59728973e2 | `pocketstation::runtime::signal::edge::TypedEdgeReceiver` | type_alias | Names the typed edge receiver type used by the public API. | `src/runtime/signal/edge.rs:243` |
| sym-7352b9d4b06db643a358 | `pocketstation::runtime::signal::io::AsyncOperatorOutput` | type_alias | Names the async operator output type used by the public API. | `src/runtime/signal/io.rs:74` |
| sym-0a9dfe98c9e22eb19dcb | `pocketstation::runtime::signal::io::AsyncOperatorOutputBranchSpec` | type_alias | Names the async operator output branch spec type used by the public API. | `src/runtime/signal/io.rs:76` |
| sym-5ccc7612d44d99605d70 | `pocketstation::runtime::signal::io::AsyncOperatorOutputObservationHandle` | type_alias | Names the async operator output observation handle type used by the public API. | `src/runtime/signal/io.rs:75` |
| sym-4a8ab47b24e5eccd2b27 | `pocketstation::runtime::signal::io::AsyncOperatorOutputObservations` | type_alias | Names the async operator output observations type used by the public API. | `src/runtime/signal/io.rs:77` |
| sym-53172a7b181ffae7e24a | `pocketstation::session::declaration::spec::OperatorSpec` | type_alias | Names the operator spec type used by the public API. | `src/session/declaration/spec.rs:274` |
| sym-4580bbafdcd5ddb63569 | `pocketstation::SessionCancelDisposition::AlreadyStopped` | variant | Indicates that the operation had already stopped. | `src/lib.rs:1087` |
| sym-9bfe8cdd90c3e55c66e7 | `pocketstation::SessionCancelDisposition::Cancelled` | variant | Indicates that the operation was cancelled. | `src/lib.rs:1086` |
| sym-0cf45c3fc8a63576e799 | `pocketstation::SessionEndpointError::DuplicateNodeTypeId` | variant | Reported when the owning operation encounters duplicate node type identifier. | `src/lib.rs:1041` |
| sym-2b24a2353ef3ee26ef15 | `pocketstation::SessionEndpointError::DuplicateOperatorId` | variant | Reported when the owning operation encounters duplicate operator identifier. | `src/lib.rs:1039` |
| sym-8d472de5e26dcc656903 | `pocketstation::SessionEndpointError::RegistrationStateUnavailable` | variant | Reported when the owning operation encounters registration state unavailable. | `src/lib.rs:1037` |
| sym-9d118a6edd603889293f | `pocketstation::SessionOperatorError::RegistrationStateUnavailable` | variant | Reported when the owning operation encounters registration state unavailable. | `src/lib.rs:1053` |
| sym-4c3c5e22f0c00d94f310 | `pocketstation::SessionRuntimeError::MissingMetricsSnapshot` | variant | Reported when the owning operation encounters missing metrics snapshot. | `src/lib.rs:1075` |
| sym-881a91f6824b2c0711e5 | `pocketstation::SessionSidecarError::RegistrationStateUnavailable` | variant | Reported when the owning operation encounters registration state unavailable. | `src/lib.rs:1047` |
| sym-0ce56be8b0b85f1f8e46 | `pocketstation::SessionSourceError::RegistrationStateUnavailable` | variant | Reported when the owning operation encounters registration state unavailable. | `src/lib.rs:1059` |
| sym-0c55cc392069b0f95d20 | `pocketstation::SessionStartErrorKind::Cancelled` | variant | Indicates that the operation was cancelled. | `src/lib.rs:1066` |
| sym-3b00b8e5c4228a0da0f3 | `pocketstation::SessionStartErrorKind::Engine` | variant | Selects engine behavior for `SessionStartErrorKind`. | `src/lib.rs:1065` |
| sym-07fa8d718e3bc10520ac | `pocketstation::SessionStartErrorKind::Host` | variant | Selects host behavior for `SessionStartErrorKind`. | `src/lib.rs:1064` |
| sym-c3819ef52a0e2c250cca | `pocketstation::SessionStartErrorKind::InvalidSelector` | variant | Selects invalid selector behavior for `SessionStartErrorKind`. | `src/lib.rs:1067` |
| sym-0b462c3f3a9c438e671c | `pocketstation::SessionStartErrorKind::Invariant` | variant | Selects invariant behavior for `SessionStartErrorKind`. | `src/lib.rs:1069` |
| sym-3fd71feba7910b81fbe6 | `pocketstation::SessionStartErrorKind::MissingRecordingConfiguration` | variant | Selects missing recording configuration behavior for `SessionStartErrorKind`. | `src/lib.rs:1068` |
| sym-ce725de7deeb2b02d02d | `pocketstation::SessionStopDisposition::AlreadyStopped` | variant | Indicates that the operation had already stopped. | `src/lib.rs:1081` |
| sym-493f6338374a28c2e225 | `pocketstation::SessionStopDisposition::Stopped` | variant | Indicates that the operation stopped normally. | `src/lib.rs:1080` |
| sym-172b87972a83540872e9 | `pocketstation::abi::extension::PksExtensionKind::Endpoint` | variant | Selects endpoint behavior for `PksExtensionKind`. | `src/abi/extension.rs:35` |
| sym-fae17803b87577e05dd6 | `pocketstation::abi::extension::PksExtensionKind::Operator` | variant | Selects operator behavior for `PksExtensionKind`. | `src/abi/extension.rs:34` |
| sym-4ab3d8a5880738f1694f | `pocketstation::abi::extension::PksExtensionKind::Source` | variant | Selects source behavior for `PksExtensionKind`. | `src/abi/extension.rs:33` |
| sym-57195f1473fa69a408c2 | `pocketstation::abi::extension::PksExtensionPortDirection::Input` | variant | Selects input behavior for `PksExtensionPortDirection`. | `src/abi/extension.rs:41` |
| sym-491654412e542b513980 | `pocketstation::abi::extension::PksExtensionPortDirection::Output` | variant | Selects output behavior for `PksExtensionPortDirection`. | `src/abi/extension.rs:42` |
| sym-c1fcb2ac1efedaf31c16 | `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| sym-5a63c1ed4326cf37196e | `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| sym-936ba5fd69c6e7b77c23 | `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| sym-cab1eff076e630226b06 | `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| sym-ccc76ce999251f56b0d4 | `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |
| sym-d590715654d95777c466 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidArgument` | variant | Identifies the invalid argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:89` |
| sym-939c5af9e88df8f8760b | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidHandle` | variant | Identifies the invalid handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:84` |
| sym-d2d226c409a9bb33fa18 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidLifecycleState` | variant | Identifies the invalid lifecycle state state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:91` |
| sym-0453fb20ed68426edc03 | `pocketstation::abi::session::abi::PksSessionStatusCode::InvalidStructSize` | variant | Identifies the invalid struct size state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:83` |
| sym-a60c1c41efe27d542e16 | `pocketstation::abi::session::abi::PksSessionStatusCode::MisalignedPointer` | variant | Identifies the misaligned pointer state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:88` |
| sym-213592b134be6e286ea7 | `pocketstation::abi::session::abi::PksSessionStatusCode::NoCapacity` | variant | Identifies the no capacity state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:86` |
| sym-24c3f427ad95bdf367ed | `pocketstation::abi::session::abi::PksSessionStatusCode::NullArgument` | variant | Identifies the null argument state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:81` |
| sym-e36d61b3a2c4f50cb3da | `pocketstation::abi::session::abi::PksSessionStatusCode::Ok` | variant | Indicates that the operation completed successfully. | `src/abi/session/abi.rs:80` |
| sym-dc58bedf22cc975fcc48 | `pocketstation::abi::session::abi::PksSessionStatusCode::StaleHandle` | variant | Identifies the stale handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:85` |
| sym-aa0c62cf4e33186d0878 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMajor` | variant | Identifies the unsupported ABI major state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:82` |
| sym-8443fe45e008bbfe7ac9 | `pocketstation::abi::session::abi::PksSessionStatusCode::UnsupportedAbiMinor` | variant | Identifies the unsupported ABI minor state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:96` |
| sym-cf6189ca826520e36070 | `pocketstation::abi::session::abi::PksSessionStatusCode::WouldBlock` | variant | Identifies the would block state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:92` |
| sym-9d85a11352d79f01f1d3 | `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | Selects allowed behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:232` |
| sym-fe9d9eac044ef1f58ceb | `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | Selects denied behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:233` |
| sym-ac58d63ca2febd6a15ca | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | Selects not applicable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:235` |
| sym-44e567dfd2c2f2b9b542 | `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | Selects not observable behavior for `ApplicationPolicyObservation`. | `src/capture/authorization.rs:234` |
| sym-8c1f79805150c9ccc20f | `pocketstation::capture::authorization::CaptureCapabilityState::Available` | variant | Identifies the available state or stage represented by `CaptureCapabilityState`. | `src/capture/authorization.rs:146` |
| sym-bca501cab1774fe943c1 | `pocketstation::capture::authorization::CaptureCapabilityState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/authorization.rs:147` |
| sym-66ee5f7f8569898ee0e5 | `pocketstation::capture::authorization::CaptureCapabilityState::Unsupported` | variant | Reports that the requested operation is unsupported. | `src/capture/authorization.rs:148` |
| sym-7419845e9f6b90c0b1da | `pocketstation::capture::authorization::CaptureError::BackendInit` | variant | Reported when the owning operation encounters backend init. | `src/capture/authorization.rs:294` |
| sym-7e170ddda34f65f4e3a5 | `pocketstation::capture::authorization::CaptureError::BackendSetupRequired` | variant | Reported when the owning operation encounters backend setup required. | `src/capture/authorization.rs:296` |
| sym-730ae118b5ac64faa504 | `pocketstation::capture::authorization::CaptureError::BackendStatus` | variant | Reported when the owning operation encounters backend status. | `src/capture/authorization.rs:303` |
| sym-e52fe39c34aba3580d17 | `pocketstation::capture::authorization::CaptureError::CaptureWorkerPanicked` | variant | Reported when the owning operation encounters capture worker panicked. | `src/capture/authorization.rs:316` |
| sym-0426859199cf142736d8 | `pocketstation::capture::authorization::CaptureError::InvalidRuntimeEventCapacity` | variant | Reported when the owning operation encounters invalid runtime event capacity. | `src/capture/authorization.rs:314` |
| sym-4228008a504ef2c88e48 | `pocketstation::capture::authorization::CaptureError::InvalidStreamCapacity` | variant | Reported when the owning operation encounters invalid stream capacity. | `src/capture/authorization.rs:312` |
| sym-cc6306620d673442212b | `pocketstation::capture::authorization::CaptureError::ModeUnsupported` | variant | Reported when the owning operation encounters mode unsupported. | `src/capture/authorization.rs:310` |
| sym-be0fe58f7d919ff27687 | `pocketstation::capture::authorization::CaptureError::NotSupported` | variant | Reported when the owning operation encounters not supported. | `src/capture/authorization.rs:292` |
| sym-56efe5b4102fb9c13d9c | `pocketstation::capture::authorization::CaptureError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:301` |
| sym-c4ccd46a71b412131e37 | `pocketstation::capture::authorization::CaptureError::SourceUnavailable` | variant | Reported when the owning operation encounters source unavailable. | `src/capture/authorization.rs:308` |
| sym-32a65ffe51c76a00fb56 | `pocketstation::capture::authorization::CaptureOpenOutcome::BackendFailed` | variant | Identifies the backend failed state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:286` |
| sym-579f00235bce31081f79 | `pocketstation::capture::authorization::CaptureOpenOutcome::NotAttempted` | variant | Identifies the not attempted state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:282` |
| sym-db271f950d22d660ccff | `pocketstation::capture::authorization::CaptureOpenOutcome::PermissionDenied` | variant | Reports that the required permission was denied. | `src/capture/authorization.rs:284` |
| sym-f02538797584a16ec25a | `pocketstation::capture::authorization::CaptureOpenOutcome::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:285` |
| sym-eb44f98338206b8a62cd | `pocketstation::capture::authorization::CaptureOpenOutcome::Succeeded` | variant | Identifies the succeeded state or stage represented by `CaptureOpenOutcome`. | `src/capture/authorization.rs:283` |
| sym-9dcb10a46b543c5d91e5 | `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | Selects exact application behavior for `CaptureScope`. | `src/capture/authorization.rs:249` |
| sym-f1578f1bc1c096ad11cb | `pocketstation::capture::authorization::CaptureScope::ExactInputDevice` | variant | Selects exact input device behavior for `CaptureScope`. | `src/capture/authorization.rs:250` |
| sym-2ba5ef3cc738a6d31f5b | `pocketstation::capture::authorization::CaptureScope::ExactOutputDevice` | variant | Selects exact output device behavior for `CaptureScope`. | `src/capture/authorization.rs:251` |
| sym-167981219e08deb8c616 | `pocketstation::capture::authorization::CaptureScope::SystemMix` | variant | Selects system mix behavior for `CaptureScope`. | `src/capture/authorization.rs:252` |
| sym-4c0055e7ec9408115cae | `pocketstation::capture::authorization::CaptureSessionGrant::Denied` | variant | Represents the denied alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:242` |
| sym-0cf6353da61564c0ca58 | `pocketstation::capture::authorization::CaptureSessionGrant::GrantedByExplicitSelection` | variant | Represents the granted by explicit selection alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:241` |
| sym-f4078600b4352349cee1 | `pocketstation::capture::authorization::CaptureSessionGrant::NotEvaluated` | variant | Represents the not evaluated alternative defined by `CaptureSessionGrant`. | `src/capture/authorization.rs:243` |
| sym-1fc87c8305313531efe6 | `pocketstation::capture::authorization::PermissionObservation::Allowed` | variant | Represents the allowed alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:154` |
| sym-facf3ff04a88c04996da | `pocketstation::capture::authorization::PermissionObservation::Denied` | variant | Represents the denied alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:155` |
| sym-4c790e42737adb1f1c14 | `pocketstation::capture::authorization::PermissionObservation::NotApplicable` | variant | Represents the not applicable alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:160` |
| sym-782721b32b6c86cb5925 | `pocketstation::capture::authorization::PermissionObservation::NotDetermined` | variant | Represents the not determined alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:157` |
| sym-bf4871e2e096aa97a696 | `pocketstation::capture::authorization::PermissionObservation::NotObservable` | variant | Represents the not observable alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:159` |
| sym-27e85161e59e0b1c0e49 | `pocketstation::capture::authorization::PermissionObservation::Restricted` | variant | Represents the restricted alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:156` |
| sym-feb9daafd9056de88d5f | `pocketstation::capture::authorization::PermissionObservation::Revoked` | variant | Represents the revoked alternative defined by `PermissionObservation`. | `src/capture/authorization.rs:158` |
| sym-2e095c149e555cbc616a | `pocketstation::capture::authorization::SourceIdentityStrength::ApplicationIdAndProcessId` | variant | Represents the application id and process identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:258` |
| sym-edb2cf96c6ea4ecf8b43 | `pocketstation::capture::authorization::SourceIdentityStrength::PlatformStableId` | variant | Represents the platform stable identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:262` |
| sym-8c6f42ee51ece6d0f894 | `pocketstation::capture::authorization::SourceIdentityStrength::ProcessId` | variant | Represents the process identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:260` |
| sym-889a572d97d2656b9cc3 | `pocketstation::capture::authorization::SourceIdentityStrength::StableApplicationId` | variant | Represents the stable application identifier alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:259` |
| sym-69a13b94218b11965e97 | `pocketstation::capture::authorization::SourceIdentityStrength::StableDeviceUid` | variant | Represents the stable device uid alternative defined by `SourceIdentityStrength`. | `src/capture/authorization.rs:261` |
| sym-969caf50e47805c6bd71 | `pocketstation::capture::events::CaptureRuntimeFailureClass::BackendClass` | variant | Reported when the owning operation encounters backend class. | `src/capture/events.rs:43` |
| sym-aeaa8597495f9676bb0f | `pocketstation::capture::events::CaptureRuntimeFailureClass::PlatformStatus` | variant | Reported when the owning operation encounters platform status. | `src/capture/events.rs:42` |
| sym-98b093cbdf297e085b22 | `pocketstation::capture::events::CaptureRuntimeFailureClass::SourceInstanceExited` | variant | Reported when the owning operation encounters source instance exited. | `src/capture/events.rs:41` |
| sym-beffba22af505abe54b3 | `pocketstation::capture::events::SourceLifecycleEventKind::PermissionChanged` | variant | Identifies the permission changed state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:28` |
| sym-91105137ebc1c7d4465e | `pocketstation::capture::events::SourceLifecycleEventKind::PermissionRevoked` | variant | Identifies the permission revoked state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:29` |
| sym-facb6cc1dba9854f36f7 | `pocketstation::capture::events::SourceLifecycleEventKind::ReplacementObserved` | variant | Identifies the replacement observed state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:27` |
| sym-bf37d3e4e581f1591d6d | `pocketstation::capture::events::SourceLifecycleEventKind::SourceReappeared` | variant | Identifies the source reappeared state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:30` |
| sym-6994b6a943f2c219e691 | `pocketstation::capture::events::SourceLifecycleEventKind::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:26` |
| sym-1a726f8b1e353bf99f9c | `pocketstation::capture::events::SourceRecoveryRequirement::ExplicitRediscoveryAndNewSession` | variant | Selects explicit rediscovery and new session behavior for `SourceRecoveryRequirement`. | `src/capture/events.rs:36` |
| sym-ce6e30cb56cd8fce9d43 | `pocketstation::capture::events::SourceRuntimeEvent::BackendFailure` | variant | Identifies the backend failure state or stage represented by `SourceRuntimeEvent`. | `src/capture/events.rs:60` |
| sym-8cf3f9719ba11b08e27c | `pocketstation::capture::events::SourceRuntimeEvent::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `SourceRuntimeEvent`. | `src/capture/events.rs:54` |
| sym-6d267a367b8529aaf282 | `pocketstation::capture::events::SourceRuntimeEventDelivery::DroppedFull` | variant | Identifies the dropped full state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:98` |
| sym-34a0f33375030a433283 | `pocketstation::capture::events::SourceRuntimeEventDelivery::DroppedOversized` | variant | Identifies the dropped oversized state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:99` |
| sym-aff6e2f4402dcf0a8512 | `pocketstation::capture::events::SourceRuntimeEventDelivery::Enqueued` | variant | Identifies the enqueued state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:97` |
| sym-cbb37886ab37c4d7218a | `pocketstation::capture::events::SourceRuntimeEventDelivery::ReceiverClosed` | variant | Identifies the receiver closed state or stage represented by `SourceRuntimeEventDelivery`. | `src/capture/events.rs:100` |
| sym-c92b7f1b545efbc07b04 | `pocketstation::capture::events::SourceRuntimeEventReceive::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/capture/events.rs:107` |
| sym-26bac4ec3ade30618581 | `pocketstation::capture::events::SourceRuntimeEventReceive::Empty` | variant | Represents an empty value or collection. | `src/capture/events.rs:106` |
| sym-a8382aebca854cb96ab3 | `pocketstation::capture::events::SourceRuntimeEventReceive::Event` | variant | Identifies the event state or stage represented by `SourceRuntimeEventReceive`. | `src/capture/events.rs:105` |
| sym-98d28fdd53d6bb46ddba | `pocketstation::capture::frame_stream::CapturedFrameDelivery::Delivered` | variant | Identifies the delivered state or stage represented by `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:11` |
| sym-b2993d30ab19dac99742 | `pocketstation::capture::frame_stream::CapturedFrameDelivery::DiscardedBeforeStart` | variant | Identifies the discarded before start state or stage represented by `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:13` |
| sym-1bbeb9dca00e11e850ad | `pocketstation::capture::frame_stream::CapturedFrameDelivery::DroppedNewest` | variant | Identifies the dropped newest state or stage represented by `CapturedFrameDelivery`. | `src/capture/frame_stream.rs:12` |
| sym-34e19bca0beac399e03d | `pocketstation::capture::identity::SourceKind::Application` | variant | Selects application behavior for `SourceKind`. | `src/capture/identity.rs:10` |
| sym-a60a7cee229b99be05c3 | `pocketstation::capture::identity::SourceKind::InputDevice` | variant | Selects input device behavior for `SourceKind`. | `src/capture/identity.rs:12` |
| sym-fadc6b6ce6bac5a1875f | `pocketstation::capture::identity::SourceKind::OutputDevice` | variant | Selects output device behavior for `SourceKind`. | `src/capture/identity.rs:11` |
| sym-cb525c112b0a09a38e7c | `pocketstation::capture::identity::SourceKind::SystemMix` | variant | Selects system mix behavior for `SourceKind`. | `src/capture/identity.rs:13` |
| sym-f7d32e6d314edd52bf02 | `pocketstation::capture::identity::SourceState::Available` | variant | Identifies the available state or stage represented by `SourceState`. | `src/capture/identity.rs:18` |
| sym-58722953872513e88bf1 | `pocketstation::capture::identity::SourceState::PermissionBlocked` | variant | Identifies the permission blocked state or stage represented by `SourceState`. | `src/capture/identity.rs:22` |
| sym-4f64554b739e141f483d | `pocketstation::capture::identity::SourceState::Playing` | variant | Identifies the playing state or stage represented by `SourceState`. | `src/capture/identity.rs:19` |
| sym-cefc810bfb9d7bc07b99 | `pocketstation::capture::identity::SourceState::Silent` | variant | Identifies the silent state or stage represented by `SourceState`. | `src/capture/identity.rs:20` |
| sym-22071c6eee236a952928 | `pocketstation::capture::identity::SourceState::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/capture/identity.rs:21` |
| sym-0f0267af9491f0c7ef37 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition::Disappeared` | variant | Represents the disappeared alternative defined by `SourceGenerationTransition`. | `src/capture/lifecycle_registry.rs:9` |
| sym-09574748a70f2495f2c6 | `pocketstation::capture::lifecycle_registry::SourceGenerationTransition::Reappeared` | variant | Represents the reappeared alternative defined by `SourceGenerationTransition`. | `src/capture/lifecycle_registry.rs:13` |
| sym-6f42880369b925369f07 | `pocketstation::capture::query::SourceQuery::Any` | variant | Represents the any alternative defined by `SourceQuery`. | `src/capture/query.rs:14` |
| sym-8ce7161640aecfa7bae9 | `pocketstation::capture::query::SourceQuery::App` | variant | Represents the app alternative defined by `SourceQuery`. | `src/capture/query.rs:15` |
| sym-98bde70368fe1a2f0b22 | `pocketstation::capture::query::SourceQuery::ByKind` | variant | Represents the by kind alternative defined by `SourceQuery`. | `src/capture/query.rs:16` |
| sym-140ec03a9371b8851492 | `pocketstation::capture::query::SourceQuery::ByStableKey` | variant | Represents the by stable key alternative defined by `SourceQuery`. | `src/capture/query.rs:17` |
| sym-0be71e9e5905408da61d | `pocketstation::capture::query::SourceQuery::Playing` | variant | Represents the playing alternative defined by `SourceQuery`. | `src/capture/query.rs:18` |
| sym-d42a22680ee0b9ca4285 | `pocketstation::capture::selection::CaptureMode::Application` | variant | Selects application behavior for `CaptureMode`. | `src/capture/selection.rs:19` |
| sym-f1bfc6659f95bbbf6880 | `pocketstation::capture::selection::CaptureMode::ExactApplication` | variant | Selects exact application behavior for `CaptureMode`. | `src/capture/selection.rs:21` |
| sym-d1cfb38049e4411527fe | `pocketstation::capture::selection::CaptureMode::ExactApplicationStable` | variant | Selects exact application stable behavior for `CaptureMode`. | `src/capture/selection.rs:25` |
| sym-39bd17142c4d063865ca | `pocketstation::capture::selection::CaptureMode::InputDevice` | variant | Selects input device behavior for `CaptureMode`. | `src/capture/selection.rs:28` |
| sym-6abc455570301c9aedac | `pocketstation::capture::selection::CaptureMode::Process` | variant | Selects process behavior for `CaptureMode`. | `src/capture/selection.rs:20` |
| sym-24f5991eb6887d1f41e3 | `pocketstation::capture::selection::CaptureMode::SystemMix` | variant | Selects system mix behavior for `CaptureMode`. | `src/capture/selection.rs:18` |
| sym-0b66a6ade0aad1898e5c | `pocketstation::capture::selection::InputDeviceSelector::Default` | variant | Selects default behavior for `InputDeviceSelector`. | `src/capture/selection.rs:11` |
| sym-2696d75798bbf73690c2 | `pocketstation::capture::selection::InputDeviceSelector::StableId` | variant | Selects stable identifier behavior for `InputDeviceSelector`. | `src/capture/selection.rs:12` |
| sym-f65d932faba573db0e15 | `pocketstation::capture::selection::ProcessTreeScope::ApplicationIdentity` | variant | Selects application identity behavior for `ProcessTreeScope`. | `src/capture/selection.rs:86` |
| sym-fdd4ca59639ac43beb96 | `pocketstation::capture::selection::ProcessTreeScope::NotApplicable` | variant | Selects not applicable behavior for `ProcessTreeScope`. | `src/capture/selection.rs:87` |
| sym-3d6087bfd404c66cf083 | `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessAndDescendants` | variant | Selects selected process and descendants behavior for `ProcessTreeScope`. | `src/capture/selection.rs:85` |
| sym-49dd76bbf53227a3c9c6 | `pocketstation::capture::selection::ProcessTreeScope::SelectedProcessOnly` | variant | Selects selected process only behavior for `ProcessTreeScope`. | `src/capture/selection.rs:84` |
| sym-f6d9f954df08302c5cf7 | `pocketstation::capture::selection::SelectorPersistenceScope::ApplicationIdentity` | variant | Selects application identity behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:75` |
| sym-5013eed11364753279e7 | `pocketstation::capture::selection::SelectorPersistenceScope::DeviceIdentity` | variant | Selects device identity behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:76` |
| sym-e4c3e0bd69cf7c2e99a5 | `pocketstation::capture::selection::SelectorPersistenceScope::PlatformIdentity` | variant | Selects platform identity behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:78` |
| sym-8e61efafed808673f4bd | `pocketstation::capture::selection::SelectorPersistenceScope::ProcessLifetime` | variant | Selects process lifetime behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:74` |
| sym-68071bacdb5bcb8d7708 | `pocketstation::capture::selection::SelectorPersistenceScope::SessionDefaultDevice` | variant | Selects session default device behavior for `SelectorPersistenceScope`. | `src/capture/selection.rs:77` |
| sym-f8f8a762b70f10a33aa9 | `pocketstation::capture::timeline::CaptureSampleTimelineError::MixedAdvanceModes` | variant | Reported when the owning operation encounters mixed advance modes. | `src/capture/timeline.rs:42` |
| sym-fb7636a50ffcdefec6d9 | `pocketstation::capture::timeline::CaptureSampleTimelineError::SourcePositionMovedBackward` | variant | Reported when the owning operation encounters source position moved backward. | `src/capture/timeline.rs:44` |
| sym-b3d4507d72b91d64cfff | `pocketstation::capture::timeline::CaptureSampleTimelineError::SourcePositionOverflow` | variant | Reported when the owning operation encounters source position overflow. | `src/capture/timeline.rs:43` |
| sym-10a4cf032c94e76b3506 | `pocketstation::codec::decoder::OpusDecodeError::FrameDurationExceedsConfiguredMaximum` | variant | Reported when the owning operation encounters frame duration exceeds configured maximum. | `src/codec/decoder.rs:29` |
| sym-f417caaef2042123ce64 | `pocketstation::codec::decoder::OpusDecodeError::Opus` | variant | Reported when the owning operation encounters opus. | `src/codec/decoder.rs:34` |
| sym-ecc00b68a32277246721 | `pocketstation::codec::encoder::OpusApplication::Audio` | variant | Optimised for audio quality (music/broadcast). | `src/codec/encoder.rs:64` |
| sym-1652077fd260d98f177c | `pocketstation::codec::encoder::OpusApplication::LowDelay` | variant | Optimised for low algorithmic delay. Use for real-time voice agents. | `src/codec/encoder.rs:62` |
| sym-cbf6c012d02d0ae69f39 | `pocketstation::codec::encoder::OpusApplication::Voip` | variant | Optimised for voice (VOIP). Default for PocketStation broadcast. | `src/codec/encoder.rs:60` |
| sym-9c6b15493cde15faf095 | `pocketstation::codec::encoder::OpusChannels::Mono` | variant | Represents the mono alternative defined by `OpusChannels`. | `src/codec/encoder.rs:28` |
| sym-3c73cd164c2199acfa05 | `pocketstation::codec::encoder::OpusChannels::Stereo` | variant | Represents the stereo alternative defined by `OpusChannels`. | `src/codec/encoder.rs:29` |
| sym-377c0738bf43a1977a97 | `pocketstation::codec::encoder::OpusEncodeError::InvalidFrameSampleCount` | variant | Reported when the owning operation encounters invalid frame sample count. | `src/codec/encoder.rs:135` |
| sym-f8c0585f37697c747541 | `pocketstation::codec::encoder::OpusEncodeError::Opus` | variant | Reported when the owning operation encounters opus. | `src/codec/encoder.rs:141` |
| sym-e932cc0c78380cd86c51 | `pocketstation::codec::encoder::OpusFrameDuration::Ms10` | variant | Represents the ms10 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:8` |
| sym-18f0d7c4b9b79ff0cdc2 | `pocketstation::codec::encoder::OpusFrameDuration::Ms20` | variant | Represents the ms20 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:9` |
| sym-8cf875a3f62bd248876e | `pocketstation::codec::encoder::OpusFrameDuration::Ms40` | variant | Represents the ms40 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:10` |
| sym-a27eab0d01739805fe30 | `pocketstation::codec::encoder::OpusFrameDuration::Ms60` | variant | Represents the ms60 alternative defined by `OpusFrameDuration`. | `src/codec/encoder.rs:11` |
| sym-3c06e15db5dea6f25652 | `pocketstation::codec::encoder::OpusSampleRate::Hz48000` | variant | Represents the hz48000 alternative defined by `OpusSampleRate`. | `src/codec/encoder.rs:45` |
| sym-77eec08117142051b187 | `pocketstation::codec::profile::StreamProfile::BroadcastStereo20ms` | variant | Represents the broadcast stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:16` |
| sym-3f3ddb38bbd3cd82a214 | `pocketstation::codec::profile::StreamProfile::HifiStereo20ms` | variant | Represents the hifi stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:17` |
| sym-bd0cd0fdb9cfa1a50c72 | `pocketstation::codec::profile::StreamProfile::MusicStereo10ms` | variant | Represents the music stereo10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:15` |
| sym-ca2cefc05155eb7ef074 | `pocketstation::codec::profile::StreamProfile::MusicStereo20ms` | variant | Represents the music stereo20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:14` |
| sym-8a5da12da7e042a23279 | `pocketstation::codec::profile::StreamProfile::VoiceAgentMono10ms` | variant | Represents the voice agent mono10ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:13` |
| sym-abccff0cbe71b4b13565 | `pocketstation::codec::profile::StreamProfile::VoiceMono20ms` | variant | Represents the voice mono20ms alternative defined by `StreamProfile`. | `src/codec/profile.rs:12` |
| sym-cba1a7b0f6d0a94c16eb | `pocketstation::conformance::ObservedEndpointError::ConnectorDeclaration` | variant | Reported when the owning operation encounters connector declaration. | `src/conformance.rs:355` |
| sym-fdf1902a10d1862ecce4 | `pocketstation::conformance::ObservedEndpointError::ConnectorRegistration` | variant | Reported when the owning operation encounters connector registration. | `src/conformance.rs:353` |
| sym-ff1cd66fdeedb3841278 | `pocketstation::conformance::ObservedEndpointError::Contract` | variant | Reported when the owning operation encounters contract. | `src/conformance.rs:347` |
| sym-f8d6569b2869c4a909d2 | `pocketstation::conformance::ObservedEndpointError::Declaration` | variant | Reported when the owning operation encounters declaration. | `src/conformance.rs:349` |
| sym-09cfa8127875b2fbfa73 | `pocketstation::conformance::ObservedEndpointError::Registration` | variant | Reported when the owning operation encounters registration. | `src/conformance.rs:351` |
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
| sym-adf9ed3e536945535d39 | `pocketstation::connector::error::ConnectorErrorStage::Prepare` | variant | Identifies the prepare state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:62` |
| sym-68f22b9efb7f3cb91142 | `pocketstation::connector::error::ConnectorErrorStage::Readiness` | variant | Identifies the readiness state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:64` |
| sym-53fab4d7135b4d99ef15 | `pocketstation::connector::error::ConnectorErrorStage::Retry` | variant | Identifies the retry state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:66` |
| sym-b6a46d264b4359a6f4be | `pocketstation::connector::error::ConnectorErrorStage::Shutdown` | variant | Identifies the shutdown state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:67` |
| sym-533324d979a18246cea9 | `pocketstation::connector::error::ConnectorErrorStage::Startup` | variant | Identifies the startup state or stage represented by `ConnectorErrorStage`. | `src/connector/error.rs:63` |
| sym-7813c6ed750ca969129b | `pocketstation::connector::error::ConnectorRetryability::Never` | variant | Selects never behavior for `ConnectorRetryability`. | `src/connector/error.rs:73` |
| sym-7f7294a34264cdb5e433 | `pocketstation::connector::error::ConnectorRetryability::RetryAfterReconfiguration` | variant | Selects retry after reconfiguration behavior for `ConnectorRetryability`. | `src/connector/error.rs:75` |
| sym-dacec76584f24ca391cd | `pocketstation::connector::error::ConnectorRetryability::Retryable` | variant | Selects retryable behavior for `ConnectorRetryability`. | `src/connector/error.rs:74` |
| sym-a7f6d0fc9876a352846b | `pocketstation::connector::manifest::ConnectorManifestError::DuplicateManifestEntry` | variant | Reported when the owning operation encounters duplicate manifest entry. | `src/connector/manifest.rs:253` |
| sym-13329d447a2988d255f0 | `pocketstation::connector::manifest::ConnectorManifestError::EmptyOperatorId` | variant | Reported when the owning operation encounters empty operator identifier. | `src/connector/manifest.rs:237` |
| sym-84434a354df82d86fca0 | `pocketstation::connector::manifest::ConnectorManifestError::InvalidManifestEntry` | variant | Reported when the owning operation encounters invalid manifest entry. | `src/connector/manifest.rs:247` |
| sym-7a16648de40ae3cdf9c9 | `pocketstation::connector::manifest::ConnectorManifestError::InvalidManifestRevision` | variant | Reported when the owning operation encounters invalid manifest revision. | `src/connector/manifest.rs:235` |
| sym-4d53ebe7d9d91bcb9dbe | `pocketstation::connector::manifest::ConnectorManifestError::InvalidPackageVersion` | variant | Reported when the owning operation encounters invalid package version. | `src/connector/manifest.rs:239` |
| sym-1c6ad3a11014a758a9d0 | `pocketstation::connector::manifest::ConnectorManifestError::ManifestEntryTooLarge` | variant | Reported when the owning operation encounters manifest entry too large. | `src/connector/manifest.rs:249` |
| sym-73d62d950ad1ff4f57a1 | `pocketstation::connector::manifest::ConnectorManifestError::MissingInputPort` | variant | Reported when the owning operation encounters missing input port. | `src/connector/manifest.rs:241` |
| sym-b878b5cd20870f0ae531 | `pocketstation::connector::manifest::ConnectorManifestError::OutputPortNotSupported` | variant | Reported when the owning operation encounters output port not supported. | `src/connector/manifest.rs:243` |
| sym-d9143bd9c8290e1ff809 | `pocketstation::connector::manifest::ConnectorManifestError::RealtimeExecutionForbidden` | variant | Reported when the owning operation encounters realtime execution forbidden. | `src/connector/manifest.rs:245` |
| sym-7f1c7a9a357142006ebb | `pocketstation::connector::manifest::ConnectorManifestError::TooManyManifestEntries` | variant | Reported when the owning operation encounters too many manifest entries. | `src/connector/manifest.rs:251` |
| sym-1e79fcb3fa749ae34fd0 | `pocketstation::connector::manifest::ConnectorManifestError::UnsupportedApiRevision` | variant | Reported when the owning operation encounters unsupported API revision. | `src/connector/manifest.rs:233` |
| sym-490c5eaa9665b5dcd6f4 | `pocketstation::connector::observations::ConnectorObservationError::StateUnavailable` | variant | Reported when the owning operation encounters state unavailable. | `src/connector/observations.rs:177` |
| sym-f0c0accf63a9b2bb0295 | `pocketstation::connector::readiness::ConnectorReadinessPolicyError::InvalidDeadline` | variant | Selects invalid deadline behavior for `ConnectorReadinessPolicyError`. | `src/connector/readiness.rs:63` |
| sym-c27864399aa21b94e3ff | `pocketstation::connector::readiness::ConnectorReadinessPolicyError::InvalidThreshold` | variant | Selects invalid threshold behavior for `ConnectorReadinessPolicyError`. | `src/connector/readiness.rs:65` |
| sym-53149f3934114d3083ca | `pocketstation::connector::status::ConnectorDeliveryReadiness::NotReady` | variant | Identifies the not ready state or stage represented by `ConnectorDeliveryReadiness`. | `src/connector/status.rs:5` |
| sym-9f2d9aabba06d7a25f0f | `pocketstation::connector::status::ConnectorDeliveryReadiness::Ready` | variant | Identifies the ready state or stage represented by `ConnectorDeliveryReadiness`. | `src/connector/status.rs:6` |
| sym-d628f819720f0dd5fbff | `pocketstation::connector::status::ConnectorHealth::Degraded` | variant | Represents the degraded alternative defined by `ConnectorHealth`. | `src/connector/status.rs:19` |
| sym-885d2a7d7e409e050b9f | `pocketstation::connector::status::ConnectorHealth::Healthy` | variant | Represents the healthy alternative defined by `ConnectorHealth`. | `src/connector/status.rs:18` |
| sym-4151f4a148ea385f2940 | `pocketstation::connector::status::ConnectorRecovery::Idle` | variant | Represents the idle alternative defined by `ConnectorRecovery`. | `src/connector/status.rs:25` |
| sym-f0abcb9454b2df6d9412 | `pocketstation::connector::status::ConnectorRecovery::Reconnecting` | variant | Represents the reconnecting alternative defined by `ConnectorRecovery`. | `src/connector/status.rs:26` |
| sym-f8b5783bafd837c8d4ac | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidConnectorId` | variant | Reported when the owning operation encounters invalid connector identifier. | `src/connector/transport.rs:596` |
| sym-6a7c27cf7fdd1cfb87ab | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidHeaderSize` | variant | Reported when the owning operation encounters invalid header size. | `src/connector/transport.rs:582` |
| sym-3b92dcfafecdd8270dd8 | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidLineage` | variant | Reported when the owning operation encounters invalid lineage. | `src/connector/transport.rs:594` |
| sym-2b394d4d87f3837b7281 | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidMagic` | variant | Reported when the owning operation encounters invalid magic. | `src/connector/transport.rs:576` |
| sym-42b9190180a5e0f14ee9 | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidPortName` | variant | Reported when the owning operation encounters invalid port name. | `src/connector/transport.rs:586` |
| sym-1f55b44e769f2327ef0e | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidSampleCount` | variant | Reported when the owning operation encounters invalid sample count. | `src/connector/transport.rs:592` |
| sym-2ae773293b27d947bac6 | `pocketstation::connector::transport::ConnectorAudioRecordError::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/connector/transport.rs:588` |
| sym-fb876ac8780227e8f828 | `pocketstation::connector::transport::ConnectorAudioRecordError::LengthOverflow` | variant | Reported when the owning operation encounters length overflow. | `src/connector/transport.rs:598` |
| sym-0b04cf8e6b37dd52bbc1 | `pocketstation::connector::transport::ConnectorAudioRecordError::NotAudio` | variant | Reported when the owning operation encounters not audio. | `src/connector/transport.rs:570` |
| sym-6016147c646f1bfbbac3 | `pocketstation::connector::transport::ConnectorAudioRecordError::ReservedFieldSet` | variant | Reported when the owning operation encounters reserved field set. | `src/connector/transport.rs:584` |
| sym-e83863b3e1f33f3485f6 | `pocketstation::connector::transport::ConnectorAudioRecordError::TrailingBytes` | variant | Reported when the owning operation encounters trailing bytes. | `src/connector/transport.rs:574` |
| sym-bb03decc2a89ef8b10fa | `pocketstation::connector::transport::ConnectorAudioRecordError::Truncated` | variant | Reported when the owning operation encounters truncated. | `src/connector/transport.rs:572` |
| sym-4732076fec12bd49d7fc | `pocketstation::connector::transport::ConnectorAudioRecordError::UnsupportedMajor` | variant | Reported when the owning operation encounters unsupported major. | `src/connector/transport.rs:578` |
| sym-1bc46daa09062e865b6d | `pocketstation::connector::transport::ConnectorAudioRecordError::UnsupportedMinor` | variant | Reported when the owning operation encounters unsupported minor. | `src/connector/transport.rs:580` |
| sym-572dcac1f7053872555b | `pocketstation::connector::transport::ConnectorAudioRecordError::UnsupportedSampleFormat` | variant | Reported when the owning operation encounters unsupported sample format. | `src/connector/transport.rs:590` |
| sym-d609d8af7a0a9e50f0cf | `pocketstation::connector::transport::ConnectorConfigurationRecordError::DuplicateField` | variant | Reported when the owning operation encounters duplicate field. | `src/connector/transport.rs:269` |
| sym-a5c48ea38d88c3c96862 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::InvalidFieldName` | variant | Reported when the owning operation encounters invalid field name. | `src/connector/transport.rs:267` |
| sym-b3a27193a4c7a5d9d39c | `pocketstation::connector::transport::ConnectorConfigurationRecordError::InvalidMagic` | variant | Reported when the owning operation encounters invalid magic. | `src/connector/transport.rs:257` |
| sym-f86d99abeb56138fe06c | `pocketstation::connector::transport::ConnectorConfigurationRecordError::InvalidValue` | variant | Reported when the owning operation encounters invalid value. | `src/connector/transport.rs:273` |
| sym-eef56010553ec6c0779e | `pocketstation::connector::transport::ConnectorConfigurationRecordError::LengthOverflow` | variant | Reported when the owning operation encounters length overflow. | `src/connector/transport.rs:277` |
| sym-f36574e334737648cd47 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::ReservedFieldSet` | variant | Reported when the owning operation encounters reserved field set. | `src/connector/transport.rs:263` |
| sym-18f5fdc29e84e576ea8d | `pocketstation::connector::transport::ConnectorConfigurationRecordError::TooManyFields` | variant | Reported when the owning operation encounters too many fields. | `src/connector/transport.rs:265` |
| sym-4eff07b3859087fecc00 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::TrailingBytes` | variant | Reported when the owning operation encounters trailing bytes. | `src/connector/transport.rs:255` |
| sym-00a4a6c71ebaf6ee605c | `pocketstation::connector::transport::ConnectorConfigurationRecordError::Truncated` | variant | Reported when the owning operation encounters truncated. | `src/connector/transport.rs:253` |
| sym-2f489995d77b02d0f3ca | `pocketstation::connector::transport::ConnectorConfigurationRecordError::UnknownValueKind` | variant | Reported when the owning operation encounters unknown value kind. | `src/connector/transport.rs:275` |
| sym-33cceb2f5cbff68aaa12 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::UnsupportedMajor` | variant | Reported when the owning operation encounters unsupported major. | `src/connector/transport.rs:259` |
| sym-5ada5188c7b207f130b0 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::UnsupportedMinor` | variant | Reported when the owning operation encounters unsupported minor. | `src/connector/transport.rs:261` |
| sym-aeb5e68296869c9d0527 | `pocketstation::connector::transport::ConnectorConfigurationRecordError::ValueTooLarge` | variant | Reported when the owning operation encounters value too large. | `src/connector/transport.rs:271` |
| sym-a82cf068f598e438c42f | `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome::Delivered` | variant | Identifies the delivered state or stage represented by `ConnectorDeliveryOutcome`. | `src/connector/worker/driver.rs:84` |
| sym-1e65fb70d41526749841 | `pocketstation::connector::worker::driver::ConnectorDeliveryOutcome::Dropped` | variant | Identifies the dropped state or stage represented by `ConnectorDeliveryOutcome`. | `src/connector/worker/driver.rs:85` |
| sym-d4d6f0df4998d001083f | `pocketstation::connector::worker::driver::ConnectorItem::Audio` | variant | Represents the audio alternative defined by `ConnectorItem`. | `src/connector/worker/driver.rs:63` |
| sym-c8e38dbaf54892dfd49c | `pocketstation::connector::worker::driver::ConnectorItem::Signal` | variant | Represents the signal alternative defined by `ConnectorItem`. | `src/connector/worker/driver.rs:67` |
| sym-7f4aeca62674c8943a40 | `pocketstation::endpoint::contract::EndpointReceiver::Audio` | variant | Represents the audio alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:146` |
| sym-8737f9eed43a76550ab1 | `pocketstation::endpoint::contract::EndpointReceiver::Signal` | variant | Represents the signal alternative defined by `EndpointReceiver`. | `src/endpoint/contract.rs:150` |
| sym-5599686d8ec27b98e6eb | `pocketstation::endpoint::identity::EndpointPreparationGroup::Route` | variant | Represents the route alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:24` |
| sym-54ca783c4fba6a4196ae | `pocketstation::endpoint::identity::EndpointPreparationGroup::Shared` | variant | Represents the shared alternative defined by `EndpointPreparationGroup`. | `src/endpoint/identity.rs:25` |
| sym-e4a04729c579cf3f97a7 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::BatchCapacityTooLarge` | variant | Reported when the owning operation encounters batch capacity too large. | `src/endpoint/polled_audio_driver.rs:50` |
| sym-5173a65b36933cba104d | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::LeaseCapacityTooLarge` | variant | Reported when the owning operation encounters lease capacity too large. | `src/endpoint/polled_audio_driver.rs:52` |
| sym-0f105d20b5b705c6e529 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::QueueCapacityTooLarge` | variant | Reported when the owning operation encounters queue capacity too large. | `src/endpoint/polled_audio_driver.rs:48` |
| sym-0eaa6c4838504decfc4b | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroBatchCapacity` | variant | Reported when the owning operation encounters zero batch capacity. | `src/endpoint/polled_audio_driver.rs:44` |
| sym-cd965e6c8bb2ad90f7c5 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroLeaseCapacity` | variant | Reported when the owning operation encounters zero lease capacity. | `src/endpoint/polled_audio_driver.rs:46` |
| sym-2d771b1fb2e321dffc44 | `pocketstation::endpoint::polled_audio_driver::PolledAudioEndpointConfigError::ZeroQueueCapacity` | variant | Reported when the owning operation encounters zero queue capacity. | `src/endpoint/polled_audio_driver.rs:42` |
| sym-93cf3d8f2ccf7fc41ae1 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::Empty` | variant | Represents an empty value or collection. | `src/endpoint/polled_audio_driver.rs:76` |
| sym-e21a258537edfaf06b50 | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::LeaseCapacityExhausted` | variant | Reported when the owning operation encounters lease capacity exhausted. | `src/endpoint/polled_audio_driver.rs:78` |
| sym-0143da2bc75c3b5a49fd | `pocketstation::endpoint::polled_audio_driver::PolledAudioPollError::StatePoisoned` | variant | Reported when the owning operation encounters state poisoned. | `src/endpoint/polled_audio_driver.rs:80` |
| sym-5f1dc7ba77d614c8ad63 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::Duplicate` | variant | Reported when the owning operation encounters duplicate. | `src/endpoint/registry.rs:24` |
| sym-3b8dcd186b5884562b73 | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyNodeTypeId` | variant | Reported when the owning operation encounters empty node type identifier. | `src/endpoint/registry.rs:20` |
| sym-64992d6a343255948ced | `pocketstation::endpoint::registry::EndpointDriverRegistryError::EmptyOperatorId` | variant | Reported when the owning operation encounters empty operator identifier. | `src/endpoint/registry.rs:18` |
| sym-e0d277825d6d0b9cc68d | `pocketstation::endpoint::registry::EndpointDriverRegistryError::OperatorNodeTypeConflict` | variant | Reported when the owning operation encounters operator node type conflict. | `src/endpoint/registry.rs:31` |
| sym-ddb0f269a5665d3455c9 | `pocketstation::endpoint::registry::EndpointPrepareError::Driver` | variant | Reported when the owning operation encounters driver. | `src/endpoint/registry.rs:50` |
| sym-e291107299a9bbb2ca3a | `pocketstation::endpoint::registry::EndpointPrepareError::EmptyBatch` | variant | Reported when the owning operation encounters empty batch. | `src/endpoint/registry.rs:41` |
| sym-7d8561c8b8023fabebba | `pocketstation::endpoint::registry::EndpointPrepareError::NotRegistered` | variant | Reported when the owning operation encounters not registered. | `src/endpoint/registry.rs:45` |
| sym-6a6d38d4c601427330af | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Never` | variant | Selects never behavior for `EndpointFailureRetryability`. | `src/endpoint/runtime.rs:167` |
| sym-27bebbab40ccd0f74b41 | `pocketstation::endpoint::runtime::EndpointFailureRetryability::ReconfigurationRequired` | variant | Selects reconfiguration required behavior for `EndpointFailureRetryability`. | `src/endpoint/runtime.rs:169` |
| sym-73adaff1abb76f2f712d | `pocketstation::endpoint::runtime::EndpointFailureRetryability::Retryable` | variant | Selects retryable behavior for `EndpointFailureRetryability`. | `src/endpoint/runtime.rs:168` |
| sym-4eedcbb89c8816aa1b07 | `pocketstation::endpoint::runtime::EndpointFailureStage::CancelPreparation` | variant | Identifies the cancel preparation state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:158` |
| sym-849a56a51ddca22ab870 | `pocketstation::endpoint::runtime::EndpointFailureStage::JoinFinalize` | variant | Identifies the join finalize state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:161` |
| sym-50657833fb0f41682c40 | `pocketstation::endpoint::runtime::EndpointFailureStage::Prepare` | variant | Identifies the prepare state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:157` |
| sym-a13f3acad67ef72304b0 | `pocketstation::endpoint::runtime::EndpointFailureStage::RequestStop` | variant | Identifies the request stop state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:160` |
| sym-26861b1620f8b9bde7a2 | `pocketstation::endpoint::runtime::EndpointFailureStage::Start` | variant | Identifies the start state or stage represented by `EndpointFailureStage`. | `src/endpoint/runtime.rs:159` |
| sym-02aa77e01bbd1b0c0b02 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Signal` | variant | A typed signal whose detailed provenance is carried by `SignalLineage`. | `src/endpoint/runtime.rs:34` |
| sym-692120f8084c94070506 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Source` | variant | Represents the source alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:35` |
| sym-ef6b2d7325398598ff93 | `pocketstation::endpoint::runtime::EndpointInputOrigin::Stem` | variant | Represents the stem alternative defined by `EndpointInputOrigin`. | `src/endpoint/runtime.rs:32` |
| sym-0cc2735a1212157cc2f8 | `pocketstation::endpoint::runtime::EndpointShutdownMode::Abort` | variant | Selects abort behavior for `EndpointShutdownMode`. | `src/endpoint/runtime.rs:358` |
| sym-3b5c0f7180057e58da21 | `pocketstation::endpoint::runtime::EndpointShutdownMode::Drain` | variant | Selects drain behavior for `EndpointShutdownMode`. | `src/endpoint/runtime.rs:357` |
| sym-316c920624b4f5b2d208 | `pocketstation::endpoint::runtime::EndpointStartFailureCause::Driver` | variant | Reported when the owning operation encounters driver. | `src/endpoint/runtime.rs:440` |
| sym-2d45b4984613219b1c31 | `pocketstation::endpoint::runtime::EndpointStartFailureCause::GateAlreadyOpen` | variant | Reported when the owning operation encounters gate already open. | `src/endpoint/runtime.rs:439` |
| sym-bd67e35bf3477713d4ed | `pocketstation::frame::audio::AudioFrameBuildError::MisalignedSamples` | variant | Reported when the owning operation encounters misaligned samples. | `src/frame/audio.rs:57` |
| sym-2f30cf24c8cc92e83cc7 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroChannels` | variant | Reported when the owning operation encounters zero channels. | `src/frame/audio.rs:55` |
| sym-b059c49197ff40cdabb6 | `pocketstation::frame::audio::AudioFrameBuildError::ZeroSampleRate` | variant | Reported when the owning operation encounters zero sample rate. | `src/frame/audio.rs:53` |
| sym-05582883e7c54d857255 | `pocketstation::frame::audio::FrameLineageError::SequenceNumber` | variant | Reported when the owning operation encounters sequence number. | `src/frame/audio.rs:254` |
| sym-4f15d6eda96842b8d846 | `pocketstation::frame::audio::FrameLineageError::Source` | variant | Reported when the owning operation encounters source. | `src/frame/audio.rs:252` |
| sym-1a41011a0c9cf77b1b3e | `pocketstation::frame::audio::FrameLineageError::Timestamp` | variant | Reported when the owning operation encounters timestamp. | `src/frame/audio.rs:256` |
| sym-ba6ef304059f1dc62977 | `pocketstation::frame::audio::SampleFormat::F32Interleaved` | variant | Selects f32 interleaved behavior for `SampleFormat`. | `src/frame/audio.rs:14` |
| sym-5851089e4a084a802423 | `pocketstation::frame::lineage::FrameLineageBuildError::TimestampOverflow` | variant | Reported when the owning operation encounters timestamp overflow. | `src/frame/lineage.rs:99` |
| sym-802fa64e9e6b997312ae | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroDuration` | variant | Reported when the owning operation encounters zero duration. | `src/frame/lineage.rs:95` |
| sym-82871ec25b1689ff12cd | `pocketstation::frame::lineage::FrameLineageBuildError::ZeroSourceGeneration` | variant | Reported when the owning operation encounters zero source generation. | `src/frame/lineage.rs:97` |
| sym-e9754b2fdc87d4bffcc4 | `pocketstation::frame::platform::Platform::Android` | variant | Represents the android alternative defined by `Platform`. | `src/frame/platform.rs:9` |
| sym-01f17ea0041484e3e7cf | `pocketstation::frame::platform::Platform::Ios` | variant | Represents the ios alternative defined by `Platform`. | `src/frame/platform.rs:8` |
| sym-83e779059510799c1958 | `pocketstation::frame::platform::Platform::Linux` | variant | Represents the linux alternative defined by `Platform`. | `src/frame/platform.rs:7` |
| sym-42bd7a670de741864ac3 | `pocketstation::frame::platform::Platform::Macos` | variant | Represents the macos alternative defined by `Platform`. | `src/frame/platform.rs:5` |
| sym-4dd8cdd75b2df7843b4a | `pocketstation::frame::platform::Platform::Unknown` | variant | Represents the unknown alternative defined by `Platform`. | `src/frame/platform.rs:11` |
| sym-994c16c5d185d949e045 | `pocketstation::frame::platform::Platform::Web` | variant | Represents the web alternative defined by `Platform`. | `src/frame/platform.rs:10` |
| sym-eda937305eab45906b57 | `pocketstation::frame::platform::Platform::Windows` | variant | Represents the windows alternative defined by `Platform`. | `src/frame/platform.rs:6` |
| sym-197b96354549fa29229a | `pocketstation::frame::pool::AudioBufferWriteError::CapacityExceeded` | variant | Reported when the owning operation encounters capacity exceeded. | `src/frame/pool.rs:18` |
| sym-d0f406180fa59d2b4069 | `pocketstation::graph::compile::resolve::CompileError::AdapterUnavailable` | variant | Reported when the owning operation encounters adapter unavailable. | `src/graph/compile/resolve.rs:62` |
| sym-f6154bd5ba7a65caaeb4 | `pocketstation::graph::compile::resolve::CompileError::ClockDomainMismatch` | variant | Reported when the owning operation encounters clock domain mismatch. | `src/graph/compile/resolve.rs:38` |
| sym-76d5b24f7d049542b600 | `pocketstation::graph::compile::resolve::CompileError::CycleDetected` | variant | Reported when the owning operation encounters cycle detected. | `src/graph/compile/resolve.rs:60` |
| sym-955ef6dd65f120bfb515 | `pocketstation::graph::compile::resolve::CompileError::InvalidConfig` | variant | Reported when the owning operation encounters invalid config. | `src/graph/compile/resolve.rs:30` |
| sym-3ec3f6ce3b299a64d02e | `pocketstation::graph::compile::resolve::CompileError::InvalidRealtimeEdge` | variant | Reported when the owning operation encounters invalid realtime edge. | `src/graph/compile/resolve.rs:58` |
| sym-8db9f626b57fa7808dfc | `pocketstation::graph::compile::resolve::CompileError::InvalidSafetyContract` | variant | Reported when the owning operation encounters invalid safety contract. | `src/graph/compile/resolve.rs:51` |
| sym-615c8f825c27714a87d1 | `pocketstation::graph::compile::resolve::CompileError::MediaMismatch` | variant | Reported when the owning operation encounters media mismatch. | `src/graph/compile/resolve.rs:45` |
| sym-590f8a807c86b5896a61 | `pocketstation::graph::compile::resolve::CompileError::SignalMismatch` | variant | Reported when the owning operation encounters signal mismatch. | `src/graph/compile/resolve.rs:47` |
| sym-203c1bd4784af40cd629 | `pocketstation::graph::compile::resolve::CompileError::UnknownNode` | variant | Reported when the owning operation encounters unknown node. | `src/graph/compile/resolve.rs:32` |
| sym-469ae995583a12fca4ce | `pocketstation::graph::compile::resolve::CompileError::UnknownNodeType` | variant | Reported when the owning operation encounters unknown node type. | `src/graph/compile/resolve.rs:28` |
| sym-729aa475115d93e5cd14 | `pocketstation::graph::compile::resolve::CompileError::UnknownPort` | variant | Reported when the owning operation encounters unknown port. | `src/graph/compile/resolve.rs:34` |
| sym-7314d8524f5e75bc37c6 | `pocketstation::graph::compile::resolve::CompileError::WrongPortDirection` | variant | Reported when the owning operation encounters wrong port direction. | `src/graph/compile/resolve.rs:36` |
| sym-6e9c6181cb50787467cc | `pocketstation::graph::node::ConfigError::Invalid` | variant | Reported when the owning operation encounters invalid. | `src/graph/node.rs:145` |
| sym-b454641fd28e27af7002 | `pocketstation::graph::node::ConfigError::Missing` | variant | Reported when the owning operation encounters missing. | `src/graph/node.rs:143` |
| sym-7d027346e6ddbc9c3530 | `pocketstation::graph::node::NodeDescriptorError::DuplicatePort` | variant | Reported when the owning operation encounters duplicate port. | `src/graph/node.rs:262` |
| sym-8b11ab123544fbbb95a6 | `pocketstation::graph::node::NodeDescriptorError::EmptyDisplayName` | variant | Reported when the owning operation encounters empty display name. | `src/graph/node.rs:256` |
| sym-d95c832ca35d9c610e74 | `pocketstation::graph::node::NodeDescriptorError::EmptyTypeId` | variant | Reported when the owning operation encounters empty type identifier. | `src/graph/node.rs:254` |
| sym-4470b9644e0204c08a44 | `pocketstation::graph::node::NodeDescriptorError::InvalidSafetyContract` | variant | Reported when the owning operation encounters invalid safety contract. | `src/graph/node.rs:258` |
| sym-d01e3079666fc5df64c2 | `pocketstation::graph::node::NodeDescriptorError::PortDirectionMismatch` | variant | Reported when the owning operation encounters port direction mismatch. | `src/graph/node.rs:260` |
| sym-2a72ecbe04910bb29b67 | `pocketstation::graph::node::NodeError::Config` | variant | Reported when the owning operation encounters config. | `src/graph/node.rs:161` |
| sym-54c0f6130f88109c8a0f | `pocketstation::graph::node::NodeError::ExternalBoundaryExecution` | variant | Reported when the owning operation encounters external boundary execution. | `src/graph/node.rs:159` |
| sym-ab26b16d8da33faaa082 | `pocketstation::graph::node::NodeError::Prepare` | variant | Reported when the owning operation encounters prepare. | `src/graph/node.rs:151` |
| sym-01ddd85e467ab9e9707e | `pocketstation::graph::node::NodeError::Process` | variant | Reported when the owning operation encounters process. | `src/graph/node.rs:153` |
| sym-6c95797cfb6691767681 | `pocketstation::graph::node::NodeError::ProcessTimeout` | variant | Reported when the owning operation encounters process timeout. | `src/graph/node.rs:155` |
| sym-b08a8848ce89fd4c5498 | `pocketstation::graph::partition::ExecutionPartition::AsyncWorker` | variant | Tokio async task. | `src/graph/partition.rs:36` |
| sym-166c8e408c3fa2dee710 | `pocketstation::graph::partition::ExecutionPartition::AudioCallback` | variant | Platform OS audio callback — the strictest domain. | `src/graph/partition.rs:24` |
| sym-8269902e9449a108171e | `pocketstation::graph::partition::ExecutionPartition::BlockingWorker` | variant | `spawn_blocking` thread. | `src/graph/partition.rs:42` |
| sym-e939252394dc413beb1d | `pocketstation::graph::partition::ExecutionPartition::External` | variant | Remote service — always async, always network-required. | `src/graph/partition.rs:48` |
| sym-4f9b170011fc3f36c3c7 | `pocketstation::graph::partition::ExecutionPartition::RealtimeCpu` | variant | Dedicated real-time processing thread. | `src/graph/partition.rs:30` |
| sym-a906ee6c15c310976eb3 | `pocketstation::graph::partition::SafetyContract::AllocationAllowed` | variant | May heap-allocate but must not block or make network calls. | `src/graph/partition.rs:90` |
| sym-b87254439c58024e71e6 | `pocketstation::graph::partition::SafetyContract::BlockingAllowed` | variant | May block the current OS thread. | `src/graph/partition.rs:93` |
| sym-acf39ef2353a17bf62da | `pocketstation::graph::partition::SafetyContract::ExternalService` | variant | Backed by a remote service; all calls are async network operations. | `src/graph/partition.rs:99` |
| sym-9e72335b09a2f3dd7f6d | `pocketstation::graph::partition::SafetyContract::NetworkAllowed` | variant | May make network calls (implies async + allocation allowed). | `src/graph/partition.rs:96` |
| sym-300f25799353b7f7579c | `pocketstation::graph::partition::SafetyContract::RealtimeSafe` | variant | No heap allocation, no locking, no blocking, no logging. | `src/graph/partition.rs:87` |
| sym-7ec1e5a640239120bb30 | `pocketstation::graph::plan::PlanError::FanInOnSinglePort` | variant | Reported when the owning operation encounters fan in on single port. | `src/graph/plan.rs:23` |
| sym-38f2fc10a673388189c2 | `pocketstation::graph::plan::PlanError::MissingEdgeContract` | variant | Reported when the owning operation encounters missing edge contract. | `src/graph/plan.rs:27` |
| sym-9e1cd9e88033d7116303 | `pocketstation::graph::plan::PlanError::MissingOutputSignal` | variant | Reported when the owning operation encounters missing output signal. | `src/graph/plan.rs:29` |
| sym-3cd1490dd1d1347d745d | `pocketstation::graph::plan::PlanError::MoveExclusiveFanOut` | variant | Reported when the owning operation encounters move exclusive fan out. | `src/graph/plan.rs:25` |
| sym-e09b48405b6d056331cc | `pocketstation::graph::ports::BackpressurePolicy::BlockForbidden` | variant | Selects block forbidden behavior for `BackpressurePolicy`. | `src/graph/ports.rs:269` |
| sym-a0fc9261f1cc5fd0018f | `pocketstation::graph::ports::BackpressurePolicy::BoundedQueue` | variant | Selects bounded queue behavior for `BackpressurePolicy`. | `src/graph/ports.rs:268` |
| sym-b63164de77a09d0430dc | `pocketstation::graph::ports::BackpressurePolicy::DropNewest` | variant | Selects drop newest behavior for `BackpressurePolicy`. | `src/graph/ports.rs:266` |
| sym-4bd4affc94bac09be3ac | `pocketstation::graph::ports::BackpressurePolicy::DropOldest` | variant | Selects drop oldest behavior for `BackpressurePolicy`. | `src/graph/ports.rs:267` |
| sym-64a5f6853cb480f0ef46 | `pocketstation::graph::ports::ChannelLayout::Any` | variant | Represents the any alternative defined by `ChannelLayout`. | `src/graph/ports.rs:30` |
| sym-8522a4312f9898b12a10 | `pocketstation::graph::ports::ChannelLayout::Mono` | variant | Represents the mono alternative defined by `ChannelLayout`. | `src/graph/ports.rs:28` |
| sym-afa185f7e955221ef580 | `pocketstation::graph::ports::ChannelLayout::Stereo` | variant | Represents the stereo alternative defined by `ChannelLayout`. | `src/graph/ports.rs:29` |
| sym-2db6ea09960d13008a4b | `pocketstation::graph::ports::ClockDomain::Capture` | variant | Represents the capture alternative defined by `ClockDomain`. | `src/graph/ports.rs:250` |
| sym-619c0a53ceb91f1eef5c | `pocketstation::graph::ports::ClockDomain::Inherited` | variant | Preserve the clock carried by the producer's signal envelope. | `src/graph/ports.rs:254` |
| sym-33a24d7844702b9f99db | `pocketstation::graph::ports::ClockDomain::Network` | variant | Represents the network alternative defined by `ClockDomain`. | `src/graph/ports.rs:252` |
| sym-1e88c138823ed6f8d29c | `pocketstation::graph::ports::ClockDomain::Playback` | variant | Represents the playback alternative defined by `ClockDomain`. | `src/graph/ports.rs:251` |
| sym-24cf39c70246a1976292 | `pocketstation::graph::ports::ClockDomain::Wallclock` | variant | Represents the wallclock alternative defined by `ClockDomain`. | `src/graph/ports.rs:255` |
| sym-0cfe87da4886682235c8 | `pocketstation::graph::ports::CopyPolicy::CopyToBranchPool` | variant | Selects copy to branch pool behavior for `CopyPolicy`. | `src/graph/ports.rs:283` |
| sym-7f20bdf65a7b15629177 | `pocketstation::graph::ports::CopyPolicy::MoveExclusive` | variant | Selects move exclusive behavior for `CopyPolicy`. | `src/graph/ports.rs:281` |
| sym-4517bb30f47c8a7481ab | `pocketstation::graph::ports::CopyPolicy::ShareReadOnly` | variant | Selects share read only behavior for `CopyPolicy`. | `src/graph/ports.rs:282` |
| sym-051ffc6a69020237a11d | `pocketstation::graph::ports::DeliverySemantics::BestEffortRealtime` | variant | Identifies the best effort realtime state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:274` |
| sym-d9ec5a413f93945a282e | `pocketstation::graph::ports::DeliverySemantics::ExactlyOnceNotRealtime` | variant | Identifies the exactly once not realtime state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:276` |
| sym-6b199ec1a9cda41742a6 | `pocketstation::graph::ports::DeliverySemantics::Ordered` | variant | Identifies the ordered state or stage represented by `DeliverySemantics`. | `src/graph/ports.rs:275` |
| sym-c4f169e64b5428a66f0e | `pocketstation::graph::ports::EdgeObservabilityLevel::Counters` | variant | Selects counters behavior for `EdgeObservabilityLevel`. | `src/graph/ports.rs:296` |
| sym-c1c0e2c04f2fe066ae65 | `pocketstation::graph::ports::EdgeObservabilityLevel::Full` | variant | Reports that bounded capacity is full. | `src/graph/ports.rs:297` |
| sym-be657a2803d007904822 | `pocketstation::graph::ports::EdgeObservabilityLevel::Off` | variant | Selects off behavior for `EdgeObservabilityLevel`. | `src/graph/ports.rs:295` |
| sym-846120891281a702e67a | `pocketstation::graph::ports::LossPolicy::ConcealForAudio` | variant | Selects conceal for audio behavior for `LossPolicy`. | `src/graph/ports.rs:288` |
| sym-15698724d738c9c9d8a5 | `pocketstation::graph::ports::LossPolicy::DropAllowed` | variant | Selects drop allowed behavior for `LossPolicy`. | `src/graph/ports.rs:290` |
| sym-6dabd2c71f57230c0848 | `pocketstation::graph::ports::LossPolicy::MustDeliverOrFail` | variant | Selects must deliver or fail behavior for `LossPolicy`. | `src/graph/ports.rs:289` |
| sym-3e9f7b39ec687d4ca20b | `pocketstation::graph::ports::MediaCaps::Any` | variant | Represents the any alternative defined by `MediaCaps`. | `src/graph/ports.rs:93` |
| sym-76469612d179062b2fc1 | `pocketstation::graph::ports::MediaCaps::Audio` | variant | Represents the audio alternative defined by `MediaCaps`. | `src/graph/ports.rs:86` |
| sym-c49927ddbf40b560c837 | `pocketstation::graph::ports::MediaCaps::Binary` | variant | Represents the binary alternative defined by `MediaCaps`. | `src/graph/ports.rs:92` |
| sym-64fec55ff6fe0791aa71 | `pocketstation::graph::ports::MediaCaps::Control` | variant | Represents the control alternative defined by `MediaCaps`. | `src/graph/ports.rs:91` |
| sym-d388efb02fef5554e73e | `pocketstation::graph::ports::MediaCaps::EncodedAudio` | variant | Represents the encoded audio alternative defined by `MediaCaps`. | `src/graph/ports.rs:87` |
| sym-216f89f1007d4600472c | `pocketstation::graph::ports::MediaCaps::Event` | variant | Represents the event alternative defined by `MediaCaps`. | `src/graph/ports.rs:89` |
| sym-2f101e9a0345109d06d4 | `pocketstation::graph::ports::MediaCaps::Metrics` | variant | Represents the metrics alternative defined by `MediaCaps`. | `src/graph/ports.rs:90` |
| sym-0a9c8c23a5eff953b731 | `pocketstation::graph::ports::MediaCaps::Text` | variant | Represents the text alternative defined by `MediaCaps`. | `src/graph/ports.rs:88` |
| sym-a9194bd0bd24824e7fff | `pocketstation::graph::ports::MediaKind::AudioEncoded` | variant | Selects audio encoded behavior for `MediaKind`. | `src/graph/ports.rs:18` |
| sym-77a2fecb7c77f8ca5a1e | `pocketstation::graph::ports::MediaKind::AudioPcm` | variant | Selects audio PCM behavior for `MediaKind`. | `src/graph/ports.rs:17` |
| sym-b60809559800c0ccbbbc | `pocketstation::graph::ports::MediaKind::Binary` | variant | Selects binary behavior for `MediaKind`. | `src/graph/ports.rs:23` |
| sym-8f2af9be838e55b11f8f | `pocketstation::graph::ports::MediaKind::Control` | variant | Selects control behavior for `MediaKind`. | `src/graph/ports.rs:22` |
| sym-f765847643c7f4be8fb1 | `pocketstation::graph::ports::MediaKind::Event` | variant | Selects event behavior for `MediaKind`. | `src/graph/ports.rs:20` |
| sym-dfb082f8ff800f924a42 | `pocketstation::graph::ports::MediaKind::Metrics` | variant | Selects metrics behavior for `MediaKind`. | `src/graph/ports.rs:21` |
| sym-c3020f3bf3266245f40c | `pocketstation::graph::ports::MediaKind::Text` | variant | Selects text behavior for `MediaKind`. | `src/graph/ports.rs:19` |
| sym-199a160b492f930ce83a | `pocketstation::graph::ports::Multiplicity::Many` | variant | Represents the many alternative defined by `Multiplicity`. | `src/graph/ports.rs:171` |
| sym-9d9bb2329634763b332c | `pocketstation::graph::ports::Multiplicity::One` | variant | Represents the one alternative defined by `Multiplicity`. | `src/graph/ports.rs:170` |
| sym-9cd95b2730a003d7c04a | `pocketstation::graph::ports::PortDirection::Input` | variant | Selects input behavior for `PortDirection`. | `src/graph/ports.rs:164` |
| sym-5c1a5e9d5a3115ce9df5 | `pocketstation::graph::ports::PortDirection::Output` | variant | Selects output behavior for `PortDirection`. | `src/graph/ports.rs:165` |
| sym-749ca81ce065c941f50f | `pocketstation::graph::ports::PortSpecError::EmptyName` | variant | Reported when the owning operation encounters empty name. | `src/graph/ports.rs:241` |
| sym-99b8243093c438374277 | `pocketstation::graph::ports::PortSpecError::InvalidSignal` | variant | Reported when the owning operation encounters invalid signal. | `src/graph/ports.rs:243` |
| sym-607e25db365330f62796 | `pocketstation::graph::ports::PortSpecError::SignalMediaMismatch` | variant | Reported when the owning operation encounters signal media mismatch. | `src/graph/ports.rs:245` |
| sym-4d597e8aef04c263b748 | `pocketstation::graph::registry::NodeDefinitionRef::Async` | variant | Represents the async alternative defined by `NodeDefinitionRef`. | `src/graph/registry.rs:34` |
| sym-a332b682e75e041ac2f4 | `pocketstation::graph::registry::NodeDefinitionRef::Definition` | variant | Represents the definition alternative defined by `NodeDefinitionRef`. | `src/graph/registry.rs:35` |
| sym-a41a881c57d4c7ea3084 | `pocketstation::graph::registry::NodeDefinitionRef::Runtime` | variant | Represents the runtime alternative defined by `NodeDefinitionRef`. | `src/graph/registry.rs:33` |
| sym-915db522d900c15ec29f | `pocketstation::graph::registry::NodeRegistrationError::DuplicateNodeType` | variant | Reported when the owning operation encounters duplicate node type. | `src/graph/registry.rs:61` |
| sym-571622d12fb1cdd31ab1 | `pocketstation::graph::registry::NodeRegistrationError::DuplicateOperatorId` | variant | Reported when the owning operation encounters duplicate operator identifier. | `src/graph/registry.rs:63` |
| sym-72b9a19f5750214e10af | `pocketstation::graph::registry::NodeRegistrationError::InvalidAsyncManifest` | variant | Reported when the owning operation encounters invalid async manifest. | `src/graph/registry.rs:59` |
| sym-8953bc17a1051d693313 | `pocketstation::graph::signal::continuity::SignalContinuityError::DiscontinuityRegressed` | variant | Reported when the owning operation encounters discontinuity regressed. | `src/graph/signal/continuity.rs:101` |
| sym-b00dd81ee70aa043156e | `pocketstation::graph::signal::continuity::SignalContinuityError::GenerationRegressed` | variant | Reported when the owning operation encounters generation regressed. | `src/graph/signal/continuity.rs:103` |
| sym-9fc848add2bbf32bef80 | `pocketstation::graph::signal::continuity::SignalContinuityError::IdentityChanged` | variant | Reported when the owning operation encounters identity changed. | `src/graph/signal/continuity.rs:95` |
| sym-ec950350f3b859b7b795 | `pocketstation::graph::signal::continuity::SignalContinuityError::InvalidEnvelope` | variant | Reported when the owning operation encounters invalid envelope. | `src/graph/signal/continuity.rs:91` |
| sym-f47512782d54a5867fd9 | `pocketstation::graph::signal::continuity::SignalContinuityError::MissingLineage` | variant | Reported when the owning operation encounters missing lineage. | `src/graph/signal/continuity.rs:93` |
| sym-791001d14aee99b03704 | `pocketstation::graph::signal::continuity::SignalContinuityError::PolicyRegressed` | variant | Reported when the owning operation encounters policy regressed. | `src/graph/signal/continuity.rs:107` |
| sym-f264ef1a46b7edd4bc9d | `pocketstation::graph::signal::continuity::SignalContinuityError::RecoveryWithoutDiscontinuity` | variant | Reported when the owning operation encounters recovery without discontinuity. | `src/graph/signal/continuity.rs:105` |
| sym-971773a032a5a65b85fc | `pocketstation::graph::signal::continuity::SignalContinuityError::SequenceGapWithoutDiscontinuity` | variant | Reported when the owning operation encounters sequence gap without discontinuity. | `src/graph/signal/continuity.rs:97` |
| sym-ea2fecaa03c872de1621 | `pocketstation::graph::signal::continuity::SignalContinuityError::TimestampRegression` | variant | Reported when the owning operation encounters timestamp regression. | `src/graph/signal/continuity.rs:99` |
| sym-b3b48cfd2fe7344f54be | `pocketstation::graph::signal::envelope::SignalEnvelopeError::InvalidSignalSpec` | variant | Reported when the owning operation encounters invalid signal spec. | `src/graph/signal/envelope.rs:139` |
| sym-88acae22d785796ce00b | `pocketstation::graph::signal::envelope::SignalEnvelopeError::PayloadSpecMismatch` | variant | Reported when the owning operation encounters payload spec mismatch. | `src/graph/signal/envelope.rs:141` |
| sym-614e6f2c87d6d47589d4 | `pocketstation::graph::signal::envelope::SignalEnvelopeError::SequenceMismatch` | variant | Reported when the owning operation encounters sequence mismatch. | `src/graph/signal/envelope.rs:143` |
| sym-20a0616300ba93307965 | `pocketstation::graph::signal::envelope::SignalEnvelopeError::SourceMismatch` | variant | Reported when the owning operation encounters source mismatch. | `src/graph/signal/envelope.rs:145` |
| sym-657e7a904322996c1438 | `pocketstation::graph::signal::lineage::SignalDerivationError::EmptyOperatorId` | variant | Reported when the owning operation encounters empty operator identifier. | `src/graph/signal/lineage.rs:163` |
| sym-1dc3bf71938d42a32e8b | `pocketstation::graph::signal::lineage::SignalDerivationError::InvalidTimestampRange` | variant | Reported when the owning operation encounters invalid timestamp range. | `src/graph/signal/lineage.rs:161` |
| sym-daee8d09f2d4f5165a4c | `pocketstation::graph::signal::lineage::SignalDerivationError::ZeroOperatorVersion` | variant | Reported when the owning operation encounters zero operator version. | `src/graph/signal/lineage.rs:165` |
| sym-950b911a76196382cfd6 | `pocketstation::graph::signal::lineage::SignalLineageError::ZeroSourceGeneration` | variant | Reported when the owning operation encounters zero source generation. | `src/graph/signal/lineage.rs:88` |
| sym-032476340f73c9ac5865 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::DuplicateOutputRole` | variant | Reported when the owning operation encounters duplicate output role. | `src/graph/signal/operator.rs:363` |
| sym-21b60ce75f9261511777 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::EmptyOperatorId` | variant | Reported when the owning operation encounters empty operator identifier. | `src/graph/signal/operator.rs:323` |
| sym-0c3bc67eaa9edc31d998 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::EmptyOutputRole` | variant | Reported when the owning operation encounters empty output role. | `src/graph/signal/operator.rs:361` |
| sym-bc3722e1247d75711150 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InputEdgeMediaMismatch` | variant | Reported when the owning operation encounters input edge media mismatch. | `src/graph/signal/operator.rs:349` |
| sym-12801519ebf5d3874576 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InputSignalMediaMismatch` | variant | Reported when the owning operation encounters input signal media mismatch. | `src/graph/signal/operator.rs:357` |
| sym-b23bb85cb569e6175065 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InvalidInputSignal` | variant | Reported when the owning operation encounters invalid input signal. | `src/graph/signal/operator.rs:353` |
| sym-75d4d6a9c48a3178d913 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InvalidOutputSignal` | variant | Reported when the owning operation encounters invalid output signal. | `src/graph/signal/operator.rs:355` |
| sym-b160eee90f82dad16d46 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::InvalidSafetyContract` | variant | Reported when the owning operation encounters invalid safety contract. | `src/graph/signal/operator.rs:335` |
| sym-9f0ed1b5eaa635ecede4 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::MissingInputPort` | variant | Reported when the owning operation encounters missing input port. | `src/graph/signal/operator.rs:339` |
| sym-3c17531e8daf85e83be7 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::MissingOutputPort` | variant | Reported when the owning operation encounters missing output port. | `src/graph/signal/operator.rs:341` |
| sym-1a66748f922e7b935e51 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::NetworkPermissionMismatch` | variant | Reported when the owning operation encounters network permission mismatch. | `src/graph/signal/operator.rs:337` |
| sym-7d7d37578090834e6322 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::OutputEdgeMediaMismatch` | variant | Reported when the owning operation encounters output edge media mismatch. | `src/graph/signal/operator.rs:351` |
| sym-ee305b8da95625510b90 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::OutputSignalMediaMismatch` | variant | Reported when the owning operation encounters output signal media mismatch. | `src/graph/signal/operator.rs:359` |
| sym-657e454eb7b0714f8ba2 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::RealtimePartition` | variant | Reported when the owning operation encounters realtime partition. | `src/graph/signal/operator.rs:333` |
| sym-fa34ddfbb3adbbfcd66e | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::TerminalOutputRoleNotAllowed` | variant | Reported when the owning operation encounters terminal output role not allowed. | `src/graph/signal/operator.rs:365` |
| sym-b194050c74878ed781d0 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::UnsupportedBackpressure` | variant | Reported when the owning operation encounters unsupported backpressure. | `src/graph/signal/operator.rs:343` |
| sym-7a324740125ee0185a65 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::UnsupportedInputCopyPolicy` | variant | Reported when the owning operation encounters unsupported input copy policy. | `src/graph/signal/operator.rs:345` |
| sym-6b68559480eedd5274ff | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::UnsupportedOutputBackpressure` | variant | Reported when the owning operation encounters unsupported output backpressure. | `src/graph/signal/operator.rs:347` |
| sym-611f20139dd10dc2d762 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroGeneration` | variant | Reported when the owning operation encounters zero generation. | `src/graph/signal/operator.rs:327` |
| sym-ca02bbab1976a3f29d01 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroProcessTimeout` | variant | Reported when the owning operation encounters zero process timeout. | `src/graph/signal/operator.rs:331` |
| sym-6b1e2d69a68631950785 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroQueueCapacity` | variant | Reported when the owning operation encounters zero queue capacity. | `src/graph/signal/operator.rs:329` |
| sym-4df48a7a2b86a0b80f46 | `pocketstation::graph::signal::operator::AsyncOperatorManifestError::ZeroRevision` | variant | Reported when the owning operation encounters zero revision. | `src/graph/signal/operator.rs:325` |
| sym-8e8f30a85994b102df0b | `pocketstation::graph::signal::operator::OperatorCancellationPolicy::DiscardQueued` | variant | Selects discard queued behavior for `OperatorCancellationPolicy`. | `src/graph/signal/operator.rs:58` |
| sym-21374ab9032d049b1c07 | `pocketstation::graph::signal::operator::OperatorCancellationPolicy::DrainQueued` | variant | Selects drain queued behavior for `OperatorCancellationPolicy`. | `src/graph/signal/operator.rs:59` |
| sym-f8c13a380cf2c3b4c6af | `pocketstation::graph::signal::operator::OperatorFailurePolicy::Continue` | variant | Selects continue behavior for `OperatorFailurePolicy`. | `src/graph/signal/operator.rs:64` |
| sym-4b1162bb5a725221f726 | `pocketstation::graph::signal::operator::OperatorFailurePolicy::StopWorker` | variant | Selects stop worker behavior for `OperatorFailurePolicy`. | `src/graph/signal/operator.rs:65` |
| sym-7e263c83f50eff746173 | `pocketstation::graph::signal::payload::SignalPayload::Audio` | variant | Represents the audio alternative defined by `SignalPayload`. | `src/graph/signal/payload.rs:11` |
| sym-9876aec4b032579f2487 | `pocketstation::graph::signal::payload::SignalPayload::Bytes` | variant | Represents the bytes alternative defined by `SignalPayload`. | `src/graph/signal/payload.rs:13` |
| sym-aa366d15a8cc5eb47fc1 | `pocketstation::graph::signal::payload::SignalPayload::Text` | variant | Represents the text alternative defined by `SignalPayload`. | `src/graph/signal/payload.rs:12` |
| sym-a301e96d844b2e0497fd | `pocketstation::graph::signal::spec::BinaryFormat::Cbor` | variant | Selects cbor behavior for `BinaryFormat`. | `src/graph/signal/spec.rs:145` |
| sym-7419f576ea4d8ce39908 | `pocketstation::graph::signal::spec::BinaryFormat::Flatbuffers` | variant | Selects flatbuffers behavior for `BinaryFormat`. | `src/graph/signal/spec.rs:144` |
| sym-d6ae4cf9e40fded08938 | `pocketstation::graph::signal::spec::BinaryFormat::Protobuf` | variant | Selects protobuf behavior for `BinaryFormat`. | `src/graph/signal/spec.rs:143` |
| sym-c270aaedefb5b7a643b7 | `pocketstation::graph::signal::spec::BinaryFormat::Raw` | variant | Selects raw behavior for `BinaryFormat`. | `src/graph/signal/spec.rs:142` |
| sym-0ff0d9ca8ba1a50c714f | `pocketstation::graph::signal::spec::Codec::Aac` | variant | Represents the aac alternative defined by `Codec`. | `src/graph/signal/spec.rs:115` |
| sym-38043f429c7eea980f86 | `pocketstation::graph::signal::spec::Codec::G711Alaw` | variant | Represents the g711 alaw alternative defined by `Codec`. | `src/graph/signal/spec.rs:118` |
| sym-81bd3a56a535f061533d | `pocketstation::graph::signal::spec::Codec::G711Ulaw` | variant | Represents the g711 ulaw alternative defined by `Codec`. | `src/graph/signal/spec.rs:117` |
| sym-4f0d55352bacaf1c0c67 | `pocketstation::graph::signal::spec::Codec::Mp3` | variant | Represents the mp3 alternative defined by `Codec`. | `src/graph/signal/spec.rs:116` |
| sym-bb9e1acfcb6d13d7f167 | `pocketstation::graph::signal::spec::Codec::Opus` | variant | Represents the opus alternative defined by `Codec`. | `src/graph/signal/spec.rs:114` |
| sym-bf728e1f08c0609d52ad | `pocketstation::graph::signal::spec::Codec::WebmOpus` | variant | Represents the webm opus alternative defined by `Codec`. | `src/graph/signal/spec.rs:119` |
| sym-b854f90973b3b44bf27f | `pocketstation::graph::signal::spec::EventFormat::Cbor` | variant | Identifies the cbor state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:136` |
| sym-2824d16b874d9c69aef6 | `pocketstation::graph::signal::spec::EventFormat::Flatbuffers` | variant | Identifies the flatbuffers state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:135` |
| sym-a87db57a4c68b5999088 | `pocketstation::graph::signal::spec::EventFormat::Json` | variant | Identifies the json state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:133` |
| sym-ec7fb19102e00aabf324 | `pocketstation::graph::signal::spec::EventFormat::Protobuf` | variant | Identifies the protobuf state or stage represented by `EventFormat`. | `src/graph/signal/spec.rs:134` |
| sym-1bd849ca9101e2f40c96 | `pocketstation::graph::signal::spec::SignalClass::Any` | variant | Wildcard accepted only at deliberately open graph boundaries. | `src/graph/signal/spec.rs:158` |
| sym-5e4c56d10595ab57f0f7 | `pocketstation::graph::signal::spec::SignalClass::Binary` | variant | Carries an opaque binary payload described by a `BinaryFormat`. | `src/graph/signal/spec.rs:172` |
| sym-de4bc2f3bcb2a4a5177a | `pocketstation::graph::signal::spec::SignalClass::Control` | variant | Graph control messages (route patches, session lifecycle, mute/unmute). | `src/graph/signal/spec.rs:170` |
| sym-52dca2097acbb3de61c5 | `pocketstation::graph::signal::spec::SignalClass::Custom` | variant | Extension point for community / vendor signals. Use a stable reverse-domain identifier. | `src/graph/signal/spec.rs:175` |
| sym-e5c7baaf154f21aa4215 | `pocketstation::graph::signal::spec::SignalClass::EncodedAudio` | variant | Compressed audio bitstream (Opus packet, AAC frame, …). | `src/graph/signal/spec.rs:162` |
| sym-ce6df19edaaf45efd2ee | `pocketstation::graph::signal::spec::SignalClass::Event` | variant | Carries discrete event payloads described by an `EventFormat`. | `src/graph/signal/spec.rs:166` |
| sym-35597f127c20c047c236 | `pocketstation::graph::signal::spec::SignalClass::Metrics` | variant | Telemetry and observability counters / gauges. | `src/graph/signal/spec.rs:168` |
| sym-29d7e0543fdd76640ba2 | `pocketstation::graph::signal::spec::SignalClass::PcmAudio` | variant | Interleaved PCM audio samples (format described by the edge AudioCaps). | `src/graph/signal/spec.rs:160` |
| sym-21a2fa6500e425acb1c1 | `pocketstation::graph::signal::spec::SignalClass::Text` | variant | UTF-8 or structured text. | `src/graph/signal/spec.rs:164` |
| sym-401a031bae9f256a6620 | `pocketstation::graph::signal::spec::SignalSpecError::EmptyCustomId` | variant | Reported when the owning operation encounters empty custom identifier. | `src/graph/signal/spec.rs:353` |
| sym-f038dc5f305737debec8 | `pocketstation::graph::signal::spec::SignalSpecError::EmptyRole` | variant | Reported when the owning operation encounters empty role. | `src/graph/signal/spec.rs:355` |
| sym-3f0cf7e8f9c80a0d8f6c | `pocketstation::graph::signal::spec::SignalSpecError::EmptySchema` | variant | Reported when the owning operation encounters empty schema. | `src/graph/signal/spec.rs:357` |
| sym-4040f8321d21cec4b5a8 | `pocketstation::graph::signal::spec::TextFormat::Json` | variant | Selects json behavior for `TextFormat`. | `src/graph/signal/spec.rs:126` |
| sym-235c4dc6a73a1dac4666 | `pocketstation::graph::signal::spec::TextFormat::Markdown` | variant | Selects markdown behavior for `TextFormat`. | `src/graph/signal/spec.rs:127` |
| sym-8a9dca11433fadbcaaf1 | `pocketstation::graph::signal::spec::TextFormat::Utf8` | variant | Selects utf8 behavior for `TextFormat`. | `src/graph/signal/spec.rs:125` |
| sym-dcfc279047f4ce36dd69 | `pocketstation::graph::signal::timing::SignalTimingError::TimestampOverflow` | variant | Reported when the owning operation encounters timestamp overflow. | `src/graph/signal/timing.rs:93` |
| sym-279cbfa0ef01c3c3c31c | `pocketstation::graph::signal::timing::SignalTimingError::ZeroDuration` | variant | Reported when the owning operation encounters zero duration. | `src/graph/signal/timing.rs:91` |
| sym-d30fd6e12689f5243904 | `pocketstation::native_extension::NativeExtensionKind::Endpoint` | variant | Selects endpoint behavior for `NativeExtensionKind`. | `src/native_extension/mod.rs:30` |
| sym-f408870952814e3618ef | `pocketstation::native_extension::NativeExtensionKind::Operator` | variant | Selects operator behavior for `NativeExtensionKind`. | `src/native_extension/mod.rs:29` |
| sym-6a44610e90e03cf7c6f2 | `pocketstation::native_extension::NativeExtensionKind::Source` | variant | Selects source behavior for `NativeExtensionKind`. | `src/native_extension/mod.rs:28` |
| sym-48ee726de0c65fe82dfe | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::DuplicateRegistration` | variant | Reported when the owning operation encounters duplicate registration. | `src/native_extension/mod.rs:92` |
| sym-779580127202705588b2 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointFailed` | variant | Reported when the owning operation encounters entrypoint failed. | `src/native_extension/mod.rs:85` |
| sym-a7bc23ed348a9a40e54b | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointMissing` | variant | Reported when the owning operation encounters entrypoint missing. | `src/native_extension/mod.rs:83` |
| sym-4181805148fc0b07f174 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::EntrypointPanicked` | variant | Reported when the owning operation encounters entrypoint panicked. | `src/native_extension/mod.rs:84` |
| sym-af30c25caef06e1480ff | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidLibraryDescriptor` | variant | Reported when the owning operation encounters invalid library descriptor. | `src/native_extension/mod.rs:88` |
| sym-a2c38d2ed33b40f02cbb | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::InvalidRegistration` | variant | Reported when the owning operation encounters invalid registration. | `src/native_extension/mod.rs:91` |
| sym-d58bcae00a1d215a2450 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::LibraryLoadFailed` | variant | Reported when the owning operation encounters library load failed. | `src/native_extension/mod.rs:82` |
| sym-f03edb8da6e112e5d64b | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathCanonicalizationFailed` | variant | Reported when the owning operation encounters path canonicalization failed. | `src/native_extension/mod.rs:80` |
| sym-346b3b8e2172ad39d824 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotAbsolute` | variant | Reported when the owning operation encounters path not absolute. | `src/native_extension/mod.rs:79` |
| sym-e75efacbb2da9099c475 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::PathNotFile` | variant | Reported when the owning operation encounters path not file. | `src/native_extension/mod.rs:81` |
| sym-99e7e4f05b2be8a0e4a8 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionFailed` | variant | Reported when the owning operation encounters registration acquisition failed. | `src/native_extension/mod.rs:90` |
| sym-9210bc775734ea4a4f13 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationAcquisitionPanicked` | variant | Reported when the owning operation encounters registration acquisition panicked. | `src/native_extension/mod.rs:89` |
| sym-bc00a333660a6f92a8f3 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::RegistrationStateUnavailable` | variant | Reported when the owning operation encounters registration state unavailable. | `src/native_extension/mod.rs:93` |
| sym-7eaa4451dcde1550c1b8 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMajor` | variant | Reported when the owning operation encounters unsupported ABI major. | `src/native_extension/mod.rs:86` |
| sym-7df0771866f13d3e1584 | `pocketstation::native_extension::NativeExtensionLibraryErrorCode::UnsupportedAbiMinor` | variant | Reported when the owning operation encounters unsupported ABI minor. | `src/native_extension/mod.rs:87` |
| sym-1051b0c2eb03babe6742 | `pocketstation::recording::config::PermissionDecision::Allowed` | variant | Represents the allowed alternative defined by `PermissionDecision`. | `src/recording/config.rs:44` |
| sym-9b2324ae246c45f16c4f | `pocketstation::recording::config::PermissionDecision::Denied` | variant | Represents the denied alternative defined by `PermissionDecision`. | `src/recording/config.rs:45` |
| sym-7f4bf2169422bbba6e7d | `pocketstation::recording::config::PermissionScope::SessionCaptureGrant` | variant | Selects session capture grant behavior for `PermissionScope`. | `src/recording/config.rs:51` |
| sym-bc72319fd0af645d0508 | `pocketstation::recording::config::RecorderLineageField::Clock` | variant | Represents the clock alternative defined by `RecorderLineageField`. | `src/recording/config.rs:14` |
| sym-edc80b30cffb9d7bdbda | `pocketstation::recording::config::RecorderLineageField::PermissionEpoch` | variant | Represents the permission epoch alternative defined by `RecorderLineageField`. | `src/recording/config.rs:16` |
| sym-6d41fc8c3dc0142f91b9 | `pocketstation::recording::config::RecorderLineageField::Session` | variant | Represents the session alternative defined by `RecorderLineageField`. | `src/recording/config.rs:11` |
| sym-63ce6c5b7236bef69978 | `pocketstation::recording::config::RecorderLineageField::Source` | variant | Represents the source alternative defined by `RecorderLineageField`. | `src/recording/config.rs:12` |
| sym-34776690b0720e1eb5c4 | `pocketstation::recording::config::RecorderLineageField::SourceGeneration` | variant | Represents the source generation alternative defined by `RecorderLineageField`. | `src/recording/config.rs:15` |
| sym-8ecaef8805638fac01db | `pocketstation::recording::config::RecorderLineageField::Stem` | variant | Represents the stem alternative defined by `RecorderLineageField`. | `src/recording/config.rs:13` |
| sym-7d85bd344060da5b9f89 | `pocketstation::recording::error_code::RecordingErrorCode::DuplicateStemLabel` | variant | Reported when the owning operation encounters duplicate stem label. | `src/recording/error_code.rs:12` |
| sym-b7f2ed2090a92554d1ed | `pocketstation::recording::error_code::RecordingErrorCode::FrameSpecMismatch` | variant | Reported when the owning operation encounters frame spec mismatch. | `src/recording/error_code.rs:18` |
| sym-4d21318f4fd9ea0a7fb4 | `pocketstation::recording::error_code::RecordingErrorCode::GapTooLarge` | variant | Reported when the owning operation encounters gap too large. | `src/recording/error_code.rs:21` |
| sym-6f6a262d68369fc05e8b | `pocketstation::recording::error_code::RecordingErrorCode::Incomplete` | variant | Reported when the owning operation encounters incomplete. | `src/recording/error_code.rs:28` |
| sym-bdd235056a4f9790f4f8 | `pocketstation::recording::error_code::RecordingErrorCode::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/recording/error_code.rs:15` |
| sym-181929c742670b831954 | `pocketstation::recording::error_code::RecordingErrorCode::InvalidStemLabel` | variant | Reported when the owning operation encounters invalid stem label. | `src/recording/error_code.rs:11` |
| sym-18fd50278ed6840f60be | `pocketstation::recording::error_code::RecordingErrorCode::IoFailed` | variant | Reported when the owning operation encounters I/O failed. | `src/recording/error_code.rs:24` |
| sym-dcb3d2c64d65124fa855 | `pocketstation::recording::error_code::RecordingErrorCode::JsonFailed` | variant | Reported when the owning operation encounters json failed. | `src/recording/error_code.rs:26` |
| sym-0c3a5287294bdb68c6e8 | `pocketstation::recording::error_code::RecordingErrorCode::LineageMismatch` | variant | Reported when the owning operation encounters lineage mismatch. | `src/recording/error_code.rs:17` |
| sym-e870f098f3fc6f37a6df | `pocketstation::recording::error_code::RecordingErrorCode::NotFinalized` | variant | Reported when the owning operation encounters not finalized. | `src/recording/error_code.rs:27` |
| sym-a2a90cdd4a99b624fa65 | `pocketstation::recording::error_code::RecordingErrorCode::OutputExists` | variant | Reported when the owning operation encounters output exists. | `src/recording/error_code.rs:10` |
| sym-d077e4454fa188e87f61 | `pocketstation::recording::error_code::RecordingErrorCode::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/error_code.rs:14` |
| sym-4628e03b1ab60563e31d | `pocketstation::recording::error_code::RecordingErrorCode::SessionMismatch` | variant | Reported when the owning operation encounters session mismatch. | `src/recording/error_code.rs:13` |
| sym-fa8ca7fd54c6dad87881 | `pocketstation::recording::error_code::RecordingErrorCode::SourceMismatch` | variant | Reported when the owning operation encounters source mismatch. | `src/recording/error_code.rs:16` |
| sym-80eb48ce6fb03179ad93 | `pocketstation::recording::error_code::RecordingErrorCode::TimestampOutOfRange` | variant | Reported when the owning operation encounters timestamp out of range. | `src/recording/error_code.rs:20` |
| sym-187654a56f50bdc64104 | `pocketstation::recording::error_code::RecordingErrorCode::TooManyGaps` | variant | Reported when the owning operation encounters too many gaps. | `src/recording/error_code.rs:22` |
| sym-dc3f942a67f028277891 | `pocketstation::recording::error_code::RecordingErrorCode::UnalignedSamples` | variant | Reported when the owning operation encounters unaligned samples. | `src/recording/error_code.rs:19` |
| sym-d30e3ded5ad282d52460 | `pocketstation::recording::error_code::RecordingErrorCode::WavFailed` | variant | Reported when the owning operation encounters wav failed. | `src/recording/error_code.rs:25` |
| sym-0197e76cc91a39c99b7f | `pocketstation::recording::error_code::RecordingErrorCode::WorkerPanicked` | variant | Reported when the owning operation encounters worker panicked. | `src/recording/error_code.rs:23` |
| sym-52a62e0c868aa1e2fc01 | `pocketstation::recording::writer::DiscontinuityKind::OverlapRejected` | variant | Selects overlap rejected behavior for `DiscontinuityKind`. | `src/recording/writer.rs:107` |
| sym-59887c5a079f1b20ea6f | `pocketstation::recording::writer::DiscontinuityKind::SequenceGap` | variant | Selects sequence gap behavior for `DiscontinuityKind`. | `src/recording/writer.rs:106` |
| sym-5df382854d46d7ec5af0 | `pocketstation::recording::writer::DiscontinuityKind::TimestampGap` | variant | Selects timestamp gap behavior for `DiscontinuityKind`. | `src/recording/writer.rs:105` |
| sym-69b7754427bd82572376 | `pocketstation::recording::writer::RecorderError::DuplicateStemLabel` | variant | Reported when the owning operation encounters duplicate stem label. | `src/recording/writer.rs:29` |
| sym-6cbe33f995c99621972e | `pocketstation::recording::writer::RecorderError::FrameSpecMismatch` | variant | Reported when the owning operation encounters frame spec mismatch. | `src/recording/writer.rs:58` |
| sym-35235e7c62cda3697266 | `pocketstation::recording::writer::RecorderError::GapTooLarge` | variant | Reported when the owning operation encounters gap too large. | `src/recording/writer.rs:70` |
| sym-0f6c4f657e98fdd19871 | `pocketstation::recording::writer::RecorderError::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/recording/writer.rs:39` |
| sym-74c1146dfcbd54ad1e9b | `pocketstation::recording::writer::RecorderError::InvalidStemLabel` | variant | Reported when the owning operation encounters invalid stem label. | `src/recording/writer.rs:27` |
| sym-3b68e704962144ca19c6 | `pocketstation::recording::writer::RecorderError::Io` | variant | Reported when the owning operation encounters I/O. | `src/recording/writer.rs:76` |
| sym-c9c8b30e2e6f65347ff8 | `pocketstation::recording::writer::RecorderError::Json` | variant | Reported when the owning operation encounters json. | `src/recording/writer.rs:80` |
| sym-c4476c73d1b1124b4b8c | `pocketstation::recording::writer::RecorderError::LineageMismatch` | variant | Reported when the owning operation encounters lineage mismatch. | `src/recording/writer.rs:51` |
| sym-ec4b5863a964193e05e8 | `pocketstation::recording::writer::RecorderError::OutputExists` | variant | Reported when the owning operation encounters output exists. | `src/recording/writer.rs:25` |
| sym-4858643bf5eb0df27a40 | `pocketstation::recording::writer::RecorderError::PermissionDenied` | variant | Reports that the required permission was denied. | `src/recording/writer.rs:37` |
| sym-12f9f5a214e19457f852 | `pocketstation::recording::writer::RecorderError::SessionMismatch` | variant | Reported when the owning operation encounters session mismatch. | `src/recording/writer.rs:31` |
| sym-09f3d07430eef7d27c59 | `pocketstation::recording::writer::RecorderError::SourceMismatch` | variant | Reported when the owning operation encounters source mismatch. | `src/recording/writer.rs:45` |
| sym-ca54c477fcd3b74cf4d7 | `pocketstation::recording::writer::RecorderError::TimestampOutOfRange` | variant | Reported when the owning operation encounters timestamp out of range. | `src/recording/writer.rs:68` |
| sym-c72eeddfea335de04be5 | `pocketstation::recording::writer::RecorderError::TooManyGaps` | variant | Reported when the owning operation encounters too many gaps. | `src/recording/writer.rs:72` |
| sym-b24861afdeb150023f3d | `pocketstation::recording::writer::RecorderError::UnalignedSamples` | variant | Reported when the owning operation encounters unaligned samples. | `src/recording/writer.rs:66` |
| sym-75fa0a90f8a898449d9d | `pocketstation::recording::writer::RecorderError::Wav` | variant | Reported when the owning operation encounters wav. | `src/recording/writer.rs:78` |
| sym-08fb6e771620eedfa601 | `pocketstation::recording::writer::RecorderError::WorkerPanicked` | variant | Reported when the owning operation encounters worker panicked. | `src/recording/writer.rs:74` |
| sym-14c652d6945ce064f300 | `pocketstation::recording::writer::RecordingState::Complete` | variant | Identifies the complete state or stage represented by `RecordingState`. | `src/recording/writer.rs:87` |
| sym-d8adec0971c94b5450bf | `pocketstation::recording::writer::RecordingState::Incomplete` | variant | Identifies the incomplete state or stage represented by `RecordingState`. | `src/recording/writer.rs:88` |
| sym-e1aa3176ee3237d9baab | `pocketstation::recording::writer::RecordingState::Recording` | variant | Identifies the recording state or stage represented by `RecordingState`. | `src/recording/writer.rs:86` |
| sym-0a1342a572a33c6cff1e | `pocketstation::runtime::audio::executor::ExecError::Node` | variant | Reported when the owning operation encounters node. | `src/runtime/audio/executor.rs:22` |
| sym-802a32373221037e3788 | `pocketstation::runtime::audio::router::PlanEdgeFrame::Exclusive` | variant | Represents the exclusive alternative defined by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:30` |
| sym-2efc7a00ac65d5caf7f9 | `pocketstation::runtime::audio::router::PlanEdgeFrame::Shared` | variant | Represents the shared alternative defined by `PlanEdgeFrame`. | `src/runtime/audio/router.rs:31` |
| sym-595fa53929b3f8d17d78 | `pocketstation::runtime::audio::router::PlanRouterError::InvalidFrameBytes` | variant | Reported when the owning operation encounters invalid frame bytes. | `src/runtime/audio/router.rs:23` |
| sym-c2895580116fc99cc503 | `pocketstation::runtime::audio::router::PlanRouterError::MissingMemoryPlan` | variant | Reported when the owning operation encounters missing memory plan. | `src/runtime/audio/router.rs:19` |
| sym-a2c5d8a85110281fbf11 | `pocketstation::runtime::audio::router::PlanRouterError::ZeroCapacity` | variant | Reported when the owning operation encounters zero capacity. | `src/runtime/audio/router.rs:21` |
| sym-7c92ba0678bf60895ffd | `pocketstation::runtime::audio::runner::PlanRunnerDrainPolicy::DiscardQueued` | variant | Selects discard queued behavior for `PlanRunnerDrainPolicy`. | `src/runtime/audio/runner.rs:18` |
| sym-c302a2495a16ae551cdb | `pocketstation::runtime::audio::runner::PlanRunnerDrainPolicy::DrainQueued` | variant | Selects drain queued behavior for `PlanRunnerDrainPolicy`. | `src/runtime/audio/runner.rs:17` |
| sym-5d7169a86e45f8f1b322 | `pocketstation::runtime::audio::runner::PlanRunnerError::AlreadyFinished` | variant | Reported when the owning operation encounters already finished. | `src/runtime/audio/runner.rs:264` |
| sym-4a721d1b5be0e7569500 | `pocketstation::runtime::audio::runner::PlanRunnerError::DuplicateSource` | variant | Reported when the owning operation encounters duplicate source. | `src/runtime/audio/runner.rs:260` |
| sym-11722c0dbeb9d922f1fc | `pocketstation::runtime::audio::runner::PlanRunnerError::Execution` | variant | Reported when the owning operation encounters execution. | `src/runtime/audio/runner.rs:266` |
| sym-2ddf5a8a94794b954999 | `pocketstation::runtime::audio::runner::PlanRunnerError::ZeroSourceCapacity` | variant | Reported when the owning operation encounters zero source capacity. | `src/runtime/audio/runner.rs:258` |
| sym-a0ae8446deaecc61e31e | `pocketstation::runtime::audio::runner::PlanRunnerError::ZeroWorkBudget` | variant | Reported when the owning operation encounters zero work budget. | `src/runtime/audio/runner.rs:262` |
| sym-9469494d7d4a4ec5279e | `pocketstation::runtime::audio::runner::PlanSourceSendError::Cancelled` | variant | Indicates that the operation was cancelled. | `src/runtime/audio/runner.rs:117` |
| sym-8120d07071158ee95ea1 | `pocketstation::runtime::audio::runner::PlanSourceSendError::Full` | variant | Reports that bounded capacity is full. | `src/runtime/audio/runner.rs:118` |
| sym-70c7cf9f7d4fbe4bd08b | `pocketstation::runtime::audio::runner::PlanSourceSendOutcome::Enqueued` | variant | Identifies the enqueued state or stage represented by `PlanSourceSendOutcome`. | `src/runtime/audio/runner.rs:124` |
| sym-61d6bdffc9634ad0cb3f | `pocketstation::runtime::audio::runner::PlanSourceSendOutcome::Rejected` | variant | Identifies the rejected state or stage represented by `PlanSourceSendOutcome`. | `src/runtime/audio/runner.rs:125` |
| sym-29cf6f5fa17cbc0e9a1e | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidPoolSlots` | variant | Reported when the owning operation encounters invalid pool slots. | `src/runtime/bridge/audio.rs:52` |
| sym-1896df226eda80ee3c2e | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::InvalidSampleSpec` | variant | Reported when the owning operation encounters invalid sample spec. | `src/runtime/bridge/audio.rs:48` |
| sym-b3d25ef4ec34747f12c3 | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::ThreadStart` | variant | Reported when the owning operation encounters thread start. | `src/runtime/bridge/audio.rs:54` |
| sym-58522af5df72e288db8f | `pocketstation::runtime::bridge::audio::GeneratedAudioBridgeStartError::ZeroFrameSamples` | variant | Reported when the owning operation encounters zero frame samples. | `src/runtime/bridge/audio.rs:50` |
| sym-ca756b68932f7bff795e | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::HostTimeout` | variant | Reported when the owning operation encounters host timeout. | `src/runtime/lifecycle/async_host.rs:18` |
| sym-af84c53a4c3e75cdb8e1 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::RuntimeStopped` | variant | Reported when the owning operation encounters runtime stopped. | `src/runtime/lifecycle/async_host.rs:14` |
| sym-7f41a1931cc7a5608df5 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::ShutdownPanicked` | variant | Reported when the owning operation encounters shutdown panicked. | `src/runtime/lifecycle/async_host.rs:16` |
| sym-567c95dd77f49f193aa5 | `pocketstation::runtime::lifecycle::async_host::AsyncRuntimeHostError::Start` | variant | Reported when the owning operation encounters start. | `src/runtime/lifecycle/async_host.rs:12` |
| sym-41818ffdffa18d254aea | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::AlreadyReaped` | variant | Reported when the owning operation encounters already reaped. | `src/runtime/lifecycle/sidecar_host.rs:730` |
| sym-1032936d4010c9927508 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/runtime/lifecycle/sidecar_host.rs:706` |
| sym-b03fa6e4962455a68f74 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ControlQueueFull` | variant | Reported when the owning operation encounters control queue full. | `src/runtime/lifecycle/sidecar_host.rs:704` |
| sym-6efc47f4fd99ff6fea0f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::DataQueueFull` | variant | Reported when the owning operation encounters data queue full. | `src/runtime/lifecycle/sidecar_host.rs:702` |
| sym-467cdf573e06e4e8d371 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::FrameTooLarge` | variant | Reported when the owning operation encounters frame too large. | `src/runtime/lifecycle/sidecar_host.rs:700` |
| sym-c4ab9aed090c2d2980b7 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidConfiguration` | variant | Reported when the owning operation encounters invalid configuration. | `src/runtime/lifecycle/sidecar_host.rs:688` |
| sym-28cfea31ecff10122eef | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidDataKind` | variant | Reported when the owning operation encounters invalid data kind. | `src/runtime/lifecycle/sidecar_host.rs:724` |
| sym-c154f9a3f712e3c725c0 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::InvalidState` | variant | Reported when the owning operation encounters invalid state. | `src/runtime/lifecycle/sidecar_host.rs:719` |
| sym-0df07c31df8a970e07ad | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Io` | variant | Reported when the owning operation encounters I/O. | `src/runtime/lifecycle/sidecar_host.rs:696` |
| sym-1b831a5f8bee6b701803 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Kill` | variant | Reported when the owning operation encounters kill. | `src/runtime/lifecycle/sidecar_host.rs:728` |
| sym-f4519ec5a0c4faf25ee7 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::MissingPipe` | variant | Reported when the owning operation encounters missing pipe. | `src/runtime/lifecycle/sidecar_host.rs:694` |
| sym-e5403db51c192a923518 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ProcessingTimeout` | variant | Reported when the owning operation encounters processing timeout. | `src/runtime/lifecycle/sidecar_host.rs:717` |
| sym-c053f64a80665ba73e81 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Protocol` | variant | Reported when the owning operation encounters protocol. | `src/runtime/lifecycle/sidecar_host.rs:698` |
| sym-3c862a1301fd8707d8aa | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Spawn` | variant | Reported when the owning operation encounters spawn. | `src/runtime/lifecycle/sidecar_host.rs:690` |
| sym-0df438e915e9ea9934d7 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::ThreadSpawn` | variant | Reported when the owning operation encounters thread spawn. | `src/runtime/lifecycle/sidecar_host.rs:692` |
| sym-17bdf1f03dc1976fc91a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Timeout` | variant | Reported when the owning operation encounters timeout. | `src/runtime/lifecycle/sidecar_host.rs:715` |
| sym-ee58a41495bdbd59c66d | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::UnexpectedEof` | variant | Reported when the owning operation encounters unexpected eof. | `src/runtime/lifecycle/sidecar_host.rs:708` |
| sym-600f187e9e5d79c2b08e | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::UnexpectedMessage` | variant | Reported when the owning operation encounters unexpected message. | `src/runtime/lifecycle/sidecar_host.rs:710` |
| sym-c55b991f49f791f4aaf3 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::UnknownSidecar` | variant | Reported when the owning operation encounters unknown sidecar. | `src/runtime/lifecycle/sidecar_host.rs:732` |
| sym-67ea834795cfcbba8bd4 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarHostError::Wait` | variant | Reported when the owning operation encounters wait. | `src/runtime/lifecycle/sidecar_host.rs:726` |
| sym-0489d04d0ca1420e34f6 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Cancelling` | variant | Identifies the cancelling state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:28` |
| sym-18f54c3bc70c46e3665e | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/runtime/lifecycle/sidecar_host.rs:30` |
| sym-037931532e7bed56f7dc | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Closing` | variant | Identifies the closing state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:29` |
| sym-39f5331c3f88e64e5086 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Configure` | variant | Identifies the configure state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:25` |
| sym-61323ea30e9a661e176a | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Failed` | variant | Identifies the failed state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:32` |
| sym-0abe3d2d8c31e8b96ce0 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Hello` | variant | Identifies the hello state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:23` |
| sym-17d66413212a6b703b50 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Manifest` | variant | Identifies the manifest state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:24` |
| sym-09060fe7826f30079caf | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Ready` | variant | Identifies the ready state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:26` |
| sym-3022551f15bfaf1138a2 | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Reaped` | variant | Identifies the reaped state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:31` |
| sym-0663ae20dda48f9f385f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Running` | variant | Identifies the running state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:27` |
| sym-7010482897acf8e6ee1f | `pocketstation::runtime::lifecycle::sidecar_host::SidecarState::Spawned` | variant | Identifies the spawned state or stage represented by `SidecarState`. | `src/runtime/lifecycle/sidecar_host.rs:22` |
| sym-51b57731f5e340cd1634 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Cancel` | variant | Selects cancel behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:13` |
| sym-8685f91a4b12ed78ebcf | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Close` | variant | Selects close behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:14` |
| sym-e5eb02c89586571aadc2 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/runtime/lifecycle/sidecar_protocol.rs:19` |
| sym-e355a68ef7e8de2ea031 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Configure` | variant | Selects configure behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:17` |
| sym-24423e2deb943f4385f8 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Error` | variant | Selects error behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:12` |
| sym-f52eab9183ce02724f6c | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Hello` | variant | Selects hello behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:15` |
| sym-e0d096299cf8d95c765f | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Manifest` | variant | Selects manifest behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:16` |
| sym-ccde79d78571e6eaf6e8 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Observation` | variant | Selects observation behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:18` |
| sym-b93dd40b1c0e4b3de0c7 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Ready` | variant | Selects ready behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:11` |
| sym-54435c362738e4011a9e | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarMessageKind::Signal` | variant | Selects signal behavior for `SidecarMessageKind`. | `src/runtime/lifecycle/sidecar_protocol.rs:10` |
| sym-3c6823e252d934ea9b45 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::EmptySignalId` | variant | Reported when the owning operation encounters empty signal identifier. | `src/runtime/lifecycle/sidecar_protocol.rs:310` |
| sym-00f9995f340145225e02 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::FieldTooLarge` | variant | Reported when the owning operation encounters field too large. | `src/runtime/lifecycle/sidecar_protocol.rs:312` |
| sym-cd437cc0cdf3880b7a8b | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::FrameLengthOverflow` | variant | Reported when the owning operation encounters frame length overflow. | `src/runtime/lifecycle/sidecar_protocol.rs:320` |
| sym-e590f2b9de1c86c280b5 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::FrameTooLarge` | variant | Reported when the owning operation encounters frame too large. | `src/runtime/lifecycle/sidecar_protocol.rs:322` |
| sym-54e68a3ddeb5b6a1777a | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::InvalidMagic` | variant | Reported when the owning operation encounters invalid magic. | `src/runtime/lifecycle/sidecar_protocol.rs:298` |
| sym-325c206750aa94389852 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::InvalidTerminal` | variant | Reported when the owning operation encounters invalid terminal. | `src/runtime/lifecycle/sidecar_protocol.rs:306` |
| sym-6677cf244cae0d6cef71 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::InvalidUtf8` | variant | Reported when the owning operation encounters invalid UTF-8. | `src/runtime/lifecycle/sidecar_protocol.rs:318` |
| sym-4770ec5c1a866473a151 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::ReservedFieldSet` | variant | Reported when the owning operation encounters reserved field set. | `src/runtime/lifecycle/sidecar_protocol.rs:308` |
| sym-f0fe45820924a3a20d1b | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::TrailingBytes` | variant | Reported when the owning operation encounters trailing bytes. | `src/runtime/lifecycle/sidecar_protocol.rs:296` |
| sym-e26a626db34f2768e393 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::Truncated` | variant | Reported when the owning operation encounters truncated. | `src/runtime/lifecycle/sidecar_protocol.rs:294` |
| sym-b87c8c0afc58725d5731 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::UnknownMessageKind` | variant | Reported when the owning operation encounters unknown message kind. | `src/runtime/lifecycle/sidecar_protocol.rs:304` |
| sym-0e3ed7c980628abf5995 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::UnsupportedMajor` | variant | Reported when the owning operation encounters unsupported major. | `src/runtime/lifecycle/sidecar_protocol.rs:300` |
| sym-eb67e95c05a27cb70b51 | `pocketstation::runtime::lifecycle::sidecar_protocol::SidecarProtocolError::UnsupportedMinor` | variant | Reported when the owning operation encounters unsupported minor. | `src/runtime/lifecycle/sidecar_protocol.rs:302` |
| sym-976e36b3372040b0cc4d | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::CapacityTooLarge` | variant | Reported when the owning operation encounters capacity too large. | `src/runtime/signal/edge.rs:392` |
| sym-a65f80c2692dd16e1a0e | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::MissingPayloadLimit` | variant | Reported when the owning operation encounters missing payload limit. | `src/runtime/signal/edge.rs:397` |
| sym-88af7e152212d6b9e6a0 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::NoBranches` | variant | Reported when the owning operation encounters no branches. | `src/runtime/signal/edge.rs:388` |
| sym-d1d0ceec339c3c9b0195 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::PayloadLimitTooLarge` | variant | Reported when the owning operation encounters payload limit too large. | `src/runtime/signal/edge.rs:401` |
| sym-843e670f217265daa224 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::ZeroCapacity` | variant | Reported when the owning operation encounters zero capacity. | `src/runtime/signal/edge.rs:390` |
| sym-fccb6b2df489db9d50e1 | `pocketstation::runtime::signal::edge::TypedEdgeBuildError::ZeroPayloadLimit` | variant | Reported when the owning operation encounters zero payload limit. | `src/runtime/signal/edge.rs:399` |
| sym-2a67279d301e321c7dd0 | `pocketstation::runtime::signal::edge::TypedEdgePublishError::InvalidEnvelope` | variant | Reported when the owning operation encounters invalid envelope. | `src/runtime/signal/edge.rs:412` |
| sym-7f7f4d4f409b72813079 | `pocketstation::runtime::signal::edge::TypedEdgePublishError::NoBranches` | variant | Reported when the owning operation encounters no branches. | `src/runtime/signal/edge.rs:410` |
| sym-827adfdbc7393911e5ac | `pocketstation::runtime::signal::edge::TypedEdgePublishError::PayloadTooLarge` | variant | Reported when the owning operation encounters payload too large. | `src/runtime/signal/edge.rs:416` |
| sym-ca911d3c1aa546801052 | `pocketstation::runtime::signal::edge::TypedEdgePublishError::RequiredBranchFull` | variant | Reported when the owning operation encounters required branch full. | `src/runtime/signal/edge.rs:422` |
| sym-d3b9eb8d253801e092cb | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::AmbiguousOutputPort` | variant | Reported when the owning operation encounters ambiguous output port. | `src/runtime/signal/error.rs:36` |
| sym-c49f218687ad3ba7061d | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Cancel` | variant | Reported when the owning operation encounters cancel. | `src/runtime/signal/error.rs:20` |
| sym-0fe39be4f77dc254ffa2 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::CancelTimeout` | variant | Reported when the owning operation encounters cancel timeout. | `src/runtime/signal/error.rs:22` |
| sym-caa556ce5154892f7e92 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Close` | variant | Reported when the owning operation encounters close. | `src/runtime/signal/error.rs:16` |
| sym-ce3f9f7481b485f157fc | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::CloseTimeout` | variant | Reported when the owning operation encounters close timeout. | `src/runtime/signal/error.rs:18` |
| sym-c01a21d463db767ca8bc | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::DerivedLineageMismatch` | variant | Reported when the owning operation encounters derived lineage mismatch. | `src/runtime/signal/error.rs:26` |
| sym-9d034fd06224e471aef0 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::InvalidPlanInput` | variant | Reported when the owning operation encounters invalid plan input. | `src/runtime/signal/error.rs:50` |
| sym-c96a81da4125b3211b94 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Join` | variant | Reported when the owning operation encounters join. | `src/runtime/signal/error.rs:48` |
| sym-61b1cb562c5f193de25f | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::MissingDerivedLineage` | variant | Reported when the owning operation encounters missing derived lineage. | `src/runtime/signal/error.rs:24` |
| sym-e3da8e26b35c601d652a | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::MissingOutputContract` | variant | Reported when the owning operation encounters missing output contract. | `src/runtime/signal/error.rs:32` |
| sym-f7f7cd0573908c6e2ac1 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::OutputPayloadTooLarge` | variant | Reported when the owning operation encounters output payload too large. | `src/runtime/signal/error.rs:42` |
| sym-f3035a22c8af87dac56e | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::OutputSignalMismatch` | variant | Reported when the owning operation encounters output signal mismatch. | `src/runtime/signal/error.rs:28` |
| sym-ec6b0ed1f42c0c429713 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::PlanInputLineageMismatch` | variant | Reported when the owning operation encounters plan input lineage mismatch. | `src/runtime/signal/error.rs:52` |
| sym-0c06a0a1ad34895c8f1a | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Prepare` | variant | Reported when the owning operation encounters prepare. | `src/runtime/signal/error.rs:8` |
| sym-a865a23e9be7e2b32fd4 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::PrepareTimeout` | variant | Reported when the owning operation encounters prepare timeout. | `src/runtime/signal/error.rs:10` |
| sym-fdb953aadca95175a4aa | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Process` | variant | Reported when the owning operation encounters process. | `src/runtime/signal/error.rs:12` |
| sym-a1d21ad360bc28e4dc50 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::SharedAudioTypedInput` | variant | Reported when the owning operation encounters shared audio typed input. | `src/runtime/signal/error.rs:54` |
| sym-483a9770751464dd7e76 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::TerminalOutputDropped` | variant | Reported when the owning operation encounters terminal output dropped. | `src/runtime/signal/error.rs:38` |
| sym-679cb4268d1ac90acd2b | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::Timeout` | variant | Reported when the owning operation encounters timeout. | `src/runtime/signal/error.rs:14` |
| sym-35e5fa7e6c949ea662fd | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::UndeclaredOutputRole` | variant | Reported when the owning operation encounters undeclared output role. | `src/runtime/signal/error.rs:30` |
| sym-c5a1df69155b9148aa32 | `pocketstation::runtime::signal::error::AsyncOperatorWorkerError::UnknownInputPort` | variant | Reported when the owning operation encounters unknown input port. | `src/runtime/signal/error.rs:34` |
| sym-550b8068cb6f087336b3 | `pocketstation::session::compile::error::SessionCompileError::AmbiguousEndpointInput` | variant | Reported when the owning operation encounters ambiguous endpoint input. | `src/session/compile/error.rs:27` |
| sym-a26052449ffcac3205bf | `pocketstation::session::compile::error::SessionCompileError::AmbiguousOperatorPort` | variant | Reported when the owning operation encounters ambiguous operator port. | `src/session/compile/error.rs:32` |
| sym-e8578adc8a4f892c9ff7 | `pocketstation::session::compile::error::SessionCompileError::AudioBridgeOutputNotExclusive` | variant | Reported when the owning operation encounters audio bridge output not exclusive. | `src/session/compile/error.rs:57` |
| sym-145f9a1365fca07beb28 | `pocketstation::session::compile::error::SessionCompileError::DuplicateOperatorInputConnection` | variant | Reported when the owning operation encounters duplicate operator input connection. | `src/session/compile/error.rs:62` |
| sym-59543c5a2c841ad6fd10 | `pocketstation::session::compile::error::SessionCompileError::GraphCompile` | variant | Reported when the owning operation encounters graph compile. | `src/session/compile/error.rs:88` |
| sym-e40c19c649e89edc31e7 | `pocketstation::session::compile::error::SessionCompileError::InvalidAudioBridgeOutput` | variant | Reported when the owning operation encounters invalid audio bridge output. | `src/session/compile/error.rs:50` |
| sym-aa8b44bb426b88867d9e | `pocketstation::session::compile::error::SessionCompileError::InvalidExternalSourceConfiguration` | variant | Reported when the owning operation encounters invalid external source configuration. | `src/session/compile/error.rs:78` |
| sym-4dc749a72ea6dc06893c | `pocketstation::session::compile::error::SessionCompileError::InvalidSpec` | variant | Reported when the owning operation encounters invalid spec. | `src/session/compile/error.rs:9` |
| sym-974035d10f087c8ea028 | `pocketstation::session::compile::error::SessionCompileError::MissingRequiredOperatorInput` | variant | Reported when the owning operation encounters missing required operator input. | `src/session/compile/error.rs:43` |
| sym-722bf6db864f898826e2 | `pocketstation::session::compile::error::SessionCompileError::OperatorNodeTypeMismatch` | variant | Reported when the owning operation encounters operator node type mismatch. | `src/session/compile/error.rs:15` |
| sym-92611f3decc327f55be6 | `pocketstation::session::compile::error::SessionCompileError::RuntimePlan` | variant | Reported when the owning operation encounters runtime plan. | `src/session/compile/error.rs:90` |
| sym-9fc8990a295db7873f06 | `pocketstation::session::compile::error::SessionCompileError::UnknownAsyncOperator` | variant | Reported when the owning operation encounters unknown async operator. | `src/session/compile/error.rs:21` |
| sym-c1b55bf40ec3ff167015 | `pocketstation::session::compile::error::SessionCompileError::UnknownEndpointInputPort` | variant | Reported when the owning operation encounters unknown endpoint input port. | `src/session/compile/error.rs:83` |
| sym-a6a0f26233871b925e9e | `pocketstation::session::compile::error::SessionCompileError::UnknownEndpointNodeType` | variant | Reported when the owning operation encounters unknown endpoint node type. | `src/session/compile/error.rs:23` |
| sym-6bfa99e42901d2ef782e | `pocketstation::session::compile::error::SessionCompileError::UnknownExternalSource` | variant | Reported when the owning operation encounters unknown external source. | `src/session/compile/error.rs:69` |
| sym-afefbd1f1778d622faf6 | `pocketstation::session::compile::error::SessionCompileError::UnknownExternalSourceOutput` | variant | Reported when the owning operation encounters unknown external source output. | `src/session/compile/error.rs:73` |
| sym-c434a90fcc40328bc726 | `pocketstation::session::compile::error::SessionCompileError::UnknownOperator` | variant | Reported when the owning operation encounters unknown operator. | `src/session/compile/error.rs:11` |
| sym-ebe8f2935c734110d305 | `pocketstation::session::compile::error::SessionCompileError::UnknownOperatorPort` | variant | Reported when the owning operation encounters unknown operator port. | `src/session/compile/error.rs:37` |
| sym-7ec14078e3cb9c618e05 | `pocketstation::session::compile::error::SessionCompileError::UnknownSourceNodeType` | variant | Reported when the owning operation encounters unknown source node type. | `src/session/compile/error.rs:67` |
| sym-23f186145be6935d4121 | `pocketstation::session::declaration::selector::ApplicationSelector::BundleId` | variant | Selects bundle identifier behavior for `ApplicationSelector`. | `src/session/declaration/selector.rs:33` |
| sym-68ea7cf46927739abe63 | `pocketstation::session::declaration::selector::ApplicationSelector::Name` | variant | Selects name behavior for `ApplicationSelector`. | `src/session/declaration/selector.rs:40` |
| sym-fb28ce9597b80c322473 | `pocketstation::session::declaration::selector::ApplicationSelector::ProcessId` | variant | Selects process identifier behavior for `ApplicationSelector`. | `src/session/declaration/selector.rs:34` |
| sym-2145a4fc1ace5b6bd904 | `pocketstation::session::declaration::selector::ApplicationSelector::ProcessInstance` | variant | Selects process instance behavior for `ApplicationSelector`. | `src/session/declaration/selector.rs:35` |
| sym-6e16f75e18386247f425 | `pocketstation::session::declaration::selector::ApplicationSelector::StableId` | variant | Selects stable identifier behavior for `ApplicationSelector`. | `src/session/declaration/selector.rs:39` |
| sym-00bc0eb47fe5d167a352 | `pocketstation::session::declaration::selector::DeviceSelector::Default` | variant | Selects default behavior for `DeviceSelector`. | `src/session/declaration/selector.rs:108` |
| sym-f659d8b70dcb10104a94 | `pocketstation::session::declaration::selector::DeviceSelector::Id` | variant | Selects id behavior for `DeviceSelector`. | `src/session/declaration/selector.rs:109` |
| sym-1689b5f5eb0a35dcbab9 | `pocketstation::session::declaration::selector::Source::Application` | variant | Represents the application alternative defined by `Source`. | `src/session/declaration/selector.rs:135` |
| sym-a992b7020ffd985a2446 | `pocketstation::session::declaration::selector::Source::Microphone` | variant | Represents the microphone alternative defined by `Source`. | `src/session/declaration/selector.rs:136` |
| sym-d2e5b217855abbdc3679 | `pocketstation::session::declaration::spec::ConnectionTarget::EndpointInput` | variant | Represents the endpoint input alternative defined by `ConnectionTarget`. | `src/session/declaration/spec.rs:229` |
| sym-3f05194b4cbb38ff29c9 | `pocketstation::session::declaration::spec::ConnectionTarget::OperatorInput` | variant | Represents the operator input alternative defined by `ConnectionTarget`. | `src/session/declaration/spec.rs:225` |
| sym-541d673a0ad9f3daa2c9 | `pocketstation::session::declaration::spec::StreamOrigin::OperatorOutput` | variant | Represents the operator output alternative defined by `StreamOrigin`. | `src/session/declaration/spec.rs:216` |
| sym-f83ba277fce022e6e0d6 | `pocketstation::session::declaration::spec::StreamOrigin::SourceOutput` | variant | Represents the source output alternative defined by `StreamOrigin`. | `src/session/declaration/spec.rs:210` |
| sym-24cee880c4817242efb2 | `pocketstation::session::declaration::spec::StreamOrigin::Stem` | variant | Represents the stem alternative defined by `StreamOrigin`. | `src/session/declaration/spec.rs:209` |
| sym-23dd9de03b2837b70ced | `pocketstation::session::declaration::typed_stream::TypedStreamError::AmbiguousPort` | variant | Reported when the owning operation encounters ambiguous port. | `src/session/declaration/typed_stream.rs:203` |
| sym-13e35dfc4840b95a89d0 | `pocketstation::session::declaration::typed_stream::TypedStreamError::InputSignalMismatch` | variant | Reported when the owning operation encounters input signal mismatch. | `src/session/declaration/typed_stream.rs:205` |
| sym-0a7dc6fa24011a348d9c | `pocketstation::session::declaration::typed_stream::TypedStreamError::InvalidManifest` | variant | Reported when the owning operation encounters invalid manifest. | `src/session/declaration/typed_stream.rs:189` |
| sym-780b9bd2b96eb427f7e8 | `pocketstation::session::declaration::typed_stream::TypedStreamError::InvalidSignal` | variant | Reported when the owning operation encounters invalid signal. | `src/session/declaration/typed_stream.rs:187` |
| sym-b5b455f17000d1c4b22b | `pocketstation::session::declaration::typed_stream::TypedStreamError::MissingPort` | variant | Reported when the owning operation encounters missing port. | `src/session/declaration/typed_stream.rs:201` |
| sym-d8c7fa4ffb0d9e12f39e | `pocketstation::session::declaration::typed_stream::TypedStreamError::OperatorIdentityMismatch` | variant | Reported when the owning operation encounters operator identity mismatch. | `src/session/declaration/typed_stream.rs:191` |
| sym-3b877d188b0c7e90a859 | `pocketstation::session::declaration::typed_stream::TypedStreamError::OutputSignalMismatch` | variant | Reported when the owning operation encounters output signal mismatch. | `src/session/declaration/typed_stream.rs:207` |
| sym-ad754169198825bee927 | `pocketstation::session::declaration::typed_stream::TypedStreamError::Session` | variant | Reported when the owning operation encounters session. | `src/session/declaration/typed_stream.rs:213` |
| sym-0ecc76a05685262eb324 | `pocketstation::session::declaration::typed_stream::TypedStreamError::StemRequiresPcmAudio` | variant | Reported when the owning operation encounters stem requires PCM audio. | `src/session/declaration/typed_stream.rs:209` |
| sym-7dacd863e92923667192 | `pocketstation::session::declaration::typed_stream::TypedStreamError::StreamInputMismatch` | variant | Reported when the owning operation encounters stream input mismatch. | `src/session/declaration/typed_stream.rs:211` |
| sym-0508ab0214b87dca2f86 | `pocketstation::session::declaration::typed_stream::TypedStreamError::UnknownPort` | variant | Reported when the owning operation encounters unknown port. | `src/session/declaration/typed_stream.rs:196` |
| sym-f3dc93313bc0da2a539d | `pocketstation::session::error::SessionError::DraftFrozen` | variant | Reported when the owning operation encounters draft frozen. | `src/session/error.rs:36` |
| sym-905989f33005638d54e0 | `pocketstation::session::error::SessionError::DraftPoisoned` | variant | Reported when the owning operation encounters draft poisoned. | `src/session/error.rs:38` |
| sym-a1e8292ece976351f2fe | `pocketstation::session::error::SessionError::ForeignEndpoint` | variant | Reported when the owning operation encounters foreign endpoint. | `src/session/error.rs:31` |
| sym-6e844191705e153695b9 | `pocketstation::session::error::SessionError::IdExhausted` | variant | Reported when the owning operation encounters id exhausted. | `src/session/error.rs:40` |
| sym-166ed653a8c380f12a00 | `pocketstation::session::error::SessionError::InvalidEndpoint` | variant | Reported when the owning operation encounters invalid endpoint. | `src/session/error.rs:25` |
| sym-2dec4a3ccc10a96a0825 | `pocketstation::session::error::SessionError::InvalidOperator` | variant | Reported when the owning operation encounters invalid operator. | `src/session/error.rs:27` |
| sym-51a8c475ac02f3424883 | `pocketstation::session::error::SessionError::InvalidRoute` | variant | Reported when the owning operation encounters invalid route. | `src/session/error.rs:29` |
| sym-85c9d4937f20a74d09ca | `pocketstation::session::error::SessionError::InvalidSelector` | variant | Reported when the owning operation encounters invalid selector. | `src/session/error.rs:23` |
| sym-fd2a73f8633c94492510 | `pocketstation::session::error::SessionError::NoRoutes` | variant | Reported when the owning operation encounters no routes. | `src/session/error.rs:10` |
| sym-242ec9c386e9b1bdfc0c | `pocketstation::session::error::SessionError::NoSourceOutputRoutes` | variant | Reported when the owning operation encounters no source output routes. | `src/session/error.rs:18` |
| sym-dc3d38578399bbf7970a | `pocketstation::session::error::SessionError::NoSourceOutputs` | variant | Reported when the owning operation encounters no source outputs. | `src/session/error.rs:12` |
| sym-ecbd349d4132fcc50510 | `pocketstation::session::error::SessionError::NoSources` | variant | Reported when the owning operation encounters no sources. | `src/session/error.rs:8` |
| sym-c1ccdf4ec55b46f18684 | `pocketstation::session::error::SessionError::OperatorHasNoDestination` | variant | Reported when the owning operation encounters operator has no destination. | `src/session/error.rs:63` |
| sym-a30e23a51575678a8416 | `pocketstation::session::error::SessionError::UnknownEndpoint` | variant | Reported when the owning operation encounters unknown endpoint. | `src/session/error.rs:44` |
| sym-2532fe7a98759e7037a5 | `pocketstation::session::error::SessionError::UnknownOperatorInstance` | variant | Reported when the owning operation encounters unknown operator instance. | `src/session/error.rs:59` |
| sym-8f3926157b287634a314 | `pocketstation::session::error::SessionError::UnknownSourceInstance` | variant | Reported when the owning operation encounters unknown source instance. | `src/session/error.rs:48` |
| sym-d0b9054f30330b9ef1b7 | `pocketstation::session::error::SessionError::UnknownSourceOutput` | variant | Reported when the owning operation encounters unknown source output. | `src/session/error.rs:54` |
| sym-341c373bc37ab2d14580 | `pocketstation::session::error::SessionError::UnknownStem` | variant | Reported when the owning operation encounters unknown stem. | `src/session/error.rs:46` |
| sym-9478c00bf8e7bc87d307 | `pocketstation::session::error::SessionError::UnsupportedVersion` | variant | Reported when the owning operation encounters unsupported version. | `src/session/error.rs:42` |
| sym-3d2d15f3b2c485d16f7b | `pocketstation::session::error_code::PolledAudioPollErrorCode::Empty` | variant | Represents an empty value or collection. | `src/session/error_code.rs:132` |
| sym-aa318221e176fb48d6c8 | `pocketstation::session::error_code::PolledAudioPollErrorCode::InternalStateUnavailable` | variant | Reported when the owning operation encounters internal state unavailable. | `src/session/error_code.rs:134` |
| sym-420b5cb5203bfc3fe1bd | `pocketstation::session::error_code::PolledAudioPollErrorCode::LeaseCapacityExhausted` | variant | Reported when the owning operation encounters lease capacity exhausted. | `src/session/error_code.rs:133` |
| sym-802cb99819d5208d5f52 | `pocketstation::session::error_code::SessionDeclarationErrorCode::DraftFrozen` | variant | Reported when the owning operation encounters draft frozen. | `src/session/error_code.rs:19` |
| sym-9595951bd3fa272b6183 | `pocketstation::session::error_code::SessionDeclarationErrorCode::ForeignEndpoint` | variant | Reported when the owning operation encounters foreign endpoint. | `src/session/error_code.rs:18` |
| sym-22be7c01f2e6ee59c420 | `pocketstation::session::error_code::SessionDeclarationErrorCode::IdExhausted` | variant | Reported when the owning operation encounters id exhausted. | `src/session/error_code.rs:21` |
| sym-bb0079c1c4c981f137bf | `pocketstation::session::error_code::SessionDeclarationErrorCode::InternalStateUnavailable` | variant | Reported when the owning operation encounters internal state unavailable. | `src/session/error_code.rs:20` |
| sym-af7623cba95b2403e694 | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidEndpoint` | variant | Reported when the owning operation encounters invalid endpoint. | `src/session/error_code.rs:15` |
| sym-18d28e8c66c3e954274d | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidOperator` | variant | Reported when the owning operation encounters invalid operator. | `src/session/error_code.rs:16` |
| sym-49a6fb8bda1e417b9580 | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidRoute` | variant | Reported when the owning operation encounters invalid route. | `src/session/error_code.rs:17` |
| sym-5dcd9c588d3383491e4f | `pocketstation::session::error_code::SessionDeclarationErrorCode::InvalidSelector` | variant | Reported when the owning operation encounters invalid selector. | `src/session/error_code.rs:14` |
| sym-d0346ddeb620a6ae8ab9 | `pocketstation::session::error_code::SessionDeclarationErrorCode::NoRoutes` | variant | Reported when the owning operation encounters no routes. | `src/session/error_code.rs:12` |
| sym-2aebb816aeb84124315c | `pocketstation::session::error_code::SessionDeclarationErrorCode::NoSourceOutputs` | variant | Reported when the owning operation encounters no source outputs. | `src/session/error_code.rs:13` |
| sym-2dcfb624f44ae7916a2d | `pocketstation::session::error_code::SessionDeclarationErrorCode::NoSources` | variant | Reported when the owning operation encounters no sources. | `src/session/error_code.rs:11` |
| sym-cb29d9f1f9e2239b55f1 | `pocketstation::session::error_code::SessionDeclarationErrorCode::OperatorHasNoDestination` | variant | Reported when the owning operation encounters operator has no destination. | `src/session/error_code.rs:27` |
| sym-105b45a22c545f688fc2 | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownEndpoint` | variant | Reported when the owning operation encounters unknown endpoint. | `src/session/error_code.rs:23` |
| sym-c760c40445d47570062c | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownOperatorInstance` | variant | Reported when the owning operation encounters unknown operator instance. | `src/session/error_code.rs:26` |
| sym-827efa910ee73d794224 | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownSource` | variant | Reported when the owning operation encounters unknown source. | `src/session/error_code.rs:25` |
| sym-ee7251f2f8ca348a99f6 | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnknownStem` | variant | Reported when the owning operation encounters unknown stem. | `src/session/error_code.rs:24` |
| sym-29c50e735c65ddf5b2fe | `pocketstation::session::error_code::SessionDeclarationErrorCode::UnsupportedVersion` | variant | Reported when the owning operation encounters unsupported version. | `src/session/error_code.rs:22` |
| sym-962bb355e5bc351592de | `pocketstation::session::error_code::SessionRuntimeErrorCode::MissingMetricsSnapshot` | variant | Reported when the owning operation encounters missing metrics snapshot. | `src/session/error_code.rs:117` |
| sym-8a9539d5ebf5d0bfdbe6 | `pocketstation::session::error_code::SessionStartErrorCode::CaptureBackendFailed` | variant | Reported when the owning operation encounters capture backend failed. | `src/session/error_code.rs:76` |
| sym-5859df17e5a231c20709 | `pocketstation::session::error_code::SessionStartErrorCode::CapturePermissionDenied` | variant | Reported when the owning operation encounters capture permission denied. | `src/session/error_code.rs:73` |
| sym-124e6e5b33f8ad11d318 | `pocketstation::session::error_code::SessionStartErrorCode::CaptureSourceUnavailable` | variant | Reported when the owning operation encounters capture source unavailable. | `src/session/error_code.rs:74` |
| sym-60901e361744989824f7 | `pocketstation::session::error_code::SessionStartErrorCode::CaptureUnsupported` | variant | Reported when the owning operation encounters capture unsupported. | `src/session/error_code.rs:75` |
| sym-68be6ae6cf278abac85c | `pocketstation::session::error_code::SessionStartErrorCode::CompileFailed` | variant | Reported when the owning operation encounters compile failed. | `src/session/error_code.rs:67` |
| sym-782d3024e8826b1cc9bb | `pocketstation::session::error_code::SessionStartErrorCode::DeclarationInvalid` | variant | Reported when the owning operation encounters declaration invalid. | `src/session/error_code.rs:66` |
| sym-d5625bf54e08dd0cdfbd | `pocketstation::session::error_code::SessionStartErrorCode::EndpointPrepareFailed` | variant | Reported when the owning operation encounters endpoint prepare failed. | `src/session/error_code.rs:72` |
| sym-489c12fcd68ec525f256 | `pocketstation::session::error_code::SessionStartErrorCode::EndpointStartFailed` | variant | Reported when the owning operation encounters endpoint start failed. | `src/session/error_code.rs:77` |
| sym-084787497963f9b45769 | `pocketstation::session::error_code::SessionStartErrorCode::HostSetupFailed` | variant | Reported when the owning operation encounters host setup failed. | `src/session/error_code.rs:62` |
| sym-2365fbc7d2ac66a108da | `pocketstation::session::error_code::SessionStartErrorCode::InvalidSelector` | variant | Reported when the owning operation encounters invalid selector. | `src/session/error_code.rs:65` |
| sym-f3facdcdc0475e95fbe0 | `pocketstation::session::error_code::SessionStartErrorCode::InvalidStartOptions` | variant | Reported when the owning operation encounters invalid start options. | `src/session/error_code.rs:69` |
| sym-d54f8989c76261554bbe | `pocketstation::session::error_code::SessionStartErrorCode::MissingAudioReceipt` | variant | Reported when the owning operation encounters missing audio receipt. | `src/session/error_code.rs:79` |
| sym-ea70b5eb9d509e68ba81 | `pocketstation::session::error_code::SessionStartErrorCode::MissingEndpointDeclaration` | variant | Reported when the owning operation encounters missing endpoint declaration. | `src/session/error_code.rs:71` |
| sym-70c6f4eb30f77a010169 | `pocketstation::session::error_code::SessionStartErrorCode::MissingEventReceiver` | variant | Reported when the owning operation encounters missing event receiver. | `src/session/error_code.rs:81` |
| sym-4730acfef207adcced4e | `pocketstation::session::error_code::SessionStartErrorCode::MissingRecordingConfiguration` | variant | Reported when the owning operation encounters missing recording configuration. | `src/session/error_code.rs:80` |
| sym-df41a141bb5117137035 | `pocketstation::session::error_code::SessionStartErrorCode::RuntimePrepareFailed` | variant | Reported when the owning operation encounters runtime prepare failed. | `src/session/error_code.rs:68` |
| sym-7faa15c84ac58546349a | `pocketstation::session::error_code::SessionStartErrorCode::RuntimeStartFailed` | variant | Reported when the owning operation encounters runtime start failed. | `src/session/error_code.rs:78` |
| sym-3125f52d47a969fc3026 | `pocketstation::session::error_code::SessionStartErrorCode::StartCancelled` | variant | Reported when the owning operation encounters start cancelled. | `src/session/error_code.rs:64` |
| sym-5a7951ff95caeaa95ba7 | `pocketstation::session::error_code::SessionStartErrorCode::TraceRecorderSetupFailed` | variant | Reported when the owning operation encounters trace recorder setup failed. | `src/session/error_code.rs:82` |
| sym-77fd144bb7db9ca17da1 | `pocketstation::session::error_code::SessionStartErrorCode::UnsupportedPlatform` | variant | Reported when the owning operation encounters unsupported platform. | `src/session/error_code.rs:63` |
| sym-afe0f71d0ff4cc1c7396 | `pocketstation::session::error_code::SessionStartErrorCode::UnsupportedSourceTopology` | variant | Reported when the owning operation encounters unsupported source topology. | `src/session/error_code.rs:70` |
| sym-e4a69edcdd50ef3f2a7d | `pocketstation::session::error_code::SessionStopCode::AlreadyStopped` | variant | Indicates that the operation had already stopped. | `src/session/error_code.rs:152` |
| sym-76c45e79e4596c5bfab4 | `pocketstation::session::error_code::SessionStopCode::StopFailed` | variant | Represents the stop failed alternative defined by `SessionStopCode`. | `src/session/error_code.rs:153` |
| sym-c6c8eb9c36620e6f00de | `pocketstation::session::error_code::SessionStopCode::Stopped` | variant | Indicates that the operation stopped normally. | `src/session/error_code.rs:151` |
| sym-9356709d8339c37e3e8c | `pocketstation::session::error_code::SessionStopFailureCode::CaptureFinalizationFailed` | variant | Reported when the owning operation encounters capture finalization failed. | `src/session/error_code.rs:173` |
| sym-64d4aeceaaa2dcdde2ad | `pocketstation::session::error_code::SessionStopFailureCode::EndpointFinalizationFailed` | variant | Reported when the owning operation encounters endpoint finalization failed. | `src/session/error_code.rs:175` |
| sym-69c4f1750a89a79faeda | `pocketstation::session::error_code::SessionStopFailureCode::LineageFailed` | variant | Reported when the owning operation encounters lineage failed. | `src/session/error_code.rs:177` |
| sym-5082afec1e4a1d6c450e | `pocketstation::session::error_code::SessionStopFailureCode::OperatorFinalizationFailed` | variant | Reported when the owning operation encounters operator finalization failed. | `src/session/error_code.rs:174` |
| sym-9ac309b7b1b23b822c8c | `pocketstation::session::error_code::SessionStopFailureCode::RuntimeFailed` | variant | Reported when the owning operation encounters runtime failed. | `src/session/error_code.rs:176` |
| sym-51a46db0197a72e8cd58 | `pocketstation::session::error_code::SessionStopFailureCode::RuntimeWorkerPanicked` | variant | Reported when the owning operation encounters runtime worker panicked. | `src/session/error_code.rs:172` |
| sym-11a646ca4c32c6a5a204 | `pocketstation::session::error_code::SessionStopFailureCode::SourceSendRejected` | variant | Reported when the owning operation encounters source send rejected. | `src/session/error_code.rs:178` |
| sym-6a3095b332d838889a33 | `pocketstation::session::extensions::audio_input::AudioInputConfigError::FrameSampleCountOverflow` | variant | Reported when the owning operation encounters frame sample count overflow. | `src/session/extensions/audio_input/mod.rs:89` |
| sym-e7f2d3fcd2b0fa0824bb | `pocketstation::session::extensions::audio_input::AudioInputConfigError::InvalidCapacity` | variant | Reported when the owning operation encounters invalid capacity. | `src/session/extensions/audio_input/mod.rs:85` |
| sym-a39e47a52558c003921a | `pocketstation::session::extensions::audio_input::AudioInputConfigError::UnsupportedChannelCount` | variant | Reported when the owning operation encounters unsupported channel count. | `src/session/extensions/audio_input/mod.rs:81` |
| sym-e421bc8e8daaa2568422 | `pocketstation::session::extensions::audio_input::AudioInputConfigError::UnsupportedSampleFormat` | variant | Reported when the owning operation encounters unsupported sample format. | `src/session/extensions/audio_input/mod.rs:83` |
| sym-5a715f01db8e9bcf145c | `pocketstation::session::extensions::audio_input::AudioInputConfigError::ZeroFrameSamples` | variant | Reported when the owning operation encounters zero frame samples. | `src/session/extensions/audio_input/mod.rs:87` |
| sym-232adc97ae158ee7b4d8 | `pocketstation::session::extensions::audio_input::AudioInputConfigError::ZeroSampleRate` | variant | Reported when the owning operation encounters zero sample rate. | `src/session/extensions/audio_input/mod.rs:79` |
| sym-f7f5eab81d72a3bc4d7a | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Cancelled` | variant | Indicates that the operation was cancelled. | `src/session/extensions/audio_input/buffer.rs:277` |
| sym-3ba0f167d2cc29ffa9b0 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/session/extensions/audio_input/buffer.rs:275` |
| sym-ed9623a90093e929517b | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError::Full` | variant | Reports that bounded capacity is full. | `src/session/extensions/audio_input/buffer.rs:273` |
| sym-7b2abb8725c424dc8d50 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::Capacity` | variant | Reported when the owning operation encounters capacity. | `src/session/extensions/audio_input/buffer.rs:294` |
| sym-9562ca201ddf05420537 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::Empty` | variant | Represents an empty value or collection. | `src/session/extensions/audio_input/buffer.rs:285` |
| sym-b57c0cb028a1feb37e44 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::MisalignedChannels` | variant | Reported when the owning operation encounters misaligned channels. | `src/session/extensions/audio_input/buffer.rs:287` |
| sym-a0ee6146e9b897dd11a2 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::WrongFrameLength` | variant | Reported when the owning operation encounters wrong frame length. | `src/session/extensions/audio_input/buffer.rs:289` |
| sym-6742d86ee50081f87edd | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError::WrongSource` | variant | Reported when the owning operation encounters wrong source. | `src/session/extensions/audio_input/buffer.rs:283` |
| sym-6a6ab5db6570875f3b2e | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Cancelled` | variant | Indicates that the operation was cancelled. | `src/session/extensions/audio_input/buffer.rs:301` |
| sym-49dcee8d530703b0df85 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/session/extensions/audio_input/buffer.rs:300` |
| sym-4b3b1a33e195e3eb123c | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::Full` | variant | Reports that bounded capacity is full. | `src/session/extensions/audio_input/buffer.rs:299` |
| sym-a85940e42fcf56329589 | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind::InvalidBuffer` | variant | Selects invalid buffer behavior for `AudioInputWriteErrorKind`. | `src/session/extensions/audio_input/buffer.rs:302` |
| sym-bdc0f874b0e9b4d4f987 | `pocketstation::session::extensions::audio_input::source::AudioInputError::Configuration` | variant | Reported when the owning operation encounters configuration. | `src/session/extensions/audio_input/source.rs:87` |
| sym-24e1913e6033851b5b94 | `pocketstation::session::extensions::audio_input::source::AudioInputError::IncompatibleContract` | variant | Reported when the owning operation encounters incompatible contract. | `src/session/extensions/audio_input/source.rs:99` |
| sym-a10ec0dacb5c41480dd8 | `pocketstation::session::extensions::audio_input::source::AudioInputError::InstanceIdentityExhausted` | variant | Reported when the owning operation encounters instance identity exhausted. | `src/session/extensions/audio_input/source.rs:101` |
| sym-6171fefa62731db89660 | `pocketstation::session::extensions::audio_input::source::AudioInputError::Manifest` | variant | Reported when the owning operation encounters manifest. | `src/session/extensions/audio_input/source.rs:91` |
| sym-49cc44441dff06e5257b | `pocketstation::session::extensions::audio_input::source::AudioInputError::RegistrationStateUnavailable` | variant | Reported when the owning operation encounters registration state unavailable. | `src/session/extensions/audio_input/source.rs:95` |
| sym-70aff62cb8197d6072cb | `pocketstation::session::extensions::audio_input::source::AudioInputError::Session` | variant | Reported when the owning operation encounters session. | `src/session/extensions/audio_input/source.rs:93` |
| sym-8786d6de9139c0951f8d | `pocketstation::session::extensions::audio_input::source::AudioInputError::SourceTypeId` | variant | Reported when the owning operation encounters source type identifier. | `src/session/extensions/audio_input/source.rs:89` |
| sym-97625c1edb3e8d29b28f | `pocketstation::session::extensions::builtins::SessionGraphRegistrationError::DuplicateNodeType` | variant | Reported when the owning operation encounters duplicate node type. | `src/session/extensions/builtins.rs:32` |
| sym-40958b58444ac18b8f2a | `pocketstation::session::extensions::source::SourceDriverError::Failed` | variant | Reported when the owning operation encounters failed. | `src/session/extensions/source.rs:750` |
| sym-9304868a3e8bd4fadd26 | `pocketstation::session::extensions::source::SourceManifestError::DuplicateOutputName` | variant | Reported when the owning operation encounters duplicate output name. | `src/session/extensions/source.rs:689` |
| sym-b6d02ec391f150abf961 | `pocketstation::session::extensions::source::SourceManifestError::EmptyOutputName` | variant | Reported when the owning operation encounters empty output name. | `src/session/extensions/source.rs:687` |
| sym-a58546d89d0ebf641b9b | `pocketstation::session::extensions::source::SourceManifestError::EmptySourceTypeId` | variant | Reported when the owning operation encounters empty source type identifier. | `src/session/extensions/source.rs:679` |
| sym-3a90b6d5281d49bae890 | `pocketstation::session::extensions::source::SourceManifestError::InvalidSafetyContract` | variant | Reported when the owning operation encounters invalid safety contract. | `src/session/extensions/source.rs:695` |
| sym-5db2f6745114f09c0c85 | `pocketstation::session::extensions::source::SourceManifestError::InvalidSignal` | variant | Reported when the owning operation encounters invalid signal. | `src/session/extensions/source.rs:691` |
| sym-d2a7211904702eb9bd8c | `pocketstation::session::extensions::source::SourceManifestError::NoOutputs` | variant | Reported when the owning operation encounters no outputs. | `src/session/extensions/source.rs:683` |
| sym-e7df678c932a2ab2ef25 | `pocketstation::session::extensions::source::SourceManifestError::NonOutputPort` | variant | Reported when the owning operation encounters non output port. | `src/session/extensions/source.rs:685` |
| sym-2dcc821313f51e2cf324 | `pocketstation::session::extensions::source::SourceManifestError::SignalMediaMismatch` | variant | Reported when the owning operation encounters signal media mismatch. | `src/session/extensions/source.rs:693` |
| sym-6d323b0bead84181e834 | `pocketstation::session::extensions::source::SourceManifestError::UnsupportedExecutionPartition` | variant | Reported when the owning operation encounters unsupported execution partition. | `src/session/extensions/source.rs:697` |
| sym-534335f7d6fc6dca14cd | `pocketstation::session::extensions::source::SourceManifestError::ZeroVersion` | variant | Reported when the owning operation encounters zero version. | `src/session/extensions/source.rs:681` |
| sym-2a179f9c1ac1624096eb | `pocketstation::session::extensions::source::SourceRegistrationError::DuplicateSourceType` | variant | Reported when the owning operation encounters duplicate source type. | `src/session/extensions/source.rs:705` |
| sym-d96afeaec9322c513c47 | `pocketstation::session::extensions::source::SourceRegistrationError::InvalidManifest` | variant | Reported when the owning operation encounters invalid manifest. | `src/session/extensions/source.rs:703` |
| sym-3d2df707e8321afa33fd | `pocketstation::session::extensions::source::SourceRegistrationError::NodeTypeConflict` | variant | Reported when the owning operation encounters node type conflict. | `src/session/extensions/source.rs:707` |
| sym-de27d52f138e75145b25 | `pocketstation::session::extensions::source::SourceRuntimeError::AlreadyJoined` | variant | Reported when the owning operation encounters already joined. | `src/session/extensions/source.rs:784` |
| sym-0d31e30006e4343aea6a | `pocketstation::session::extensions::source::SourceRuntimeError::Continuity` | variant | Reported when the owning operation encounters continuity. | `src/session/extensions/source.rs:776` |
| sym-a1738819edc7150feb87 | `pocketstation::session::extensions::source::SourceRuntimeError::Driver` | variant | Reported when the owning operation encounters driver. | `src/session/extensions/source.rs:760` |
| sym-ad758e285ea15ad3ee3b | `pocketstation::session::extensions::source::SourceRuntimeError::EdgeBuild` | variant | Reported when the owning operation encounters edge build. | `src/session/extensions/source.rs:762` |
| sym-ff2273d7d0cb2dc450d0 | `pocketstation::session::extensions::source::SourceRuntimeError::InvalidConfiguration` | variant | Reported when the owning operation encounters invalid configuration. | `src/session/extensions/source.rs:758` |
| sym-285a26a5583db2042ceb | `pocketstation::session::extensions::source::SourceRuntimeError::InvalidManifest` | variant | Reported when the owning operation encounters invalid manifest. | `src/session/extensions/source.rs:756` |
| sym-d547e3aae623a050a8e4 | `pocketstation::session::extensions::source::SourceRuntimeError::MissingSessionLineage` | variant | Reported when the owning operation encounters missing session lineage. | `src/session/extensions/source.rs:772` |
| sym-057c464d6bd81f535e2c | `pocketstation::session::extensions::source::SourceRuntimeError::NoRoutedOutputs` | variant | Reported when the owning operation encounters no routed outputs. | `src/session/extensions/source.rs:764` |
| sym-22c6ef0f478490e306bf | `pocketstation::session::extensions::source::SourceRuntimeError::OutputContractMismatch` | variant | Reported when the owning operation encounters output contract mismatch. | `src/session/extensions/source.rs:770` |
| sym-9f5bb2e66aee52991c65 | `pocketstation::session::extensions::source::SourceRuntimeError::OutputIdentityMismatch` | variant | Reported when the owning operation encounters output identity mismatch. | `src/session/extensions/source.rs:774` |
| sym-50a3998f32565987a1b3 | `pocketstation::session::extensions::source::SourceRuntimeError::PreparedStateConsumed` | variant | Reported when the owning operation encounters prepared state consumed. | `src/session/extensions/source.rs:786` |
| sym-54bf4ac2edbf6c0b5f95 | `pocketstation::session::extensions::source::SourceRuntimeError::Publish` | variant | Reported when the owning operation encounters publish. | `src/session/extensions/source.rs:778` |
| sym-e328dc5206d8c63f9a2c | `pocketstation::session::extensions::source::SourceRuntimeError::Spawn` | variant | Reported when the owning operation encounters spawn. | `src/session/extensions/source.rs:780` |
| sym-2f42ff3090981cf5fd60 | `pocketstation::session::extensions::source::SourceRuntimeError::UnknownOutput` | variant | Reported when the owning operation encounters unknown output. | `src/session/extensions/source.rs:766` |
| sym-8d19f64eef7259ed0bdb | `pocketstation::session::extensions::source::SourceRuntimeError::UnregisteredSource` | variant | Reported when the owning operation encounters unregistered source. | `src/session/extensions/source.rs:788` |
| sym-1972b5e8932af8af4000 | `pocketstation::session::extensions::source::SourceRuntimeError::UnroutedOutput` | variant | Reported when the owning operation encounters unrouted output. | `src/session/extensions/source.rs:768` |
| sym-b3b269600137ba743c5e | `pocketstation::session::extensions::source::SourceRuntimeError::WorkerPanicked` | variant | Reported when the owning operation encounters worker panicked. | `src/session/extensions/source.rs:782` |
| sym-d77d9dd836c2c036f4b4 | `pocketstation::session::extensions::source::SourceTypeIdError::Empty` | variant | Represents an empty value or collection. | `src/session/extensions/source.rs:70` |
| sym-d66b4e8607c6b5f78b24 | `pocketstation::session::extensions::source::SourceTypeIdError::InvalidContractSyntax` | variant | Reported when the owning operation encounters invalid contract syntax. | `src/session/extensions/source.rs:81` |
| sym-dee5903526af46c8a99d | `pocketstation::session::extensions::source::SourceTypeIdError::MissingSourceCategory` | variant | Reported when the owning operation encounters missing source category. | `src/session/extensions/source.rs:83` |
| sym-0937863f43cb082b3a82 | `pocketstation::session::extensions::source::SourceTypeIdError::NonAscii` | variant | Reported when the owning operation encounters non ascii. | `src/session/extensions/source.rs:79` |
| sym-fb3bd5110bb9263dda52 | `pocketstation::session::extensions::source::SourceTypeIdError::SurroundingWhitespace` | variant | Reported when the owning operation encounters surrounding whitespace. | `src/session/extensions/source.rs:72` |
| sym-569386ab2a2bfb343c25 | `pocketstation::session::extensions::source::SourceTypeIdError::TooLong` | variant | Reported when the owning operation encounters too long. | `src/session/extensions/source.rs:74` |
| sym-5f5d747ecc78b360c6b1 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::ConflictingDefinition` | variant | Reported when the owning operation encounters conflicting definition. | `src/session/lifecycle/engine.rs:311` |
| sym-41a3cd3fd1ddce8ffb69 | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Definition` | variant | Reported when the owning operation encounters definition. | `src/session/lifecycle/engine.rs:307` |
| sym-b1f96461c73b56d614dc | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError::Driver` | variant | Reported when the owning operation encounters driver. | `src/session/lifecycle/engine.rs:309` |
| sym-a37607df7311f00c476f | `pocketstation::session::lifecycle::engine::SessionEngineBuildError::DuplicateSidecarId` | variant | Reported when the owning operation encounters duplicate sidecar identifier. | `src/session/lifecycle/engine.rs:301` |
| sym-2bca05c940891520d5e1 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError::InvalidConfiguration` | variant | Reported when the owning operation encounters invalid configuration. | `src/session/lifecycle/engine.rs:299` |
| sym-043f951bea8a3d480a8d | `pocketstation::session::lifecycle::engine::SessionEngineBuildError::StructuralNodeRegistration` | variant | Reported when the owning operation encounters structural node registration. | `src/session/lifecycle/engine.rs:297` |
| sym-316fd054e20a108cd37e | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Compile` | variant | Reported when the owning operation encounters compile. | `src/session/lifecycle/engine.rs:319` |
| sym-6357be5d29aa10c0ef86 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Freeze` | variant | Reported when the owning operation encounters freeze. | `src/session/lifecycle/engine.rs:317` |
| sym-4bcc23f3f146a5a06d32 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Prepare` | variant | Reported when the owning operation encounters prepare. | `src/session/lifecycle/engine.rs:321` |
| sym-3cf356558354b5f7a5f5 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Sidecar` | variant | Reported when the owning operation encounters sidecar. | `src/session/lifecycle/engine.rs:325` |
| sym-3d733667ef71b8579174 | `pocketstation::session::lifecycle::engine::SessionEngineStartError::Start` | variant | Reported when the owning operation encounters start. | `src/session/lifecycle/engine.rs:323` |
| sym-b4d9749f427105d432c9 | `pocketstation::session::lifecycle::events::SessionComponentId::Endpoint` | variant | Represents the endpoint alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:55` |
| sym-75266964b48626c4569a | `pocketstation::session::lifecycle::events::SessionComponentId::Operator` | variant | Represents the operator alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:59` |
| sym-27314c22d26d6c5eae83 | `pocketstation::session::lifecycle::events::SessionComponentId::Runtime` | variant | Represents the runtime alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:65` |
| sym-68e6cd1bbbed81180c6b | `pocketstation::session::lifecycle::events::SessionComponentId::Sidecar` | variant | Represents the sidecar alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:62` |
| sym-ec2d631b84a65475f492 | `pocketstation::session::lifecycle::events::SessionComponentId::Source` | variant | Represents the source alternative defined by `SessionComponentId`. | `src/session/lifecycle/events.rs:52` |
| sym-8df0d2c604b770ee6790 | `pocketstation::session::lifecycle::events::SessionEventKind::Endpoint` | variant | Identifies the endpoint state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:297` |
| sym-23532a8e7c1039802c2d | `pocketstation::session::lifecycle::events::SessionEventKind::Finalization` | variant | Identifies the finalization state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:299` |
| sym-7472094ceac74dd24318 | `pocketstation::session::lifecycle::events::SessionEventKind::Lifecycle` | variant | Identifies the lifecycle state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:295` |
| sym-6c48c0520e07f7fcb46a | `pocketstation::session::lifecycle::events::SessionEventKind::Rollback` | variant | Identifies the rollback state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:298` |
| sym-09289458a1ab0b49c890 | `pocketstation::session::lifecycle::events::SessionEventKind::Source` | variant | Identifies the source state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:296` |
| sym-0c09c5ff9540775946f9 | `pocketstation::session::lifecycle::events::SessionEventKind::Terminal` | variant | Identifies the terminal state or stage represented by `SessionEventKind`. | `src/session/lifecycle/events.rs:300` |
| sym-358e16ccc5dd9f3d4e4d | `pocketstation::session::lifecycle::events::SessionEventReceive::Closed` | variant | Reports that the underlying channel or resource is closed. | `src/session/lifecycle/events.rs:495` |
| sym-523acb21b435ad1d705e | `pocketstation::session::lifecycle::events::SessionEventReceive::Empty` | variant | Represents an empty value or collection. | `src/session/lifecycle/events.rs:494` |
| sym-5fd7196a6bf1e6d931f4 | `pocketstation::session::lifecycle::events::SessionEventReceive::Event` | variant | Identifies the event state or stage represented by `SessionEventReceive`. | `src/session/lifecycle/events.rs:493` |
| sym-bf2e899311996d80dafd | `pocketstation::session::lifecycle::events::SessionFinalizationStage::DrainOperator` | variant | Identifies the drain operator state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:42` |
| sym-46eb8c0b9e8a81c662d7 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::DrainRuntime` | variant | Identifies the drain runtime state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:41` |
| sym-bbdb175fabc7a8f85d33 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::DrainSidecar` | variant | Identifies the drain sidecar state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:46` |
| sym-0392105321d86d859cb7 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::FinalizeEndpoint` | variant | Identifies the finalize endpoint state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:45` |
| sym-d53691904838d426efba | `pocketstation::session::lifecycle::events::SessionFinalizationStage::JoinEndpoint` | variant | Identifies the join endpoint state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:44` |
| sym-d9f81d5de557b8c2e846 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::RequestEndpointStop` | variant | Identifies the request endpoint stop state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:43` |
| sym-8bc9da9cfab9a64a5771 | `pocketstation::session::lifecycle::events::SessionFinalizationStage::StopCapture` | variant | Identifies the stop capture state or stage represented by `SessionFinalizationStage`. | `src/session/lifecycle/events.rs:40` |
| sym-0da3bbba54a50b22b72e | `pocketstation::session::lifecycle::events::SessionLifecycleState::Failed` | variant | Identifies the failed state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:24` |
| sym-1611340716e5bf8d68fa | `pocketstation::session::lifecycle::events::SessionLifecycleState::Running` | variant | Identifies the running state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:21` |
| sym-9470a6ae79efa04f5b4a | `pocketstation::session::lifecycle::events::SessionLifecycleState::Starting` | variant | Identifies the starting state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:20` |
| sym-09ab4d5cc24c71030e51 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopped` | variant | Indicates that the operation stopped normally. | `src/session/lifecycle/events.rs:23` |
| sym-69a398ad2d2974e2d338 | `pocketstation::session::lifecycle::events::SessionLifecycleState::Stopping` | variant | Identifies the stopping state or stage represented by `SessionLifecycleState`. | `src/session/lifecycle/events.rs:22` |
| sym-4488651fea10cad7ecd4 | `pocketstation::session::lifecycle::events::SessionRollbackStage::CancelEndpointPreparation` | variant | Identifies the cancel endpoint preparation state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:31` |
| sym-e74d3605a8fb7c1f7e73 | `pocketstation::session::lifecycle::events::SessionRollbackStage::CancelOperator` | variant | Identifies the cancel operator state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:30` |
| sym-53d8592c683e2d57a923 | `pocketstation::session::lifecycle::events::SessionRollbackStage::DiscardRuntimeQueues` | variant | Identifies the discard runtime queues state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:34` |
| sym-eb6d3b5830751bed0a84 | `pocketstation::session::lifecycle::events::SessionRollbackStage::FinalizeStartedEndpoint` | variant | Identifies the finalize started endpoint state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:32` |
| sym-e8b27d4ffe54f2ce3ec6 | `pocketstation::session::lifecycle::events::SessionRollbackStage::StopOpenedCapture` | variant | Identifies the stop opened capture state or stage represented by `SessionRollbackStage`. | `src/session/lifecycle/events.rs:33` |
| sym-4a6989db48dac50e4862 | `pocketstation::session::lifecycle::events::SessionTerminalState::Failed` | variant | Identifies the failed state or stage represented by `SessionTerminalState`. | `src/session/lifecycle/events.rs:212` |
| sym-c85149f33f62894ce5ac | `pocketstation::session::lifecycle::events::SessionTerminalState::Stopped` | variant | Indicates that the operation stopped normally. | `src/session/lifecycle/events.rs:211` |
| sym-b89b81d5a3ea325a5b9b | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::EndpointExtensionRegistration` | variant | Reported when the owning operation encounters endpoint extension registration. | `src/session/lifecycle/host.rs:368` |
| sym-a2f1077a5fd42c71073e | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::EndpointRegistration` | variant | Reported when the owning operation encounters endpoint registration. | `src/session/lifecycle/host.rs:366` |
| sym-1c34cf5783ed746710a2 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::Engine` | variant | Reported when the owning operation encounters engine. | `src/session/lifecycle/host.rs:364` |
| sym-5092a4eed2c468dcfad9 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::MissingApplicationBackend` | variant | Reported when the owning operation encounters missing application backend. | `src/session/lifecycle/host.rs:374` |
| sym-50965a4b33230f186e32 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::MissingMicrophoneBackend` | variant | Reported when the owning operation encounters missing microphone backend. | `src/session/lifecycle/host.rs:376` |
| sym-2994978e231cd675ca7a | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::OperatorRegistration` | variant | Reported when the owning operation encounters operator registration. | `src/session/lifecycle/host.rs:370` |
| sym-815ae1d31c36513dec68 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::PolledAudioEndpoint` | variant | Reported when the owning operation encounters polled audio endpoint. | `src/session/lifecycle/host.rs:372` |
| sym-8615439393ffad17caa2 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError::UnsupportedPlatform` | variant | Reported when the owning operation encounters unsupported platform. | `src/session/lifecycle/host.rs:378` |
| sym-b25a0df9706a801372b4 | `pocketstation::session::lifecycle::observations::EndpointObservationStage::Finalized` | variant | Identifies the finalized state or stage represented by `EndpointObservationStage`. | `src/session/lifecycle/observations.rs:444` |
| sym-792d7ba1b9453cacb119 | `pocketstation::session::lifecycle::observations::EndpointObservationStage::Live` | variant | Identifies the live state or stage represented by `EndpointObservationStage`. | `src/session/lifecycle/observations.rs:443` |
| sym-362726e5721ce31c2e00 | `pocketstation::session::lifecycle::observations::EndpointObservationStage::Unavailable` | variant | Reports that the requested resource is unavailable. | `src/session/lifecycle/observations.rs:442` |
| sym-a7a99c72079736756a52 | `pocketstation::session::lifecycle::observations::SessionRouteLatencyBoundary::SourceMonotonicTimestampToRouteReceive` | variant | Represents the source monotonic timestamp to route receive alternative defined by `SessionRouteLatencyBoundary`. | `src/session/lifecycle/observations.rs:197` |
| sym-ad599e0a42aeee10acdd | `pocketstation::session::lifecycle::observations::SessionRouteLatencyUnit::Nanoseconds` | variant | Represents the nanoseconds alternative defined by `SessionRouteLatencyUnit`. | `src/session/lifecycle/observations.rs:202` |
| sym-e1579d311f7e15e5ab12 | `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval::RouteLifetimeToSnapshot` | variant | From route start through the instant of the Session snapshot. | `src/session/lifecycle/observations.rs:152` |
| sym-1371667a87c44c46c64b | `pocketstation::session::lifecycle::start_contract::SessionStartError::Cancelled` | variant | Indicates that the operation was cancelled. | `src/session/lifecycle/start_contract.rs:194` |
| sym-036bfcc1f80a3775ca7c | `pocketstation::session::lifecycle::start_contract::SessionStartError::CaptureOpen` | variant | Reported when the owning operation encounters capture open. | `src/session/lifecycle/start_contract.rs:165` |
| sym-1ca6b7611a389a2557b8 | `pocketstation::session::lifecycle::start_contract::SessionStartError::CapturePrepare` | variant | Reported when the owning operation encounters capture prepare. | `src/session/lifecycle/start_contract.rs:158` |
| sym-80fbfb6fd71659d71528 | `pocketstation::session::lifecycle::start_contract::SessionStartError::EndpointPrepare` | variant | Reported when the owning operation encounters endpoint prepare. | `src/session/lifecycle/start_contract.rs:152` |
| sym-0f317a3d50694de9123c | `pocketstation::session::lifecycle::start_contract::SessionStartError::EndpointStart` | variant | Reported when the owning operation encounters endpoint start. | `src/session/lifecycle/start_contract.rs:172` |
| sym-855e292a4fa0ad239346 | `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalAudioBridge` | variant | Reported when the owning operation encounters external audio bridge. | `src/session/lifecycle/start_contract.rs:124` |
| sym-957dfbb634f669c5f205 | `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalSourcePrepare` | variant | Reported when the owning operation encounters external source prepare. | `src/session/lifecycle/start_contract.rs:119` |
| sym-e49825f4402e82008395 | `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalSourceStart` | variant | Reported when the owning operation encounters external source start. | `src/session/lifecycle/start_contract.rs:134` |
| sym-de7a3476b7682acda601 | `pocketstation::session::lifecycle::start_contract::SessionStartError::GeneratedAudioBridge` | variant | Reported when the owning operation encounters generated audio bridge. | `src/session/lifecycle/start_contract.rs:129` |
| sym-e47e1489dcc75cb5b78d | `pocketstation::session::lifecycle::start_contract::SessionStartError::InvalidOptions` | variant | Reported when the owning operation encounters invalid options. | `src/session/lifecycle/start_contract.rs:115` |
| sym-41153325bac4d63bd4ee | `pocketstation::session::lifecycle::start_contract::SessionStartError::MissingEndpointDeclaration` | variant | Reported when the owning operation encounters missing endpoint declaration. | `src/session/lifecycle/start_contract.rs:150` |
| sym-a0df2633a5bc66b4a0b1 | `pocketstation::session::lifecycle::start_contract::SessionStartError::OperatorPrepare` | variant | Reported when the owning operation encounters operator prepare. | `src/session/lifecycle/start_contract.rs:144` |
| sym-403edca1b9aa39fa981a | `pocketstation::session::lifecycle::start_contract::SessionStartError::OperatorRuntimeHost` | variant | Reported when the owning operation encounters operator runtime host. | `src/session/lifecycle/start_contract.rs:139` |
| sym-3f41d31996b76eb4a966 | `pocketstation::session::lifecycle::start_contract::SessionStartError::RuntimeRunner` | variant | Reported when the owning operation encounters runtime runner. | `src/session/lifecycle/start_contract.rs:178` |
| sym-12d8b236b8168983e39d | `pocketstation::session::lifecycle::start_contract::SessionStartError::RuntimeWorkerReady` | variant | Reported when the owning operation encounters runtime worker ready. | `src/session/lifecycle/start_contract.rs:189` |
| sym-bbe1b72588e546399ac0 | `pocketstation::session::lifecycle::start_contract::SessionStartError::RuntimeWorkerSpawn` | variant | Reported when the owning operation encounters runtime worker spawn. | `src/session/lifecycle/start_contract.rs:184` |
| sym-b30b321e510a44232114 | `pocketstation::session::lifecycle::start_contract::SessionStartError::UnsupportedSourceTopology` | variant | Reported when the owning operation encounters unsupported source topology. | `src/session/lifecycle/start_contract.rs:117` |
| sym-2e2e6912da652a00824c | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::EndpointFailure` | variant | Selects endpoint failure behavior for `SessionTraceRecordKind`. | `src/session/lifecycle/trace.rs:34` |
| sym-4ad63a8953d5cbf8a7b7 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::FinalizationFailure` | variant | Selects finalization failure behavior for `SessionTraceRecordKind`. | `src/session/lifecycle/trace.rs:42` |
| sym-280c4a3c96e87108fca8 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::Lifecycle` | variant | Selects lifecycle behavior for `SessionTraceRecordKind`. | `src/session/lifecycle/trace.rs:28` |
| sym-6ec2bfcbc961fe3e282d | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::RollbackFailure` | variant | Selects rollback failure behavior for `SessionTraceRecordKind`. | `src/session/lifecycle/trace.rs:39` |
| sym-d665031ffcca9e799e27 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::SourceFailure` | variant | Selects source failure behavior for `SessionTraceRecordKind`. | `src/session/lifecycle/trace.rs:31` |
| sym-427abc4bd3633a198897 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind::Terminal` | variant | Selects terminal behavior for `SessionTraceRecordKind`. | `src/session/lifecycle/trace.rs:45` |
| sym-7bef46d2b95f677d16bb | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError::ChannelClosed` | variant | Reported when the owning operation encounters channel closed. | `src/session/lifecycle/trace.rs:100` |
| sym-1ba74d23edad5e50cd6b | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError::Io` | variant | Reported when the owning operation encounters I/O. | `src/session/lifecycle/trace.rs:104` |
| sym-9abaad88f1ae50ce7f0e | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError::WorkerPanicked` | variant | Reported when the owning operation encounters worker panicked. | `src/session/lifecycle/trace.rs:102` |
| sym-f2f61e6081b8e0b5c3d4 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError::Io` | variant | Reported when the owning operation encounters I/O. | `src/session/lifecycle/trace.rs:94` |
| sym-164e3e7643ba06a20d8b | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError::OutputExists` | variant | Reported when the owning operation encounters output exists. | `src/session/lifecycle/trace.rs:92` |
| sym-26bfc71dd44381bc2659 | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError::ZeroCapacity` | variant | Reported when the owning operation encounters zero capacity. | `src/session/lifecycle/trace.rs:90` |
| sym-43896d035dda8782c697 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::IncompleteTrace` | variant | Reported when the owning operation encounters incomplete trace. | `src/session/lifecycle/trace.rs:370` |
| sym-9e432310317609d64e86 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidChecksum` | variant | Reported when the owning operation encounters invalid checksum. | `src/session/lifecycle/trace.rs:368` |
| sym-bd2b266c08c627d72b5a | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidLayout` | variant | Reported when the owning operation encounters invalid layout. | `src/session/lifecycle/trace.rs:364` |
| sym-d9780411753fdcaf203c | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidLifecycleTransition` | variant | Reported when the owning operation encounters invalid lifecycle transition. | `src/session/lifecycle/trace.rs:378` |
| sym-1bb596f949b223ff506b | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::InvalidMagic` | variant | Reported when the owning operation encounters invalid magic. | `src/session/lifecycle/trace.rs:360` |
| sym-cbe188d50e544d0f91a8 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::Io` | variant | Reported when the owning operation encounters I/O. | `src/session/lifecycle/trace.rs:358` |
| sym-9589a16fd490782055ba | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::MissingTerminal` | variant | Reported when the owning operation encounters missing terminal. | `src/session/lifecycle/trace.rs:380` |
| sym-9589431bb634aedddcfb | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::RecordAfterTerminal` | variant | Reported when the owning operation encounters record after terminal. | `src/session/lifecycle/trace.rs:384` |
| sym-318496a50aed9d576e13 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::SequenceGap` | variant | Reported when the owning operation encounters sequence gap. | `src/session/lifecycle/trace.rs:372` |
| sym-43f9ff02bbcb6cd20e26 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::SessionMismatch` | variant | Reported when the owning operation encounters session mismatch. | `src/session/lifecycle/trace.rs:374` |
| sym-11743858bff6687233e0 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::TerminalMismatch` | variant | Reported when the owning operation encounters terminal mismatch. | `src/session/lifecycle/trace.rs:382` |
| sym-ba6c793fa512b7edc066 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::TimestampRegression` | variant | Reported when the owning operation encounters timestamp regression. | `src/session/lifecycle/trace.rs:376` |
| sym-7e2edcce89f731f7a608 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::Truncated` | variant | Reported when the owning operation encounters truncated. | `src/session/lifecycle/trace.rs:366` |
| sym-abfd75f025a8a41b8230 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::UnknownRecordType` | variant | Reported when the owning operation encounters unknown record type. | `src/session/lifecycle/trace.rs:386` |
| sym-f3fbb6556f8daf6f0365 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError::UnsupportedVersion` | variant | Reported when the owning operation encounters unsupported version. | `src/session/lifecycle/trace.rs:362` |
| sym-f508af93b2e86089e57d | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateOperatorInput` | variant | Reported when the owning operation encounters duplicate operator input. | `src/session/prepare/error.rs:76` |
| sym-0a1575040f6c83aabe19 | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateSignalRoute` | variant | Reported when the owning operation encounters duplicate signal route. | `src/session/prepare/error.rs:80` |
| sym-0fabebd81a18ff56ce8f | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateSourceNode` | variant | Reported when the owning operation encounters duplicate source node. | `src/session/prepare/error.rs:17` |
| sym-3f779950f9c1900f2ad0 | `pocketstation::session::prepare::error::SessionPrepareError::DuplicateWorkerRoute` | variant | Reported when the owning operation encounters duplicate worker route. | `src/session/prepare/error.rs:53` |
| sym-a352aff0913e532e1254 | `pocketstation::session::prepare::error::SessionPrepareError::IncompatibleNodeBinding` | variant | Reported when the owning operation encounters incompatible node binding. | `src/session/prepare/error.rs:74` |
| sym-2abc1184afe29f553d03 | `pocketstation::session::prepare::error::SessionPrepareError::InvalidExternalAudioMedia` | variant | Reported when the owning operation encounters invalid external audio media. | `src/session/prepare/error.rs:26` |
| sym-14f1956ec235b9220419 | `pocketstation::session::prepare::error::SessionPrepareError::InvalidGeneratedAudioMedia` | variant | Reported when the owning operation encounters invalid generated audio media. | `src/session/prepare/error.rs:33` |
| sym-c65a4766c944c8cdd847 | `pocketstation::session::prepare::error::SessionPrepareError::InvalidOperatorInputPort` | variant | Reported when the owning operation encounters invalid operator input port. | `src/session/prepare/error.rs:49` |
| sym-4a175cadbe6f9af9c869 | `pocketstation::session::prepare::error::SessionPrepareError::MissingAsyncOperatorFactory` | variant | Reported when the owning operation encounters missing async operator factory. | `src/session/prepare/error.rs:68` |
| sym-c1d25d1a6efb72ce1667 | `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalAudioIngress` | variant | Reported when the owning operation encounters missing external audio ingress. | `src/session/prepare/error.rs:19` |
| sym-7b750ae0edc9bb9913d7 | `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalSourceDefinition` | variant | Reported when the owning operation encounters missing external source definition. | `src/session/prepare/error.rs:24` |
| sym-5b4ff26f64519f919d5c | `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalSourceRouteEdge` | variant | Reported when the owning operation encounters missing external source route edge. | `src/session/prepare/error.rs:37` |
| sym-cd39473b69fbcb5568f1 | `pocketstation::session::prepare::error::SessionPrepareError::MissingGeneratedAudioBridge` | variant | Reported when the owning operation encounters missing generated audio bridge. | `src/session/prepare/error.rs:35` |
| sym-d16bb7149a6f812987a9 | `pocketstation::session::prepare::error::SessionPrepareError::MissingGeneratedAudioIngress` | variant | Reported when the owning operation encounters missing generated audio ingress. | `src/session/prepare/error.rs:31` |
| sym-49947c3fd97f3242144a | `pocketstation::session::prepare::error::SessionPrepareError::MissingNodeBinding` | variant | Reported when the owning operation encounters missing node binding. | `src/session/prepare/error.rs:72` |
| sym-041b72949098b1526917 | `pocketstation::session::prepare::error::SessionPrepareError::MissingOperatorSignalInput` | variant | Reported when the owning operation encounters missing operator signal input. | `src/session/prepare/error.rs:78` |
| sym-65871c9d88376664de39 | `pocketstation::session::prepare::error::SessionPrepareError::MissingSourceNode` | variant | Reported when the owning operation encounters missing source node. | `src/session/prepare/error.rs:15` |
| sym-aca4193faede253ef8c3 | `pocketstation::session::prepare::error::SessionPrepareError::MissingTypedEdgePlan` | variant | Reported when the owning operation encounters missing typed edge plan. | `src/session/prepare/error.rs:66` |
| sym-0a414271d573ecad33d0 | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerCapacity` | variant | Reported when the owning operation encounters missing worker capacity. | `src/session/prepare/error.rs:45` |
| sym-ba7d492f23d8d501a8ac | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerEdge` | variant | Reported when the owning operation encounters missing worker edge. | `src/session/prepare/error.rs:41` |
| sym-77bc8d48a4a6327a8beb | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerEdgeContract` | variant | Reported when the owning operation encounters missing worker edge contract. | `src/session/prepare/error.rs:43` |
| sym-715f1a6e4aa40d79d2c4 | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerSampleSpec` | variant | Reported when the owning operation encounters missing worker sample spec. | `src/session/prepare/error.rs:47` |
| sym-3af6125f293a8880207e | `pocketstation::session::prepare::error::SessionPrepareError::MissingWorkerTarget` | variant | Reported when the owning operation encounters missing worker target. | `src/session/prepare/error.rs:39` |
| sym-7815363cf4d7a8af622c | `pocketstation::session::prepare::error::SessionPrepareError::OperatorDeclarationMismatch` | variant | Reported when the owning operation encounters operator declaration mismatch. | `src/session/prepare/error.rs:70` |
| sym-8b5e4b04d53928e8cb3c | `pocketstation::session::prepare::error::SessionPrepareError::Runtime` | variant | Reported when the owning operation encounters runtime. | `src/session/prepare/error.rs:11` |
| sym-7b2107fb903dba41a176 | `pocketstation::session::prepare::error::SessionPrepareError::SignalRouteMismatch` | variant | Reported when the owning operation encounters signal route mismatch. | `src/session/prepare/error.rs:82` |
| sym-8d367e4d7d59da3b36f0 | `pocketstation::session::prepare::error::SessionPrepareError::SourceChannel` | variant | Reported when the owning operation encounters source channel. | `src/session/prepare/error.rs:13` |
| sym-9dc0e7d24a990c3b9934 | `pocketstation::session::prepare::error::SessionPrepareError::UnknownWorkerRoute` | variant | Reported when the owning operation encounters unknown worker route. | `src/session/prepare/error.rs:51` |
| sym-84c8e84e37ce62f8ca06 | `pocketstation::session::prepare::error::SessionPrepareError::WorkerRouteMismatch` | variant | Reported when the owning operation encounters worker route mismatch. | `src/session/prepare/error.rs:57` |
| sym-fe85bb8cd071d3fbd05e | `pocketstation::session::prepare::error::SessionPrepareError::WorkerTopologyMismatch` | variant | Reported when the owning operation encounters worker topology mismatch. | `src/session/prepare/error.rs:86` |

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

The claims on **Rust API reference** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/lib.rs:1-1129` (`DIRECT`)

For **Rust API reference**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
