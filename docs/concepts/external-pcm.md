# External PCM input

<!-- claims: CLM-DOC-032-CAP-001,CLM-DOC-032-SOURCE-001 -->

Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.

## Scope

- **Inject external PCM.** Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::extensions::audio_input::AudioInput` | struct | Intent-first façade for feeding audio already owned by the embedding application into a Session. | `src/session/extensions/audio_input/mod.rs:94` |
| `pocketstation::session::extensions::audio_input::source::PcmSource` | struct | Low-level PCM source ownership for integrations that separately retain the Session handles and producer writer. | `src/session/extensions/audio_input/source.rs:33` |
| `with_input_edge` | function | Declares the bounded delivery policy for routes entering this endpoint. | `src/session/declaration/endpoint.rs:136` |
| `pocketstation::session::extensions::audio_input::PCM_SOURCE_TYPE_ID` | constant | Stable runtime identity of the underlying PCM source implementation. | `src/session/extensions/audio_input/mod.rs:19` |
| `SessionOperatorMetrics::input_edge` | struct_field | Sole counter authority for input delivered by the compiled Session plan. | `src/session/lifecycle/observations.rs:389` |
| `SessionOperatorMetrics::input_ports` | struct_field | Exact per-port input accounting. `input_edge` is the compatibility aggregate across this slice. | `src/session/lifecycle/observations.rs:392` |
| `pocketstation::session::declaration::draft::OperatorInputHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/declaration/draft.rs:713` |
| `pocketstation::session::extensions::audio_input::AudioInputConfig` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/mod.rs:22` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBuffer` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:11` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:72` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteError` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:305` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriter` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:91` |
| `pocketstation::session::lifecycle::observations::SessionExternalSourceMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:124` |
| `pocketstation::session::lifecycle::observations::SessionOperatorInputMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:242` |
| `pocketstation::capture::selection::InputDeviceSelector` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:9` |
| `pocketstation::session::extensions::audio_input::AudioInputConfigError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/mod.rs:77` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferAcquireError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:271` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputBufferError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:281` |
| `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/buffer.rs:298` |
| `pocketstation::session::extensions::audio_input::source::AudioInputError` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/extensions/audio_input/source.rs:85` |

## Where you encounter it

- **Inject external PCM** — Acquire bounded buffers, write PCM, and observe source runtime outcomes.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_denied_permission_when_opening_input_then_capture_fails_closed` — given denied permission when opening input then capture fails closed (`src/capture/platform/macos/input.rs:377`; `test-2b664c22fd511e3c2f45`).
- `given_promptable_or_observable_permission_when_opening_input_then_native_open_decides` — given promptable or observable permission when opening input then native open decides (`src/capture/platform/macos/input.rs:393`; `test-847d3fefe4665db8dd14`).
- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` — given active asp when required then sdk accepts external provisioning (`src/capture/platform/macos/loopback.rs:317`; `test-094ea52b81e34a03e0e1`).
- `given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity` — given stable input selector when wrapped then capture mode preserves identity (`src/capture/tests.rs:202`; `test-074f0bb3502ed6267879`).
- `external_source_declarations` — external source declarations (`src/session/compile/compiled.rs:34`; `test-067a9c1179f9fa65bb67`).
- `given_derived_stream_chain_when_compiled_then_operator_output_feeds_next_named_input` — given derived stream chain when compiled then operator output feeds next named input (`src/session/compile/tests.rs:587`; `test-e9a24a392741b4dbe6e7`).
- `given_required_named_input_missing_when_compiled_then_failure_precedes_graph_runtime` — given required named input missing when compiled then failure precedes graph runtime (`src/session/compile/tests.rs:384`; `test-99e881c59d26f6126f74`).
- `given_duplicate_named_input_when_connected_then_declaration_fails_immediately` — given duplicate named input when connected then declaration fails immediately (`src/session/declaration/tests/operator_connections.rs:110`; `test-f9a6ec4f71dbaf6d8083`).
- `given_foreign_input_handle_when_connected_then_declaration_fails_before_freeze` — given foreign input handle when connected then declaration fails before freeze (`src/session/declaration/tests/operator_connections.rs:133`; `test-766194f5939b3ddb896d`).
- `given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used` — given external output through operator when compiled then normal typed edges are used (`src/session/extensions/tests/composition.rs:355`; `test-1e9492347c366dc04946`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-477958c0b22fe8487982`).
- `given_unregistered_external_source_when_compiled_then_registry_error_is_typed` — given unregistered external source when compiled then registry error is typed (`src/session/extensions/tests/composition.rs:411`; `test-713c01edd07447a5d6d1`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Inject external PCM](/docs/how-to/inject-external-pcm.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/extensions/audio_input/mod.rs:1-152` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
