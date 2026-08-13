# PocketStation

PocketStation is a realtime media and signal engine for Rust. Capture desktop
audio once, keep every source independently identifiable, and route it through
bounded Operators and Endpoints without putting application code on the audio
callback.

[![crates.io](https://img.shields.io/crates/v/pocketstation.svg)](https://crates.io/crates/pocketstation)
[![docs.rs](https://img.shields.io/docsrs/pocketstation)](https://docs.rs/pocketstation/latest/pocketstation/)
[![license](https://img.shields.io/crates/l/pocketstation.svg)](https://github.com/pocketstation-io/pocketstation)

PocketStation is built for workflows such as:

- independent application and microphone stems for agents and meeting tools;
- one capture feeding transcription, recording, transport, and monitoring at
  the same time;
- typed telemetry or control sources feeding custom analysis and storage;
- speech-to-speech pipelines that return generated PCM to the audio runtime;
- native or managed extensions that share one Session lifecycle.

## Install

```toml
[dependencies]
pocketstation = "1.0.1"
```

PocketStation requires Rust 1.95 or newer. Native capture is enabled by
default. A contracts-only build for documentation or tooling can disable it:

```toml
pocketstation = { version = "1.0.1", default-features = false }
```

## Quick start

This Session captures one named desktop application and the default microphone,
keeps them as separate stems, exposes bounded audio polling, and records both
stems.

```rust,no_run
use pocketstation::{ApplicationSelector, Session, SessionRecordingState, Source};
use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let session = Session::builder()
        .recording_root("recordings")
        .build();

    let application = session.capture(Source::application(
        ApplicationSelector::name("PocketStation Demo"),
    ))?;
    let microphone = session.capture(Source::microphone_default())?;

    application.send(session.polled_audio()?)?;
    microphone.send(session.polled_audio()?)?;
    application.record("application")?;
    microphone.record("microphone")?;

    let mut running = session.start()?;
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch.frame(index).expect("valid bounded batch index");
                println!(
                    "stem={} source={} sequence={}",
                    frame.lineage().stem_id().get(),
                    frame.lineage().source_id().get(),
                    frame.sequence_number()
                );
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    let stop = running.stop();
    assert!(stop.is_success());
    let recording = running.recording_outcome().expect("recording outcome");
    assert_eq!(recording.state, SessionRecordingState::Complete);
    Ok(())
}
```

The complete compiling example is
[`examples/product_quickstart.rs`](examples/product_quickstart.rs).

## One engine, two execution lanes

```text
application · microphone · external source
                    │
                    ▼
                  Session
                    │
            compiled RuntimePlan
          ┌─────────┴──────────┐
          ▼                    ▼
 realtime audio lane     typed signal lane
 pooled AudioFrame       SignalEnvelope
          └─────────┬──────────┘
                    ▼
         independent bounded routes
       source · Operator · Endpoint
```

The audio lane is specialized for fixed-capacity, predictable work. The typed
lane carries text, events, control data, metrics, binary data, and
schema-identified custom signals. `SignalSpec`, lineage, timing, named ports,
and edge policy remain stable across Rust, C, and process boundaries.

Rust `Stream<T>` provides compile-time composition. `T` is developer-facing
metadata; it is not the runtime storage format and never crosses the C ABI or
sidecar protocol.

## Realtime guarantees

Audio callbacks and realtime partitions are designed and gated to remain:

```text
allocation-free · lock-free · blocking-free · async-free · log-free · panic-free
```

Audio buffers come from fixed-capacity pools. Realtime crossings use bounded
SPSC queues. Every route declares capacity, backpressure, loss, clock, copy,
and observation policy. Saturation is counted or returned as a typed failure;
it is never hidden behind an unbounded queue.

## Extend PocketStation

PocketStation keeps product and provider behavior outside the engine:

| Build | Public contract |
|---|---|
| Audio or typed source | `SourceFactory` / `SourceDriver` |
| Processing stage | `AsyncOperatorFactory` with named ports |
| Destination or connector | `EndpointDriverFactory` |
| Strongly typed Rust pipeline | `Stream<T>` / `TypedOperator<I, O>` |
| Native C integration | versioned callbacks in `pocketstation.h` |
| Python, JavaScript, or another process | bounded PKSS sidecar lifecycle |

Operators can be chained, can expose multiple named inputs and outputs, and can
return generated PCM through the bounded audio-reentry Bridge. Extensions use
the same Session compiler, runtime, observations, cancellation, and shutdown;
they do not implement another engine.

See the [extension guide](docs/guides/extensions.md) and
[signal model](docs/concepts/signals-and-streams.md).

## Platform support

| Platform | Native implementation | Current evidence boundary |
|---|---|---|
| macOS | application capture and microphone | physical final-candidate proof |
| Windows | WASAPI system, application, and microphone capture | automated VM qualification; physical matrix remains separate |
| Linux | PipeWire application/system capture and ALSA microphone capture | automated VM qualification; physical matrix remains separate |

Applications own operating-system permission prompts and source selection UX.
PocketStation reports typed permission, source-loss, discontinuity, saturation,
and lifecycle outcomes.

## Native dependencies

macOS requires the Xcode command-line tools. Linux development packages depend
on the distribution; on Debian/Ubuntu install:

```bash
sudo apt install build-essential cmake pkg-config \
  libasound2-dev libpipewire-0.3-dev
```

Windows builds use the MSVC Rust toolchain and Windows SDK.

## Documentation

- [Developer documentation](docs/README.md)
- [Architecture](docs/architecture/overview.md)
- [Rust quickstart](docs/getting-started/rust-quickstart.md)
- [Extension authoring](docs/guides/extensions.md)
- [Compatibility policy](docs/development/compatibility-and-freeze.md)
- [API reference](https://docs.rs/pocketstation/latest/pocketstation/)

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo build --release --example product_quickstart --locked
bash scripts/check_protocol.sh
```

Local component tests establish code correctness, not physical-device,
cross-network, or universal performance claims. Those claims remain tied to
versioned evidence artifacts and their stated scope.

## License

PocketStation is licensed under either MIT or Apache-2.0, at your option.
