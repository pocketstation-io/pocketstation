# PocketStation

Build source-aware desktop audio workflows in Rust.

PocketStation provides one `Session` for a desktop application, a microphone,
and application-owned PCM. Each source remains independently identifiable while
the Session routes it to Operators, outbound Endpoints, application code, and
multistem recording.

[![crates.io](https://img.shields.io/crates/v/pocketstation.svg)](https://crates.io/crates/pocketstation)
[![docs.rs](https://img.shields.io/docsrs/pocketstation)](https://docs.rs/pocketstation/latest/pocketstation/)
[![license](https://img.shields.io/crates/l/pocketstation.svg)](https://github.com/pocketstation-io/pocketstation)

```text
desktop application ─┐
microphone ──────────┼─ source-aware Session ─┬─ Operator
application PCM ─────┘                        ├─ Endpoint or Connector
                                              ├─ application polling
                                              └─ multistem recording
```

## Capture any running application

You need Rust 1.95 or newer. Native capture is enabled by default.

```bash
cargo add pocketstation@1.1.3
```

Capture any running application with the same API on macOS, Windows, and Linux:

```rust,no_run
use pocketstation::{ApplicationSelector, Session, Source};

let session = Session::new();
session.capture(Source::application(ApplicationSelector::name("Spotify"))).expect("application capture failed").send(session.polled_audio().expect("audio polling is unavailable")).expect("audio route failed");
let mut running = session.start().expect("Session failed to start");
```

Replace `Spotify` with the exact name or identifier of any running application.
The same declaration is used on macOS, Windows, and Linux; permissions and
native build prerequisites remain platform-specific. No microphone opens and
no file is written unless you request those routes.

The [Rust quickstart](docs/getting-started/rust-quickstart.md) provides the
complete program: interactive application selection, bounded frame polling,
optional microphone and recording routes, a finite deadline, and joined
shutdown.

For contracts-only tooling or documentation builds, disable native capture:

```toml
pocketstation = { version = "1.1.3", default-features = false }
```

## Choose a task

| Task | Start with |
|---|---|
| Capture a desktop application or microphone | [`Session::capture`](https://docs.rs/pocketstation/latest/pocketstation/struct.Session.html#method.capture) |
| Ingest PCM your application already owns | `Session::audio_input` and `PcmSource` |
| Process media or typed signals | `Operator` and named ports |
| Publish to an external system | [`Connector` guide](docs/guides/connectors.md) |
| Add a source, computation, or destination | [`Extension` guide](docs/guides/extensions.md) |
| Inspect lineage and delivery behavior | [`Signals and streams`](docs/concepts/signals-and-streams.md) |

## How a Session handles audio

PocketStation keeps the following information explicit as work moves between
realtime audio and off-realtime integrations:

| Contract | Available information |
|---|---|
| Provenance | source, stream, and stem identity |
| Time | sequence, timestamp, clock, and derivation |
| Change | source generation, discontinuity, and permission epochs |
| Delivery | route capacity, backpressure, copy, and loss policy |
| Operations | queue depth, saturation, drops, failures, cancellation, and final outcome |

```text
authorized Source
      │
      ▼
source identity + immutable lineage
      │
      ▼
    Session ── compiled RuntimePlan
      │
      ├─ realtime audio lane: pooled AudioFrame
      └─ typed signal lane: SignalEnvelope
                │
                ▼
      independent bounded routes
      Operator · Endpoint · recording
```

The audio lane uses fixed-capacity pools and bounded realtime crossings. The
typed lane carries text, events, control data, metrics, binary data, and custom
signals identified by schema. Generated audio returns to the normal audio plan
through the bounded reentry Bridge.

`Stream<T>` provides Rust declaration-time checking. Runtime and cross-language
identity comes from `SignalSpec`, lineage, named ports, and edge policy; Rust
generic types do not cross the C ABI or sidecar protocol.

## Extend the engine

Provider and customer protocols remain outside the `pocketstation` package.
Choose the public boundary that owns the work:

| Capability | Public contract |
|---|---|
| Audio or typed source | `SourceFactory` / `SourceDriver` |
| Processing stage | `AsyncOperatorFactory` with named ports |
| Destination | `EndpointDriverFactory` |
| Provider destination | `ConnectorDriverFactory` / `ConnectorDriver` |
| Native C integration | versioned callbacks in `pocketstation.h` |
| Trusted compiled extension | `pks_extension_library_v1` |
| Managed process | bounded PKSS sidecar lifecycle |

These boundaries use the same Session compiler, lifecycle, observations,
cancellation, and shutdown. Python, JavaScript, and provider packages must not
create a second Session or media runtime.

## Resource and realtime behavior

Audio callbacks and realtime partitions are designed and checked to remain:

```text
allocation-free · lock-free · blocking-free · async-free · log-free · panic-free
```

Every route declares finite capacity and delivery policy. Saturation is counted
or returned as a typed outcome; the engine does not replace it with an
unbounded queue.

Applications own operating-system consent UI and source selection. PocketStation
reports permission, source-loss, discontinuity, saturation, lifecycle, and
recording outcomes through typed contracts.

| Platform | Native source support | Current qualification boundary |
|---|---|---|
| macOS | application and microphone | physical-device evidence exists for the recorded host |
| Windows | system, application, and microphone | automated VM evidence; physical coverage is separate |
| Linux | PipeWire application/system and ALSA microphone | automated VM evidence; physical coverage is separate |

These boundaries do not imply platform parity, WAN behavior, or a universal
performance result. Use the evidence attached to the specific release and
environment for those claims.

## Native prerequisites

macOS requires the Xcode command-line tools. Windows requires the MSVC Rust
toolchain and Windows SDK. On Debian or Ubuntu, install:

```bash
sudo apt install build-essential cmake pkg-config \
  libasound2-dev libpipewire-0.3-dev
```

## Documentation

- [Start with the Rust quickstart](docs/getting-started/rust-quickstart.md)
- [Develop against the current architecture](docs/architecture/overview.md)
- [Understand signals and typed streams](docs/concepts/signals-and-streams.md)
- [Build a Connector](docs/guides/connectors.md)
- [Extend PocketStation](docs/guides/extensions.md)
- [Check compatibility](docs/compatibility/README.md)
- [Read the release notes](RELEASE_NOTES.md)
- [Read the API reference](https://docs.rs/pocketstation/latest/pocketstation/)

## Verify a local change

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo build --release --example quickstart --locked
bash scripts/check_protocol.sh
```

These commands verify the package and compiling quickstart. Physical-device,
cross-network, and platform claims require their separately scoped Lab
artifacts.

## License

PocketStation is licensed under either MIT or Apache-2.0, at your option.
