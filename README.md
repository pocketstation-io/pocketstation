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

## Get started

You need Rust 1.95 or newer. Native capture is enabled by default.

```toml
[dependencies]
pocketstation = "1.1.1"
```

Clone this repository and build the product quickstart:

```bash
cargo build --release --example product_quickstart --locked
```

The example declares one application and the default microphone, observes both
stems, and finalizes a two-stem recording. Running it also requires the named
application, operating-system capture permission, and an available microphone:

```bash
cargo run --release --example product_quickstart --locked
```

It stops after observing both stems or returns a typed setup/runtime failure.
Completed recordings are written under `pocketstation-recordings/`. See the
[Rust quickstart](docs/getting-started/rust-quickstart.md) for native
prerequisites, expected results, and cleanup.

For contracts-only tooling or documentation builds, disable native capture:

```toml
pocketstation = { version = "1.1.1", default-features = false }
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

## Understand the Session contract

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

## Operate within explicit bounds

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
- [Read the API reference](https://docs.rs/pocketstation/latest/pocketstation/)

## Verify a change

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo build --release --example product_quickstart --locked
bash scripts/check_protocol.sh
```

These commands verify the package and compiling quickstart. Physical-device,
cross-network, and platform claims require their separately scoped Lab
artifacts.

## License

PocketStation is licensed under either MIT or Apache-2.0, at your option.
