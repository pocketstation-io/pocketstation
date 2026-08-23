# Permissions and source lifecycle

<!-- claims: CLM-DOC-011-CAP-001,CLM-DOC-011-SOURCE-001 -->

Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

## Scope

- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::events::SourceLifecycleEventKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:25` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionChanged` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:28` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionRevoked` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:29` |
| `pocketstation::capture::events::SourceLifecycleEventKind::ReplacementObserved` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:27` |
| `pocketstation::capture::events::SourceLifecycleEventKind::SourceReappeared` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:30` |
| `pocketstation::capture::events::SourceLifecycleEventKind::SourceUnavailable` | variant | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:26` |
| `pocketstation::capture::authorization::CapturePermissionLifecycle` | struct | Control-plane owner for one source's observed authorization epoch. | `src/capture/authorization.rs:183` |
| `ActiveCaptureBackend::source_id` | function | Resolved native source identity for every frame emitted by this open. | `src/capture/capture_owner.rs:105` |
| `source_id` | function | Derives the immutable captured-frame identity for this resolved source. | `src/capture/identity.rs:46` |
| `pocketstation::capture::query::SourceProvider` | trait | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/query.rs:48` |
| `pocketstation::capture::events::SourceGeneration` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:12` |
| `pocketstation::capture::events::SourceRuntimeEventObservationHandle` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:200` |
| `pocketstation::capture::events::SourceRuntimeEventObservations` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:111` |
| `pocketstation::capture::events::SourceRuntimeEventSender` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:224` |
| `pocketstation::capture::identity::CaptureSource` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:82` |
| `pocketstation::capture::identity::StableSourceId` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:26` |
| `pocketstation::capture::query::LocalSourceProvider` | struct | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/query.rs:52` |
| `pocketstation::capture::authorization::SourceIdentityStrength` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/authorization.rs:257` |
| `pocketstation::capture::events::SourceRecoveryRequirement` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:35` |
| `pocketstation::capture::events::SourceRuntimeEvent` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:53` |

## Where you encounter it

- **Select a durable source** — Discover and resolve a source selector while preserving identity and source-generation changes.
- **Handle platform permission** — Perform non-prompting observation, own the prompt UX, and treat source opening as authoritative.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-a8dbef4f3b61c752ce0e`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-50620fcc9117c7ad3cf6`).
- `given_device_invalidated_hresult_when_classified_then_source_is_unavailable` — given device invalidated hresult when classified then source is unavailable (`src/capture/platform/windows/runtime_lifecycle.rs:27`; `test-d2f761449f8212754ae7`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` — given resources invalidated hresult when classified then failure is not guessed as disappearance (`src/capture/platform/windows/runtime_lifecycle.rs:35`; `test-acc6963aea9a1e14e631`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:174`; `test-39fa4a1bc5fb034e360f`).
- `given_native_source_gap_when_advanced_then_gap_is_preserved_once` — given native source gap when advanced then gap is preserved once (`src/capture/tests.rs:295`; `test-6f92449ae2068cad145e`).
- `given_native_source_overlap_when_advanced_then_timeline_fails_closed` — given native source overlap when advanced then timeline fails closed (`src/capture/tests.rs:302`; `test-99083db5a93958229c27`).
- `given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical` — given permission lifecycle when authorization changes then epoch and kind are canonical (`src/capture/tests.rs:470`; `test-72b7390fb29e3b3a2756`).
- `given_source_generation_when_rediscovered_then_generation_advances` — given source generation when rediscovered then generation advances (`src/capture/tests.rs:252`; `test-c76dd2256adec294aa7f`).
- `given_source_unavailable_error_when_displayed_then_stable_identity_is_retained` — given source unavailable error when displayed then stable identity is retained (`src/capture/tests.rs:256`; `test-d9515c41464fa15374fd`).
- `given_stable_source_id_when_derived_twice_then_same_source_id` — given stable source id when derived twice then same source id (`src/capture/tests.rs:165`; `test-fed684d712fbb6a9afdb`).
- `given_two_different_stable_ids_when_derived_then_different_source_ids` — given two different stable ids when derived then different source ids (`src/capture/tests.rs:183`; `test-b80d89e4327e5d2695b5`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Permission ownership](/docs/platform/permissions.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/authorization.rs:1-318` (`DIRECT`)
- `src/capture/events.rs:1-344` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
