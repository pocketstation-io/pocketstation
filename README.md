# PocketStation

**Capture once. Keep every source. Build the live media pipeline your
application needs.**

PocketStation gives Rust developers one `Session` for capturing a desktop
application and microphone as independent, source-aware live stems. The same
capture can flow concurrently to AI Operators, application callbacks, a remote
receiver, and aligned multistem recording without moving application or model
work onto the audio callback.

[![crates.io](https://img.shields.io/crates/v/pocketstation.svg)](https://crates.io/crates/pocketstation)
[![docs.rs](https://img.shields.io/docsrs/pocketstation)](https://docs.rs/pocketstation/latest/pocketstation/)
[![license](https://img.shields.io/crates/l/pocketstation.svg)](https://github.com/pocketstation-io/pocketstation)

```text
desktop application ─┐
                     ├─ capture once ─ source-aware Session ─┬─ AI / model connector
microphone ──────────┘                                       ├─ browser / remote receiver
                                                             └─ aligned multistem recording
```

Build workflows such as:

- live translation or captioning that keeps application speech and microphone
  speech distinguishable;
- meeting agents and copilots that send the same live stems to inference,
  monitoring, transport, and recording;
- support, accessibility, and QA tools that isolate a slow or failed
  destination instead of stalling every branch;
- speech-to-speech systems that return generated PCM through an explicit
  bounded Bridge;
- native and managed integrations that participate in the same Session
  lifecycle rather than building another media runtime.

## Contract

PocketStation does not reduce live media to an anonymous `AudioFrame`. Its
execution contract keeps the information needed to reason about a running
system:

| Contract | What remains explicit |
|---|---|
| Provenance | source, stream, and stem identity |
| Time | sequence, timestamp, clock, and derivation |
| Change | source generation, discontinuity, and permission epochs |
| Delivery | route capacity, backpressure, copy, and loss policy |
| Operations | queue depth, saturation, drops, failures, cancellation, and final outcome |

Those semantics remain coherent as work crosses realtime audio, typed signals,
Rust, the versioned C ABI, and bounded process sidecars. Each integration uses
the same compiler, lifecycle, observations, and failure model.

## Install

```toml
[dependencies]
pocketstation = "1.0.0"
```

PocketStation requires Rust 1.95 or newer. Native capture is enabled by
default. A contracts-only build for documentation or tooling can disable it:

```toml
pocketstation = { version = "1.0.0", default-features = false }
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
                    frame.lineage().sequence_number()
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

## One Session, two execution lanes

```text
OS capture · typed source · generated audio
                     │
                     ▼
        source identity + immutable lineage
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
       Operator · Endpoint · recording
                     │
                     ▼
              Rust · C · sidecar
```

The audio lane is specialized for fixed-capacity, predictable work. The typed
lane carries text, events, control data, metrics, binary data, and
schema-identified custom signals. The explicit Bridge between them allows
async work to produce audio without making a managed runtime or model provider
part of the capture callback.

`SignalSpec`, lineage, timing, named ports, and edge policy are the stable
cross-language contract. Operators can be composed without each connector
inventing its own source identity, buffering, cancellation, or failure
semantics.

Rust `Stream<T>` provides compile-time composition. `T` is developer-facing
metadata; it is not the runtime storage format and never crosses the C ABI or
sidecar protocol.

## Realtime engineering guarantees

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
| Packaged compiled extension | `pks_extension_library_v1` loaded into the same `Session` |
| Python, JavaScript, or another managed process | bounded PKSS sidecar lifecycle for out-of-process work |

Operators can be chained, can expose multiple named inputs and outputs, and can
return generated PCM through the bounded audio-reentry Bridge. Extensions use
the same Session compiler, runtime, observations, cancellation, and shutdown;
they do not implement another engine.

Language SDKs bind these same Session authorities. A trusted compiled package
is loaded only from a canonical absolute path, all registrations are imported
transactionally, and Core retains its executable library until every callback
context is destroyed. SDKs do not own a second loader policy or Session
runtime.

See the [extension guide](docs/guides/extensions.md) and
[signal model](docs/concepts/signals-and-streams.md).

## Platform and evidence boundaries

| Platform | Native implementation | Current evidence boundary |
|---|---|---|
| macOS | application capture and microphone | physical final-candidate proof |
| Windows | WASAPI system, application, and microphone capture | automated VM qualification; physical matrix remains separate |
| Linux | PipeWire application/system capture and ALSA microphone capture | automated VM qualification; physical matrix remains separate |

Applications own operating-system permission prompts and source selection UX.
PocketStation reports typed permission, source-loss, discontinuity, saturation,
and lifecycle outcomes.

These classifications are deliberately narrow. Physical macOS proof does not
silently become a physical Windows or Linux claim, and local cross-language or
transport evidence does not become a universal performance claim.

### Permission semantics

`microphone_permission_observation()` is a non-prompting preflight query. macOS
uses its process authorization state. Windows 10 version 1903 and newer uses
the current process' `Microphone` AppCapability status and can report allowed,
denied, restricted, or prompt-required without displaying UI. Linux returns
`PermissionObservation::NotObservable` because PipeWire portals, session
policy, direct ALSA access, device ACLs, and containers do not share one
authoritative process-wide query. `NotObservable` means neither granted nor
denied. On every platform, Session preparation and source opening still return
the authoritative typed success or failure for the selected source.

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
