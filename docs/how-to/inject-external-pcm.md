# Inject external PCM

<!-- claims: CLM-GUIDE-023-CAP-001,CLM-GUIDE-023-SOURCE-001 -->

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

## Important consequence

Acquisition exhaustion, invalid write shape, closure, and cancellation require different responses.

## Verify the outcome

The writer acquires, fills, and submits a buffer whose frames arrive on the declared Session route.

Executable evidence selected for **Inject external PCM** is limited to each test's recorded setup and assertions:

- `given_application_owned_audio_when_written_through_facade_then_session_delivers_its_lineage` — given application owned audio when written through facade then session delivers its lineage (`tests/audio_input.rs:37`; `test-fdcedcc753e41fe3767e`).
- `given_audio_input_when_session_runs_then_lineage_fanout_reentry_and_recording_are_real` — given audio input when session runs then lineage fanout reentry and recording are real (`tests/audio_input.rs:345`; `test-11fb08eefd3feaca7cfe`).
- `given_bounded_audio_input_when_writes_are_invalid_or_saturated_then_ownership_is_explicit` — given bounded audio input when writes are invalid or saturated then ownership is explicit (`tests/audio_input.rs:73`; `test-795ca5a283c59dbf6066`).
- `given_running_audio_input_when_writer_closes_then_accepted_frames_are_drained` — given running audio input when writer closes then accepted frames are drained (`tests/audio_input.rs:155`; `test-93f8a3bbe8b67e6e71ea`).
- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` — given active asp when required then sdk accepts external provisioning (`src/capture/platform/macos/loopback.rs:317`; `test-094ea52b81e34a03e0e1`).
- `given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used` — given external output through operator when compiled then normal typed edges are used (`src/session/extensions/tests/composition.rs:355`; `test-1e9492347c366dc04946`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-477958c0b22fe8487982`).
- `given_unregistered_external_source_when_compiled_then_registry_error_is_typed` — given unregistered external source when compiled then registry error is typed (`src/session/extensions/tests/composition.rs:411`; `test-713c01edd07447a5d6d1`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-1d9f4de1e64929bbc714`).
- `given_one_external_source_failure_when_session_runs_then_unrelated_source_completes` — given one external source failure when session runs then unrelated source completes (`src/session/extensions/tests/runtime.rs:734`; `test-bdedfb8ca6cbd5442810`).
- `given_public_session_when_external_source_declared_then_handles_are_nameable` — given public session when external source declared then handles are nameable (`tests/external_source.rs:16`; `test-b5c32ca30bd2143fa264`).
- `given_public_facade_when_external_destinations_run_then_all_branches_receive_media` — given public facade when external destinations run then all branches receive media (`tests/session_facade.rs:20`; `test-2d7cf1284199bcec7268`).

## Failure signals

- `pocketstation::session::compile::error::SessionCompileError` / `InvalidExternalSourceConfiguration` — `error-f39e1d5ca300f380beb9`
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownExternalSource` — `error-231bc903bf77aa3f85cc`
- `pocketstation::session::compile::error::SessionCompileError` / `UnknownExternalSourceOutput` — `error-037aa7c44b2e63b89b5b`
- `pocketstation::session::error::SessionError` / `NoSourceOutputRoutes` — `error-64505d377325b507747f`
- `pocketstation::session::error::SessionError` / `NoSourceOutputs` — `error-d932f27c30a809afe3de`
- `pocketstation::session::error::SessionError` / `NoSources` — `error-9eb3fbac890e3bf91775`
- `pocketstation::session::error::SessionError` / `UnknownSourceInstance` — `error-5ff90c62bdca8982aa9b`
- `pocketstation::session::error::SessionError` / `UnknownSourceOutput` — `error-871c9f44851885637584`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `ExternalAudioBridge` — `error-87902c069db58b4b0049`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `ExternalSourcePrepare` — `error-bbc9e8298f41cb00dbbf`

## API reference

- [External Pcm](/docs/concepts/external-pcm.md)
- [Capture](/docs/reference/capture.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::extensions::audio_input::AudioInput` | struct | Intent-first façade for feeding audio already owned by the embedding application into a Session. | `src/session/extensions/audio_input/mod.rs:94` |
| `pocketstation::session::extensions::audio_input::AudioInputConfig` | struct | Configures audio input behavior at its owning API boundary. | `src/session/extensions/audio_input/mod.rs:22` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBuffer` | struct | Leases bounded PCM storage from an external-audio input until the caller submits or releases it. | `src/session/extensions/audio_input/buffer.rs:11` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputObservations` | struct | Reports the audio input observations collected at an observation boundary. | `src/session/extensions/audio_input/buffer.rs:72` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` | struct | Reports a audio input write error. | `src/session/extensions/audio_input/buffer.rs:305` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriter` | struct | Sends audio input values across its declared ownership boundary. | `src/session/extensions/audio_input/buffer.rs:91` |
| `pocketstation::session::extensions::audio_input::source::PcmSource` | struct | Low-level PCM source ownership for integrations that separately retain the Session handles and producer writer. | `src/session/extensions/audio_input/source.rs:33` |
| `pocketstation::session::extensions::audio_input::AudioInputConfigError` | enum | Classifies failures reported as audio input config error. | `src/session/extensions/audio_input/mod.rs:77` |

## Related documentation

- [External PCM input](/docs/concepts/external-pcm.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Evidence boundary

The claims on **Inject external PCM** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/audio_input.rs:1-577` (`DIRECT`)

For **Inject external PCM**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
