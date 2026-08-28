# PocketStation release notes

This page covers user-visible changes in the PocketStation 1.x release line.

## 1.1.3 — Interrupt generated audio without stopping capture

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

## 1.1.2 — Use application-owned audio in a Session

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
