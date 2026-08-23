# Capture application and microphone stems

<!-- claims: CLM-GUIDE-003-CAP-001,CLM-GUIDE-003-CAP-002,CLM-GUIDE-003-CAP-003,CLM-GUIDE-003-CAP-004,CLM-GUIDE-003-SOURCE-001 -->

## Scope

- **Declare a Session.** Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.
- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.
- **Capture microphone audio.** Select the default or identified input device and open native microphone capture.
- **Preserve frame identity and lineage.** Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Prerequisites

Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.

## Procedure

1. Declare application and microphone sources in one Session.
2. Give each source an independent endpoint or route.
3. Retain stem and source identity from frame lineage.
4. Start once and consume both bounded routes.
5. Stop once and inspect Session plus recording outcomes.

## APIs used

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:231` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Allowed` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:232` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::Denied` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:233` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotApplicable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:235` |
| `pocketstation::capture::authorization::ApplicationPolicyObservation::NotObservable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:234` |
| `pocketstation::capture::authorization::CaptureScope::ExactApplication` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:249` |
| `pocketstation::capture::authorization::SourceIdentityStrength::ApplicationIdAndProcessId` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:258` |
| `pocketstation::capture::authorization::SourceIdentityStrength::StableApplicationId` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:259` |
| `pocketstation::capture::identity::SourceKind::Application` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:10` |
| `pocketstation::capture::selection::CaptureMode::Application` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:19` |
| `pocketstation::capture::selection::CaptureMode::ExactApplication` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:21` |
| `pocketstation::capture::selection::CaptureMode::ExactApplicationStable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:25` |
| `pocketstation::capture::selection::ProcessTreeScope::ApplicationIdentity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:86` |
| `pocketstation::capture::selection::SelectorPersistenceScope::ApplicationIdentity` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:75` |
| `authorization::CaptureAuthorizationSnapshot::application_policy` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:20` |

## Verify the outcome

The following test bodies are evidence only for their recorded setup:

- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `join_capture_worker` — join capture worker (`src/capture/capture_owner.rs:332`; `test-89b10abefa1f5c9a47e2`).
- `prepare_capture` — prepare capture (`src/capture/capture_owner.rs:296`; `test-59d7e50bbae31896948a`).
- `captured_frame_stream` — captured frame stream (`src/capture/frame_stream.rs:188`; `test-0e40457259bf43cdd2a7`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-a19b8c36cc500e40f220`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1a09c0b9480a09c36429`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-69d4e0c97753aed54953`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-7477ad1c961dad51886d`).

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

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `examples/product_quickstart.rs:1-61` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
