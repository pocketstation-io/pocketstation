# Find the Rust API for a task

Start with `Session`, `Source`, and the methods on their returned handles. Use
the advanced graph and extension types when another package implements a new
Source, Operator, Connector, or Endpoint.

## Session setup

| Task | API |
|---|---|
| Create a Session with defaults | `Session::new` |
| Configure recording, sample format, frame duration, or tracing | `Session::builder` |
| Start declared work | `Session::start` or `start_cancellable` |
| Stop and drain accepted work | `RunningSession::stop` |
| Abort pending work | `RunningSession::cancel` |

## Sources and selection

| Task | API |
|---|---|
| Select a desktop application | `Source::application` |
| Capture the complete output mix | `Source::system_audio` |
| Select the default microphone | `Source::microphone_default` |
| Select an input device | `Source::device` and `DeviceSelector` |
| Discover applications and devices | `discover_sources` |
| Discover process instances with retained start observations | `discover_process_instances` |
| Capture an immutable exact process set through one native tap | `ObservedProcessInstance`, `ApplicationProcessSet`, `Session::capture_process_set` |
| Resolve a structured query | `resolve_query` and `SourceQuery` |
| Inspect microphone permission without prompting | `microphone_permission_observation` |
| Reopen an exact running microphone after teardown | `RunningSession::reopen_microphone_source` |
| Replace a running microphone without first detaching it | `RunningSession::replace_microphone_source` |
| Add application-owned PCM | `Session::audio_input` |
| Add a package-owned Source | `Session::register_source` |

`ApplicationSelector` supports display name, application identifier, process
ID, stable source ID, and process instance. Ambiguous names fail before capture.

Exact process sets are an additive Session source operation; existing exhaustive
selectors are unchanged. They require a native macOS 26+ build and return a
`SourceOutputHandle`. Unsupported platforms/builds return `capture.unsupported`.
Discovery omits unobservable rows, so callers must establish complete intended
membership independently and require a valid original observation for each
member. See [exact process-set capture](../guides/capture-and-route.md#capture-a-frozen-process-set).

## Route source output

| Task | API |
|---|---|
| Read audio in the application | `Session::polled_audio` |
| Record an independent stem | `SourceOutputHandle::record` |
| Send to a Connector | `SourceOutputHandle::send_to` |
| Send to an Endpoint | `SourceOutputHandle::send` |
| Connect to an Operator input | `SourceOutputHandle::connect` |

The normal methods create finite routes. `RouteSettings` lets advanced packages
select accepted media and apply a `DeliveryPolicy` explicitly.

## Operators and typed signals

| Task | API |
|---|---|
| Declare an Operator | `Operator` |
| Register an async implementation | `Session::register_operator` |
| Describe ports | `PortSpec` and `SignalSpec` |
| Describe audio compatibility | `MediaCaps` and `AudioCaps` |
| Carry runtime payload and lineage | `SignalEnvelope` |
| Add Rust declaration-time typing | `Stream<T>` and `TypedOperator` |

Operator work executes outside capture callbacks. Its manifest declares named
ports, execution partition, deadlines, cancellation, permissions, and failure
behavior.

## Connectors and Endpoints

| Task | API |
|---|---|
| Send frames with one Rust function | `Connector::from_audio_fn` |
| Implement a focused stateful destination | `AudioConnector` |
| Register a Connector driver | `Session::register_connector_driver` |
| Implement complete Endpoint setup and shutdown | `EndpointDriverFactory` |
| Receive source-aware frames | `EndpointAudioReceiver` |
| Report readiness and wait for Session start | `EndpointStartGate` |
| Distinguish drain from abort | `EndpointShutdownMode` |

Use a Connector for outbound provider delivery. Use an Operator for
computation that emits media or typed signals. Use a Source when an external
system brings data into the Session.

## Recording and observations

| Task | API |
|---|---|
| Configure the recording directory | `SessionBuilder::recording_root` |
| Read the terminal recording result | `RunningSession::recording_outcome` |
| Read one metrics snapshot | `RunningSession::metrics_snapshot` |
| Inspect actual native capture format | `SessionMetricsSnapshot::source_native_format` |
| Evaluate first-frame and stall activity | `source_activity` and `SessionSourceActivityPolicy` |
| Evaluate delivered PCM signal | `source_signal` and `SessionSourceSignalPolicy` |
| Inspect explicit microphone replacement | `SessionMetricsSnapshot::source_replacement` |
| Receive lifecycle events | `RunningSession::try_recv_event` |
| Record a Session trace with a fixed record capacity | `SessionBuilder::session_trace` |
| Validate a trace | `SessionTraceValidation` |

## Native extensions and managed processes

| Task | API |
|---|---|
| Load a trusted compiled extension | `Session::load_native_extension_library` |
| Register a managed child process | `Session::register_sidecar` |
| Set child deadlines | `SidecarDeadlines` |
| Set protocol limits | `SidecarProtocolLimits` |
| Send or receive sidecar signals | `try_send_sidecar_signal`, `try_receive_sidecar_signal` |

Native libraries execute inside the host process and require an absolute file
location plus application trust policy. Sidecars use the versioned PKSS
protocol and remain subject to finite message, queue, startup, and shutdown
limits.

## Stable identities

PocketStation uses typed identities for Sessions, Sources, Streams, Stems,
routes, Connectors, Endpoints, Operator instances, output generations, and
clock domains. Preserve these values in logs and provider metadata so a frame,
failure, recording stem, and remote publication can be correlated.

## Continue developing

- [Own capture, routing, and shutdown with one Session](../concepts/session-lifecycle.md)
- [Set delivery behavior for each route](../concepts/delivery-and-failure.md)
- [Read events, metrics, outcomes, and errors](events-and-errors.md)

### C ABI 1.2 exact process sets

`pks_session_discover_process_instances` copies original PID, platform, stable
key and nanosecond start observations into caller-owned buffers. A size query
and its retry are separate discoveries. An adequate call copies one complete
observation; it never partially fills rows or key bytes. Initialize every row's
1.2 size/version header before each retry, and keep its key arena alive until
`pks_session_create_process_set` returns.

`pks_session_create_process_set` copies 1–32 original observations and declares
48 kHz stereo audio in 20 ms frames. It opens no microphone. Compile and start
remain explicit. The existing app-plus-microphone constructor is unchanged.
Only macOS 26+ builds with native capture support this selection; other builds
return the documented unsupported detail without selecting another source.

The library reports capability version 1.2. Existing event, metrics and audio
records retain their 1.1 record version and layout; the additive process-set,
external-source and stop-result records require 1.2 headers.

`pks_session_external_source_metrics_at` reports actual registered-source
observations. `pks_session_stop_outcome_poll` requires a completed stop and
reports its actual failure counters. Failed native closure keeps the Session
and engine handles available for inspection. Repeated stop/destroy calls do
not erase an unconfirmed close or promise another native attempt. Confirmed
closure following source loss can release the handles while the stop result
continues to report the source failure. Shut-down engines reject new work.
