# Read events, metrics, outcomes, and errors

PocketStation reports setup failure, live observations, and terminal outcomes
separately. Do not infer successful delivery from a successful declaration or
from a provider worker starting.

## Setup errors

| Stage | Main result |
|---|---|
| Declaration | `SessionError` |
| Compilation | `SessionCompileDiagnostic` |
| Preparation and start | `SessionStartError` |
| Application-owned PCM write | `AudioInputWriteError` |
| Polled audio read | `PolledAudioPollError` |

Stable error codes are available for language SDKs and automation. Keep the
code with the human-readable message.

## Session events

`SessionEvent` identifies the component, event kind, and Session time. Events
cover source lifecycle, permission changes, discontinuities, route saturation,
provider readiness and failure, recording, cancellation, and finalization.

The event queue is finite. `SessionEventQueueObservations` reports capacity,
delivery, and dropped events. A dropped diagnostic event does not imply that
media was dropped; inspect route metrics separately.

## Metrics snapshots

`SessionMetricsSnapshot` groups the current measurements for:

- Sources and source-owned delivery;
- routes and route latency;
- Operators and their named inputs;
- Connectors and Endpoints;
- polled application audio;
- application-owned PCM and generated-audio reentry;
- sidecars; and
- recording.

Counters are cumulative unless their type states otherwise. Durations include
their unit in the field or enum. Unavailable measurements remain unavailable;
they are not reported as zero.

## Route latency

`SessionRouteLatencyObservations` identifies what was measured with
`RouteLatencyMeasurement` and `SessionRouteLatencyUnit`. Compare values only
when both definitions match. Source timestamp to route receive includes a
different amount of work than capture callback to network send or browser
receive.

Virtual-machine scheduler spikes are correctness evidence, not host latency
qualification. Record p50, p95, p99, maximum, sample count, operating system,
hardware, frame duration, and load for a performance claim.

## Recording outcome

`SessionRecordingOutcome` contains one result per requested stem. Check:

- terminal recording state;
- file and manifest locations;
- frames and samples written;
- first and last timestamps;
- discontinuities;
- source and stem identity; and
- final error code and message when present.

A Session can stop while a stem remains empty because its Source never produced
media. Treat an empty stem according to the application requirement rather than
assuming the recorder failed.

## Connector and Endpoint failure

Endpoint failures retain their stage and may include an external stable code
and retry classification. The stage distinguishes preparation, readiness,
delivery, shutdown, and finalization. A retry classification does not execute a
retry; the provider package or application must apply a finite retry policy.

During shutdown, require joined finalization. A detached provider thread or
unreaped child process is a failure even if media delivery previously worked.

## Output cancellation

Generated PCM can carry an `OutputGeneration`. Cancelling that output causes
Core routes to discard matching queued frames and increment discard counters.
Inspect Connector and receiver observations for later queues. PocketStation
does not claim acoustic hearing when a receiver cannot report playout.

## Trace files

A `SessionTrace` records selected declarations, events, metrics, and terminal
state for later diagnosis. Tracing stops at its configured record capacity. Check
`SessionTraceRecorderOutcome` and `SessionTraceValidation` before relying on a
trace as evidence.

## Recovery sequence

1. Preserve the stable error code and component identity.
2. Inspect whether Sources and unrelated routes remain valid.
3. Request drain or abort once.
4. Wait for the terminal Session result.
5. Confirm recording and provider finalization.
6. Rediscover Sources or refresh credentials when the error requires it.
7. Start a new Session only after the previous one has joined.

Continue with [troubleshooting](../troubleshooting.md) for failure-specific
actions.
