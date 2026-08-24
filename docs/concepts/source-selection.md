# Source selection

<!-- claims: CLM-DOC-007-SCOPE-001,CLM-DOC-007-TEXT-001,CLM-DOC-007-TEXT-002,CLM-DOC-007-TEXT-003,CLM-DOC-007-TEXT-004,CLM-DOC-007-TEXT-005,CLM-DOC-007-TEXT-006,CLM-DOC-007-SOURCE-001 -->

## What it is

Source selection turns an application, process, microphone, or system query into a capture identity. A query states what you want; resolution records which concrete source matched at that observation point.

## Why it exists

Process identifiers, application membership, devices, and permissions can change. A distinct resolution step keeps those changes visible instead of treating a display name or process ID as permanent identity.

## Relationships

- `ApplicationSelector` and capture queries express selection intent.
- `CaptureSelection` records the resolved candidate and identity strength.
- Source generation and permission epoch report changes after selection.

## Invariants and guarantees

- An empty or malformed stable key is rejected.
- Process-scoped selection retains the exact process instance rather than only a numeric PID.
- Resolution evidence does not prove the source can be opened; preparation returns that result.

## When you encounter it

- **Select a durable source** — Discover and resolve a source selector while preserving identity and source-generation changes.

## Use it

- [Select a process-scoped source](/docs/how-to/select-process-source.md)
- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Diagnose missing application audio](/docs/troubleshooting/no-application-audio.md)

## Scope

- **Select and resolve capture sources.** Discover capture candidates and resolve application, process, device, and system queries to stable source identities.

The scope of **Source selection** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `selection` | module | Capture selection semantics; control-plane only. | `src/capture/selection.rs:1` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| `pocketstation::capture::query::SourceQuery` | enum | Describes the source kind and optional application or device selector used for discovery. | `src/capture/query.rs:13` |
| `pocketstation::capture::selection::CaptureMode` | enum | Selects the capture mode used by PocketStation. | `src/capture/selection.rs:16` |
| `pocketstation::capture::selection::InputDeviceSelector` | enum | Selects either the default input device or one exact device identity. | `src/capture/selection.rs:9` |
| `pocketstation::capture::selection::ProcessTreeScope` | enum | Selects the process tree scope used by PocketStation. | `src/capture/selection.rs:83` |
| `pocketstation::capture::selection::SelectorPersistenceScope` | enum | Selects the selector persistence scope used by PocketStation. | `src/capture/selection.rs:73` |
| `pocketstation::capture::query::discover_sources` | function | Discovers capture sources available from the local provider. | `src/capture/query.rs:85` |
| `query::SourceProvider::discover` | function | Discovers the resources visible to `SourceProvider`. | `src/capture/query.rs:49` |

## Executable evidence

Executable evidence selected for **Source selection** is limited to each test's recorded setup and assertions:

- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1e40dd4ec9e96cd35eb7`).
- `given_exact_application_selector_when_multiple_nodes_match_then_selection_is_ambiguous` — given exact application selector when multiple nodes match then selection is ambiguous (`src/capture/platform/linux/pipewire.rs:1998`; `test-2843e96f914d98065a94`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-d288558b68fc54333e50`).
- `given_device_invalidated_hresult_when_classified_then_source_is_unavailable` — given device invalidated hresult when classified then source is unavailable (`src/capture/platform/windows/runtime_lifecycle.rs:27`; `test-d191d2cb74b1f34f301b`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` — given resources invalidated hresult when classified then failure is not guessed as disappearance (`src/capture/platform/windows/runtime_lifecycle.rs:35`; `test-f7437f6b9062abefafe0`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:183`; `test-9c549d91f364bb436c12`).
- `given_native_source_gap_when_advanced_then_gap_is_preserved_once` — given native source gap when advanced then gap is preserved once (`src/capture/tests.rs:304`; `test-67cd94cce881395f7a8f`).
- `given_native_source_overlap_when_advanced_then_timeline_fails_closed` — given native source overlap when advanced then timeline fails closed (`src/capture/tests.rs:311`; `test-25a09fb5b40411afaa30`).
- `given_source_generation_when_rediscovered_then_generation_advances` — given source generation when rediscovered then generation advances (`src/capture/tests.rs:261`; `test-805b3deb0c3e0ced1c78`).
- `given_source_unavailable_error_when_displayed_then_stable_identity_is_retained` — given source unavailable error when displayed then stable identity is retained (`src/capture/tests.rs:265`; `test-8f3a284602ceedaf956f`).
- `given_stable_source_id_when_derived_twice_then_same_source_id` — given stable source id when derived twice then same source id (`src/capture/tests.rs:174`; `test-7e91883f38860c96231f`).

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

The claims on **Source selection** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/query.rs:1-1` (`DECLARED`)
- `src/capture/selection.rs:1-1` (`DECLARED`)

For **Source selection**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
