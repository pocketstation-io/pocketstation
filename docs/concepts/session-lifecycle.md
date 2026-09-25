# Own capture, routing, and shutdown with one Session

Use one `Session` for all sources and destinations that must start, stop, and
report an outcome together. Declare the work first. `Session::start` validates
the declaration, prepares every component, and begins delivery only after
preparation succeeds.

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let application = session.capture(Source::application("Spotify"))?;
application.send(session.polled_audio()?)?;

let mut running = session.start()?;
// Read frames, events, or metrics while the Session is running.
let result = running.stop();
if !result.is_success() {
    return Err(format!("Session failed during shutdown: {:?}", result.outcome()).into());
}
# Ok(())
# }
```

## Declare before starting

A Session declaration may contain:

- desktop application and microphone Sources;
- application-owned PCM from `audio_input`;
- typed Sources supplied by another package;
- Operators with named inputs and outputs;
- Connectors and lower-level Endpoints;
- polled audio for application code;
- multistem recording;
- managed sidecars and trusted native extensions; and
- a Session trace with a configured record capacity.

Declarations are setup-time work. Capture callbacks do not register routes,
allocate providers, or call application code.

## Start as one transaction

`Session::start` performs these steps:

1. Freeze the declaration so it can no longer change.
2. Validate source, port, media, and route settings.
3. Compile the typed execution plan.
4. Prepare Sources, Operators, Connectors, Endpoints, recording, and sidecars.
5. Roll back prepared resources if any component fails.
6. Open the start gates after the complete Session is ready.

A start error includes a stable code. Compile errors may also include a
`SessionCompileDiagnostic` with the affected component and reason.

## Observe while running

`RunningSession` provides separate APIs for media and operational state:

| Need | API |
|---|---|
| Poll source-aware audio | `try_poll_audio` or `wait_audio` |
| Receive lifecycle events | `try_recv_event` |
| Read route and component metrics | `metrics_snapshot` |
| Inspect native format, activity, signal, and replacement | `metrics_snapshot` Source observations |
| Reopen or replace one microphone explicitly | `reopen_microphone_source` or `replace_microphone_source` |
| Read Operator metrics | `operator_metrics` |
| Read external Source metrics | `external_source_metrics` |
| Read sidecar metrics | `sidecar_metrics` |
| Read generated-audio metrics | `audio_reentry_metrics` |
| Read derived-route metrics | `derived_route_metrics` |

Polling an empty queue is not a failure. Distinguish empty, closed, cancelled,
and terminal failure results in the caller.

## Stop or cancel

Use `stop` for an orderly finish. Accepted work may drain before workers join.
Use `cancel` when the application must stop promptly. Cancellation upgrades
shutdown to abort and cannot later be weakened to drain.

Both operations return a terminal result. Require success before treating
recordings or remote publication as complete.

## Recover a disappeared source

PocketStation does not silently switch to another application or microphone.
When an application Source disappears:

1. record the source event and discontinuity;
2. stop the current Session;
3. discover Sources again;
4. confirm the new selection when identity changed; and
5. declare and start a new Session.

The Source reports how long its selector may be reused. A process-lifetime
selector must be rediscovered after that process exits.

For one microphone, the host may instead keep the Session and unrelated stems
running. `replace_microphone_source` safely opens the selected replacement
before detaching the current microphone. `reopen_microphone_source` tears down
the current microphone first when the same native route must be reset. Both
operations preserve the logical stem and routes while advancing physical
source generation and discontinuity.

Core does not infer the need for recovery. Use activity to establish frame
delivery, signal observations for caller-owned numeric thresholds, and the
opened native format for acquisition diagnostics. After an operation succeeds,
wait again for a first frame and useful signal according to the host's finite
policy.

## Continue developing

- [Set delivery behavior for each route](delivery-and-failure.md)
- [Record stems and inspect Session delivery](../guides/record-and-observe.md)
- [Read events, metrics, outcomes, and errors](../reference/events-and-errors.md)
