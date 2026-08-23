# Source selection

<!-- claims: CLM-DOC-007-CAP-001,CLM-DOC-007-SOURCE-001 -->

Discover capture candidates and resolve application, process, device, and system queries to stable source identities.

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.

These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.

## Contract surface

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
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
| `pocketstation::capture::events::SourceLifecycleEventKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:25` |
| `pocketstation::capture::events::SourceRecoveryRequirement` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:35` |
| `pocketstation::capture::events::SourceRuntimeEvent` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:53` |
| `pocketstation::capture::events::SourceRuntimeEventDelivery` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/events.rs:96` |
| `pocketstation::capture::identity::SourceKind` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:9` |
| `pocketstation::capture::identity::SourceState` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/identity.rs:17` |
| `pocketstation::capture::query::SourceQuery` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/query.rs:13` |
| `pocketstation::capture::selection::CaptureMode` | enum | The compiler exposes this declaration; its native description remains a Gate 9 obligation. | `src/capture/selection.rs:16` |

## Where you encounter it

- **Select a durable source** — Discover and resolve a source selector while preserving identity and source-generation changes.

## Behavior established by tests

The following test bodies are evidence only for their recorded setup:

- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-a8dbef4f3b61c752ce0e`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1a09c0b9480a09c36429`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-69d4e0c97753aed54953`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-50620fcc9117c7ad3cf6`).
- `given_device_invalidated_hresult_when_classified_then_source_is_unavailable` — given device invalidated hresult when classified then source is unavailable (`src/capture/platform/windows/runtime_lifecycle.rs:27`; `test-d2f761449f8212754ae7`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` — given resources invalidated hresult when classified then failure is not guessed as disappearance (`src/capture/platform/windows/runtime_lifecycle.rs:35`; `test-acc6963aea9a1e14e631`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:174`; `test-39fa4a1bc5fb034e360f`).
- `given_native_source_gap_when_advanced_then_gap_is_preserved_once` — given native source gap when advanced then gap is preserved once (`src/capture/tests.rs:295`; `test-6f92449ae2068cad145e`).
- `given_native_source_overlap_when_advanced_then_timeline_fails_closed` — given native source overlap when advanced then timeline fails closed (`src/capture/tests.rs:302`; `test-99083db5a93958229c27`).
- `given_source_generation_when_rediscovered_then_generation_advances` — given source generation when rediscovered then generation advances (`src/capture/tests.rs:252`; `test-c76dd2256adec294aa7f`).
- `given_source_unavailable_error_when_displayed_then_stable_identity_is_retained` — given source unavailable error when displayed then stable identity is retained (`src/capture/tests.rs:256`; `test-d9515c41464fa15374fd`).
- `given_stable_source_id_when_derived_twice_then_same_source_id` — given stable source id when derived twice then same source id (`src/capture/tests.rs:165`; `test-fed684d712fbb6a9afdb`).

## Boundaries

The compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Capture system audio](/docs/how-to/capture-system-audio.md)
- [Select a process-scoped source](/docs/how-to/select-process-source.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)
- [Rust API reference](/docs/reference/rust-api.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/query.rs:1-137` (`DIRECT`)
- `src/capture/selection.rs:1-89` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
