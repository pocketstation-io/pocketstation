# Record stems and inspect Session delivery

Recording and metrics use the same source-aware routes as every other
destination. Add them before `Session::start`, then inspect their terminal
outcomes after stopping the Session.

## Record independent stems

Configure one recording root and give every recorded stem a distinct name:

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::builder()
    .recording_root("recordings")
    .build();
let application = session.capture(Source::application("Zoom"))?;
let microphone = session.capture(Source::microphone_default())?;

application.record("application")?;
microphone.record("microphone")?;
let mut running = session.start()?;
# let _ = running.stop();
# Ok(())
# }
```

Each stem begins when its first authoritative frame arrives. A silent or late
stem does not prevent active stems from progressing. The Session manifest
retains source, stream, stem, timing, and discontinuity information beside the
WAV files.

## Observe bounded delivery

Use `metrics_snapshot()` while the Session is running. Route metrics report
finite capacity, queue depth, delivered frames, drops, and latency observations
where the route can measure them. Use `try_recv_event()` for permission,
source, endpoint, rollback, and terminal lifecycle events.

Unavailable values remain unavailable. Do not replace a missing receiver or
acoustic measurement with a sender timestamp.

## Stop or cancel deliberately

`stop()` requests a normal drain: work already accepted by bounded routes is
allowed to finalize. `cancel()` aborts active asynchronous Operators before the
Session joins capture, endpoints, and recording. Calling either operation
again returns an already-stopped disposition rather than starting another
shutdown.

After shutdown:

- require a successful Session stop result;
- inspect `recording_outcome()` and each stem result;
- preserve structured failures and discontinuities;
- remove recordings only when the application no longer needs them.

A completed recording proves persistence for that run. It does not by itself
prove remote delivery, loudspeaker playout, or behavior on another host.
