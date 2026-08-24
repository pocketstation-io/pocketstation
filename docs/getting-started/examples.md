# Run the examples

<!-- claims: CLM-DOC-004-SCOPE-001,CLM-DOC-004-TEXT-001,CLM-DOC-004-TEXT-002,CLM-DOC-004-TEXT-003,CLM-DOC-004-SOURCE-001 -->

## Compile the examples

Run `cargo test --examples --all-features` for top-level examples. Run `cargo check --manifest-path examples/operator-consumer/Cargo.toml` and `cargo check --manifest-path examples/whisper-transcribe/Cargo.toml` for nested packages.

Compilation establishes API compatibility. It does not establish that capture devices, a named application, or an external transcription process is available.

## Choose an example

- `product_quickstart.rs` exercises capture, polling, stop, and recording outcomes.
- `connector_authoring.rs` declares and registers a connector without contacting its example destination.
- `operator-consumer` consumes the operator contract as a separate package.
- `whisper-transcribe` owns an external-process evidence boundary.

## Scope

- **Install and feature-select the crate.** Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.
- **Validate protocol and conformance boundaries.** Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.
- **Integrate transcription processing.** Use the repository-owned Whisper example to send captured stems to an external transcription process with evidence output.

The scope of **Run the examples** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Public entry points

No intentionally public Rust declaration is owned directly by **Run the examples**. Its contract is expressed by the linked repository, protocol, or qualification evidence instead.

## Executable evidence

Executable evidence selected for **Run the examples** is limited to each test's recorded setup and assertions:

- `given_discontinuity_change_inside_window_when_processed_then_window_is_rejected` — given discontinuity change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1400`; `test-2a5f4a8f1e04f5b220c4`).
- `given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped` — given hung provider when deadline expires then child is killed and reaped (`examples/whisper-transcribe/src/lib.rs:1108`; `test-aa794f0809b00c2d3394`).
- `given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration` — given instance timeout when manifest resolves then deadline matches configuration (`examples/whisper-transcribe/src/lib.rs:1055`; `test-899bb5750fda98d0832b`).
- `given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame` — given lineaged window when transcribed then derived range covers every frame (`examples/whisper-transcribe/src/lib.rs:1311`; `test-5978528a8ea570fad70d`).
- `given_missing_binary_when_prepare_runs_then_connector_fails_closed` — given missing binary when prepare runs then connector fails closed (`examples/whisper-transcribe/src/lib.rs:1098`; `test-bd5ed751c752083c7711`).
- `given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized` — given outer cancellation when process is active then child receipt is finalized (`examples/whisper-transcribe/src/lib.rs:1220`; `test-841a6b80171cfb0f55e8`).
- `given_permission_change_inside_window_when_processed_then_window_is_rejected` — given permission change inside window when processed then window is rejected (`examples/whisper-transcribe/src/lib.rs:1419`; `test-1601ba20883aee1ac630`).
- `given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted` — given process evidence when provider succeeds then actual invocation is persisted (`examples/whisper-transcribe/src/lib.rs:1129`; `test-004f9f3662355f6c02cc`).
- `given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted` — given process evidence when provider times out then kill and reap are persisted (`examples/whisper-transcribe/src/lib.rs:1180`; `test-2ed5fdd4ba19977c8dc9`).
- `given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset` — given source change inside window when processed then window is rejected and reset (`examples/whisper-transcribe/src/lib.rs:1379`; `test-384d43cad3cb43576f09`).
- `given_two_complete_windows_when_finished_then_partials_and_single_final_cover_stream` — given two complete windows when finished then partials and single final cover stream (`examples/whisper-transcribe/src/lib.rs:1338`; `test-46ae6f451f003a166202`).
- `given_typed_audio_when_window_fills_then_partial_precedes_one_final_transcript` — given typed audio when window fills then partial precedes one final transcript (`examples/whisper-transcribe/src/lib.rs:1263`; `test-3d1cc0ecef9a89cf23ff`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Feature flags](/docs/reference/features.md)
- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Cargo features and build surfaces](/docs/concepts/cargo-features.md)
- [Conformance and qualification](/docs/concepts/conformance.md)

## Evidence boundary

The claims on **Run the examples** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `Cargo.toml:1-21` (`DIRECT`)
- `examples/product_quickstart.rs:1-21` (`DIRECT`)

For **Run the examples**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
