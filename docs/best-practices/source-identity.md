# Preserve source identity

<!-- claims: CLM-BEST-003-CAP-001,CLM-BEST-003-CAP-002,CLM-BEST-003-CAP-003,CLM-BEST-003-SOURCE-001 -->

## Recommendation

Retain source, stream, stem, generation, clock, sequence, and derivation identity instead of flattening frames into anonymous PCM.

## Why

The repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.

## Tradeoff

The recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.

## When it does not apply

Do not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.

## Repository evidence

- `clock_correlation` at `src/frame/audio.rs` (`pattern-0311869ce2fb5212a274`).
- `typed_error` at `src/capture/platform/macos/session_backend.rs` (`pattern-035a929706d6f2663fc7`).
- `typed_error` at `src/frame/pool.rs` (`pattern-145c92021b15c7666c35`).
- `clock_correlation` at `src/capture/platform/macos/macos_tap.rs` (`pattern-23af111759191a2f8466`).
- `bounded_queue` at `src/capture/platform/windows/windows.rs` (`pattern-309edebd4b99b7fb4fa4`).
- `buffer_pool` at `src/capture/frame_stream.rs` (`pattern-4ebf36dac7fcbed340c2`).
- `clock_correlation` at `src/frame/lineage.rs` (`pattern-503fc1d95fe068d3ac6f`).
- `typed_error` at `src/capture/platform/macos/loopback.rs` (`pattern-55b152cec6c363114987`).
- `buffer_pool` at `benches/capture_to_stream.rs` (`pattern-56f8584f50d6f6096f99`).
- `buffer_pool` at `src/capture/observations.rs` (`pattern-593c226a7db903f9671d`).
- `bounded_queue` at `src/capture/events.rs` (`pattern-5f72bbb5f3b23f8d98e0`).
- `typed_error` at `src/capture/authorization.rs` (`pattern-60b331af4808cc0a57a0`).
- `buffer_pool` at `src/capture/platform/macos/input.rs` (`pattern-60de980358115d0b3e5b`).
- `typed_error` at `src/capture/platform/macos/input.rs` (`pattern-62b059719a1f9dafb171`).
- `buffer_pool` at `tests/capture_hot_path_alloc.rs` (`pattern-6c7ca1b02510b89f5480`).
- `typed_error` at `src/capture/capture_owner.rs` (`pattern-759bee11ea894801e9d4`).
- `clock_correlation` at `src/capture/timeline.rs` (`pattern-85798d9f164c752449ba`).
- `clock_correlation` at `src/capture/tests.rs` (`pattern-87f80c5f5df30e1c05d6`).
- `buffer_pool` at `benches/audio_buffer_pool.rs` (`pattern-88d5d10d4c8e80149f0f`).
- `buffer_pool` at `src/frame/audio.rs` (`pattern-8c2dc4f399c8eb61d8ce`).

## Executable evidence

The following test bodies are evidence only for their recorded setup:

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

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/capture/identity.rs:1-166` (`DIRECT`)
- `src/frame/lineage.rs:1-101` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
