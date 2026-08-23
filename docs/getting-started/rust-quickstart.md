# Rust quickstart

<!-- claims: CLM-DOC-003-CAP-001,CLM-DOC-003-CAP-002,CLM-DOC-003-CAP-003,CLM-DOC-003-CAP-004,CLM-DOC-003-CAP-005,CLM-DOC-003-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Poll bounded audio batches.** Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.
- **Record aligned multistem output.** Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Choose a host with an application named `PocketStation Demo`, an available default microphone, permission to open both sources, and a writable recording path.

## Program

This source is synchronized with `examples/product_quickstart.rs`:

```rust
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

## Run and verify

Run `cargo run --example product_quickstart`. The example rejects the run unless it observes two frames on each of two stems, obtains a successful stop outcome, and sees two completed recording stems with no failed stems.

## Public entry points

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `authorization` | module | Explicit capture authorization evidence and open outcomes. | `src/capture/authorization.rs:1` |
| `identity` | module | Stable source identity and source discovery state. | `src/capture/identity.rs:1` |
| `query` | module | Control-plane source discovery queries used by the first-party CLI. | `src/capture/query.rs:1` |
| `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| `pocketstation::capture::capture_owner::ActiveCaptureBackend` | trait | Native capture resources owned for exactly one active capture. | `src/capture/capture_owner.rs:100` |
| `pocketstation::capture::capture_owner::CallbackCaptureBackend` | trait | Platform-neutral prepare/open boundary for callback-oriented capture. | `src/capture/capture_owner.rs:83` |
| `pocketstation::capture::capture_owner::PreparedCaptureBackend` | trait | Backend state that has passed validation but has not started delivery. | `src/capture/capture_owner.rs:88` |
| `pocketstation::session::declaration::typed_stream::StreamSignal` | trait | Compile-time marker supplied by an SDK or external package. | `src/session/declaration/typed_stream.rs:15` |
| `pocketstation::capture::authorization::CaptureAuthorizationSnapshot` | struct | Point-in-time authorization evidence for opening one exact capture source. | `src/capture/authorization.rs:17` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `pocketstation::capture::authorization::CapturePermissionTransition` | struct | One authoritative authorization-state transition observed by the host. | `src/capture/authorization.rs:168` |
| `pocketstation::capture::capture_owner::CaptureDelivery` | struct | Callback delivery endpoints transferred to a prepared native backend. | `src/capture/capture_owner.rs:73` |
| `pocketstation::capture::capture_owner::CaptureOwnerObservations` | struct | Aggregate observations from one active capture ownership boundary. | `src/capture/capture_owner.rs:160` |
| `pocketstation::capture::frame_stream::CapturedFrameSender` | struct | Single-producer endpoint passed into a platform capture callback. | `src/capture/frame_stream.rs:102` |
| `pocketstation::session::declaration::spec::ConnectionSpec` | struct | The single Session connection declaration used for every stream origin and every operator/endpoint destination. | `src/session/declaration/spec.rs:238` |
| `pocketstation::session::declaration::typed_stream::Stream` | struct | Typed Rust declaration façade compiled into stable dynamic signal, schema, port, and edge contracts. This wrapper carries no frames and is not a generic runtime queue. | `src/session/declaration/typed_stream.rs:96` |

## Executable evidence

The following test bodies are evidence only for their recorded setup:

- `frame_stream_closed` — frame stream closed (`src/capture/capture_owner.rs:248`; `test-3ab763bff0cd08d4b4e1`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-a8dbef4f3b61c752ce0e`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `given_zero_frame_capacity_when_preparing_then_backend_is_not_prepared` — given zero frame capacity when preparing then backend is not prepared (`src/capture/capture_owner.rs:590`; `test-f42d54d3bd1632c2ccfa`).
- `join_capture_worker` — join capture worker (`src/capture/capture_owner.rs:332`; `test-89b10abefa1f5c9a47e2`).
- `observations` — observations (`src/capture/capture_owner.rs:253`; `test-6c30e98c2843011d2b2e`).
- `prepare_capture` — prepare capture (`src/capture/capture_owner.rs:296`; `test-59d7e50bbae31896948a`).
- `observations` — observations (`src/capture/events.rs:314`; `test-09066e0a4bfc4d299258`).
- `publish_backend_failure` — publish backend failure (`src/capture/events.rs:278`; `test-d6ee1878cb3cf2d3f452`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Capture application and microphone stems](/docs/how-to/capture-app-and-mic.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
