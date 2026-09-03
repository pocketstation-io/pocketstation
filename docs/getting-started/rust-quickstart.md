# Capture a desktop application in Rust

This quickstart runs one `Session` and observes the desktop application you
select. Microphone capture and recording are explicit opt-ins.

## Prerequisites

- Rust 1.95 or newer;
- a desktop application you want to capture running on the host;
- operating-system permission to capture that application.

Native build requirements:

- macOS: Xcode command-line tools;
- Windows: MSVC Rust toolchain and Windows SDK;
- Debian or Ubuntu:

  ```bash
  sudo apt install build-essential cmake pkg-config \
    libasound2-dev libpipewire-0.3-dev
  ```

## Add PocketStation

```bash
cargo add pocketstation@1.1.6
```

## Capture in three lines

Capture any running application with the same API on macOS, Windows, and Linux:

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
running application. These lines select one application, route its audio to a
polling Endpoint, and start the Session. A complete program must poll
the frames and stop the Session; the repository quickstart below includes both.

## Run the complete quickstart

From this repository:

```bash
cargo run --release --example quickstart --locked
```

Choose a running application from the list. PocketStation resolves the selected
process and stable source identity before the Session starts. The Session stops
after it observes at least two frames from that application. It does not open a
microphone or write a recording by default.

The complete program is
[`examples/quickstart.rs`](../../examples/quickstart.rs).

## Options

Use an exact display name, application identifier, or process ID when you want
to skip the prompt:

```bash
cargo run --release --example quickstart --locked -- --application "Spotify"
```

To include the default microphone as a separate stem:

```bash
cargo run --release --example quickstart --locked -- --microphone
```

To record every selected stem, provide an output directory:

```bash
cargo run --release --example quickstart --locked -- --record pocketstation-recordings
```

Options can be combined. For example, add `--microphone --record recordings` to
capture and record the selected application and default microphone as separate
stems.

## Inspect the result

When `--record` is set, completed artifacts are written under the directory you
provide. Application and microphone audio remain separately identified by
their source and stem lineage.

If setup fails, inspect the typed permission or source error. A preflight
`PermissionObservation::NotObservable` is not permission approval; source
opening determines whether the selected backend can run. If a requested source
does not produce media, the example exits without claiming success.

Remove the directory passed to `--record` when you no longer need the artifacts.

## Continue developing

- [Capture and route more than one source](../guides/capture-and-route.md)
- [Record stems and inspect delivery](../guides/record-and-observe.md)
- [Understand the Session architecture](../architecture/overview.md)
- [Send Session audio to an external system](../guides/connectors.md)
- [Add an integration](../guides/extensions.md)

Building this example verifies its use of the public Rust API. Running it on
one computer does not establish behavior for other devices, operating systems,
or networks.
