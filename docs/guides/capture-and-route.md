# Capture and route application or microphone audio

Use one `Session` when the same source must feed more than one destination.
PocketStation opens each source once, preserves its identity, and gives every
route its own finite delivery policy.

## Select an application

The concise selector accepts an exact application display name or identifier:

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let application = session.capture(Source::application("Zoom"))?;
# Ok(())
# }
```

Selection is case-insensitive and must resolve one running application. A
missing or ambiguous match fails before capture starts. PocketStation does not
silently choose the first process.

Use an explicit selector when your application already stores a platform
identifier or discovered source identity:

```rust,no_run
use pocketstation::{ApplicationSelector, ProcessId, Source};

let by_identifier = Source::application(
    ApplicationSelector::bundle_id("us.zoom.xos"),
);
let by_process = Source::application(ProcessId::new(1234));
# let _ = (by_identifier, by_process);
```

A process ID is temporary. Resolve it again after the process restarts.
`discover_sources()` provides the stable identity and process instance needed
when an application owns more than one process.

For a selection that must survive beyond the current process, inspect
`CaptureSource::selector_persistence_scope()` and store only identities whose
reported scope matches the intended reuse. The
[platform operations guide](../operations/platform-support.md#reuse-a-discovered-source-safely)
shows how to handle permission, persistence, and rediscovery.

## Add the microphone only when needed

Application and microphone capture are independent declarations:

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let application = session.capture(Source::application("Zoom"))?;
let microphone = session.capture(Source::microphone_default())?;

application.send(session.polled_audio()?)?;
microphone.send(session.polled_audio()?)?;
let mut running = session.start()?;
# let _ = running.stop();
# Ok(())
# }
```

Each frame retains its source, stream, and stem lineage. Combining sources in a
Session does not mix their PCM.

## Choose the audio cadence

Twenty milliseconds is the default for general capture. Select the 10 ms
profile for latency-sensitive voice work that can accept twice the frame and
packet rate:

```rust,no_run
use pocketstation::{AudioFrameDuration, Session};

let session = Session::builder()
    .audio_frame_duration(AudioFrameDuration::Ms10)
    .build();
# let _ = session;
```

Native callbacks may deliver different buffer sizes. PocketStation normalizes
those buffers into the selected 480-sample or 960-sample frame cadence at
48 kHz without allocating on the capture callback.

## Fan out without coupling destinations

Call `send`, `record`, or a Connector-specific publish method on the same stem.
Every destination receives an independent bounded route. A slow destination
reports saturation or drops according to its `RouteSettings`; it does not grow
an unbounded queue or stop unrelated routes.

Start the Session only after every route is declared. Stop it to drain accepted
work and join its workers. Use `cancel` when active asynchronous work must be
aborted.

## Verify the result

During execution, inspect `metrics_snapshot`, `audio_observations`, and Session
events. After shutdown, inspect the stop result and any recording outcome.
Treat a successful start as lifecycle readiness, not proof that every source
has produced audio.

Continue with [recording and observations](record-and-observe.md), or prepare
the host using the [platform guide](../operations/platform-support.md).
