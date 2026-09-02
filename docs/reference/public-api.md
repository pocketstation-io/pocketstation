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
| Select the default microphone | `Source::microphone_default` |
| Select an input device | `Source::device` and `DeviceSelector` |
| Discover applications and devices | `discover_sources` |
| Resolve a structured query | `resolve_query` and `SourceQuery` |
| Inspect microphone permission without prompting | `microphone_permission_observation` |
| Add application-owned PCM | `Session::audio_input` |
| Add a package-owned Source | `Session::register_source` |

`ApplicationSelector` supports display name, application identifier, process
ID, stable source ID, and process instance. Ambiguous names fail before capture.

## Route source output

| Task | API |
|---|---|
| Read audio in the application | `Session::polled_audio` |
| Record an independent stem | `SourceOutputHandle::record` |
| Send to a Connector | `SourceOutputHandle::send_to` |
| Send to an Endpoint | `SourceOutputHandle::send` |
| Connect to an Operator input | `SourceOutputHandle::connect` |

The normal methods create finite routes. `RouteSettings` lets advanced packages
select accepted media and apply a `DeliveryPolicy` explicitly. `EdgeContract`
remains a 1.1.x compatibility name.

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
| Receive lifecycle events | `RunningSession::try_recv_event` |
| Record a bounded trace | `SessionBuilder::session_trace` |
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
