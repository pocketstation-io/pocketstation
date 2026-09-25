# Capture and route desktop audio

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

## Capture everything playing on the computer

Use system audio when the workflow needs the complete output mix instead of
one application:

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let system_audio = session.capture(Source::system_audio())?;
system_audio.send(session.polled_audio()?)?;
let mut running = session.start()?;
# let _ = running.stop();
# Ok(())
# }
```

System audio can include notifications, media, and other applications. Prefer
`Source::application` when the user selected one application and unrelated
desktop audio must remain outside the Session.

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
Every destination receives an independent route with its own queue. A slow
destination reports saturation or drops according to its `RouteSettings`; the queue does not grow
an unbounded queue or stop unrelated routes.

Start the Session only after every route is declared. Stop it to drain accepted
work and join its workers. Use `cancel` when active asynchronous work must be
aborted.

## Verify the result

During execution, inspect `metrics_snapshot`, `audio_observations`, and Session
events. After shutdown, inspect the stop result and any recording outcome.
Treat a successful start as lifecycle readiness, not proof that every source
has produced audio.

For each built-in Source, `SessionMetricsSnapshot::source_activity(index)`
reports when the Session started, when the first and latest frames reached the
Session runtime, when the snapshot was taken, and how many frames were
observed. Its index matches `SessionMetricsSnapshot::source(index)`. Evaluate
that raw activity with `SessionSourceActivityPolicy`, using deadlines justified
by your workflow. The result distinguishes `AwaitingFirstFrame`, `Active`,
`FirstFrameTimedOut`, and `Stalled` without restarting or replacing the Source.
C hosts obtain the same raw values with
`pks_session_source_activity_at`; `frames_received_total == 0` means the first
and latest frame timestamps are unavailable and encoded as zero.

Activity is not signal energy. A frame of digital silence is still an active
frame. `SessionMetricsSnapshot::source_signal(index)` separately reports the
latest canonical PCM frame's sample totals, exact-zero and non-finite counts,
peak, RMS, source time, generation, and discontinuity. Evaluate it with a
caller-created `SessionSourceSignalPolicy`:

```rust,no_run
use std::time::Duration;
use pocketstation::{SessionSourceSignalPolicy, SessionSourceSignalState};

# fn inspect(running: &pocketstation::RunningSession, source_index: usize)
#     -> Result<(), Box<dyn std::error::Error>> {
let policy = SessionSourceSignalPolicy::new(
    -36.0,                         // minimum peak, dBFS
    -48.0,                         // minimum RMS, dBFS
    Duration::from_millis(1_500), // exact-zero window
)?;
let snapshot = running.metrics_snapshot()?;
let signal = snapshot
    .source_signal(source_index)
    .ok_or("source index is unavailable")?
    .evaluate(policy);

if signal.state == SessionSourceSignalState::SustainedExactDigitalZero {
    // The host chooses whether to reopen, replace, pause, or stay degraded.
}
# Ok(())
# }
```

The thresholds describe the application's numeric requirement. They do not
prove speech, audibility, permission, or correct routing. Measurement runs on
the Session worker after capture dequeue, never on the native callback.

For a microphone, the host may keep unaffected stems running while it applies
an explicit recovery decision. `replace_microphone_source` opens a selected
device before detaching the current microphone. `reopen_microphone_source`
closes the current capture first and then reacquires the selected device:

```rust,no_run
use pocketstation::{DeviceId, DeviceSelector};

# fn reopen(
#     running: &mut pocketstation::RunningSession,
#     microphone: &pocketstation::StemHandle,
# ) -> Result<(), Box<dyn std::error::Error>> {
let replacement = running.reopen_microphone_source(
    microphone.id(),
    DeviceSelector::id(DeviceId::new("host-selected-device-id")),
)?;
println!(
    "source={:?} generation={} discontinuity={}",
    replacement.source_id,
    replacement.source_generation,
    replacement.discontinuity_epoch,
);
# Ok(())
# }
```

Success means the physical capture opened and attached. It does not mean the
first frame or useful signal arrived. Re-evaluate activity and signal after the
operation. PocketStation never chooses a fallback selector or retry policy.

`SessionMetricsSnapshot::source_native_format(index)` reports the device format
opened before conversion to PocketStation's canonical mono 48 kHz `f32`
signal. It may be unavailable for a backend that does not negotiate a native
PCM format; absence is not reported as a fabricated default.

Continue with [recording and observations](record-and-observe.md), or prepare
the host using the [platform guide](../operations/platform-support.md).
