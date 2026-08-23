# PocketStation

<!-- claims: CLM-DOC-000-CAP-001,CLM-DOC-000-CAP-002,CLM-DOC-000-CAP-003,CLM-DOC-000-CAP-004,CLM-DOC-000-CAP-005,CLM-DOC-000-CAP-006,CLM-DOC-000-CAP-007,CLM-DOC-000-CAP-008,CLM-DOC-000-CAP-009,CLM-DOC-000-CAP-010,CLM-DOC-000-CAP-011,CLM-DOC-000-CAP-012,CLM-DOC-000-CAP-013,CLM-DOC-000-CAP-014,CLM-DOC-000-CAP-015,CLM-DOC-000-CAP-016,CLM-DOC-000-CAP-017,CLM-DOC-000-CAP-018,CLM-DOC-000-CAP-019,CLM-DOC-000-CAP-020,CLM-DOC-000-CAP-021,CLM-DOC-000-CAP-022,CLM-DOC-000-CAP-023,CLM-DOC-000-CAP-024,CLM-DOC-000-CAP-025,CLM-DOC-000-CAP-026,CLM-DOC-000-CAP-027,CLM-DOC-000-CAP-028,CLM-DOC-000-CAP-029,CLM-DOC-000-CAP-030,CLM-DOC-000-CAP-031,CLM-DOC-000-CAP-032,CLM-DOC-000-CAP-033,CLM-DOC-000-SOURCE-001 -->

PocketStation is a Rust library for declaring and running source-aware desktop audio Sessions. A Session can keep application and microphone sources separate, route them through bounded paths, expose polled audio, and finalize multistem recording. Extensions, connectors, the C ABI, and sidecars participate through the same declaration and lifecycle model.

## Install

PocketStation 1.1.2 requires Rust 1.95 or newer. Native capture is the default Cargo feature.

```toml
[dependencies]
pocketstation = "1.1.2"
```

Use the contracts-only form when you need public declarations without a native capture backend:

```toml
pocketstation = { version = "1.1.2", default-features = false }
```

## Run the first Session

The repository keeps its quickstart as a compiled Cargo example. It declares application and microphone capture, gives each source an independent polled-audio route and recording stem, observes two stems, and inspects both Session and recording outcomes.

```rust,no_run
use std::collections::BTreeMap;
use std::error::Error;
use std::time::{Duration, Instant};

use pocketstation as pks;

fn main() -> Result<(), Box<dyn Error>> {
    let session = pks::Session::builder()
        .recording_root("pocketstation-recordings")
        .build();
    let app = session.capture(pks::Source::application(pks::ApplicationSelector::name(
        "PocketStation Demo",
    )))?;
    let mic = session.capture(pks::Source::microphone_default())?;
    let app_audio = session.polled_audio()?;
    let mic_audio = session.polled_audio()?;

    app.send(app_audio)?;
    mic.send(mic_audio)?;
    app.record("application")?;
    mic.record("microphone")?;

    let mut running = session.start()?;
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut frames_by_stem = BTreeMap::<u64, usize>::new();
    while Instant::now() < deadline
        && frames_by_stem.values().filter(|count| **count >= 2).count() < 2
    {
        if let Ok(batch) = running.try_poll_audio() {
            for index in 0..batch.len() {
                let frame = batch
                    .frame(index)
                    .ok_or("bounded audio batch returned an invalid frame index")?;
                let count = frames_by_stem
                    .entry(frame.lineage().stem_id().get())
                    .or_default();
                *count = count.saturating_add(1);
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    if frames_by_stem.values().filter(|count| **count >= 2).count() != 2 {
        return Err("application and microphone media were not both observed".into());
    }

    let outcome = running.stop();
    if !outcome.is_success() {
        return Err("PocketStation Session did not finalize cleanly".into());
    }
    let recording = running
        .recording_outcome()
        .ok_or("PocketStation Session did not expose a recording outcome")?;
    if recording.state != pks::SessionRecordingState::Complete
        || recording.completed_stems != 2
        || recording.failed_stems != 0
    {
        return Err("PocketStation multistem recording did not complete".into());
    }
    Ok(())
}
```

Compile before running, then run only on a host where the named application, microphone, permissions, and native dependencies are available:

```bash
cargo test --examples --all-features
cargo run --example product_quickstart
```

## Verify the outcome

Success means the example observes at least two frames from each of two distinct stems, receives a successful Session stop outcome, and receives a recording outcome with two completed stems and no failed stems. Source, permission, route, stop, or recording failures are returned instead of being counted as success.

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.
- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.
- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.
- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.
- **Map clocks and correct drift.** Map source timestamps into a Session timeline and estimate or correct clock drift.
- **Compile Session declarations.** Validate declarations, resolve bindings, and lower a Session specification into an executable plan.
- **Prepare runtime resources.** Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.
- **Start, cancel, stop, and finalize a Session.** Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.
- **Route realtime audio.** Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.
- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.
- **Describe graph contracts.** Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.
- **Implement asynchronous operators.** Register operator factories that consume and emit named typed signals on the asynchronous execution lane.
- **Carry typed signals.** Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.
- **Bridge asynchronous output into audio.** Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.
- **Implement endpoint drivers.** Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.
- **Declare connector manifests and configuration.** Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.
- **Run connector workers.** Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.
- **Load native extension libraries.** Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.
- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Host managed-process sidecars.** Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.
- **Observe Session metrics and events.** Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.
- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.
- **Inject external PCM.** Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.
- **Encode and decode Opus.** Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.
- **Build and publish repository artifacts.** Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.
- **Integrate transcription processing.** Use the repository-owned Whisper example to send captured stems to an external transcription process with evidence output.

The scope of **PocketStation** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Documentation map

- [Start with the Rust quickstart](/docs/getting-started/rust-quickstart.md).
- [Learn the Session model](/docs/concepts/session.md).
- [Choose capture and permission behavior](/docs/concepts/source-selection.md).
- [Configure routes and backpressure](/docs/concepts/realtime-routing.md).
- [Author connectors](/docs/guides/connectors.md) or [native extensions](/docs/guides/extensions.md).
- [Use the native Rust API reference](/docs/reference/rust-api.md).
- [Diagnose observable symptoms](/docs/troubleshooting/session-start.md).

## Evidence boundary

The claims on **PocketStation** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)
- `src/lib.rs:1-1161` (`DIRECT`)
- `examples/product_quickstart.rs:1-61` (`DIRECT`)

For **PocketStation**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
