# Session trace validation fails

<!-- claims: CLM-TRBL-017-SCOPE-001,CLM-TRBL-017-TEXT-001,CLM-TRBL-017-TEXT-002,CLM-TRBL-017-TEXT-003,CLM-TRBL-017-TEXT-004,CLM-TRBL-017-TEXT-005,CLM-TRBL-017-TEXT-006,CLM-TRBL-017-SOURCE-001 -->

## Symptom

Session trace validation returns a structural, identity, ordering, or terminal-record error.

## Evidenced causes

- A record is out of order or references an inconsistent Session or component ID.
- A required metrics snapshot or terminal record is missing.
- The trace is truncated or belongs to a different run.

## Distinguish the causes

Use the validation error and record index to inspect the first invalid transition, then compare stable IDs and terminal state.

## Diagnostic signals

- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` (`error-0012f834ccbfd8415156`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `IncompleteTrace` (`error-1028e9754b78cc85646f`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidChecksum` (`error-66459d908e0973a0ebb1`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLayout` (`error-b83acca814549a3467cb`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidLifecycleTransition` (`error-69344284edfe0f96c688`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `InvalidMagic` (`error-c7781f4776c648f5666e`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `Io` (`error-93993312a3878dbd9adc`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `MissingTerminal` (`error-dbd5f252ed380cede3df`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `RecordAfterTerminal` (`error-fad5c3de7f85fb28a2ec`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SequenceGap` (`error-baae09088fd7d22e3991`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `SessionMismatch` (`error-956b43716e3d26753212`)
- `pocketstation::session::lifecycle::trace::SessionTraceValidationError` / `TerminalMismatch` (`error-e63b4b6b34a90d6dd6f3`)

## Executable evidence

- `given_invalid_lifecycle_when_validated_then_validation_fails_closed` exercises given invalid lifecycle when validated then validation fails closed under its recorded setup (`test-10224fcae99d563f600b`).
- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` exercises given complete trace when validated then lifecycle and terminal match under its recorded setup (`test-20c537693a458a99f6f9`).
- `given_dropped_records_when_validated_then_trace_is_incomplete` exercises given dropped records when validated then trace is incomplete under its recorded setup (`test-5ee80a5b5a1b2ce74da0`).
- `given_existing_output_when_started_then_recorder_fails_closed` exercises given existing output when started then recorder fails closed under its recorded setup (`test-dbf6e802afe21cf4b197`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` exercises given record after terminal when validated then trace is rejected under its recorded setup (`test-3d007d4a819d105d9058`).
- `given_sequence_gap_when_validated_then_trace_is_rejected` exercises given sequence gap when validated then trace is rejected under its recorded setup (`test-71add6b39193d7909de3`).
- `given_timestamp_regression_when_validated_then_trace_is_rejected` exercises given timestamp regression when validated then trace is rejected under its recorded setup (`test-22e90ca672945b18e17f`).
- `given_truncated_trace_when_read_then_truncation_is_rejected` exercises given truncated trace when read then truncation is rejected under its recorded setup (`test-293abf2537b795f160f7`).
- `given_zero_capacity_when_started_then_recorder_fails_closed` exercises given zero capacity when started then recorder fails closed under its recorded setup (`test-0ad7a1a182e71b0727e9`).
- `given_corrupted_record_when_read_then_checksum_is_rejected` exercises given corrupted record when read then checksum is rejected under its recorded setup (`test-b8839cf2e639dd3e301a`).
- `given_unknown_version_when_read_then_version_is_rejected` exercises given unknown version when read then version is rejected under its recorded setup (`test-7d5245f8ecc878c074a6`).
- `given_unrouted_stem_when_session_frozen_then_validation_fails_closed` exercises given unrouted stem when session frozen then validation fails closed under its recorded setup (`test-8e301580cdd23a244478`).
- `given_derived_stream_without_destination_when_frozen_then_validation_fails_closed` exercises given derived stream without destination when frozen then validation fails closed under its recorded setup (`test-17c702ffaf38dad01e0a`).
- `given_public_facade_when_session_trace_enabled_then_trace_replays_complete_lifecycle` exercises given public facade when session trace enabled then trace replays complete lifecycle under its recorded setup (`test-f5d1e9009c62e6cb57d5`).
- `given_graph_mismatch_when_start_fails_then_diagnostic_is_retained` exercises given graph mismatch when start fails then diagnostic is retained under its recorded setup (`test-604c0e001a7dcb5f87ae`).

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

The claims on **Session trace validation fails** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `src/session/lifecycle/trace.rs:16-16` (`DIRECT`)
- `src/session/lifecycle/trace.rs:17-17` (`DIRECT`)
- `src/session/lifecycle/trace.rs:18-18` (`DIRECT`)
- `src/session/lifecycle/trace.rs:19-19` (`DIRECT`)
- `src/session/lifecycle/trace.rs:20-20` (`DIRECT`)
- `src/session/lifecycle/trace.rs:21-21` (`DIRECT`)
- `src/session/lifecycle/trace.rs:22-22` (`DIRECT`)
- `src/session/lifecycle/trace.rs:23-23` (`DIRECT`)
- `src/session/lifecycle/trace.rs:24-24` (`DIRECT`)
- `src/session/lifecycle/trace.rs:26-26` (`DIRECT`)
- `src/session/lifecycle/trace.rs:26-26` (`DIRECT`)
- `src/session/lifecycle/trace.rs:26-26` (`DIRECT`)
- `src/session/lifecycle/trace.rs:27-52` (`DIRECT`)
- `src/session/lifecycle/trace.rs:28-30` (`DIRECT`)
- `src/session/lifecycle/trace.rs:29-29` (`DIRECT`)
- `src/session/lifecycle/trace.rs:31-33` (`DIRECT`)
- `src/session/lifecycle/trace.rs:32-32` (`DIRECT`)
- `src/session/lifecycle/trace.rs:34-38` (`DIRECT`)
- `src/session/lifecycle/trace.rs:35-35` (`DIRECT`)
- `src/session/lifecycle/trace.rs:36-36` (`DIRECT`)
- `src/session/lifecycle/trace.rs:37-37` (`DIRECT`)
- `src/session/lifecycle/trace.rs:39-41` (`DIRECT`)
- `src/session/lifecycle/trace.rs:40-40` (`DIRECT`)
- `src/session/lifecycle/trace.rs:42-44` (`DIRECT`)

For **Session trace validation fails**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
