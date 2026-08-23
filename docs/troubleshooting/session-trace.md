# Session trace validation fails

<!-- claims: CLM-TRBL-017-CAP-001,CLM-TRBL-017-CAP-002,CLM-TRBL-017-CAP-003,CLM-TRBL-017-SOURCE-001 -->

## Symptom

Session trace validation returns a structural, identity, ordering, or terminal-record error.

## Evidenced causes

- A record is out of order or references an inconsistent Session or component ID.
- A required metrics snapshot or terminal record is missing.
- The trace is truncated or belongs to a different run.

## Distinguish the causes

Use the validation error and record index to inspect the first invalid transition, then compare stable IDs and terminal state.

## Diagnostic signals

- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` (`error-848c45be5f51a946a540`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `IncompleteTrace` (`error-25f6cc4c2eb526799c56`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidChecksum` (`error-b072b76ca0d38c96e1d1`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` (`error-593cb18777dd2057bdbf`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLifecycleTransition` (`error-9690e2f3b754faadcd88`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidMagic` (`error-836f7498bf3b02195d18`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `Io` (`error-f8b39f4bd37df3de8588`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `MissingTerminal` (`error-ec2f87e710a1d7377cf2`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `RecordAfterTerminal` (`error-19306e8b303a03a07dcf`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SequenceGap` (`error-09749855b5e930dd5279`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SessionMismatch` (`error-5e9e3fcb261c37238b9e`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `TerminalMismatch` (`error-6f5c5b09a57ad2c3ecfb`)

## Executable evidence

- `given_invalid_lifecycle_when_validated_then_validation_fails_closed` exercises given invalid lifecycle when validated then validation fails closed under its recorded setup (`test-ad4fc9ea8d172cd4b678`).
- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` exercises given complete trace when validated then lifecycle and terminal match under its recorded setup (`test-9dd947375f94d2a4f21f`).
- `given_dropped_records_when_validated_then_trace_is_incomplete` exercises given dropped records when validated then trace is incomplete under its recorded setup (`test-7647b415f36fc0ddd5a0`).
- `given_existing_output_when_started_then_recorder_fails_closed` exercises given existing output when started then recorder fails closed under its recorded setup (`test-3b4ddbb231cb8f0182e8`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` exercises given record after terminal when validated then trace is rejected under its recorded setup (`test-13dde755fb2a02648705`).
- `given_sequence_gap_when_validated_then_trace_is_rejected` exercises given sequence gap when validated then trace is rejected under its recorded setup (`test-5863853c4bbea9e5db76`).
- `given_timestamp_regression_when_validated_then_trace_is_rejected` exercises given timestamp regression when validated then trace is rejected under its recorded setup (`test-3d881d801a5b93012bc6`).
- `given_truncated_trace_when_read_then_truncation_is_rejected` exercises given truncated trace when read then truncation is rejected under its recorded setup (`test-f5a25488cf2eb39c2921`).
- `given_zero_capacity_when_started_then_recorder_fails_closed` exercises given zero capacity when started then recorder fails closed under its recorded setup (`test-fcc195708afeec03a930`).
- `given_corrupted_record_when_read_then_checksum_is_rejected` exercises given corrupted record when read then checksum is rejected under its recorded setup (`test-9c9f0e0f6d8934622867`).
- `given_unknown_version_when_read_then_version_is_rejected` exercises given unknown version when read then version is rejected under its recorded setup (`test-8ba8bc1dda2af18cb6c5`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-1633b6167eec91db04e2`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` exercises given derived stream without destination when frozen then validation fails closed under its recorded setup (`test-94e7fa143670693acd86`).
- `given_public_facade_when_session_trace_enabled_then_trace_replays_complete_lifecycle` exercises given public facade when session trace enabled then trace replays complete lifecycle under its recorded setup (`test-17d9667bcb3d339c7157`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` exercises given cloned stem when session frozen then mutation is rejected under its recorded setup (`test-1682e00b3166c4846a92`).

## Corrective action

Fix the recorder or collection workflow and produce a new trace; retain the invalid trace as failure evidence.

## Retry and incomplete state

Validation is deterministic for the artifact, so retrying unchanged bytes cannot repair it. The trace may be incomplete and must not be rewritten as observed history.

## Related reference

- [Session Traces](/docs/concepts/session-traces.md)
- [Lifecycle Evidence](/docs/reference/lifecycle-evidence.md)

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Observation API](/docs/reference/observations.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)

## Evidence boundary

The claims on **Session trace validation fails** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/trace.rs:1-1179` (`DIRECT`)

For **Session trace validation fails**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
