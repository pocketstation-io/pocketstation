# PocketStation release notes

This page covers user-visible changes in the PocketStation 1.x release line.

## 1.1.5 — 2026-09-01

Send Session audio to an external system with one function or one focused Rust
type.

### Create a Connector

Use `Connector::from_audio_fn` when a destination needs only one send
operation:

```rust
let destination = session.destination(Connector::from_audio_fn(|frame| {
    publish(frame.samples())?;
    Ok(())
})?)?;

application.send(destination)?;
```

Implement `AudioConnector` when the provider opens and closes a connection.
One Connector value can receive several source-aware stems through one
lifecycle. Separate values remain independent destinations with their own
queues, failures, and shutdown outcomes.

PocketStation keeps Connector work off realtime, bounds each route, preserves
frame lineage, and joins the provider during drain, abort, startup rollback, or
failure. The existing manifest and driver APIs remain available for
distributable integrations that need typed configuration, secrets, named
inputs, readiness, and provider observations.

### Operate capture safely

The guides now show how to inspect permission without prompting, store a
discovered source only for its reported persistence scope, rediscover after a
process or device disappears, reject ambiguous application matches, and make
fallback and provider retry policy explicit.

### Fixed

Repeated microphone permission checks now retain valid WinRT thread state for
the lifetime of the calling thread. This prevents a second non-prompting query
from terminating Python and other embedded hosts on Windows.

PocketStation still returns `NotObservable` when Windows cannot provide a safe
authorization result. Permission checks never display a system prompt.

```console
cargo update -p pocketstation --precise 1.1.5
```

## 1.1.4 — 2026-08-31

Select applications consistently across desktop platforms.

### Added

Application capture now accepts the identifier developers already have. Pass
an exact display name or application identifier for the common case, or use a
stable source identity, process ID, or process instance when the application
needs tighter control. PocketStation resolves the selector before capture and
fails clearly when a name is ambiguous.

```rust
let source = Source::application("Spotify");
let application = session.capture(source)?;
application.send(session.polled_audio()?)?;
```

For interactive voice workloads, a Session can opt into 10 ms capture frames:

```rust
let session = Session::builder()
    .audio_frame_duration(AudioFrameDuration::Ms10)
    .build();
```

### Changed

The 20 ms profile remains the default. Native backends normalize the packet
sizes delivered by Core Audio, PipeWire, ALSA, and WASAPI into the selected
frame duration, so application code receives the same frame size on every
supported desktop platform.

### Fixed

- Multistem recording starts each stem when that stem first produces media, so
  a silent generated-audio stem cannot delay application or microphone
  recording.
- PipeWire capture timestamps retain the source presentation timeline instead
  of using callback arrival time.
- Existing recording error-code discriminants and the C/PKSS ABI remain
  compatible with 1.1.3.

### Compatibility and upgrade

This is a compatible 1.x update. It requires no configuration or data
migration.

```console
cargo update -p pocketstation --precise 1.1.4
```

## 1.1.3 — 2026-08-28

Interrupt generated audio without stopping capture.

Voice applications often need to stop an outdated response as soon as a person
starts speaking again. Stopping the whole Session also stops the microphone,
recording, and every unrelated route.

PocketStation 1.1.3 adds replaceable output to application-owned audio inputs.
An application can begin an output generation, write generated PCM to it, and
cancel only that output. PocketStation rejects later writes for the cancelled
generation and discards its pending frames from bounded Core delivery paths.
Application capture, microphone capture, recording, and unrelated output keep
running.

The relevant Rust methods are:

- `AudioInput::begin_output_generation`
- `AudioInput::try_write_for_output`
- `OutputGeneration::cancel`

Cancellation cannot recall audio that an external service or playback device
has already accepted. A Connector must provide its own playout-clear operation
when that destination supports one.

### Select a macOS application by name

Application capture on macOS now accepts either the exact application name
shown to the user, such as `Brave Browser`, or its bundle identifier. When an
application uses multiple processes, PocketStation captures the processes that
share the same discovered application identity.

If the name matches different applications, selection fails before capture and
asks for the bundle identifier. PocketStation never guesses between ambiguous
matches.

### Reliability

- Multistem recording now preserves the requested `Drain` or `Abort` behavior
  through finalization.
- The default native-capture library builds on Windows through the supported
  observation boundary.

### Upgrade

This is a compatible 1.x update. It requires no configuration or data migration.

```console
cargo update -p pocketstation --precise 1.1.3
```

## 1.1.2 — 2026-08-23

Use application-owned audio in a Session.

PocketStation 1.1.2 made PCM produced by an application a normal source in the
same Session as desktop application and microphone capture. That audio can use
the same bounded routing, lineage, recording, and delivery paths as captured
audio.

This release also:

- made lifecycle, timing, discontinuity, delivery, and recording observations
  available to SDK bindings;
- allowed SDK-authored Operators to return generated PCM through the existing
  audio reentry path;
- preserved source identity when multiple stems connect to the same Operator;
  and
- added recording metadata without changing the existing
  `RecordingOutcome` layout.

This was a compatible 1.x update and required no configuration migration.
