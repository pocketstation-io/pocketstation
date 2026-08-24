# Inject external PCM

<!-- claims: CLM-GUIDE-023-SCOPE-001,CLM-GUIDE-023-TEXT-001,CLM-GUIDE-023-TEXT-002,CLM-GUIDE-023-TEXT-003,CLM-GUIDE-023-TEXT-004,CLM-GUIDE-023-TEXT-005,CLM-GUIDE-023-TEXT-006,CLM-GUIDE-023-SOURCE-001 -->

## Scope

- **Inject external PCM.** Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.

The scope of **Inject external PCM** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

An `AudioInputConfig` matching the producer's rate, channels, frame shape, and finite buffer capacity.

## Procedure

1. Create AudioInputConfig matching the producer.
2. Acquire a bounded AudioInputBuffer.
3. Write only within declared capacity and format.
4. Submit through AudioInputWriter and route the source.
5. Handle acquire, write, cancellation, and runtime errors separately.

## Concrete repository example

The executable repository test `given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage` (`test-f1139be85b9372ec989b`) shows the concrete API sequence and asserted outcome at `tests/audio_input.rs:37`.

```rust
}

#[test]
fn given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage() {
    let session = Session::builder().sample_spec(sample_spec()).build();
    let mut input = session
        .audio_input(audio_input_config(2))
        .expect("application audio input");
    let source_id = input.source().source_id();
    let stream_id = input.output().stream_id();
    let polled_audio = session.polled_audio().expect("polled audio endpoint");
    input
        .output()
        .send(polled_audio)
        .expect("audio input polling route");

    let mut running = session.start().expect("running audio input Session");
    input
        .try_write(&vec![0.25_f32; FRAME_SAMPLES])
        .expect("nonblocking façade write");

    let deadline = Instant::now() + Duration::from_secs(3);
    let (delivered_source_id, delivered_stream_id) = loop {
        if let Ok(batch) = running.try_poll_audio() {
            if let Some(frame) = batch.frame(0) {
                break (frame.lineage().source_id(), frame.stream_id());
            }
        }
        assert!(Instant::now() < deadline, "façade frame was not delivered");
        std::thread::yield_now();
    };
    assert_eq!(delivered_source_id, source_id);
    assert_eq!(delivered_stream_id, stream_id);

    input.close();
    assert!(running.stop().is_success());
}
```

```bash
cargo test --all-features given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage
```

## Important consequence

Acquisition exhaustion, invalid write shape, closure, and cancellation require different responses.

## Verify the outcome

The writer acquires, fills, and submits a buffer whose frames arrive on the declared Session route.

Executable evidence selected for **Inject external PCM** is limited to each test's recorded setup and assertions:

