# System capture

<!-- claims: CLM-DOC-010-SCOPE-001,CLM-DOC-010-TEXT-001,CLM-DOC-010-TEXT-002,CLM-DOC-010-TEXT-003,CLM-DOC-010-TEXT-004,CLM-DOC-010-TEXT-005,CLM-DOC-010-TEXT-006,CLM-DOC-010-SOURCE-001 -->

## What it is

System capture represents a platform loopback source where the selected backend implements one. It captures a broader system mix than application-scoped capture.

## Why it exists

Loopback capture has different platform support and permission behavior from application or microphone capture, so it needs an explicit source mode and qualification boundary.

## Relationships

- A system query expresses loopback intent.
- The platform module selects the target implementation.
- The resulting frames use the same routing and lineage contracts as other audio sources.

## Invariants and guarantees

- A source-query variant does not guarantee that the current target can open it.
- Target-specific source files prove implementation presence only.
- Unsupported opening remains a typed result rather than a silent fallback to another source.

## When you encounter it

- **Handle platform permission** — Perform non-prompting observation, own the prompt UX, and treat source opening as authoritative.

## Use it

- [Capture system audio](/docs/how-to/capture-system-audio.md)
- [Review platform compatibility](/docs/platform/compatibility.md)
- [Diagnose permission state](/docs/troubleshooting/permission-state.md)

## Scope

- **Capture system audio.** Represent and open platform system-loopback capture where the selected backend implements it.

The scope of **System capture** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::capture::platform::macos::loopback::SystemLoopbackSource` | struct | Manages a macOS loopback capture session. | `src/capture/platform/macos/loopback.rs:53` |
| `pocketstation::capture::query::SourceProvider` | trait | Implement this trait to provide source behavior to PocketStation; its methods define the preparation and runtime contract. | `src/capture/query.rs:48` |
| `pocketstation::capture::query::LocalSourceProvider` | struct | Discovers and resolves capture sources through the target platform backend. | `src/capture/query.rs:52` |
| `pocketstation::capture::query::SourceQuery` | enum | Describes the source kind and optional application or device selector used for discovery. | `src/capture/query.rs:13` |
| `capture_mode` | function | Returns the capture mode held by `SystemLoopbackSource`. | `src/capture/platform/macos/loopback.rs:57` |
| `pocketstation::capture::query::application_capture_available` | function | Reports whether this host exposes the native application-capture facility. | `src/capture/query.rs:64` |
| `pocketstation::capture::query::discover_sources` | function | Discovers capture sources available from the local provider. | `src/capture/query.rs:85` |
| `pocketstation::capture::query::resolve_query` | function | Filters discovered capture sources using the supplied source query. | `src/capture/query.rs:40` |
| `pocketstation::capture::query::SourceQuery::Any` | variant | Represents the any alternative defined by `SourceQuery`. | `src/capture/query.rs:14` |
| `pocketstation::capture::query::SourceQuery::App` | variant | Represents the app alternative defined by `SourceQuery`. | `src/capture/query.rs:15` |

## Executable evidence

Executable evidence selected for **System capture** is limited to each test's recorded setup and assertions:

- `given_default_capture_mode_when_compared_then_is_system_mix` — given default capture mode when compared then is system mix (`src/capture/tests.rs:207`; `test-f45d875b7b23fede26a0`).
- `given_active_capture_when_owner_is_dropped_then_backend_is_reclaimed` — given active capture when owner is dropped then backend is reclaimed (`src/capture/capture_owner.rs:567`; `test-fa34e5723160d56f560f`).
- `given_active_capture_when_stopped_then_backend_is_joined` — given active capture when stopped then backend is joined (`src/capture/capture_owner.rs:540`; `test-dd4aaaf6b93ddb500769`).
- `given_panicking_capture_worker_when_joined_then_typed_failure_is_returned` — given panicking capture worker when joined then typed failure is returned (`src/capture/capture_owner.rs:610`; `test-2d873f94835a177ce436`).
- `given_prepared_capture_when_opened_then_bounded_delivery_is_owned` — given prepared capture when opened then bounded delivery is owned (`src/capture/capture_owner.rs:463`; `test-a3a0d044f02b7f664bb9`).
- `given_application_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given application mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2097`; `test-3935b20953f69bd82dab`).
- `given_capture_mode_when_channels_selected_then_microphone_is_mono_and_output_is_stereo` — given capture mode when channels selected then microphone is mono and output is stereo (`src/capture/platform/linux/pipewire.rs:1837`; `test-df5c7fa69c2c79a8f2a1`).
- `given_exhausted_capture_pool_when_acquiring_then_failure_is_observed_once` — given exhausted capture pool when acquiring then failure is observed once (`src/capture/platform/linux/pipewire.rs:1855`; `test-3002ec0fb883ffa835f6`).
- `given_negotiated_format_when_channel_count_changes_then_capture_fails_closed` — given negotiated format when channel count changes then capture fails closed (`src/capture/platform/linux/pipewire.rs:2066`; `test-0f6c4f31518ab5e8ffd8`).
- `given_process_mode_when_node_not_found_then_backend_init_error_not_system_mix` — given process mode when node not found then backend init error not system mix (`src/capture/platform/linux/pipewire.rs:2131`; `test-b704602af68d2c7a0b53`).
- `given_process_mode_when_pipewire_unavailable_then_mode_unsupported_not_system_mix` — given process mode when pipewire unavailable then mode unsupported not system mix (`src/capture/platform/linux/pipewire.rs:2083`; `test-afbc5fb711b5d1e4c0fa`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:358`; `test-de7d536ac9b0edc1d4da`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [Linux capture](/docs/platform/linux.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Platform support and evidence](/docs/platform/compatibility.md)
- [PocketStation](/README.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)

## Evidence boundary

The claims on **System capture** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/query.rs:1-1` (`DECLARED`)

For **System capture**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
