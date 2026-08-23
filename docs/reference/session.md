# Session API

<!-- claims: CLM-REF-002-CAP-001,CLM-REF-002-CAP-002,CLM-REF-002-CAP-003,CLM-REF-002-CAP-004,CLM-REF-002-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Reference authority

The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature.

## Public surface

| Declaration | Kind | Purpose | Source |
|---|---|---|---|
| `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
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
| `SourceDriver::close` | function | Closes `SourceDriver` to further work. | `src/session/extensions/source.rs:273` |
| `SourceDriver::next` | function | Produces the next source emission from `SourceDriver`. | `src/session/extensions/source.rs:269` |
| `SourceDriver::prepare` | function | Prepares resources required by `SourceDriver`. | `src/session/extensions/source.rs:268` |
| `SourceFactory::create` | function | Creates the runtime implementation described by `SourceFactory`. | `src/session/extensions/source.rs:279` |
| `SourceFactory::manifest` | function | Returns the manifest associated with `SourceFactory`. | `src/session/extensions/source.rs:277` |
| `SourceFactory::validate_config` | function | Validates config for `SourceFactory`. | `src/session/extensions/source.rs:278` |
| `StreamSignal::signal_spec` | function | Returns the signal spec associated with `StreamSignal`. | `src/session/declaration/typed_stream.rs:16` |
| `application` | function | Returns the application associated with `Source`. | `src/session/declaration/selector.rs:140` |
| `as_str` | function | Returns the stable string representation of `DeviceId`. | `src/session/declaration/selector.rs:26` |
| `as_str` | function | Returns the stable string representation of `SessionDeclarationErrorCode`. | `src/session/error_code.rs:31` |
| `as_str` | function | Returns the stable string representation of `SessionStartErrorCode`. | `src/session/error_code.rs:86` |
| `as_str` | function | Returns the stable string representation of `SessionRuntimeErrorCode`. | `src/session/error_code.rs:121` |
| `as_str` | function | Returns the stable string representation of `PolledAudioPollErrorCode`. | `src/session/error_code.rs:138` |
| `as_str` | function | Returns the stable string representation of `SessionStopCode`. | `src/session/error_code.rs:157` |
| `as_str` | function | Returns the stable string representation of `SessionStopFailureCode`. | `src/session/error_code.rs:182` |
| `as_str` | function | Returns the stable string representation of `SourceTypeId`. | `src/session/extensions/source.rs:52` |
| `audio_frames_enqueued_total` | function | Returns the audio frames enqueued total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:368` |
| `bundle_id` | function | Returns the bundle identifier associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:44` |
| `cancellation_total` | function | Returns the cancellation total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:372` |
| `capacity_frames` | function | Returns the capacity frames associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:63` |
| `capture_finalization_failures_total` | function | Returns the capture finalization failures total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:330` |
| `close` | function | Closes `AudioInputWriter` to further work. | `src/session/extensions/audio_input/buffer.rs:242` |
| `close` | function | Closes this application-owned input after its accepted frames drain. | `src/session/extensions/audio_input/mod.rs:127` |
| `component` | function | Returns the component associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:89` |
| `configuration` | function | Returns the configuration associated with `Operator`. | `src/session/declaration/draft.rs:311` |
| `configuration` | function | Returns the configuration associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:149` |
| `configuration` | function | Returns the configuration associated with `SourceInstanceSpec`. | `src/session/declaration/spec.rs:88` |
| `configuration` | function | Returns the configuration associated with `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:253` |
| `configuration` | function | Returns the configuration associated with `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:103` |
| `connect` | function | Connects the requested ports through `StemHandle`. | `src/session/declaration/draft.rs:819` |
| `connect` | function | Connects the requested ports through `SourceOutputHandle`. | `src/session/declaration/draft.rs:955` |
| `connect` | function | Connects the requested ports through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1051` |
| `connector_id` | function | Returns the connector identifier associated with `EndpointHandle`. | `src/session/declaration/draft.rs:607` |
| `default` | function | Returns the default `DeviceSelector` value. | `src/session/declaration/selector.rs:113` |
| `derived_route` | function | Returns the derived route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:111` |
| `derived_route_count` | function | Returns the derived route count associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:107` |
| `drop` | function | Releases resources owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:249` |
| `drop_observations` | function | Returns the drop observations associated with `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:206` |
| `drop_rate_pct` | function | Returns the drop rate pct associated with `SessionRouteDropObservations`. | `src/session/lifecycle/observations.rs:171` |
| `endpoint_finalization_failures_total` | function | Returns the endpoint finalization failures total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:334` |
| `error_class` | function | Returns the error class associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:97` |
| `event_queue` | function | Returns the event queue associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:67` |
| `execution` | function | Returns the execution associated with `SourceManifest`. | `src/session/extensions/source.rs:174` |
| `external_source` | function | Returns the external source associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:87` |
| `external_source_count` | function | Returns the external source count associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:83` |
| `finish` | function | Finishes work owned by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:201` |
| `fmt` | function | Formats `OperatorInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:770` |
| `fmt` | function | Formats `OperatorInputHandle` with the requested formatter. | `src/session/declaration/draft.rs:780` |
| `fmt` | function | Formats `SourceInstanceHandle` with the requested formatter. | `src/session/declaration/draft.rs:911` |
| `fmt` | function | Formats `SourceOutputHandle` with the requested formatter. | `src/session/declaration/draft.rs:991` |
| `fmt` | function | Formats `DerivedStreamHandle` with the requested formatter. | `src/session/declaration/draft.rs:1114` |
| `fmt` | function | Formats `StemHandle` with the requested formatter. | `src/session/declaration/draft.rs:1125` |
| `fmt` | function | Formats `EndpointConfiguration` with the requested formatter. | `src/session/declaration/endpoint.rs:85` |
| `fmt` | function | Formats `AudioInputBuffer` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:52` |
| `fmt` | function | Formats `AudioInputWriter` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:261` |
| `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:336` |
| `fmt` | function | Formats `AudioInputWriteError` with the requested formatter. | `src/session/extensions/audio_input/buffer.rs:346` |
| `fmt` | function | Formats `AudioInput` with the requested formatter. | `src/session/extensions/audio_input/mod.rs:143` |
| `fmt` | function | Formats `PcmSource` with the requested formatter. | `src/session/extensions/audio_input/source.rs:74` |
| `fmt` | function | Formats `SourceTypeId` with the requested formatter. | `src/session/extensions/source.rs:742` |
| `frame_capacity_samples` | function | Returns the frame capacity samples associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:340` |
| `frame_samples_per_channel` | function | Returns the frame samples per channel associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:67` |
| `from` | function | Converts the supplied value into `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:325` |
| `from_source_output` | function | Wraps a public external-source output in the same typed Rust façade. Runtime identity remains the output's stable `SignalSpec` and schema. | `src/session/declaration/typed_stream.rs:118` |
| `from_stem` | function | Creates `Stream` from stem. | `src/session/declaration/typed_stream.rs:103` |
| `generation` | function | Returns the implementation generation. | `src/session/extensions/source.rs:166` |
| `get` | function | Returns the value held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:60` |
| `get` | function | Returns the value held by `ProcessId`. | `src/session/declaration/selector.rs:13` |
| `get` | function | Returns the value held by `SourceConfiguration`. | `src/session/extensions/source.rs:100` |
| `handle` | function | Returns the handle associated with `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:197` |
| `id` | function | Returns the id associated with `EndpointHandle`. | `src/session/declaration/draft.rs:603` |
| `id` | function | Returns the id associated with `StemHandle`. | `src/session/declaration/draft.rs:795` |
| `id` | function | Returns the id associated with `DeviceSelector`. | `src/session/declaration/selector.rs:117` |
| `id` | function | Returns the id associated with `ConnectionSpec`. | `src/session/declaration/spec.rs:259` |
| `implementation_generation` | function | Monotonic implementation generation for this manifest revision. | `src/session/extensions/source.rs:158` |
| `ingress_rejected_total` | function | Returns the ingress rejected total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:364` |
| `input` | function | Returns the input associated with `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:729` |
| `input_attempted_total` | function | Returns the input attempted total associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:413` |
| `input_delivered_total` | function | Returns the input delivered total associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:421` |
| `input_dropped_total` | function | Returns the input dropped total associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:425` |
| `input_edge` | function | Returns the input edge associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:153` |
| `input_enqueued_total` | function | Returns the input enqueued total associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:417` |
| `input_port` | function | Returns the input port associated with `TypedOperator`. | `src/session/declaration/typed_stream.rs:75` |
| `input_port` | function | Returns the input port associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:398` |
| `input_queue_capacity_frames` | function | Returns the input queue capacity frames associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:401` |
| `input_queue_depth_frames` | function | Returns the input queue depth frames associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:405` |
| `input_queue_peak_frames` | function | Returns the input queue peak frames associated with `SessionOperatorMetrics`. | `src/session/lifecycle/observations.rs:409` |
| `input_spec` | function | Returns the input spec associated with `TypedOperator`. | `src/session/declaration/typed_stream.rs:83` |
| `insert` | function | Adds declared source configuration. | `src/session/extensions/source.rs:96` |
| `instance_id` | function | Returns the instance identifier associated with `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:725` |
| `instance_id` | function | Returns the instance identifier associated with `SourceInstanceHandle`. | `src/session/declaration/draft.rs:858` |
| `instance_id` | function | Returns the instance identifier associated with `SourceInstanceSpec`. | `src/session/declaration/spec.rs:76` |
| `instance_id` | function | Returns the instance identifier associated with `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:245` |
| `into_parts` | function | Consumes `PcmSource` and returns its component values. | `src/session/extensions/audio_input/source.rs:68` |
| `into_pcm_source` | function | Converts the convenience façade into explicit source, output, and producer ownership. | `src/session/extensions/audio_input/mod.rs:137` |
| `into_rejected` | function | Converts `AudioInputWriteError` into rejected. | `src/session/extensions/audio_input/buffer.rs:319` |
| `invalid_total` | function | Returns the invalid total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:352` |
| `is_cancelled` | function | Returns whether cancelled applies to `SourceCancellation`. | `src/session/extensions/source.rs:255` |
| `is_complete` | function | Returns whether complete applies to `SessionTraceRecorderOutcome`. | `src/session/lifecycle/trace.rs:80` |
| `is_requested` | function | Returns whether requested applies to `SessionStartCancellation`. | `src/session/lifecycle/start_contract.rs:107` |
| `is_sensitive` | function | Returns whether sensitive applies to `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:64` |
| `is_success` | function | Returns whether success applies to `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:320` |
| `iter` | function | Iterates over the values held by `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:68` |
| `iter` | function | Iterates over the values held by `SourceConfiguration`. | `src/session/extensions/source.rs:104` |
| `joined` | function | Returns the joined associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:376` |
| `kind` | function | Returns the kind represented by `AudioInputWriteError`. | `src/session/extensions/audio_input/buffer.rs:315` |
| `kind` | function | Returns the kind represented by `SessionEvent`. | `src/session/lifecycle/events.rs:322` |
| `lineage_failures_total` | function | Returns the lineage failures total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:350` |
| `load_native_extension_library` | function | Loads one trusted native dynamic library from an exact absolute path and imports its supported non-realtime source, operator, and endpoint registrations into this Session as one validated set. | `src/session/extensions/native_library.rs:29` |
| `mark_discontinuity` | function | Marks the next value from `AudioInputBuffer` as discontinuous. | `src/session/extensions/audio_input/buffer.rs:46` |
| `maximum_buffered_audio_bytes` | function | Returns the maximum buffered audio bytes associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:344` |
| `microphone` | function | Creates `Source` for the selected microphone device. | `src/session/declaration/selector.rs:144` |
| `microphone_default` | function | Creates `Source` for the host default microphone. | `src/session/declaration/selector.rs:148` |
| `name` | function | Returns the name associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:63` |
| `new` | function | Creates a new `Operator`. | `src/session/declaration/draft.rs:300` |
| `new` | function | Creates a new `EndpointConfiguration`. | `src/session/declaration/endpoint.rs:33` |
| `new` | function | Creates a new `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:118` |
| `new` | function | Creates a new `ProcessId`. | `src/session/declaration/selector.rs:9` |
| `new` | function | Creates a new `DeviceId`. | `src/session/declaration/selector.rs:22` |
| `new` | function | Creates a new `SourceInstanceId`. | `src/session/declaration/spec.rs:17` |
| `new` | function | Creates a new `OperatorInstanceId`. | `src/session/declaration/spec.rs:30` |
| `new` | function | Creates a new `TypedOperator`. | `src/session/declaration/typed_stream.rs:30` |
| `new` | function | Creates a new `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:29` |
| `new` | function | Creates a stable source implementation identity. | `src/session/extensions/source.rs:26` |
| `new` | function | Creates a new `SourceManifest`. | `src/session/extensions/source.rs:122` |
| `node_type_id` | function | Returns the node type identifier associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:141` |
| `normalized_total` | function | Returns the normalized total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:348` |
| `observations` | function | Returns the observations exposed by `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:246` |
| `observations` | function | Returns the observations exposed by `AudioInput`. | `src/session/extensions/audio_input/mod.rs:131` |
| `operation` | function | Returns the operation associated with `SessionControlFailure`. | `src/session/lifecycle/events.rs:93` |
| `operator` | function | Returns the operator associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:103` |
| `operator_count` | function | Returns the operator count associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:99` |
| `operator_finalization_failures_total` | function | Returns the operator finalization failures total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:338` |
| `operator_id` | function | Returns the operator identifier associated with `Operator`. | `src/session/declaration/draft.rs:307` |
| `operator_id` | function | Returns the operator identifier associated with `EndpointDescriptor`. | `src/session/declaration/endpoint.rs:145` |
| `operator_id` | function | Returns the operator identifier associated with `OperatorInstanceSpec`. | `src/session/declaration/spec.rs:249` |
| `operator_id` | function | Returns the operator identifier associated with `TypedOperator`. | `src/session/declaration/typed_stream.rs:71` |
| `operator_instance_id` | function | Returns the operator instance identifier associated with `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1028` |
| `operator_instance_id` | function | Returns the operator instance identifier associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:304` |
| `origin` | function | Returns the origin associated with `ConnectionSpec`. | `src/session/declaration/spec.rs:263` |
| `outcome` | function | Returns the outcome associated with `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:243` |
| `outcome` | function | Returns the outcome associated with `SessionTrace`. | `src/session/lifecycle/trace.rs:276` |
| `output` | function | Returns the output associated with `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:744` |
| `output` | function | Returns the output associated with `SourceInstanceHandle`. | `src/session/declaration/draft.rs:866` |
| `output` | function | Returns the output associated with `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1036` |
| `output` | function | Returns the output associated with `AudioInput`. | `src/session/extensions/audio_input/mod.rs:107` |
| `output` | function | Returns the output associated with `PcmSource`. | `src/session/extensions/audio_input/source.rs:56` |
| `output` | function | Returns the output associated with `SourceSessionContext`. | `src/session/extensions/source.rs:242` |
| `output_port` | function | Returns the output port associated with `SourceOutputHandle`. | `src/session/declaration/draft.rs:947` |
| `output_port` | function | Returns the output port associated with `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1032` |
| `output_port` | function | Returns the output port associated with `SourceOutputSpec`. | `src/session/declaration/spec.rs:141` |
| `output_port` | function | Returns the output port associated with `TypedOperator`. | `src/session/declaration/typed_stream.rs:79` |
| `output_port` | function | Returns the output port associated with `SourceManifest`. | `src/session/extensions/source.rs:217` |
| `output_spec` | function | Returns the output spec associated with `TypedOperator`. | `src/session/declaration/typed_stream.rs:87` |
| `outputs` | function | Returns the outputs associated with `SourceManifest`. | `src/session/extensions/source.rs:170` |
| `pocketstation::session::error_code::polled_audio_poll_error_code` | function | Returns the polled audio poll error code associated with `error_code`. | `src/session/error_code.rs:255` |
| `pocketstation::session::error_code::session_declaration_error_code` | function | Returns the session declaration error code associated with `error_code`. | `src/session/error_code.rs:195` |
| `pocketstation::session::error_code::session_start_failure_code` | function | Returns the session start failure code associated with `error_code`. | `src/session/error_code.rs:225` |
| `polled_audio` | function | Declares a bounded polled-audio endpoint on `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:71` |
| `pool_exhausted_total` | function | Returns the pool exhausted total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:360` |
| `pool_slots` | function | Returns the pool slots associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:336` |
| `process_id` | function | Returns the process identifier associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:48` |
| `process_instance` | function | Creates `ApplicationSelector` for one exact process instance. | `src/session/declaration/selector.rs:52` |
| `queue_capacity_signals` | function | Returns the queue capacity signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:312` |
| `queue_depth_signals` | function | Returns the queue depth signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:316` |
| `queue_peak_signals` | function | Returns the queue peak signals associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:320` |
| `read` | function | Reads the persisted representation of `SessionTrace`. | `src/session/lifecycle/trace.rs:262` |
| `record` | function | Attaches recording output to `StemHandle`. | `src/session/extensions/recording.rs:60` |
| `record` | function | Attaches recording output to `SourceOutputHandle`. | `src/session/extensions/recording.rs:79` |
| `records` | function | Returns the records associated with `SessionTrace`. | `src/session/lifecycle/trace.rs:272` |
| `reenter_audio` | function | Re-enters this operator output into the Session's specialized audio lane. | `src/session/declaration/draft.rs:1087` |
| `request` | function | Requests the state transition represented by `SessionStartCancellation`. | `src/session/lifecycle/start_contract.rs:103` |
| `revision` | function | Additive descriptor revision within the compatibility major encoded by the [`SourceTypeId`] suffix. A breaking source contract uses a new identifier ending in the next `vN`; it does not reuse this field. | `src/session/extensions/source.rs:149` |
| `route` | function | Returns the route associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:95` |
| `route_count` | function | Returns the route count associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:91` |
| `runtime_events_total` | function | Returns the runtime events total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:358` |
| `runtime_failures_total` | function | Returns the runtime failures total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:346` |
| `runtime_worker_panicked` | function | Returns the runtime worker panicked associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:342` |
| `safety` | function | Returns the safety associated with `SourceManifest`. | `src/session/extensions/source.rs:178` |
| `sample_capacity` | function | Returns the sample capacity associated with `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:19` |
| `sample_count` | function | Returns the sample count associated with `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:23` |
| `sample_spec` | function | Returns the sample spec associated with `AudioInputConfig`. | `src/session/extensions/audio_input/mod.rs:59` |
| `samples` | function | Returns the audio samples held by `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:38` |
| `samples_mut` | function | Returns the samples mut associated with `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:42` |
| `send` | function | Sends a value through `StemHandle`. | `src/session/declaration/draft.rs:799` |
| `send` | function | Sends a value through `SourceOutputHandle`. | `src/session/declaration/draft.rs:951` |
| `send` | function | Sends a value through `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1069` |
| `send` | function | Sends a value through `Stream`. | `src/session/declaration/typed_stream.rs:151` |
| `send_to` | function | Connects this stream to one explicit endpoint input port. | `src/session/declaration/draft.rs:804` |
| `send_to` | function | Routes the current source output to the requested destination through `SourceOutputHandle`. | `src/session/declaration/draft.rs:959` |
| `send_to` | function | Connects this derived output to one explicit endpoint input port. | `src/session/declaration/draft.rs:1074` |
| `session_id` | function | Returns the session identifier associated with `EndpointHandle`. | `src/session/declaration/draft.rs:599` |
| `session_id` | function | Returns the session identifier associated with `OperatorInstanceHandle`. | `src/session/declaration/draft.rs:721` |
| `session_id` | function | Returns the session identifier associated with `StemHandle`. | `src/session/declaration/draft.rs:791` |
| `session_id` | function | Returns the session identifier associated with `SourceInstanceHandle`. | `src/session/declaration/draft.rs:854` |
| `session_id` | function | Returns the session identifier associated with `SourceOutputHandle`. | `src/session/declaration/draft.rs:931` |
| `session_id` | function | Returns the session identifier associated with `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1024` |
| `session_id` | function | Returns the session identifier associated with `SessionEvent`. | `src/session/lifecycle/events.rs:318` |
| `session_id` | function | Returns the session identifier associated with `SessionTrace`. | `src/session/lifecycle/trace.rs:268` |
| `shared_audio_rejected_total` | function | Returns the shared audio rejected total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:356` |
| `signal_spec` | function | Returns the signal spec associated with `Stream`. | `src/session/declaration/typed_stream.rs:128` |
| `signals_dropped_total` | function | Returns the signals dropped total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:332` |
| `signals_enqueued_total` | function | Returns the signals enqueued total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:324` |
| `signals_received_total` | function | Returns the signals received total associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:328` |
| `source` | function | Returns the source associated with `AudioInput`. | `src/session/extensions/audio_input/mod.rs:103` |
| `source` | function | Returns the source associated with `PcmSource`. | `src/session/extensions/audio_input/source.rs:52` |
| `source` | function | Returns the source associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:79` |
| `source_count` | function | Returns the source count associated with `SessionMetricsSnapshot`. | `src/session/lifecycle/observations.rs:75` |
| `source_id` | function | Returns the source identifier associated with `SourceInstanceHandle`. | `src/session/declaration/draft.rs:862` |
| `source_id` | function | Returns the source identifier associated with `SourceOutputHandle`. | `src/session/declaration/draft.rs:939` |
| `source_id` | function | Returns the source identifier associated with `SourceInstanceSpec`. | `src/session/declaration/spec.rs:80` |
| `source_instance_id` | function | Returns the source instance identifier associated with `SourceOutputHandle`. | `src/session/declaration/draft.rs:935` |
| `source_instance_id` | function | Returns the source instance identifier associated with `SourceOutputSpec`. | `src/session/declaration/spec.rs:137` |
| `source_send_rejections_total` | function | Returns the source send rejections total associated with `SessionStopOutcome`. | `src/session/lifecycle/start_contract.rs:354` |
| `source_to_receive_latency` | function | Returns the source to receive latency associated with `SessionRouteMetrics`. | `src/session/lifecycle/observations.rs:223` |
| `source_type_id` | function | Returns the source type identifier associated with `SourceInstanceSpec`. | `src/session/declaration/spec.rs:84` |
| `source_type_id` | function | Returns the source type identifier associated with `SourceManifest`. | `src/session/extensions/source.rs:142` |
| `stable_id` | function | Returns the stable identifier associated with `ApplicationSelector`. | `src/session/declaration/selector.rs:59` |
| `start` | function | Starts the lifecycle represented by `SessionTraceRecorder`. | `src/session/lifecycle/trace.rs:160` |
| `stem_id` | function | Returns the stem identifier associated with `SessionAudioReentryMetrics`. | `src/session/lifecycle/observations.rs:308` |
| `stream_id` | function | Returns the stream identifier associated with `SourceOutputHandle`. | `src/session/declaration/draft.rs:943` |
| `stream_id` | function | Returns the stream identifier associated with `SourceOutputSpec`. | `src/session/declaration/spec.rs:145` |
| `target` | function | Returns the target associated with `ConnectionSpec`. | `src/session/declaration/spec.rs:267` |
| `through` | function | Routes the current stream through a declared operator using `StemHandle`. | `src/session/declaration/draft.rs:823` |
| `through` | function | Routes the current stream through a declared operator using `SourceOutputHandle`. | `src/session/declaration/draft.rs:975` |
| `through` | function | Routes the current stream through a declared operator using `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1055` |
| `through` | function | Routes the current stream through a declared operator using `Stream`. | `src/session/declaration/typed_stream.rs:132` |
| `through_ports` | function | Returns the through ports associated with `StemHandle`. | `src/session/declaration/draft.rs:827` |
| `through_ports` | function | Returns the through ports associated with `SourceOutputHandle`. | `src/session/declaration/draft.rs:979` |
| `through_ports` | function | Returns the through ports associated with `DerivedStreamHandle`. | `src/session/declaration/draft.rs:1059` |
| `try_acquire` | function | Attempts to acquire through `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:107` |
| `try_acquire` | function | Acquires one preallocated buffer owned by this input. | `src/session/extensions/audio_input/mod.rs:112` |
| `try_copy_from_slice` | function | Attempts to copy from slice through `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:34` |
| `try_send` | function | Attempts to send a value through `AudioInputWriter` without waiting for capacity. | `src/session/extensions/audio_input/buffer.rs:147` |
| `try_send` | function | Submits one previously acquired buffer without blocking. | `src/session/extensions/audio_input/mod.rs:122` |
| `try_set_sample_count` | function | Attempts to set sample count through `AudioInputBuffer`. | `src/session/extensions/audio_input/buffer.rs:27` |
| `try_write` | function | Attempts to write through `AudioInputWriter`. | `src/session/extensions/audio_input/buffer.rs:135` |
| `try_write` | function | Writes one complete interleaved frame without blocking. | `src/session/extensions/audio_input/mod.rs:117` |
| `validate` | function | Validates `SourceManifest` against its declared contract. | `src/session/extensions/source.rs:182` |
| `validate` | function | Validates `SessionTrace` against its declared contract. | `src/session/lifecycle/trace.rs:280` |
| `value` | function | Returns the value associated with `SourceInstanceId`. | `src/session/declaration/spec.rs:21` |
| `value` | function | Returns the value associated with `OperatorInstanceId`. | `src/session/declaration/spec.rs:34` |
| `with` | function | Returns `EndpointConfiguration` with the supplied entry applied. | `src/session/declaration/endpoint.rs:37` |
| `with_configuration` | function | Sets the configuration on `EndpointDescriptor` and returns the updated value. | `src/session/declaration/endpoint.rs:127` |
| `with_input_edge` | function | Declares the bounded delivery policy for routes entering this endpoint. | `src/session/declaration/endpoint.rs:136` |
| `with_sensitive` | function | Adds a setup-time value whose normal debug representation is redacted. | `src/session/declaration/endpoint.rs:49` |
| `writer` | function | Returns the writer associated with `PcmSource`. | `src/session/extensions/audio_input/source.rs:60` |
| `writer_mut` | function | Returns the writer mut associated with `PcmSource`. | `src/session/extensions/audio_input/source.rs:64` |
| `pocketstation::session::declaration::draft::DerivedStreamHandle` | struct | Owns bounded access to derived stream. | `src/session/declaration/draft.rs:839` |
| `pocketstation::session::declaration::draft::EndpointHandle` | struct | Owns bounded access to endpoint. | `src/session/declaration/draft.rs:592` |
| `pocketstation::session::declaration::draft::Operator` | struct | Represents operator in the PocketStation API. | `src/session/declaration/draft.rs:294` |
| `pocketstation::session::declaration::draft::OperatorInputHandle` | struct | Owns bounded access to operator input. | `src/session/declaration/draft.rs:713` |
| `pocketstation::session::declaration::draft::OperatorInstanceHandle` | struct | Owns bounded access to operator instance. | `src/session/declaration/draft.rs:706` |
| `pocketstation::session::declaration::draft::SourceInstanceHandle` | struct | Owns bounded access to source instance. | `src/session/declaration/draft.rs:846` |

## Interpretation

An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/mod.rs:1-143` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
