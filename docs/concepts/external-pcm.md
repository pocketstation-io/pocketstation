# External PCM input

<!-- claims: CLM-DOC-032-CAP-001,CLM-DOC-032-SOURCE-001 -->

## What it is

External PCM input is a source extension that gives application code a finite buffer, validates its sample shape, and submits the initialized frame into Session routing.

## Why it exists

Externally produced audio needs the same capacity, ownership, lineage, and cancellation rules as native capture without pretending to be a platform device.

## Relationships

- `AudioInputConfig` declares sample format and bounded capacity.
- A writer acquires and fills an `AudioInputBuffer`.
- The submitted source uses ordinary Session routes and observations.

## Invariants and guarantees

- Writes stay within acquired capacity and match the declared frame shape.
- Acquisition exhaustion, write rejection, closure, and cancellation are distinct.
- Submitted frames preserve Session and source lineage.

## When you encounter it

- **Inject external PCM** — Acquire bounded buffers, write PCM, and observe source runtime outcomes.

## Use it

- [Inject external PCM](/docs/how-to/inject-external-pcm.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Scope

- **Inject external PCM.** Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.

The scope of **External PCM input** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

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
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | enum | Classifies failures reported as audio input buffer acquire error. | `src/session/extensions/audio_input/buffer.rs:271` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | enum | Classifies failures reported as audio input buffer error. | `src/session/extensions/audio_input/buffer.rs:281` |

## Executable evidence

Executable evidence selected for **External PCM input** is limited to each test's recorded setup and assertions:

- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:377`; `test-2b664c22fd511e3c2f45`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` — given promptable or observable permission when opening input then native open decides (`src/capture/platform/macos/input.rs:393`; `test-847d3fefe4665db8dd14`).
- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` — given active asp when required then sdk accepts external provisioning (`src/capture/platform/macos/loopback.rs:317`; `test-094ea52b81e34a03e0e1`).
- `given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity` — given stable input selector when wrapped then capture mode preserves identity (`src/capture/tests.rs:202`; `test-074f0bb3502ed6267879`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-e9a24a392741b4dbe6e7`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` — given required named input missing when compiled then failure precedes graph runtime (`src/session/compile/tests.rs:384`; `test-99e881c59d26f6126f74`).
- `given_duplicate_named_input_when_connected_then_declaration_fails_immediately` — given duplicate named input when connected then declaration fails immediately (`src/session/declaration/tests/operator_connections.rs:110`; `test-f9a6ec4f71dbaf6d8083`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` — given foreign input handle when connected then declaration fails before freeze (`src/session/declaration/tests/operator_connections.rs:133`; `test-766194f5939b3ddb896d`).
- `given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used` — given external output through operator when compiled then normal typed edges are used (`src/session/extensions/tests/composition.rs:355`; `test-1e9492347c366dc04946`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-477958c0b22fe8487982`).
- `given_unregistered_external_source_when_compiled_then_registry_error_is_typed` — given unregistered external source when compiled then registry error is typed (`src/session/extensions/tests/composition.rs:411`; `test-713c01edd07447a5d6d1`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-1d9f4de1e64929bbc714`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Inject external PCM](/docs/how-to/inject-external-pcm.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Evidence boundary

The claims on **External PCM input** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/extensions/audio_input/mod.rs:1-152` (`DIRECT`)

For **External PCM input**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
