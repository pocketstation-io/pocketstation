# Session API

<!-- claims: CLM-REF-002-CAP-001,CLM-REF-002-CAP-002,CLM-REF-002-CAP-003,CLM-REF-002-CAP-004,CLM-REF-002-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

The scope of **Session API** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Reference authority

For **Session API**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier.

## Public surface

| Evidence record | Declaration | Kind | Purpose | Source |
|---|---|---|---|---|
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
| sym-617d7fa081197246ac8a | `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| sym-b93a63c9f08d180a1385 | `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| sym-1791481d286846241ac9 | `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| sym-2e8c31657a884516d1db | `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| sym-6a128ee236756cc8905e | `SourceFactory::manifest` | function | Returns the manifest held by `SourceFactory`. | `src/session/extensions/source.rs:277` |
| sym-6cd2abb6461a40ec9b16 | `SourceFactory::validate_config` | function | Validates config for `SourceFactory`. | `src/session/extensions/source.rs:278` |
| sym-5d1bf5166addc34ce5d2 | `StreamSignal::signal_spec` | function | Returns the signal spec held by `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| sym-ef79fdde27b96aa64bd9 | `application` | function | Returns the application held by `Source`. | `src/session/declaration/selector.rs:140` |
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
| sym-aad1ecb21c3c011a267f | `browser` | function | Returns the browser associated with `Session`. | `src/session/declaration/draft.rs:443` |
| sym-9bf522ae209685bd39bf | `build` | function | Consumes all setup state so no partially populated registry can escape. | `src/session/lifecycle/engine.rs:176` |
| sym-2d52a1225970e7513f70 | `build` | function | Builds its owned operation for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:344` |
| sym-8dcf1610f287d6f7139c | `bundle_id` | function | Returns the bundle identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| sym-07d5fd0f7862cc3e0ddf | `cancel` | function | Requests cancellation of `SourceRuntime`. | `src/session/extensions/source.rs:575` |
| sym-cd665b6efd7a7a26e678 | `cancel` | function | Requests cancellation of `RunningSession`. | `src/session/lifecycle/running.rs:417` |
| sym-cd9034aece59b4e2f5ad | `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| sym-6a03bc81241e552b45a7 | `cancellation_total` | function | Returns the cancellation total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| sym-0347dd7ca21a781e74ea | `capacity_frames` | function | Returns the capacity frames held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| sym-53e7fa01c1555e29c482 | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:347` |
| sym-8f46c10e198bfff05070 | `capture_finalization_failures_total` | function | Returns the capture finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:330` |
| sym-5974737f23bbd935c441 | `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| sym-fe6bbea7f6396ae52e3f | `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| sym-9473a946f8e1e9763a7f | `compile` | function | Compiles its owned operation for `SessionCompiler`. | `src/session/compile/mod.rs:103` |
| sym-799be876bf20539f034a | `compile` | function | Compiles its owned operation for `SessionEngine`. | `src/session/lifecycle/engine.rs:221` |
| sym-dc5030b6c84ca3e2a8be | `compile` | function | Compiles its owned operation for `SessionEngineHost`. | `src/session/lifecycle/host.rs:55` |
| sym-57ec0512402f2f8c6afb | `component` | function | Returns the component associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:89` |
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
| sym-f4f0b73539dae0c559db | `connector_id` | function | Returns the connector identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:607` |
| sym-cf7d333c61a83867f2a0 | `connector_id` | function | Returns the connector identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:175` |
| sym-54fecfe30fb4f38fba2b | `declares_multistem_recording` | function | Returns whether `Session` declares multistem recording. | `src/session/extensions/recording.rs:98` |
| sym-9c03c0728aa593e1d73f | `default` | function | Returns the default `Session` value. | `src/session/declaration/draft.rs:577` |
| sym-b4a0bbefbec170268134 | `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| sym-fbbdd70b9d27a4341930 | `default` | function | Returns the default `NativeSessionEngineHostOptions` value. | `src/session/lifecycle/host.rs:172` |
| sym-7d2662192d98d7cb188f | `default` | function | Returns the default `SessionStartOptions` value. | `src/session/lifecycle/start_contract.rs:33` |
| sym-12355a31167104ea1ffd | `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| sym-0a891da3911ec35e5ef0 | `derived_route_count` | function | Returns the derived route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| sym-f48d91e269c7742941c0 | `derived_route_metrics` | function | Returns the derived route metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:213` |
| sym-f3973e0a054b591131ff | `drop` | function | Releases resources owned by `PreparedSourceRuntime`. | `src/session/extensions/source.rs:556` |
| sym-83f894af5771b0d83fef | `drop` | function | Releases resources owned by `SourceRuntime`. | `src/session/extensions/source.rs:594` |
| sym-b9e30ae1a157e2696385 | `drop` | function | Releases resources owned by `RunningSession`. | `src/session/lifecycle/running.rs:607` |
| sym-ff8e4ef4847b0ab0bcfd | `drop` | function | Releases resources owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:249` |
| sym-227ae9201f38ed2ea882 | `drop_observations` | function | Returns the drop observations held by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:206` |
| sym-4bfaa030ceda41db736b | `drop_rate_pct` | function | Returns the drop rate pct held by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:171` |
| sym-a1b37e1c7532d4731a9a | `edge_count` | function | Returns the edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:52` |
| sym-7ee11f80ffee60ed1ac8 | `endpoint` | function | Declares an endpoint on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:406` |
| sym-af23e03424aef049afd6 | `endpoint_declarations` | function | Returns the endpoint declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:42` |
| sym-d6fa3d41c3b036a454b2 | `endpoint_failures` | function | Returns the endpoint failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:279` |
| sym-a85ec1d660da506d3778 | `endpoint_finalization_failures_total` | function | Returns the endpoint finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:334` |
| sym-891dd071941cfeda08be | `endpoint_id` | function | Returns the endpoint identifier held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:150` |
| sym-8d63ce59b3bf9526f8bc | `endpoint_id` | function | Returns the endpoint identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:253` |
| sym-59145b978f237fdd96f9 | `endpoints` | function | Returns the endpoints associated with `SessionSpec`. | `src/session/declaration/spec.rs:339` |
| sym-c43653a3d9e4adb903ba | `engine` | function | Returns the engine associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:158` |
| sym-8a798becca22d42621db | `engine_builder` | function | Borrows the mutable engine builder owned by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:277` |
| sym-a72f5fdbce5729e7a88e | `error` | function | Returns the error associated with `SessionStartFailure`. | `src/session/lifecycle/start_contract.rs:278` |
| sym-1c652c741942e93ae3e7 | `error_class` | function | Returns the error class associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:97` |
| sym-f3a21262c49f983d794b | `event` | function | Returns the event associated with `SessionSourceFailure`. | `src/session/lifecycle/events.rs:118` |
| sym-f3d0f8aafd878c92c09e | `event_queue` | function | Returns the event queue associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:67` |
| sym-f8fa19026eb6310fea3d | `execution` | function | Returns the execution held by `SourceManifest`. | `src/session/extensions/source.rs:174` |
| sym-21b76e2d1246ca5b5daa | `external_source` | function | Returns the external source held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:87` |
| sym-abb9da13f404a699b9e5 | `external_source_count` | function | Returns the external source count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:83` |
| sym-eba44dfab1bc80191141 | `external_source_declarations` | function | Returns the external source declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:37` |
| sym-e674d51e49a4a20a99ed | `external_source_metrics` | function | Returns the external source metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:209` |
| sym-0543e802fc11f7a0a88f | `failure` | function | Returns the failure held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:158` |
| sym-25581e9181e17ff4fd33 | `failure` | function | Returns the failure held by `SessionRollbackFailure`. | `src/session/lifecycle/events.rs:179` |
| sym-9de3fdac7f06790f4286 | `failure` | function | Returns the failure held by `SessionFinalizationFailure`. | `src/session/lifecycle/events.rs:203` |
| sym-fcd9400e7f3ed4c3bf83 | `finalization_failures` | function | Returns the finalization failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:287` |
| sym-926b2b8d7c23849e658d | `finish` | function | Finishes work owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:201` |
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
| sym-4a326d4b77f2919cddc5 | `frame_capacity_samples` | function | Returns the frame capacity samples held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:340` |
| sym-fc7d805f26f6a9dfbf27 | `frame_samples_per_channel` | function | Returns the frame samples per channel associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:67` |
| sym-9d53c012f65576e10641 | `freeze` | function | Freezes mutable storage owned by `Session` into its shared immutable form. | `src/session/declaration/draft.rs:466` |
| sym-0fcf855969ace6f0828f | `from` | function | Converts the supplied value into `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:325` |
| sym-e1053a7046106d7c1ce4 | `from_source_output` | function | Wraps a public external-source output in the same typed Rust façade. Runtime identity remains the output's stable `SignalSpec` and schema. | `src/session/declaration/typed_stream.rs:118` |
| sym-1079adfc5870e06d1bc3 | `from_stem` | function | Creates `Stream` from stem. | `src/session/declaration/typed_stream.rs:103` |
| sym-4d24e0374ad96fee281d | `generated_audio_ingresses` | function | Returns the generated audio ingresses associated with `SessionSpec`. | `src/session/declaration/spec.rs:335` |
| sym-1e34f2399e99837264ce | `generation` | function | Returns the implementation generation. | `src/session/extensions/source.rs:166` |
| sym-fd78059adf00a80f26ff | `get` | function | Returns the value held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:60` |
| sym-7ec0100b785ff9974c5a | `get` | function | Returns the value held by `ProcessId`. | `src/session/declaration/selector.rs:13` |
| sym-ba96a13281a8ccf9fdfc | `get` | function | Returns the value held by `SourceConfiguration`. | `src/session/extensions/source.rs:100` |
| sym-91ef7083bf48f44efe44 | `handle` | function | Returns the handle associated with `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:197` |
| sym-cf85afd4d1673c928bfb | `id` | function | Returns the id held by `Session`. | `src/session/declaration/draft.rs:343` |
| sym-bde0a2263b883d2a02a8 | `id` | function | Returns the id held by `EndpointHandle`. | `src/session/declaration/draft.rs:603` |
| sym-38ad6acc195ce4b793a7 | `id` | function | Returns the id held by `StemHandle`. | `src/session/declaration/draft.rs:795` |
| sym-4a56d9cedda87c1a390e | `id` | function | Returns the id held by `DeviceSelector`. | `src/session/declaration/selector.rs:117` |
| sym-d424573816adf7a448a6 | `id` | function | Returns the id held by `StemSpec`. | `src/session/declaration/spec.rs:151` |
| sym-3caab190b0a5cb98ec46 | `id` | function | Returns the id held by `EndpointSpec`. | `src/session/declaration/spec.rs:171` |
| sym-e0678a06b7826d0eddf6 | `id` | function | Returns the id held by `ConnectionSpec`. | `src/session/declaration/spec.rs:259` |
| sym-0ef1efd3620a46e54fa2 | `implementation_generation` | function | Monotonic implementation generation for this manifest revision. | `src/session/extensions/source.rs:158` |
| sym-3a94fa1fdb1571c006dd | `ingress_rejected_total` | function | Returns the ingress rejected total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:364` |
| sym-144e506a1cf7c0aa078b | `input` | function | Returns the input held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:729` |
| sym-b5532cd7a4e1647076b4 | `input_attempted_total` | function | Returns the input attempted total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:413` |
| sym-0adaa4f1d995ef49edde | `input_delivered_total` | function | Returns the input delivered total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:421` |
| sym-30e6822ffaff9ba57387 | `input_dropped_total` | function | Returns the input dropped total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:425` |
| sym-7af29d2a0e0d8b4ae509 | `input_edge` | function | Returns the input edge associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:153` |
| sym-ed86c8da9240a3754da0 | `input_edge` | function | Returns the input edge associated with `EndpointSpec`. | `src/session/declaration/spec.rs:191` |
| sym-7172910d40925f998ab6 | `input_enqueued_total` | function | Returns the input enqueued total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:417` |
| sym-d504de821ea6433aaf3f | `input_port` | function | Returns the input port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:75` |
| sym-5aeb7cdb9acbef2e78f0 | `input_port` | function | Returns the input port held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:398` |
| sym-a62e16f22523087f53c7 | `input_queue_capacity_frames` | function | Returns the input queue capacity frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:401` |
| sym-50c8b0a61d9b37bf4574 | `input_queue_depth_frames` | function | Returns the input queue depth frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:405` |
| sym-5e399a46305d990f070f | `input_queue_peak_frames` | function | Returns the input queue peak frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:409` |
| sym-700a1293c724cb18159c | `input_spec` | function | Returns the input spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:83` |
| sym-1d1c0fc50a816563d4e9 | `insert` | function | Adds declared source configuration. | `src/session/extensions/source.rs:96` |
| sym-af589dfe78ceae0eac52 | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:725` |
| sym-3a7568856b61a68fec16 | `instance_id` | function | Returns the instance identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:858` |
| sym-97b7f04bf7c77ff2c93f | `instance_id` | function | Returns the instance identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:76` |
| sym-78a0a0b71d22895f295e | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:245` |
| sym-03cf916de4a45a48b3e9 | `into_error` | function | Converts `SessionStartFailure` into error. | `src/session/lifecycle/start_contract.rs:290` |
| sym-b79469e92c616d61fc7a | `into_parts` | function | Consumes `PcmSource` and returns its component values. | `src/session/extensions/audio_input/source.rs:68` |
| sym-49c367c52869a94bb6e6 | `into_pcm_source` | function | Converts the convenience façade into explicit source, output, and producer ownership. | `src/session/extensions/audio_input/mod.rs:137` |
| sym-f67b760d2d44af55cc4f | `into_rejected` | function | Converts `AudioInputWriteError` into rejected. | `src/session/extensions/audio_input/buffer.rs:319` |
| sym-a3a6b6c0accc419fd555 | `into_start_failure` | function | Converts `SessionEngineStartError` into start failure. | `src/session/lifecycle/engine.rs:336` |
| sym-c2cd40085283728ed9b7 | `invalid_total` | function | Returns the invalid total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:352` |
| sym-681c06335d54e88a43ce | `is_cancelled` | function | Returns whether cancelled applies to `SourceCancellation`. | `src/session/extensions/source.rs:255` |
| sym-2ebbe7c81fcbaaed8838 | `is_complete` | function | Returns whether complete applies to `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:80` |
| sym-aac50ad341fd43dc62ea | `is_requested` | function | Returns whether requested applies to `SessionStartCancellation`. | `src/session/lifecycle/start_contract.rs:107` |
| sym-3d30799c0f256dbe4678 | `is_sensitive` | function | Returns whether sensitive applies to `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:64` |
| sym-26bd8d847c4f74770432 | `is_success` | function | Returns whether success applies to `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:320` |
| sym-87f7b411cebdb4212f3b | `iter` | function | Iterates over the values held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:68` |
| sym-cc9676d435b55045859f | `iter` | function | Iterates over the values held by `SourceConfiguration`. | `src/session/extensions/source.rs:104` |
| sym-30958807622184a2ef06 | `join` | function | Joins its owned operation for `SourceRuntime`. | `src/session/extensions/source.rs:583` |
| sym-7de8e0a776eff6f4e656 | `joined` | function | Returns whether joined is true for `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:376` |
| sym-f177dc226e20122ed2bd | `kind` | function | Returns the kind represented by `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:315` |
| sym-97d70f67e4cda1a66bad | `kind` | function | Returns the kind represented by `SessionEvent`. | `src/session/lifecycle/events.rs:322` |
| sym-98d87216e9c161fd08bd | `lineage_failures_total` | function | Returns the lineage failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:350` |
| sym-5c690a8e2606d07e7136 | `load_native_extension_library` | function | Loads one trusted native dynamic library from an exact absolute path and imports its supported non-realtime source, operator, and endpoint registrations into this Session as one validated set. | `src/session/extensions/native_library.rs:29` |
| sym-e01c5600329c344bf7e6 | `major` | function | Returns the major associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:51` |
| sym-b97c61fed1660410aef8 | `manifest` | function | Returns the manifest held by `SourceRegistry`. | `src/session/extensions/source.rs:291` |
| sym-8ff4ff54399e007d51f7 | `mark_discontinuity` | function | Marks the next value from `AudioInputBuffer` as discontinuous. | `src/session/extensions/audio_input/buffer.rs:46` |
| sym-d080cac01208cf388fe8 | `maximum_buffered_audio_bytes` | function | Returns the maximum buffered audio bytes held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:344` |
| sym-5def61eddb0d041de241 | `metrics_snapshot` | function | Returns the metrics snapshot associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:117` |
| sym-04805bf8250099b87a8e | `microphone` | function | Creates `Source` for the selected microphone device. | `src/session/declaration/selector.rs:144` |
| sym-448810bf95d96f7b4ce5 | `microphone_default` | function | Creates `Source` for the host default microphone. | `src/session/declaration/selector.rs:148` |
| sym-2711e2b4707f92d51d9c | `minor` | function | Returns the minor associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:56` |
| sym-971a6c9d445eb7605d25 | `name` | function | Returns the name associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:63` |
| sym-5e37a61d22c00fae836f | `native` | function | Returns the native associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:40` |
| sym-13840ec974868b9fa41d | `native` | function | Creates the production host builder with the platform's native capture backend, leaving endpoint registration open to the owning application. | `src/session/lifecycle/host.rs:223` |
| sym-56dfaf3205b2a6004563 | `native_with_multistem_recording` | function | Builds the native Session host with one canonical multistem recorder. | `src/session/lifecycle/host.rs:48` |
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
| sym-59aa13fc702cd2e54b48 | `node_configuration` | function | Returns the node configuration held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:258` |
| sym-8dc057e86680cbc4a958 | `node_count` | function | Returns the node count held by `CompiledSession`. | `src/session/compile/compiled.rs:47` |
| sym-a751620187add456badf | `node_type_id` | function | Returns the node type identifier held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:141` |
| sym-03f61e68cda696e9e6e8 | `node_type_id` | function | Returns the node type identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:179` |
| sym-db8caf533c36e5677f4d | `normalized_total` | function | Returns the normalized total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:348` |
| sym-24fc69e411a1cd29f7ed | `observations` | function | Returns the observations exposed by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:246` |
| sym-4119a81ca0dae8426522 | `observations` | function | Returns the observations exposed by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:131` |
| sym-2b071730ee8a80feda9f | `observations` | function | Returns the observations exposed by `SourceRuntime`. | `src/session/extensions/source.rs:579` |
| sym-2a9eacabe9b00d6c35d6 | `observations` | function | Returns the observations exposed by `SessionEventReceiver`. | `src/session/lifecycle/events.rs:517` |
| sym-5d7c7ac0e76c64bc9c46 | `operation` | function | Returns the operation associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:93` |
| sym-c4d88d283066544dddc0 | `operator` | function | Declares exactly one Session-owned operator instance. | `src/session/declaration/draft.rs:395` |
| sym-255df4616605df3ac4c5 | `operator` | function | Returns the operator associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:103` |
| sym-047deaec359a7c7f61d7 | `operator_count` | function | Returns the operator count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:99` |
| sym-e0aa058bb06b753d1501 | `operator_finalization_failures_total` | function | Returns the operator finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:338` |
| sym-ce9c62c0c50c9915b523 | `operator_id` | function | Returns the operator identifier held by `Operator`. | `src/session/declaration/draft.rs:307` |
| sym-ad007e7e221e1c5a1fc0 | `operator_id` | function | Returns the operator identifier held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:145` |
| sym-22354fa85ea55e542cdf | `operator_id` | function | Returns the operator identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:183` |
| sym-26ac05a27c8d705f6425 | `operator_id` | function | Returns the operator identifier held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:249` |
| sym-1f4297e1167322421139 | `operator_id` | function | Returns the operator identifier held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:71` |
| sym-fa7d1beeca646b5b0ccd | `operator_instance_id` | function | Returns the operator instance identifier held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1028` |
| sym-5ec037916bd759fd8575 | `operator_instance_id` | function | Returns the operator instance identifier held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:304` |
| sym-789e8cd7bcd8d10f1171 | `operator_mappings` | function | Returns the operator mappings associated with `PreparedSession`. | `src/session/prepare/prepared.rs:56` |
| sym-cc59b91ee46c42a9c189 | `operator_metrics` | function | Returns the operator metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:205` |
| sym-f15ab11820908e6cc94d | `operators` | function | Returns the operators associated with `SessionSpec`. | `src/session/declaration/spec.rs:343` |
| sym-5d1b31642248ba6c6040 | `origin` | function | Returns the origin held by `ConnectionSpec`. | `src/session/declaration/spec.rs:263` |
| sym-dd2046e7e1bbf457c22a | `outcome` | function | Returns the outcome held by `SessionRecordingReceipt`. | `src/session/extensions/recording.rs:32` |
| sym-b197dbd4abb7a4d6524f | `outcome` | function | Returns the outcome held by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:243` |
| sym-7c043fb9c444ed2d25b4 | `outcome` | function | Returns the outcome held by `SessionTrace`. | `src/session/lifecycle/trace.rs:276` |
| sym-6c4a85df792990512f80 | `output` | function | Returns the output held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:744` |
| sym-429bc9596fb6a42ff5d9 | `output` | function | Returns the output held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:866` |
| sym-a51ba3c952b407ed044d | `output` | function | Returns the output held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1036` |
| sym-876aa14c13ad52ad6107 | `output` | function | Returns the output held by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:107` |
| sym-ea6df2e8b7c3afab51ed | `output` | function | Returns the output held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:56` |
| sym-9e88a9e1d4f9e52eb610 | `output` | function | Returns the output held by `SourceSessionContext`. | `src/session/extensions/source.rs:242` |
| sym-5f8986723f56a351a506 | `output_port` | function | Returns the output port held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:947` |
| sym-bd16619807e465020349 | `output_port` | function | Returns the output port held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1032` |
| sym-09f6aefd2dd473790e00 | `output_port` | function | Returns the output port held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:141` |
| sym-70dbd7b16369627fa5e8 | `output_port` | function | Returns the output port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:79` |
| sym-9949de72230c160ad94c | `output_port` | function | Returns the output port held by `SourceManifest`. | `src/session/extensions/source.rs:217` |
| sym-e67bb7de8a87f69fe7d8 | `output_spec` | function | Returns the output spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:87` |
| sym-69d92265c4f8e7cda77b | `outputs` | function | Returns the outputs associated with `SourceManifest`. | `src/session/extensions/source.rs:170` |
| sym-651586cf43cd82600d3b | `planned_edge_count` | function | Returns the planned edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:57` |
| sym-39ef5ea12a6d09945499 | `pocketstation::session::error_code::polled_audio_poll_error_code` | function | Returns the polled audio poll error code held by `error_code`. | `src/session/error_code.rs:255` |
| sym-9c7aed24d1b9cc8f1d9d | `pocketstation::session::error_code::session_declaration_error_code` | function | Returns the session declaration error code held by `error_code`. | `src/session/error_code.rs:195` |
| sym-60cff1b221f8ab3f6f87 | `pocketstation::session::error_code::session_start_failure_code` | function | Returns the session start failure code held by `error_code`. | `src/session/error_code.rs:225` |
| sym-60779fcdabce7a509a83 | `pocketstation::session::error_code::session_stop_failure_codes` | function | Returns every stable failure code carried by a Session stop result. | `src/session/error_code.rs:265` |
| sym-6d80dde20ce2a006601a | `pocketstation::session::extensions::builtins::register_session_graph_nodes` | function | Registers session graph nodes for `builtins`. | `src/session/extensions/builtins.rs:36` |
| sym-38553eb3f5fd53ccb7a7 | `pocketstation::session::lifecycle::running::start_prepared_session` | function | Starts prepared session for `running`. | `src/session/lifecycle/running.rs:615` |
| sym-e2db7459f61aae428d43 | `pocketstation::session::lifecycle::running::start_prepared_session_cancellable` | function | Starts prepared session cancellable for `running`. | `src/session/lifecycle/running.rs:631` |
| sym-aaef40a767909f538478 | `pocketstation::session::prepare::prepare_session_runtime` | function | Prepares session runtime for `prepare`. | `src/session/prepare/mod.rs:33` |
| sym-7a78d9bb2e991fc348d6 | `polled_audio` | function | Declares a bounded polled-audio endpoint on `Session`. | `src/session/extensions/polled_audio.rs:14` |
| sym-9783d6b783873d785d90 | `polled_audio` | function | Declares a bounded polled-audio endpoint on `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:71` |
| sym-9da1c8304e4337da5515 | `polled_audio_receipt` | function | Returns the polled audio receipt associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:99` |
| sym-f9bc638db011084c320e | `polled_audio_receipts_total` | function | Returns the polled audio receipts total held by `SessionEngineHost`. | `src/session/lifecycle/host.rs:104` |
| sym-d61eec066d9ac19ebb30 | `pool_exhausted_total` | function | Returns the pool exhausted total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:360` |

## Interpretation

The **Session API** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.

## Related documentation

- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Session failures](/docs/errors/session.md)

## Evidence boundary

The claims on **Session API** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/mod.rs:1-143` (`DIRECT`)

For **Session API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
