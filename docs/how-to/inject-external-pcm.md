# Inject external PCM

<!-- claims: CLM-GUIDE-023-CAP-001,CLM-GUIDE-023-SOURCE-001 -->

## Scope

- **Inject external PCM.** Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Create AudioInputConfig matching the producer.
2. Acquire a bounded AudioInputBuffer.
3. Write only within declared capacity and format.
4. Submit through AudioInputWriter and route the source.
5. Handle acquire, write, cancellation, and runtime errors separately.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::observations::SessionExternalSourceMetrics` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:124` |
| `external_source` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:87` |
| `external_source_count` | function | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/observations.rs:83` |
| `pocketstation::session::compile::error::SessionCompileError::InvalidExternalSourceConfiguration` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:78` |
| `pocketstation::session::compile::error::SessionCompileError::UnknownExternalSource` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:69` |
| `pocketstation::session::compile::error::SessionCompileError::UnknownExternalSourceOutput` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:73` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalAudioBridge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:124` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalSourcePrepare` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:119` |
| `pocketstation::session::lifecycle::start_contract::SessionStartError::ExternalSourceStart` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/lifecycle/start_contract.rs:134` |
| `pocketstation::session::prepare::error::SessionPrepareError::InvalidExternalAudioMedia` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/prepare/error.rs:26` |
| `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalAudioIngress` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/prepare/error.rs:19` |
| `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalSourceDefinition` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/prepare/error.rs:24` |
| `pocketstation::session::prepare::error::SessionPrepareError::MissingExternalSourceRouteEdge` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/prepare/error.rs:37` |
| `SessionCompileError::InvalidExternalSourceConfiguration::reason` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:80` |
| `SessionCompileError::InvalidExternalSourceConfiguration::source_type_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:79` |
| `SessionCompileError::UnknownExternalSource::source_type_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/session/compile/error.rs:69` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_active_asp_when_required_then_sdk_accepts_external_provisioning` — given active asp when required then sdk accepts external provisioning (`src/capture/platform/macos/loopback.rs:317`; `test-094ea52b81e34a03e0e1`).
- `external_source_declarations` — external source declarations (`src/session/compile/compiled.rs:34`; `test-067a9c1179f9fa65bb67`).
- `given_external_output_through_operator_when_compiled_then_normal_typed_edges_are_used` — given external output through operator when compiled then normal typed edges are used (`src/session/extensions/tests/composition.rs:355`; `test-1e9492347c366dc04946`).
- `given_external_pcm_output_when_compiled_then_bounded_audio_edge_is_planned` — given external pcm output when compiled then bounded audio edge is planned (`src/session/extensions/tests/composition.rs:333`; `test-477958c0b22fe8487982`).
- `given_unregistered_external_source_when_compiled_then_registry_error_is_typed` — given unregistered external source when compiled then registry error is typed (`src/session/extensions/tests/composition.rs:411`; `test-713c01edd07447a5d6d1`).
- `given_external_pcm_source_when_session_runs_then_audio_uses_bounded_ingress_with_source_identity` — given external pcm source when session runs then audio uses bounded ingress with source identity (`src/session/extensions/tests/runtime.rs:823`; `test-1d9f4de1e64929bbc714`).
- `given_one_external_source_failure_when_session_runs_then_unrelated_source_completes` — given one external source failure when session runs then unrelated source completes (`src/session/extensions/tests/runtime.rs:734`; `test-bdedfb8ca6cbd5442810`).
- `given_public_session_when_external_source_declared_then_handles_are_nameable` — given public session when external source declared then handles are nameable (`tests/external_source.rs:16`; `test-b5c32ca30bd2143fa264`).
- `given_public_facade_when_external_destinations_run_then_all_branches_receive_media` — given public facade when external destinations run then all branches receive media (`tests/session_facade.rs:22`; `test-cc60ba7d0baeb4db3d4c`).
- `frame_stream_closed` — frame stream closed (`src/capture/capture_owner.rs:248`; `test-3ab763bff0cd08d4b4e1`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).

## Failure signals

- `pocketstation::session::declaration::typed_stream::TypedStreamError` / `OutputSignalMismatch` — `error-00e5716261eba0f8cf3d`
- `pocketstation::session::error::SessionError` / `UnknownStem` — `error-00f6e798d158df66c847`
- `pocketstation::session::error_code::SessionStartErrorCode` / `StartCancelled` — `error-01d3fc855e2a00319076`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `OperatorPrepare` — `error-023d6ab0b23a50a614ff`
- `pocketstation::session::error_code::SessionStartErrorCode` / `TraceRecorderSetupFailed` — `error-0279b2b6b0cb3b5801bc`
- `pocketstation::session::prepare::error::SessionPrepareError` / `MissingOperatorSignalInput` — `error-037ddc3e193da74177f8`
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` — `error-05c60389efcb84311921`
- `pocketstation::session::prepare::error::SessionPrepareError` — `error-085082b521c14e5ecd1e`
- `pocketstation::session::extensions::audio_input::buffer::AudioInputWriteErrorKind` / `Closed` — `error-08a7536094bfb2242b17`
- `pocketstation::session::lifecycle::host::SessionEngineHostBuildError` / `EndpointExtensionRegistration` — `error-09837185c7fca0f70618`
- `pocketstation::session::lifecycle::start_contract::SessionStartError` / `MissingEndpointDeclaration` — `error-0bc2f7c0b9f9dbf8ddd7`
- `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` / `ZeroCapacity` — `error-0bd6f58be40ade9a01fe`

Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.

## Related documentation

- [External PCM input](/docs/concepts/external-pcm.md)
- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Configuration reference](/docs/reference/configuration.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `tests/audio_input.rs:1-577` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
