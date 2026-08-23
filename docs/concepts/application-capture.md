# Application capture

<!-- claims: CLM-DOC-008-CAP-001,CLM-DOC-008-SOURCE-001 -->

Prepare application-scoped capture through the platform backend selected for the current target.

## Scope

- **Capture application audio.** Prepare application-scoped capture through the platform backend selected for the current target.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

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
| `authorization::CaptureScope::ExactApplication::stable_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:249` |
| `selection::CaptureMode::ExactApplication::process_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:22` |
| `selection::CaptureMode::ExactApplication::stable_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:23` |
| `selection::CaptureMode::ExactApplicationStable::stable_id` | struct_field | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:26` |

## Where you encounter it

- **Reach first captured frames** — Build a Session that captures an application and microphone and polls their independent frames.
- **Handle platform permission** — Perform non-prompting observation, own the prompt UX, and treat source opening as authoritative.
- **Transcribe captured stems** — Run the repository transcription example and preserve process evidence for its external boundary.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-c55d7a75628c1be024f1`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-4f65c4d2e20b5226cd4f`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-889c6cfb54cc924fc2b4`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-8de0974346f9110044c2`).
- `join_capture_worker` — join capture worker (`src/capture/capture_owner.rs:332`; `test-89b10abefa1f5c9a47e2`).
- `prepare_capture` — prepare capture (`src/capture/capture_owner.rs:296`; `test-59d7e50bbae31896948a`).
- `captured_frame_stream` — captured frame stream (`src/capture/frame_stream.rs:188`; `test-0e40457259bf43cdd2a7`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-a19b8c36cc500e40f220`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-c28f1242d8a2b60457db`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1a09c0b9480a09c36429`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-69d4e0c97753aed54953`).
- `given_exact_application_selector_when_one_live_node_matches_then_current_target_is_selected` — given exact application selector when one live node matches then current target is selected (`src/capture/platform/linux/pipewire.rs:1932`; `test-7477ad1c961dad51886d`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Windows capture](/docs/platform/windows.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/capture_owner.rs:1-626` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
