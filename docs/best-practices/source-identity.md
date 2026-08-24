# Preserve source identity

<!-- claims: CLM-BEST-003-SCOPE-001,CLM-BEST-003-TEXT-001,CLM-BEST-003-TEXT-002,CLM-BEST-003-TEXT-003,CLM-BEST-003-TEXT-004,CLM-BEST-003-TEXT-005,CLM-BEST-003-SOURCE-001 -->

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

- `given_backend_frame_when_source_differs_from_open_identity_then_lineage_fails_closed` — given backend frame when source differs from open identity then lineage fails closed (`src/capture/capture_owner.rs:511`; `test-805d755d4acd2257ba9b`).
- `given_missing_exact_source_when_classified_then_stable_key_is_preserved` — given missing exact source when classified then stable key is preserved (`src/capture/platform/linux/pipewire.rs:1894`; `test-d288558b68fc54333e50`).
- `given_exact_application_target_when_framed_then_stable_identity_is_preserved` — given exact application target when framed then stable identity is preserved (`src/capture/platform/macos/macos_tap.rs:748`; `test-2a85422b1f2de56f1698`).
- `given_canonical_capture_identity_when_derived_then_source_id_matches_stable_vector` — given canonical capture identity when derived then source id matches stable vector (`src/capture/tests.rs:183`; `test-9c549d91f364bb436c12`).
- `given_native_source_gap_when_advanced_then_gap_is_preserved_once` — given native source gap when advanced then gap is preserved once (`src/capture/tests.rs:304`; `test-67cd94cce881395f7a8f`).
- `given_source_unavailable_error_when_displayed_then_stable_identity_is_retained` — given source unavailable error when displayed then stable identity is retained (`src/capture/tests.rs:265`; `test-8f3a284602ceedaf956f`).
- `given_stable_input_selector_when_wrapped_then_capture_mode_preserves_identity` — given stable input selector when wrapped then capture mode preserves identity (`src/capture/tests.rs:211`; `test-a213feaa87257702636d`).
- `given_two_audio_inputs_on_one_many_port_when_run_then_each_source_lineage_is_preserved` — given two audio inputs on one many port when run then each source lineage is preserved (`tests/audio_input.rs:351`; `test-a451e8ad08df8f006452`).
- `given_available_capacity_when_frame_is_sent_then_stream_preserves_frame` — given available capacity when frame is sent then stream preserves frame (`src/capture/frame_stream.rs:234`; `test-8f4bb6c6c11e1d2947a7`).
- `given_exact_application_selector_when_identity_is_transient_then_selection_fails_closed` — given exact application selector when identity is transient then selection fails closed (`src/capture/platform/linux/pipewire.rs:1962`; `test-1e40dd4ec9e96cd35eb7`).
- `given_pipewire_application_metadata_when_identity_is_derived_then_persistent_fields_win` — given pipewire application metadata when identity is derived then persistent fields win (`src/capture/platform/linux/pipewire.rs:1907`; `test-964084de3faa5b449071`).
- `given_process_scoped_exact_selector_when_identity_is_transient_then_matching_pid_is_allowed` — given process scoped exact selector when identity is transient then matching pid is allowed (`src/capture/platform/linux/pipewire.rs:1980`; `test-575dc7b197243c56d8f1`).

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

The claims on **Preserve source identity** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/capture/identity.rs:1-1` (`DECLARED`)
- `src/frame/lineage.rs:1-1` (`DECLARED`)

For **Preserve source identity**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
