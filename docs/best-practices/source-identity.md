# Preserve source identity

<!-- claims: CLM-BEST-003-CAP-001,CLM-BEST-003-CAP-002,CLM-BEST-003-CAP-003,CLM-BEST-003-SOURCE-001 -->

## Problem

Flattening frames into anonymous PCM removes the evidence needed to distinguish sources, generations, routes, and discontinuities.

## Recommendation

Preserve Session, source, stem, stream, route, clock, sequence, generation, and derivation identity through every transformation.

## Reason

Lineage lets recordings, operators, and diagnostics attribute output to the exact upstream source state.

## Tradeoff

Retaining metadata costs storage and requires transformations to propagate it correctly.

## When it does not apply

A final export format may omit internal identity when that loss is explicit and no later diagnostic depends on it.

## Repository evidence

- `clock_correlation` at `src/frame/lineage.rs` (`pattern-503fc1d95fe068d3ac6f`).

## Executable evidence

Executable evidence selected for **Preserve source identity** is limited to each test's recorded setup and assertions:

- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-a8dbef4f3b61c752ce0e`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-50620fcc9117c7ad3cf6`).
- `given_exact_application_target_when_framed_then_stable_identity_is_preserved` — given exact application target when framed then stable identity is preserved (`src/capture/platform/macos/macos_tap.rs:748`; `test-55d1d56a08dae220e1d4`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:174`; `test-39fa4a1bc5fb034e360f`).
- `given_native_source_gap_when_advanced_then_gap_is_preserved_once` — given native source gap when advanced then gap is preserved once (`src/capture/tests.rs:295`; `test-6f92449ae2068cad145e`).
- `given_source_unavailable_error_when_displayed_then_stable_identity_is_retained` — given source unavailable error when displayed then stable identity is retained (`src/capture/tests.rs:256`; `test-d9515c41464fa15374fd`).
- `given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity` — given stable input selector when wrapped then capture mode preserves identity (`src/capture/tests.rs:202`; `test-074f0bb3502ed6267879`).
- `given_available_capacity_when_frame_is_sent_then_stream_preserves_frame` — given available capacity when frame is sent then stream preserves frame (`src/capture/frame_stream.rs:234`; `test-82e1dcd18071b5ef2f92`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1a09c0b9480a09c36429`).
- `given_pipewire_application_metadata_when_identity_is_derived_then_persistent_fields_win` — given pipewire application metadata when identity is derived then persistent fields win (`src/capture/platform/linux/pipewire.rs:1907`; `test-e2539685a9c80008839f`).
- `given_process_scoped_exact_selector_when_identity_is_transient_then_matching_pid_is_allowed` — given process scoped exact selector when identity is transient then matching pid is allowed (`src/capture/platform/linux/pipewire.rs:1980`; `test-281d496a5f325c196fe0`).
- `given_capture_before_callback_when_mapped_then_process_timestamp_preserves_delay` — given capture before callback when mapped then process timestamp preserves delay (`src/capture/platform/macos/input.rs:351`; `test-8a2ea38f6f2c1b3ffa2f`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A capture source disappears](/docs/troubleshooting/source-loss.md)
- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Capture system audio](/docs/how-to/capture-system-audio.md)
- [Select a process-scoped source](/docs/how-to/select-process-source.md)

## Evidence boundary

The claims on **Preserve source identity** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/identity.rs:1-166` (`DIRECT`)
- `src/frame/lineage.rs:1-101` (`DIRECT`)

For **Preserve source identity**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
