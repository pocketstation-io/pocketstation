# Permissions and source lifecycle

<!-- claims: CLM-DOC-011-SCOPE-001,CLM-DOC-011-TEXT-001,CLM-DOC-011-TEXT-002,CLM-DOC-011-TEXT-003,CLM-DOC-011-TEXT-004,CLM-DOC-011-TEXT-005,CLM-DOC-011-TEXT-006,CLM-DOC-011-SOURCE-001 -->

## What it is

Permission and source lifecycle records separate authorization observation from source generation, disappearance, reappearance, and open outcomes.

## Why it exists

Desktop authorization is not one universal process-wide Boolean. Keeping observation, prompts, and open results distinct prevents advisory state from being promoted to a guarantee.

## Relationships

- The host application owns prompts and user-facing permission flow.
- Capture preparation or opening returns the authoritative typed result.
- Permission epochs and source generations identify changes that affect later frames.

## Invariants and guarantees

- Observation does not prompt.
- `NotObservable` is neither granted nor denied.
- Frames retain the generation and permission evidence in effect when they were produced.

## When you encounter it

- **Select a durable source** — Discover and resolve a source selector while preserving identity and source-generation changes.
- **Handle platform permission** — Perform non-prompting observation, own the prompt UX, and treat source opening as authoritative.

## Use it

- [Observe permission without prompting](/docs/how-to/observe-permission.md)
- [Permission state is denied or unobservable](/docs/troubleshooting/permission-state.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)

## Scope

- **Observe permission and source lifecycle.** Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.

The scope of **Permissions and source lifecycle** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::lifecycle_registry::SourceLifecycleRegistry` | struct | Assigns source generations across complete discovery snapshots. | `src/capture/lifecycle_registry.rs:31` |
| `pocketstation::capture::events::SourceLifecycleEventKind` | enum | Selects the source lifecycle event kind used by PocketStation. | `src/capture/events.rs:25` |
| `pocketstation::capture::lifecycle_registry::SourceGenerationTransition` | enum | Records whether a capture source disappeared, reappeared, or changed generation. | `src/capture/lifecycle_registry.rs:8` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionChanged` | variant | Identifies the permission changed state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:28` |
| `pocketstation::capture::events::SourceLifecycleEventKind::PermissionRevoked` | variant | Identifies the permission revoked state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:29` |
| `pocketstation::capture::events::SourceLifecycleEventKind::ReplacementObserved` | variant | Identifies the replacement observed state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:27` |
| `pocketstation::capture::events::SourceLifecycleEventKind::SourceReappeared` | variant | Identifies the source reappeared state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:30` |
| `pocketstation::capture::events::SourceLifecycleEventKind::SourceUnavailable` | variant | Identifies the source unavailable state or stage represented by `SourceLifecycleEventKind`. | `src/capture/events.rs:26` |
| `pocketstation::capture::lifecycle_registry::SourceGenerationTransition::Disappeared` | variant | Represents the disappeared alternative defined by `SourceGenerationTransition`. | `src/capture/lifecycle_registry.rs:9` |
| `pocketstation::capture::lifecycle_registry::SourceGenerationTransition::Reappeared` | variant | Represents the reappeared alternative defined by `SourceGenerationTransition`. | `src/capture/lifecycle_registry.rs:13` |

## Executable evidence

Executable evidence selected for **Permissions and source lifecycle** is limited to each test's recorded setup and assertions:

- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-d288558b68fc54333e50`).
- `given_device_invalidated_hresult_when_classified_then_source_is_unavailable` — given device invalidated hresult when classified then source is unavailable (`src/capture/platform/windows/runtime_lifecycle.rs:27`; `test-d191d2cb74b1f34f301b`).
- `given_resources_invalidated_hresult_when_classified_then_failure_is_not_guessed_as_disappearance` — given resources invalidated hresult when classified then failure is not guessed as disappearance (`src/capture/platform/windows/runtime_lifecycle.rs:35`; `test-f7437f6b9062abefafe0`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:183`; `test-9c549d91f364bb436c12`).
- `given_native_source_gap_when_advanced_then_gap_is_preserved_once` — given native source gap when advanced then gap is preserved once (`src/capture/tests.rs:304`; `test-67cd94cce881395f7a8f`).
- `given_native_source_overlap_when_advanced_then_timeline_fails_closed` — given native source overlap when advanced then timeline fails closed (`src/capture/tests.rs:311`; `test-25a09fb5b40411afaa30`).
- `given_permission_lifecycle_when_authorization_changes_then_epoch_and_kind_are_canonical` — given permission lifecycle when authorization changes then epoch and kind are canonical (`src/capture/tests.rs:479`; `test-e62afd814c0b32723785`).
- `given_source_generation_when_rediscovered_then_generation_advances` — given source generation when rediscovered then generation advances (`src/capture/tests.rs:261`; `test-805b3deb0c3e0ced1c78`).
- `given_source_unavailable_error_when_displayed_then_stable_identity_is_retained` — given source unavailable error when displayed then stable identity is retained (`src/capture/tests.rs:265`; `test-8f3a284602ceedaf956f`).
- `given_stable_source_id_when_derived_twice_then_same_source_id` — given stable source id when derived twice then same source id (`src/capture/tests.rs:174`; `test-7e91883f38860c96231f`).
- `given_two_different_stable_ids_when_derived_then_different_source_ids` — given two different stable ids when derived then different source ids (`src/capture/tests.rs:192`; `test-dd8055d76f7618400d19`).

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

The claims on **Permissions and source lifecycle** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/authorization.rs:1-1` (`DECLARED`)
- `src/capture/events.rs:1-1` (`DECLARED`)

For **Permissions and source lifecycle**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
