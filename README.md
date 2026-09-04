# PocketStation

Capture one desktop application in Rust, keep its audio identifiable, and use
the same capture for processing, delivery, and recording. Add a microphone only
when the application needs a second independent source.

[![crates.io](https://img.shields.io/crates/v/pocketstation.svg)](https://crates.io/crates/pocketstation)
[![docs.rs](https://img.shields.io/docsrs/pocketstation)](https://docs.rs/pocketstation/latest/pocketstation/)
[![license](https://img.shields.io/crates/l/pocketstation.svg)](https://github.com/pocketstation-io/pocketstation)

```text
desktop application ─┐
microphone ──────────┼─ Session ─┬─ your Rust code
application PCM ─────┘           ├─ Operator or Connector
                                 └─ one recording file per stem
```

PocketStation is for applications that need direct control of desktop audio:
voice systems, meeting tools, broadcast workflows, transcription, monitoring,
and local media automation. It is not an AI model, conferencing service, or
general codec framework.

## Capture a running application

You need Rust 1.95 or newer and one supported desktop application producing
audio.

```bash
cargo add pocketstation@1.1.8
```

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
session
    .capture(Source::application("Spotify"))?
    .send(session.polled_audio()?)?;
let mut running = session.start()?;
# let _ = running.stop();
# Ok(())
# }
```

Replace `Spotify` with the exact display name or application identifier of a
running application. PocketStation rejects an ambiguous match instead of
silently choosing a process. The declaration is the same on macOS, Windows,
and Linux; operating-system permissions and native build dependencies differ.

This example opens no microphone and writes no file. The
[quickstart](docs/getting-started/rust-quickstart.md) includes application
discovery, frame polling, a 10-second media deadline, optional microphone and
recording arguments, and joined shutdown.

Run it from this repository:

```bash
cargo run --release --example quickstart --locked
```

The command lists running applications and asks which one to capture. It stops
after receiving audio from every requested source. Add `--microphone` or
`--record recordings` only when you want those features.

## Keep sources separate

Application, microphone, and application-owned PCM enter a `Session` as
different stems. Every frame identifies its source, stream, stem, sequence,
timestamp, clock, and continuity state. Sending two stems to the same provider
does not mix them or erase where they came from.

```rust,no_run
use pocketstation::{Session, Source};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::builder().recording_root("recordings").build();
let application = session.capture(Source::application("Zoom"))?;
let microphone = session.capture(Source::microphone_default())?;

application.record("application")?;
microphone.record("microphone")?;
# Ok(())
# }
```

Recording starts with the Session and finishes during Session shutdown. The
final recording outcome reports which stems completed or failed. PocketStation
does not present a partial recording as a successful one.

## Use one capture for several jobs

A stem can feed several destinations at once:

- application code that reads frames;
- an `Operator` that turns audio into another signal, such as a transcript;
- a `Connector` that sends audio to an external service;
- another `Operator` or `Endpoint` with its own delivery settings;
- multistem recording.

Each destination has its own queue and failure outcome. Slow model or network
work runs outside the capture callback, so it does not become part of the
operating-system audio callback.

Use [capture and route audio](docs/guides/capture-and-route.md) to add a second
source or destination. Use [record and observe](docs/guides/record-and-observe.md)
to inspect delivery, discontinuities, and final outcomes.

## Send audio to your own service

A `Connector` sends Session audio to an API, socket, file, or provider. For a
destination that needs one function, PocketStation owns the worker and calls
your function away from realtime capture:

```rust,no_run
use pocketstation::connector::Connector;
use pocketstation::{Session, Source};

# fn publish(_: &[f32]) -> Result<(), pocketstation::connector::ConnectorError> { Ok(()) }
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
let application = session.capture(Source::application("Spotify"))?;
let destination = session.destination(Connector::from_audio_fn(|frame| {
    publish(frame.samples())
})?)?;

application.send(destination)?;
# Ok(())
# }
```

Implement `AudioConnector` when one provider connection needs `start`, `send`,
and `stop`. Use the driver API only for distributable packages that need typed
configuration, named inputs, service status, or provider-specific observations.
The [Connector guide](docs/guides/connectors.md) explains both forms.

Use an `Operator`, not a Connector, when audio becomes a transcript, event, or
new audio stream. Use a `Source` when an external system sends media into the
Session. The [extension guide](docs/guides/extensions.md) explains custom
Sources, Operators, Endpoints, native libraries, and managed processes.

## Write PCM your application already owns

Generated speech, call audio, and decoded network media can enter through
`audio_input()` and use the same routing and recording APIs as captured audio.
The input reports `Full`, `Closed`, `Cancelled`, and invalid-buffer outcomes;
it does not hide backpressure in an unbounded application queue.

See [write application-owned audio](docs/guides/application-audio.md) for the
preallocated writer API and selective removal of output that should no longer
be delivered.

## Understand delivery and failure

PocketStation prepares every Source, Operator, Connector, Endpoint, and
recording destination before it opens the Session start gate. A setup failure
rolls back prepared resources. Stopping a running Session joins its workers and
returns one result that includes component and recording failures.

Realtime callbacks and realtime processing are checked to remain:

```text
allocation-free · lock-free · blocking-free · async-free · log-free · panic-free
```

Queues and pools have configured capacities. When a queue fills, its delivery
policy decides whether a frame is rejected or dropped, and observations report
the event. Read [delivery and failure](docs/concepts/delivery-and-failure.md)
before choosing settings for voice, recording, or model work.

## Handle permissions and source changes

Your application owns consent UI and source selection. PocketStation reports
permission state, source loss, source generation changes, discontinuities,
queue pressure, and recording results.

Check permission without prompting when the platform supports it. Store a
discovered source only for its reported persistence scope. If the application
or microphone disappears, stop the Session, discover again, confirm the new
selection, and start another Session. PocketStation does not switch to a
different source without the application deciding to do so.

The [platform operations guide](docs/operations/platform-support.md) lists
permission states, source persistence, recovery steps, and native prerequisites.

## Platform support

| Platform | Available sources | Published evidence |
|---|---|---|
| macOS | application and microphone | physical application, microphone, 10 ms capture, Relay, Chromium, and multistem recording on the recorded host |
| Windows | system, application, and microphone | automated Windows 11 ARM64 VM selection and 10 ms capture; physical-device and latency qualification remain separate |
| Linux | PipeWire application/system and ALSA microphone | automated Ubuntu selection and 10 ms capture; physical-device qualification remains separate |

These records establish only the named environment and test. They do not
establish identical device behavior, WAN/TURN performance, or one latency
number for every computer.

Native prerequisites:

- macOS: Xcode command-line tools;
- Windows: MSVC Rust toolchain and Windows SDK;
- Debian or Ubuntu:

  ```bash
  sudo apt install build-essential cmake pkg-config \
    libasound2-dev libpipewire-0.3-dev
  ```

Disable native capture when a tool only needs PocketStation types or docs:

```toml
pocketstation = { version = "1.1.8", default-features = false }
```

## Continue from the task you have

| Task | Guide |
|---|---|
| Run the complete first capture | [Rust quickstart](docs/getting-started/rust-quickstart.md) |
| Capture and route application or microphone audio | [Capture and route](docs/guides/capture-and-route.md) |
| Write generated or received PCM | [Application audio](docs/guides/application-audio.md) |
| Record stems and inspect delivery | [Record and observe](docs/guides/record-and-observe.md) |
| Send audio to an external service | [Connectors](docs/guides/connectors.md) |
| Add a Source, Operator, Endpoint, library, or process | [Extensions](docs/guides/extensions.md) |
| Understand Session start and shutdown | [Session lifecycle](docs/concepts/session-lifecycle.md) |
| Understand signals, identity, and time | [Signals and streams](docs/concepts/signals-and-streams.md) |
| Read events, metrics, outcomes, and errors | [Events and errors](docs/reference/events-and-errors.md) |
| Find a public Rust type | [API map](docs/reference/public-api.md) or [docs.rs](https://docs.rs/pocketstation/latest/pocketstation/) |
| Prepare or troubleshoot a host | [Platform support](docs/operations/platform-support.md) and [troubleshooting](docs/troubleshooting.md) |
| Check an upgrade | [Compatibility](docs/compatibility/README.md) and [release notes](RELEASE_NOTES.md) |

## Verify a local change

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo build --release --example quickstart --locked
bash scripts/check_protocol.sh
```

These commands verify source and component behavior. Physical devices,
cross-network delivery, and platform performance require separately recorded
tests in those environments.

## License

PocketStation is available under the MIT or Apache-2.0 license.
