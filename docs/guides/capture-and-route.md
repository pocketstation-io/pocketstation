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

## Capture a frozen process set

When one selected application instance owns several processes with different
native application identities, use `Session::capture_process_set`. It opens one
native stereo mixdown tap with the existing clock and frame assembly. Delivery
uses the existing capture queue and its configured overflow behavior. The returned `SourceOutputHandle` routes like other registered-source
outputs; the individual processes are members of that one source, not separate
stems or a JavaScript mix.

```rust,no_run
use pocketstation::{ApplicationProcessSet, ObservedProcessInstance, ProcessSetError,
    Session, SourceOutputHandle};

fn capture_selected_instances(
    session: &Session,
    observed_members: Vec<ObservedProcessInstance>,
) -> Result<SourceOutputHandle, ProcessSetError> {
    session.capture_process_set(ApplicationProcessSet::new(observed_members)?)
}
```

Call `discover_process_instances()` to obtain usable PID, stable application
identity and original process-start observations. Keep these together. The
caller establishes which application instance is intended, for example by
joining its independently discovered browser-root ancestry to the observations.
Require an observation for **every** intended member before declaring the set.
Unobservable or churned rows are omitted from this discovery result; it is not
evidence that the list contains every member of an application instance. Do not
select only the loudest process or assume a silent member is irrelevant.

Sets contain 1–32 distinct processes from one platform. PIDs must fit the
positive native signed process-ID range and start observations must be nonzero.
Construction canonicalizes order and rejects empty, oversized, duplicate and
conflicting selections. Opening compares every original stable/start identity,
resolves every member to a native process object, and checks every member again
after starting the tap, before downstream delivery. A partial resolution fails;
it never falls back to a broader application identity or the system mix.

This capability requires macOS 26+ and native capture, where
[automatic process restoration](https://developer.apple.com/documentation/coreaudio/catapdescription/isprocessrestoreenabled)
can explicitly be disabled. The same capability check gates discovery,
declaration and opening. Earlier supported macOS releases retain their existing
selectors; they do not advertise strict-set availability. Other platforms and
builds return `capture.unsupported` for this operation.

The output is stereo. Its native rate and format must match the declared Session
rate and format; opening fails before delivery if they differ. No resampling or
implicit channel conversion is added. Per-source stereo media remains independent
of the Session's default microphone channel count. Queue and native-ring loss
retain sequence gaps and advance the discontinuity epoch.

After admission, original surviving members may continue if another member exits.
The selection never discovers or follows a new PID or restored bundle identity.
When all original members exit, the registered source reports failure and joins
after confirmed close. Inspect this output's external-source `failure_total` and
`joined` metrics; the whole Session may still run other sources. Rediscover and
create a new source/session explicitly to bind a changed instance.

Startup `capture.cleanup_unconfirmed` means native teardown could not be confirmed,
including cleanup after another startup resource fails. Preserve ownership and
the original failure context. During stop, an actual native close failure has
operation `close_external_capture` and error class `capture-close-unconfirmed`.
An ordinary source-ended/read failure followed by confirmed close does not have
that marker. Package tests exercise these failures; they do not prove
physical application isolation or independently decoded guest sound.

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

## Cancel capture setup

Use `Session::start_cancellable` with a `SessionStartCancellation` when the user
can cancel while a device is opening. Request the token from the application's
control thread. The same token reaches capture acquisition: after a synchronous
native call returns, PocketStation checks it before acquiring the next resource,
starting a device or creating a reader. A cancelled application open prevents
the microphone declaration from opening.

Cancellation does not interrupt a CoreAudio call already in progress. If device
start has already begun, setup waits for its return and attempts reverse cleanup.
The public `SessionStartError::rollback_failures_total()` getter preserves the
startup transaction's reported cleanup failure count for both cancellation and
backend errors. A nonzero count means rollback could not confirm cleanup; retain
that distinction in application state. Zero means no rollback failure was
reported, and does not prove physical resource release. The error's existing
code, message, cancellation kind and compiler diagnostic remain available.
Consumers need no internal engine error type or error-message parsing.

This count applies to transactional Session startup used by ordinary
recording and audio-history composition. It does not cover compensation after
an explicitly registered sidecar fails to start, or the defensive missing-event
receiver invariant fallback. These are separate cleanup-reporting limitations;
recording and history use endpoints and do not implicitly register sidecars.

On macOS, application and system capture share a process-wide limit of 64 native
tap owners, including opens in progress. Confirmed cleanup releases a slot.
If unregistering or destroying a native resource fails, or a registration or
creation result leaves ownership unknown, PocketStation retains that owner's
callback context and slot, and refuses further native tap opens in this process.
It does not retry uncertain cleanup in `Drop` or automatically restart the
process. Existing admitted owners keep their own contexts; the microphone uses
its separate input backend. This prevents repeated failed opens from retaining
an unbounded number of contexts.

The cancellation and cleanup tests use injected native operations. They establish
operation ordering and failure reporting; they do not establish native-call latency or
successful cleanup on a physical audio device.

### Observe a native call during setup

Use `SessionStartCancellation::observed()` before starting the Session to
enable native control-call observations. Keep a clone on the application's
control thread while its existing setup worker calls `start_cancellable`.
Ordinary default tokens leave this optional storage disabled.

```rust,no_run
use pocketstation::SessionStartCancellation;

let start_token = SessionStartCancellation::observed();
let observer = start_token.clone();
// Pass start_token to the existing Session setup worker.
if let Some(snapshot) = observer.native_call_observations() {
    for call in snapshot.current.iter().flatten() {
        let _operation = call.operation;
        let _started_at_ns = call.started_at_ns;
    }
}
```

Each capture open has an ordinal and its own current-call slot. A call appears
before native entry and remains current until return, including during reverse
cleanup on a reader thread. The snapshot uses PocketStation's process monotonic
nanosecond clock, never wall-clock time. Ordinals are diagnostic identities;
snapshots contain no application names, process IDs, device IDs or audio.

Up to 64 opens can be observed independently. The first 128 completed spans
retain return times and signed statuses. HAL property and device leaves expose
their OSStatus; callback registration exposes the existing controller result,
which maps zero with no callback identity to an unspecified error. Native calls
without a status, including discovery and process-instance queries, leave the
status absent. Separate non-status spans also cover the existing global/process
tap-description initializers and mute-behavior setter before HAL acquisition.
Return alone does not establish useful audio or resource release.

Reads make a fixed number of attempts per slot. `busy_slots` records overlapping
publications; omitted-open/call and truncated-history counters describe missing
observations. `counters_inexact` reports exhausted or contended counter updates.
Slots and counters do not restrict capture admission. Records are independently
coherent, rather than a simultaneous view of every worker; a returning call can
appear in current and completed records during one read.

This mechanism observes existing macOS application/system capture control
calls. Other capture backends may produce no spans. It does not enable
`PKS_TAP_DIAG`, make extra property queries, interrupt a blocked call, choose a
timeout, retry cleanup or change the native ownership limit above. Native speech
and cleanup still require evidence from the affected device and route.

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
