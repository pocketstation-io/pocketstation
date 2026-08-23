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
| sym-c6a638806c91124f0676 | `pocketstation::session::declaration::endpoint::BROWSER_NODE_TYPE_ID` | constant | Defines the public browser node type identifier value. | `src/session/declaration/endpoint.rs:9` |
| sym-fc1ec929667760d593a2 | `pocketstation::session::declaration::endpoint::BROWSER_OPERATOR_ID` | constant | Defines the public browser operator identifier value. | `src/session/declaration/endpoint.rs:10` |
| sym-648808e081cc4f7b4163 | `pocketstation::session::declaration::endpoint::CONNECTOR_NODE_TYPE_ID` | constant | Defines the public connector node type identifier value. | `src/session/declaration/endpoint.rs:8` |
| sym-6015373ecb248baeeb21 | `pocketstation::session::declaration::spec::SESSION_SPEC_VERSION` | constant | Defines the public session spec version value. | `src/session/declaration/spec.rs:11` |
| sym-990126c3b36a72a9663d | `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
| sym-f6613170e7551288121d | `pocketstation::session::extensions::builtins::APPLICATION_SOURCE_NODE_TYPE_ID` | constant | Defines the public application source node type identifier value. | `src/session/extensions/builtins.rs:23` |
| sym-212f03d762965fddfe4d | `pocketstation::session::extensions::builtins::MICROPHONE_SOURCE_NODE_TYPE_ID` | constant | Defines the public microphone source node type identifier value. | `src/session/extensions/builtins.rs:24` |
| sym-e8adce9ed0b589e380eb | `pocketstation::session::extensions::recording::DEFAULT_MULTISTEM_RECORDING_GROUP_ID` | constant | Defines the public default multistem recording group identifier value. | `src/session/extensions/recording.rs:24` |
| sym-82534ca48ce3c1db5a8f | `pocketstation::session::extensions::recording::RECORDER_NODE_TYPE_ID` | constant | Defines the public recorder node type identifier value. | `src/session/extensions/recording.rs:20` |
| sym-b22481c631aa223b9b04 | `pocketstation::session::extensions::recording::RECORDER_OPERATOR_ID` | constant | Defines the public recorder operator identifier value. | `src/session/extensions/recording.rs:21` |
| sym-9fd7c3b1a6a29ca05799 | `pocketstation::session::extensions::recording::RECORDING_GROUP_CONFIGURATION_KEY` | constant | Defines the public recording group configuration key value. | `src/session/extensions/recording.rs:23` |
| sym-da81e7605f4543a374ca | `pocketstation::session::extensions::recording::SESSION_RECORDING_MANIFEST_FILE_NAME` | constant | Defines the public session recording manifest file name value. | `src/session/extensions/recording.rs:25` |
| sym-448f0aef86c8f3825184 | `pocketstation::session::extensions::recording::SESSION_RECORDING_MANIFEST_SCHEMA_VERSION` | constant | Defines the public session recording manifest schema version value. | `src/session/extensions/recording.rs:27` |
| sym-b88918b145af6981a661 | `pocketstation::session::compile::error::SessionCompileError` | enum | Classifies failures reported as session compile error. | `src/session/compile/error.rs:7` |
| sym-d83bf3e81e623b11c715 | `pocketstation::session::declaration::selector::ApplicationSelector` | enum | Enumerates the supported application selector cases. | `src/session/declaration/selector.rs:32` |
| sym-be6e808aed7e0da57649 | `pocketstation::session::declaration::selector::DeviceSelector` | enum | Enumerates the supported device selector cases. | `src/session/declaration/selector.rs:107` |
| sym-b64a7cadc7e6f7c5fa13 | `pocketstation::session::declaration::selector::Source` | enum | Enumerates the supported source cases. | `src/session/declaration/selector.rs:134` |
| sym-e0648ee37d245fc7fd22 | `pocketstation::session::declaration::spec::ConnectionTarget` | enum | Stable destination of a declared Session connection. | `src/session/declaration/spec.rs:224` |
| sym-b0dbd87bc9581e0b7066 | `pocketstation::session::declaration::spec::StreamOrigin` | enum | Stable origin of a declared Session stream. | `src/session/declaration/spec.rs:208` |
| sym-35a864f7f5b2fbde0056 | `pocketstation::session::declaration::typed_stream::TypedStreamError` | enum | Classifies failures reported as typed stream error. | `src/session/declaration/typed_stream.rs:185` |
| sym-1955bd9759678ed98b23 | `pocketstation::session::error::SessionError` | enum | Classifies failures reported as session error. | `src/session/error.rs:6` |
| sym-4c59be13624496754b3d | `pocketstation::session::error_code::PolledAudioPollErrorCode` | enum | Stable language-neutral code for bounded polled-audio status. | `src/session/error_code.rs:131` |
| sym-43f2c5d509dad2abb71a | `pocketstation::session::error_code::SessionDeclarationErrorCode` | enum | Stable language-neutral code for a Session declaration failure. | `src/session/error_code.rs:10` |
| sym-cde24d045d00d42757d1 | `pocketstation::session::error_code::SessionRuntimeErrorCode` | enum | Stable language-neutral code for a running-Session projection failure. | `src/session/error_code.rs:116` |
| sym-24efd257193601aef470 | `pocketstation::session::error_code::SessionStartErrorCode` | enum | Stable language-neutral code for Session startup. | `src/session/error_code.rs:61` |
| sym-0b02cb5628054570780f | `pocketstation::session::error_code::SessionStopCode` | enum | Stable language-neutral status for an idempotent Session stop. | `src/session/error_code.rs:150` |
| sym-09d3e56e9e242ad53395 | `pocketstation::session::error_code::SessionStopFailureCode` | enum | Stable language-neutral cause retained by a failed Session stop. | `src/session/error_code.rs:171` |
| sym-59571b2eea7cbe8a23d8 | `pocketstation::session::extensions::audio_input::AudioInputConfigError` | enum | Classifies failures reported as audio input config error. | `src/session/extensions/audio_input/mod.rs:77` |
| sym-e62ed15b46d470f5c856 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | enum | Classifies failures reported as audio input buffer acquire error. | `src/session/extensions/audio_input/buffer.rs:271` |
| sym-cfce1a7f00670e05d9d4 | `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | enum | Classifies failures reported as audio input buffer error. | `src/session/extensions/audio_input/buffer.rs:281` |
| sym-5eadd18110842a6de92b | `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | enum | Selects the audio input write error kind used by PocketStation. | `src/session/extensions/audio_input/buffer.rs:298` |
| sym-fd0b5b1e3d01611befb0 | `pocketstation::session::extensions::audio_input::source::AudioInputError` | enum | Classifies failures reported as audio input error. | `src/session/extensions/audio_input/source.rs:85` |
| sym-6f966dee9d1515e62a7e | `pocketstation::session::extensions::builtins::SessionGraphRegistrationError` | enum | Classifies failures reported as session graph registration error. | `src/session/extensions/builtins.rs:30` |
| sym-83234335e8b4cb70f57e | `pocketstation::session::extensions::source::SourceDriverError` | enum | Classifies failures reported as source driver error. | `src/session/extensions/source.rs:748` |
| sym-1843c1ec8aae096e1d65 | `pocketstation::session::extensions::source::SourceManifestError` | enum | Classifies failures reported as source manifest error. | `src/session/extensions/source.rs:677` |
| sym-61d8e211defe79bbb1a6 | `pocketstation::session::extensions::source::SourceRegistrationError` | enum | Classifies failures reported as source registration error. | `src/session/extensions/source.rs:701` |
| sym-99bca5d12c942096f8b0 | `pocketstation::session::extensions::source::SourceRuntimeError` | enum | Classifies failures reported as source runtime error. | `src/session/extensions/source.rs:754` |
| sym-7871af4f560ec9fe3ac4 | `pocketstation::session::extensions::source::SourceTypeIdError` | enum | Classifies failures reported as source type id error. | `src/session/extensions/source.rs:68` |
| sym-5ede7fd5e41898b0061b | `pocketstation::session::lifecycle::control::SessionStartError` | enum | Classifies failures reported as session start error. | `src/session/lifecycle/control.rs:121` |
| sym-3c6c09975ce94069676c | `pocketstation::session::lifecycle::engine::EndpointExtensionRegistrationError` | enum | Classifies failures reported as endpoint extension registration error. | `src/session/lifecycle/engine.rs:305` |
| sym-c641398d8da3cb94c250 | `pocketstation::session::lifecycle::engine::SessionEngineBuildError` | enum | Classifies failures reported as session engine build error. | `src/session/lifecycle/engine.rs:295` |
| sym-c5305e4d076e18066988 | `pocketstation::session::lifecycle::engine::SessionEngineStartError` | enum | Classifies failures reported as session engine start error. | `src/session/lifecycle/engine.rs:315` |
| sym-8be5244e9ac8689c706c | `pocketstation::session::lifecycle::events::SessionComponentId` | enum | Stable identity of the component that produced a session control failure. | `src/session/lifecycle/events.rs:51` |
| sym-62d3d25fa4776760b06b | `pocketstation::session::lifecycle::events::SessionEventKind` | enum | Payload of one authoritative session event. | `src/session/lifecycle/events.rs:294` |
| sym-c6d521bc369615540a31 | `pocketstation::session::lifecycle::events::SessionEventReceive` | enum | Result of non-blocking event polling. | `src/session/lifecycle/events.rs:492` |
| sym-758d13d75953fa66b79a | `pocketstation::session::lifecycle::events::SessionFinalizationStage` | enum | The finalization operation that failed while stopping a session. | `src/session/lifecycle/events.rs:39` |
| sym-962361a1358c981f6a9f | `pocketstation::session::lifecycle::events::SessionLifecycleState` | enum | Public lifecycle states emitted by a running session. | `src/session/lifecycle/events.rs:19` |
| sym-ac79480f787ee81818a0 | `pocketstation::session::lifecycle::events::SessionRollbackStage` | enum | The rollback operation that failed while unwinding a partial start. | `src/session/lifecycle/events.rs:29` |
| sym-a2d10031270815731e7b | `pocketstation::session::lifecycle::events::SessionTerminalState` | enum | Final state carried by the terminal session event. | `src/session/lifecycle/events.rs:210` |
| sym-da46a154618824a18994 | `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` | enum | Classifies failures reported as session engine host build error. | `src/session/lifecycle/host.rs:362` |
| sym-933733f669d7719ffe7a | `pocketstation::session::lifecycle::observations::EndpointObservationStage` | enum | Selects the endpoint observation stage used by PocketStation. | `src/session/lifecycle/observations.rs:441` |
| sym-5cc026a676e0591b94af | `pocketstation::session::lifecycle::observations::SessionRouteLatencyBoundary` | enum | Enumerates the supported session route latency boundary cases. | `src/session/lifecycle/observations.rs:196` |
| sym-a758550b72412787e99d | `pocketstation::session::lifecycle::observations::SessionRouteLatencyUnit` | enum | Enumerates the supported session route latency unit cases. | `src/session/lifecycle/observations.rs:201` |
| sym-08c924c4a96c2ff4640f | `pocketstation::session::lifecycle::observations::SessionRouteObservationInterval` | enum | Interval covered by monotonic route counters. | `src/session/lifecycle/observations.rs:150` |
| sym-f9e0e26f14bb954cf294 | `pocketstation::session::lifecycle::trace::SessionTraceRecordKind` | enum | Selects the session trace record kind used by PocketStation. | `src/session/lifecycle/trace.rs:27` |
| sym-88a327bb91bf885ce10b | `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | enum | Classifies failures reported as session trace recorder finish error. | `src/session/lifecycle/trace.rs:98` |
| sym-edc7211e5a9e61bcb2ea | `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | enum | Classifies failures reported as session trace recorder start error. | `src/session/lifecycle/trace.rs:88` |
| sym-68ce5d3bae413e0f5470 | `pocketstation::session::lifecycle::trace::SessionTraceValidationError` | enum | Classifies failures reported as session trace validation error. | `src/session/lifecycle/trace.rs:356` |
| sym-796bbabbf84e0f47b0d8 | `pocketstation::session::prepare::error::SessionPrepareError` | enum | Classifies failures reported as session prepare error. | `src/session/prepare/error.rs:9` |
| sym-995aa9622b5ba2b4a748 | `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| sym-df9cd8b6772b89c63bda | `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| sym-56f0802ba994d7ac9843 | `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| sym-93000b7d23bf0f46007d | `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| sym-c1d7fb39869e2d34f307 | `SourceFactory::manifest` | function | Returns the manifest held by `SourceFactory`. | `src/session/extensions/source.rs:277` |
| sym-6a881085d8de6dbbb9e6 | `SourceFactory::validate_config` | function | Validates config for `SourceFactory`. | `src/session/extensions/source.rs:278` |
| sym-7370e5df776e21bbd7b7 | `StreamSignal::signal_spec` | function | Returns the signal spec held by `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| sym-dc0c82c0beb1a6eb87e9 | `actual` | function | Returns the observed value when a compilation diagnostic compares two values. | `src/session/compile/error.rs:153` |
| sym-1682a40407799112e8f3 | `application` | function | Returns the application held by `Source`. | `src/session/declaration/selector.rs:140` |
| sym-11d0c6769d2d0c38fd3d | `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| sym-642103f95789f4c34de4 | `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| sym-098447029f757e3ee3fc | `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| sym-7496f586df598ab4c92a | `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| sym-ef6574dbc99fcb40c5a2 | `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| sym-5643015174c990de4aee | `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| sym-e3f1223778a01130e663 | `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| sym-d23b80c999773411aaa9 | `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| sym-eac6fe4c4ec72aa2b314 | `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| sym-b06ab36e3211268290ef | `audio_reentry_metrics` | function | Returns the audio reentry metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:223` |
| sym-9527dd3130d771c3266b | `browser` | function | Returns the browser associated with `Session`. | `src/session/declaration/draft.rs:431` |
| sym-596d144d9deba0aa781a | `build` | function | Consumes all setup state so no partially populated registry can escape. | `src/session/lifecycle/engine.rs:176` |
| sym-57541748485f96465396 | `build` | function | Builds its owned operation for `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:344` |
| sym-f3af0a674083c042387f | `bundle_id` | function | Returns the bundle identifier held by `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| sym-218585d5338b32dc0e4d | `cancel` | function | Requests cancellation of `SourceRuntime`. | `src/session/extensions/source.rs:575` |
| sym-cfafc3f08dfc105ee2ca | `cancel` | function | Requests cancellation of `RunningSession`. | `src/session/lifecycle/running.rs:423` |
| sym-bf0f618c9ae56c316380 | `cancellation_requested` | function | Returns whether cancellation requested is true for `PreparedSession`. | `src/session/prepare/prepared.rs:73` |
| sym-c00eb2e5f0f365125ff1 | `cancellation_total` | function | Returns the cancellation total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| sym-651d787afb9425aca9fc | `capacity_frames` | function | Returns the capacity frames held by `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| sym-468dcc7528c63336128c | `capture` | function | Declares a capture source on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:335` |
| sym-c4062e05a64ec2cd4952 | `capture_finalization_failures_total` | function | Returns the capture finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:359` |
| sym-3aeff457f579705093f6 | `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| sym-217be49e77da4aabf632 | `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| sym-7f946d73a01b4db9ecc6 | `code` | function | Returns the stable error or status code represented by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:113` |
| sym-68093a3f20079dee1c25 | `compile` | function | Compiles its owned operation for `SessionCompiler`. | `src/session/compile/mod.rs:103` |
| sym-46d9ba85d0b5a90bb8da | `compile` | function | Compiles its owned operation for `SessionEngine`. | `src/session/lifecycle/engine.rs:221` |
| sym-ee91490be1c23a752cf2 | `compile` | function | Compiles its owned operation for `SessionEngineHost`. | `src/session/lifecycle/host.rs:55` |
| sym-e03a7b814ae052667ca9 | `component` | function | Returns the component associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:89` |
| sym-3c7d5cd5a6971e320608 | `configuration` | function | Returns the configuration held by `Operator`. | `src/session/declaration/draft.rs:299` |
| sym-83fc2990431dc05c1b6e | `configuration` | function | Returns the configuration held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:149` |
| sym-a7de949037767e556526 | `configuration` | function | Returns the configuration held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:88` |
| sym-df71b11f0bf7bd47f91e | `configuration` | function | Returns the configuration held by `EndpointSpec`. | `src/session/declaration/spec.rs:187` |
| sym-0dba4ffdc9d828c28bec | `configuration` | function | Returns the configuration held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:253` |
| sym-d7ab9dfb9f46b66659ea | `configuration` | function | Returns the configuration held by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:103` |
| sym-1c4c7cb9bedad265f145 | `connect` | function | Connects the requested ports through `StemHandle`. | `src/session/declaration/draft.rs:807` |
| sym-8f3431c4252ede73ca6c | `connect` | function | Connects the requested ports through `SourceOutputHandle`. | `src/session/declaration/draft.rs:943` |
| sym-2f51546b7bed5c1ee4c6 | `connect` | function | Connects the requested ports through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1039` |
| sym-286cb5670f377ab8ef14 | `connections` | function | Returns the connections associated with `SessionSpec`. | `src/session/declaration/spec.rs:347` |
| sym-e8963ce61c3155e55b92 | `connector` | function | Declares a connector endpoint on `Session` with the supplied operator identity and configuration. | `src/session/declaration/draft.rs:399` |
| sym-7032830c11b003b8c721 | `connector_id` | function | Returns the connector identifier held by `EndpointHandle`. | `src/session/declaration/draft.rs:595` |
| sym-73f349fcfa5dcd416141 | `connector_id` | function | Returns the connector identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:175` |
| sym-06cbcdb626b9a2e220a9 | `declares_multistem_recording` | function | Returns whether `Session` declares multistem recording. | `src/session/extensions/recording.rs:102` |
| sym-1c5179c9d400732c9b0c | `default` | function | Returns the default `Session` value. | `src/session/declaration/draft.rs:565` |
| sym-0fdf7c5a3b73738000b1 | `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| sym-e1902db1f798ce94cdea | `default` | function | Returns the default `SessionStartOptions` value. | `src/session/lifecycle/control.rs:33` |
| sym-8684f51f8ee7430fcd49 | `default` | function | Returns the default `NativeSessionEngineHostOptions` value. | `src/session/lifecycle/host.rs:172` |
| sym-7d47561aba01141608ff | `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| sym-fb88e790fd387a7ae359 | `derived_route_count` | function | Returns the derived route count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| sym-143bf8e7f69e1c868835 | `derived_route_metrics` | function | Returns the derived route metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:219` |
| sym-05eefad45f0dfa93d416 | `diagnostic` | function | Converts a Session compiler failure into stable language-neutral location and comparison fields. | `src/session/compile/error.rs:166` |
| sym-c56c74ef48d3d65a7896 | `direction` | function | Returns the direction associated with `SessionCompileDiagnostic`. | `src/session/compile/error.rs:145` |
| sym-fdff2f61cb56c8996295 | `drop` | function | Releases resources owned by `PreparedSourceRuntime`. | `src/session/extensions/source.rs:556` |
| sym-e0b320e2052a0c631823 | `drop` | function | Releases resources owned by `SourceRuntime`. | `src/session/extensions/source.rs:594` |
| sym-50cad3198e714c4154f7 | `drop` | function | Releases resources owned by `RunningSession`. | `src/session/lifecycle/running.rs:619` |
| sym-3a61bb04d6a986cbcb51 | `drop` | function | Releases resources owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:249` |
| sym-3a193e845f256dd2f356 | `drop_observations` | function | Returns the drop observations held by `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:206` |
| sym-88c3e8dfe1cf74414c54 | `drop_rate_pct` | function | Returns the drop rate pct held by `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:171` |
| sym-38977d2ca402b0e4145b | `edge_count` | function | Returns the edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:52` |
| sym-8ac4397e918d84da100f | `edge_index` | function | Returns the edge index held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:121` |
| sym-6abe660491a0c79f396d | `endpoint` | function | Declares an endpoint on `Session` and returns its Session-scoped handle. | `src/session/declaration/draft.rs:394` |
| sym-c6dbb7792fc664b765f7 | `endpoint_declarations` | function | Returns the endpoint declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:42` |
| sym-516dc966b917a4f240ef | `endpoint_failures` | function | Returns the endpoint failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:279` |
| sym-23c14c23a493121cf53b | `endpoint_finalization_failures_total` | function | Returns the endpoint finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:363` |
| sym-039bdeff82f22e216144 | `endpoint_id` | function | Returns the endpoint identifier held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:150` |
| sym-4bec51422253ac2e9473 | `endpoint_id` | function | Returns the endpoint identifier held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:253` |
| sym-68ff5ed2fe9768966073 | `endpoints` | function | Returns the endpoints associated with `SessionSpec`. | `src/session/declaration/spec.rs:339` |
| sym-b97f3ea15b1278c1a5cd | `engine` | function | Returns the engine associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:158` |
| sym-795bbe983a2449575b66 | `engine_builder` | function | Borrows the mutable engine builder owned by `SessionEngineHostBuilder`. | `src/session/lifecycle/host.rs:277` |
| sym-d0e7ecee43b4fb211527 | `error` | function | Returns the error associated with `SessionStartFailure`. | `src/session/lifecycle/control.rs:307` |
| sym-7864f46099de5598eb03 | `error_class` | function | Returns the error class associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:97` |
| sym-3871b4999157943369c3 | `event` | function | Returns the event associated with `SessionSourceFailure`. | `src/session/lifecycle/events.rs:118` |
| sym-e005811e6b121d4e888b | `event_queue` | function | Returns the event queue associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:67` |
| sym-992aa61dc221d91aba49 | `execution` | function | Returns the execution held by `SourceManifest`. | `src/session/extensions/source.rs:174` |
| sym-835c3fe6f42cb5df0be0 | `expected` | function | Returns the expected value when a compilation diagnostic compares two values. | `src/session/compile/error.rs:149` |
| sym-b813a339705fd17d01fe | `external_source` | function | Returns the external source held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:87` |
| sym-a14092e356d511085706 | `external_source_count` | function | Returns the external source count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:83` |
| sym-1a79107a12b86eca14ee | `external_source_declarations` | function | Returns the external source declarations associated with `CompiledSession`. | `src/session/compile/compiled.rs:37` |
| sym-d79af986ffbd30650d2d | `external_source_metrics` | function | Returns the external source metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:215` |
| sym-30c45a25347393930ba2 | `failure` | function | Returns the failure held by `SessionEndpointFailure`. | `src/session/lifecycle/events.rs:158` |
| sym-5d9e7c56c63f18de4dda | `failure` | function | Returns the failure held by `SessionRollbackFailure`. | `src/session/lifecycle/events.rs:179` |
| sym-73a681fc858156508e9e | `failure` | function | Returns the failure held by `SessionFinalizationFailure`. | `src/session/lifecycle/events.rs:203` |
| sym-8cf6b272ddf136ea84bd | `finalization_failures` | function | Returns the finalization failures associated with `SessionTerminalOutcome`. | `src/session/lifecycle/events.rs:287` |
| sym-3c724fc69bd1f09f64f9 | `finish` | function | Finishes work owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:201` |
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
| sym-c9374230e421a25e4065 | `frame_capacity_samples` | function | Returns the frame capacity samples held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:340` |
| sym-52af0d0b16a2213b8647 | `frame_samples_per_channel` | function | Returns the frame samples per channel associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:67` |
| sym-cb7e1786405961aea020 | `freeze` | function | Freezes mutable storage owned by `Session` into its shared immutable form. | `src/session/declaration/draft.rs:454` |
| sym-dab7653eb1cd828c2cd6 | `from` | function | Converts the supplied value into `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:325` |
| sym-bda4c6758070e1b945d8 | `from_source_output` | function | Wraps a public external-source output in the same typed Rust façade. Runtime identity remains the output's stable `SignalSpec` and schema. | `src/session/declaration/typed_stream.rs:118` |
| sym-8c6c8318a76c699d5180 | `from_stem` | function | Creates `Stream` from stem. | `src/session/declaration/typed_stream.rs:103` |
| sym-d2f25b72435c052d1f41 | `generated_audio_ingresses` | function | Returns the generated audio ingresses associated with `SessionSpec`. | `src/session/declaration/spec.rs:335` |
| sym-164550678c275cd1ca9e | `generation` | function | Returns the implementation generation. | `src/session/extensions/source.rs:166` |
| sym-a5be6413e3e1a68543cb | `get` | function | Returns the value held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:60` |
| sym-d8cbedb909a6848e51df | `get` | function | Returns the value held by `ProcessId`. | `src/session/declaration/selector.rs:13` |
| sym-e3343c30584c85eae6ef | `get` | function | Returns the value held by `SourceConfiguration`. | `src/session/extensions/source.rs:100` |
| sym-f6cc06d0b9a22649ff12 | `handle` | function | Returns the handle associated with `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:197` |
| sym-a7ad1513b93c5ad30864 | `id` | function | Returns the id held by `Session`. | `src/session/declaration/draft.rs:331` |
| sym-15050b2d888ca56d48b8 | `id` | function | Returns the id held by `EndpointHandle`. | `src/session/declaration/draft.rs:591` |
| sym-b54048695892b0319f8e | `id` | function | Returns the id held by `StemHandle`. | `src/session/declaration/draft.rs:783` |
| sym-2447943f965c85f4ddfb | `id` | function | Returns the id held by `DeviceSelector`. | `src/session/declaration/selector.rs:117` |
| sym-207a83540bcfaf042651 | `id` | function | Returns the id held by `StemSpec`. | `src/session/declaration/spec.rs:151` |
| sym-04ea444c538a333dad7d | `id` | function | Returns the id held by `EndpointSpec`. | `src/session/declaration/spec.rs:171` |
| sym-dc328e0cd0b6bacf262d | `id` | function | Returns the id held by `ConnectionSpec`. | `src/session/declaration/spec.rs:259` |
| sym-3500a56c54d98069d733 | `implementation_generation` | function | Monotonic implementation generation for this manifest revision. | `src/session/extensions/source.rs:158` |
| sym-f325d519bb3a4440848c | `ingress_rejected_total` | function | Returns the ingress rejected total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:364` |
| sym-82a3c6ac01987407ccb5 | `input` | function | Returns the input held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:717` |
| sym-eba82c263e0eb85f9aaa | `input_attempted_total` | function | Returns the input attempted total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:413` |
| sym-e4e0debc087e09405c5a | `input_delivered_total` | function | Returns the input delivered total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:421` |
| sym-e10f29d8fd098a134e69 | `input_dropped_total` | function | Returns the input dropped total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:425` |
| sym-f1ae93c990799f3815cd | `input_edge` | function | Returns the input edge associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:153` |
| sym-7cb3b2c18857bf487602 | `input_edge` | function | Returns the input edge associated with `EndpointSpec`. | `src/session/declaration/spec.rs:191` |
| sym-fcbe4e1bf437f2f0a843 | `input_enqueued_total` | function | Returns the input enqueued total held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:417` |
| sym-53ea5f8920e32a5e87d1 | `input_port` | function | Returns the input port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:75` |
| sym-a9741e2e08e6ad5feee9 | `input_port` | function | Returns the input port held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:398` |
| sym-f781aa13fb4c138d7535 | `input_queue_capacity_frames` | function | Returns the input queue capacity frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:401` |
| sym-e0bad5d8373d18d407d4 | `input_queue_depth_frames` | function | Returns the input queue depth frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:405` |
| sym-3e190ddbfb9ec39c506e | `input_queue_peak_frames` | function | Returns the input queue peak frames held by `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:409` |
| sym-8ad9865b29a5785fee96 | `input_spec` | function | Returns the input spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:83` |
| sym-8176a230bc859ac5f791 | `insert` | function | Adds declared source configuration. | `src/session/extensions/source.rs:96` |
| sym-fdc8d3af1d058f8c25cd | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:713` |
| sym-f3fd41c23b6d9ab2d5b8 | `instance_id` | function | Returns the instance identifier held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:846` |
| sym-c1301f10816aa2a31d70 | `instance_id` | function | Returns the instance identifier held by `SourceInstanceSpec`. | `src/session/declaration/spec.rs:76` |
| sym-53543b9e48524d7298bc | `instance_id` | function | Returns the instance identifier held by `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:245` |
| sym-473286cbfb3dd1982f85 | `into_error` | function | Converts `SessionStartFailure` into error. | `src/session/lifecycle/control.rs:319` |
| sym-a10bbd6f35f0f08a386f | `into_parts` | function | Consumes `PcmSource` and returns its component values. | `src/session/extensions/audio_input/source.rs:68` |
| sym-aa61f33aa27c9b536714 | `into_pcm_source` | function | Converts the convenience façade into explicit source, output, and producer ownership. | `src/session/extensions/audio_input/mod.rs:137` |
| sym-648793e6c554ee61d325 | `into_rejected` | function | Converts `AudioInputWriteError` into rejected. | `src/session/extensions/audio_input/buffer.rs:319` |
| sym-b0c9411e8bbf3f1bab03 | `into_start_failure` | function | Converts `SessionEngineStartError` into start failure. | `src/session/lifecycle/engine.rs:336` |
| sym-6239186ad2720fa23c3e | `invalid_total` | function | Returns the invalid total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:352` |
| sym-e0f463ea1df54f114b27 | `is_cancelled` | function | Returns whether cancelled applies to `SourceCancellation`. | `src/session/extensions/source.rs:255` |
| sym-3a06e547847d70e25e6b | `is_complete` | function | Returns whether complete applies to `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:80` |
| sym-5ded1e414552d586b30c | `is_requested` | function | Returns whether requested applies to `SessionStartCancellation`. | `src/session/lifecycle/control.rs:115` |
| sym-1ad646036d38dee279d4 | `is_sensitive` | function | Returns whether sensitive applies to `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:64` |
| sym-7fcb6d2fa6694ec5e39a | `is_success` | function | Returns whether success applies to `SessionStopOutcome`. | `src/session/lifecycle/control.rs:349` |
| sym-3b2e1392dc84776988a4 | `iter` | function | Iterates over the values held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:68` |
| sym-d742331b94ed5efc53cc | `iter` | function | Iterates over the values held by `SourceConfiguration`. | `src/session/extensions/source.rs:104` |
| sym-3062cd188c729dd16e4c | `join` | function | Joins its owned operation for `SourceRuntime`. | `src/session/extensions/source.rs:583` |
| sym-f41915605cfe34eae0c1 | `joined` | function | Returns whether joined is true for `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:376` |
| sym-b7faa716d321d69280cc | `kind` | function | Returns the kind represented by `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:315` |
| sym-bcdd279afc1d281f56db | `kind` | function | Returns the kind represented by `SessionEvent`. | `src/session/lifecycle/events.rs:322` |
| sym-25b1030b088f2e4bbf32 | `lineage_failures_total` | function | Returns the lineage failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:379` |
| sym-20d353b749f431e7b1d2 | `load_native_extension_library` | function | Loads one trusted native dynamic library from an exact absolute path and imports its supported non-realtime source, operator, and endpoint registrations into this Session as one validated set. | `src/session/extensions/native_library.rs:29` |
| sym-777d37dbdf0f69839bf3 | `major` | function | Returns the major associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:51` |
| sym-068a7cf335367ac83e4c | `manifest` | function | Returns the manifest held by `SourceRegistry`. | `src/session/extensions/source.rs:291` |
| sym-7ee49939dfaf9f2513af | `mark_discontinuity` | function | Marks the next value from `AudioInputBuffer` as discontinuous. | `src/session/extensions/audio_input/buffer.rs:46` |
| sym-ff2185c4e90cb1f5c4ef | `maximum_buffered_audio_bytes` | function | Returns the maximum buffered audio bytes held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:344` |
| sym-7523c93d162838f46c5a | `metrics_snapshot` | function | Returns the metrics snapshot associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:117` |
| sym-2d28a8df3bf63e332c0b | `microphone` | function | Creates `Source` for the selected microphone device. | `src/session/declaration/selector.rs:144` |
| sym-d4bcb79d234d924695a6 | `microphone_default` | function | Creates `Source` for the host default microphone. | `src/session/declaration/selector.rs:148` |
| sym-9e158e07f5d9263a8c21 | `minor` | function | Returns the minor associated with `SessionSpecVersion`. | `src/session/declaration/spec.rs:56` |
| sym-58de22dd7a29f7960d69 | `name` | function | Returns the name associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:63` |
| sym-2beace5df40535edecfd | `native` | function | Returns the native associated with `SessionEngineHost`. | `src/session/lifecycle/host.rs:40` |
| sym-5893a4d114a25f68ad01 | `native` | function | Creates the production host builder with the platform's native capture backend, leaving endpoint registration open to the owning application. | `src/session/lifecycle/host.rs:223` |
| sym-09373ca757e99da6a9e6 | `native_with_multistem_recording` | function | Builds a native Session host with one multistem recorder. | `src/session/lifecycle/host.rs:48` |
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
| sym-14186bbb667595923b39 | `node_configuration` | function | Returns the node configuration held by `PreparedWorkerMapping`. | `src/session/prepare/mappings.rs:258` |
| sym-8050fe5c5aca6992ebef | `node_count` | function | Returns the node count held by `CompiledSession`. | `src/session/compile/compiled.rs:47` |
| sym-329a764a024a64e239a7 | `node_index` | function | Returns the node index held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:117` |
| sym-48d0f42440c7fc7842ea | `node_type_id` | function | Returns the node type identifier held by `SessionCompileDiagnostic`. | `src/session/compile/error.rs:133` |
| sym-5992cbcecadf5a313006 | `node_type_id` | function | Returns the node type identifier held by `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:141` |
| sym-e0729c229c69c441b3d5 | `node_type_id` | function | Returns the node type identifier held by `EndpointSpec`. | `src/session/declaration/spec.rs:179` |
| sym-67fa3af81d3df0dfbdb2 | `normalized_total` | function | Returns the normalized total held by `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:348` |
| sym-4c663c8c64bc960e342b | `observations` | function | Returns the observations exposed by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:246` |
| sym-958cba61d7f8fc6b936e | `observations` | function | Returns the observations exposed by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:131` |
| sym-a44b57485e977f0e7bbb | `observations` | function | Returns the observations exposed by `SourceRuntime`. | `src/session/extensions/source.rs:579` |
| sym-4475a719b62bcedfe19e | `observations` | function | Returns the observations exposed by `SessionEventReceiver`. | `src/session/lifecycle/events.rs:517` |
| sym-19f26353af4e287d1753 | `operation` | function | Returns the operation associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:93` |
| sym-261781ef50f8b1fd6907 | `operator` | function | Declares exactly one Session-owned operator instance. | `src/session/declaration/draft.rs:383` |
| sym-29517160d501c6b59612 | `operator` | function | Returns the operator associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:103` |
| sym-fafe5a7548a83f07ec0a | `operator_count` | function | Returns the operator count held by `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:99` |
| sym-04507f8b0309ee95a3c7 | `operator_finalization_failures_total` | function | Returns the operator finalization failures total held by `SessionStopOutcome`. | `src/session/lifecycle/control.rs:367` |
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
| sym-ba51b3dc2b3ebb67e7e3 | `operator_metrics` | function | Returns the operator metrics held by `RunningSession`. | `src/session/lifecycle/running.rs:211` |
| sym-3d32e9f8f65c96eefc79 | `operators` | function | Returns the operators associated with `SessionSpec`. | `src/session/declaration/spec.rs:343` |
| sym-aa50e22c859bc41e18b5 | `origin` | function | Returns the origin held by `ConnectionSpec`. | `src/session/declaration/spec.rs:263` |
| sym-8fc7f17a4a5ce92f7008 | `outcome` | function | Returns the outcome held by `SessionRecordingReceipt`. | `src/session/extensions/recording.rs:36` |
| sym-e72f307e0a3e8bfb016c | `outcome` | function | Returns the outcome held by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:243` |
| sym-46184da0e5f6844c22d9 | `outcome` | function | Returns the outcome held by `SessionTrace`. | `src/session/lifecycle/trace.rs:276` |
| sym-1e848a89312484f725a3 | `output` | function | Returns the output held by `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:732` |
| sym-bbf6442f1b447ba134e3 | `output` | function | Returns the output held by `SourceInstanceHandle`. | `src/session/declaration/draft.rs:854` |
| sym-3ab351fd3a63820a3179 | `output` | function | Returns the output held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1024` |
| sym-b3b8b7cbc2207c5d5995 | `output` | function | Returns the output held by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:107` |
| sym-1e3bd4d53be1938eacb2 | `output` | function | Returns the output held by `PcmSource`. | `src/session/extensions/audio_input/source.rs:56` |
| sym-3454febcc9704bc56791 | `output` | function | Returns the output held by `SourceSessionContext`. | `src/session/extensions/source.rs:242` |
| sym-90d3ffa48560606eaf27 | `output_port` | function | Returns the output port held by `SourceOutputHandle`. | `src/session/declaration/draft.rs:935` |
| sym-4b938d8c386fac8b05ab | `output_port` | function | Returns the output port held by `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1020` |
| sym-a944e0ff9385d1203678 | `output_port` | function | Returns the output port held by `SourceOutputSpec`. | `src/session/declaration/spec.rs:141` |
| sym-147483776f9a02d73781 | `output_port` | function | Returns the output port held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:79` |
| sym-8dfdc8dd3a09c0f1b609 | `output_port` | function | Returns the output port held by `SourceManifest`. | `src/session/extensions/source.rs:217` |
| sym-af99ed72a6b2cc205f2e | `output_spec` | function | Returns the output spec held by `TypedOperator`. | `src/session/declaration/typed_stream.rs:87` |
| sym-0265bfc696710c57e223 | `outputs` | function | Returns the outputs associated with `SourceManifest`. | `src/session/extensions/source.rs:170` |
| sym-6d77239070a4c75eb2a1 | `planned_edge_count` | function | Returns the planned edge count held by `CompiledSession`. | `src/session/compile/compiled.rs:57` |
| sym-ae934e93f5b4bf2d9646 | `pocketstation::session::error_code::polled_audio_poll_error_code` | function | Returns the polled audio poll error code held by `error_code`. | `src/session/error_code.rs:255` |

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

The claims on **Session API** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/mod.rs:1-145` (`DIRECT`)

For **Session API**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