- `given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage` — given application owned audio when written through facade then session delivers its lineage (`tests/audio_input.rs:37`; `test-f1139be85b9372ec989b`).
- `given_audio_input_when_session_runs_then_lineage_fanout_reentry_and_recording_are_real` — given audio input when session runs then lineage fanout reentry and recording are real (`tests/audio_input.rs:427`; `test-c3a710b6a0936cfd1065`).
- `given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit` — given bounded audio input when writes are invalid or saturated then ownership is explicit (`tests/audio_input.rs:73`; `test-5a764dda823599c553f3`).
- `given_running_audio_input_when_writer_closes_then_accepted_frames_are_drained` — given running audio input when writer closes then accepted frames are drained (`tests/audio_input.rs:155`; `test-b2b127f93977bf0ce175`).
- `given_two_audio_inputs_on_one_many_port_when_run_then_each_source_lineage_is_preserved` — given two audio inputs on one many port when run then each source lineage is preserved (`tests/audio_input.rs:351`; `test-a451e8ad08df8f006452`).
- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` — given active asp when required then sdk accepts external provisioning (`src/capture/platform/macos/loopback.rs:317`; `test-0f83918e778e3285908b`).
- `given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used` — given external output through operator when compiled then normal typed edges are used (`src/session/extensions/tests/composition.rs:355`; `test-16199d206f3d5dfd3054`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-72f08a54e97cf69789ac`).
- `given_unregistered_external_source_when_compiled_then_registry_error_is_typed` — given unregistered external source when compiled then registry error is typed (`src/session/extensions/tests/composition.rs:411`; `test-bff09e350c584b9f042e`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-4d0f3e5a95ea9490a090`).
- `given_one_external_source_failure_when_session_runs_then_unrelated_source_completes` — given one external source failure when session runs then unrelated source completes (`src/session/extensions/tests/runtime.rs:734`; `test-9839e75a34cc80e4b057`).
- `given_public_session_when_external_source_declared_then_handles_are_nameable` — given public session when external source declared then handles are nameable (`tests/external_source.rs:16`; `test-075cf6099f60862bb276`).

## Failure signals

- `pocketstation::session::compile::error::SessionCompileError` / `InvalidExternalSourceConfiguration` — `error-1be7c159405620caebf8`
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownExternalSource` — `error-12856b03833c2e0bb1ff`
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownExternalSourceOutput` — `error-8be2bdcc04e8f30e616e`
- `pocketstation::session::error::SessionError` / `NoSourceOutputRoutes` — `error-b07516f4b9ba40fdb882`
- `pocketstation::session::error::SessionError` / `NoSourceOutputs` — `error-a8908dd20100e9a5703a`
- `pocketstation::session::error::SessionError` / `NoSources` — `error-960f2ecbe521a8333f23`
- `pocketstation::session::error::SessionError` / `UnknownSourceInstance` — `error-4d7d2b2124a9abfda7d8`
- `pocketstation::session::error::SessionError` / `UnknownSourceOutput` — `error-6d68cf14a05afa9a6ec8`
- `pocketstation::session::lifecycle::control::SessionStartError` / `ExternalAudioBridge` — `error-3a234ccba235becc2ab7`
- `pocketstation::session::lifecycle::control::SessionStartError` / `ExternalSourcePrepare` — `error-59e2d48ac5c22ac45b2a`

## API reference

- [External Pcm](/docs/concepts/external-pcm.md)
- [Capture](/docs/reference/capture.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::extensions::audio_input::AudioInput` | struct | Intent-first façade for feeding audio already owned by the embedding application into a Session. | `src/session/extensions/audio_input/mod.rs:94` |
| `pocketstation::session::extensions::audio_input::AudioInputConfig` | struct | Configures audio input behavior at its owning API boundary. | `src/session/extensions/audio_input/mod.rs:22` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBuffer` | struct | Leases bounded PCM storage from an external-audio input until the caller submits or releases it. | `src/session/extensions/audio_input/buffer.rs:11` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputObservations` | struct | Reports the audio input observations collected at an observation boundary. | `src/session/extensions/audio_input/buffer.rs:72` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` | struct | Classifies failures produced during audio input writing. | `src/session/extensions/audio_input/buffer.rs:305` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriter` | struct | Sends audio input values across its declared ownership boundary. | `src/session/extensions/audio_input/buffer.rs:91` |
| `pocketstation::session::extensions::audio_input::source::PcmSource` | struct | Low-level PCM source ownership for integrations that separately retain the Session handles and producer writer. | `src/session/extensions/audio_input/source.rs:33` |
| `pocketstation::session::extensions::audio_input::AudioInputConfigError` | enum | Classifies failures surfaced by audio input config operations. | `src/session/extensions/audio_input/mod.rs:77` |

## Related documentation

- [External PCM input](/docs/concepts/external-pcm.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Evidence boundary

The claims on **Inject external PCM** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `tests/audio_input.rs:73-155` (`TESTED`)
- `tests/audio_input.rs:351-427` (`TESTED`)
- `tests/audio_input.rs:155-210` (`TESTED`)
- `tests/audio_input.rs:427-591` (`TESTED`)
- `tests/audio_input.rs:37-73` (`TESTED`)

For **Inject external PCM**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
