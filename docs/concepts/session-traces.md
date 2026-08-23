# Session traces

<!-- claims: CLM-DOC-031-CAP-001,CLM-DOC-031-SOURCE-001 -->

## What it is

A Session trace is a persisted sequence of versioned lifecycle records that can be checked for identity, ordering, and terminal consistency.

## Why it exists

Runtime logs may be unstructured or incomplete. A trace preserves a machine-readable diagnostic artifact tied to stable component and Session identities.

## Relationships

- The running Session records lifecycle events into the trace owner.
- Stop finalizes the trace separately from recording output.
- Trace validation returns the failing record and structural reason.

## Invariants and guarantees

- Record order and identity must satisfy the trace format.
- A terminal trace includes the required terminal record.
- Validation does not rewrite malformed evidence to make it pass.

## When you encounter it

- **Diagnose a running Session** — Correlate events, metrics, trace records, stable error codes, and terminal outcomes.

## Use it

- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Session trace validation fails](/docs/troubleshooting/session-trace.md)
- [Lifecycle evidence reference](/docs/reference/lifecycle-evidence.md)

## Scope

- **Record and validate Session traces.** Persist lifecycle trace records and validate their structural and terminal consistency.

The scope of **Session traces** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Key API

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::session::lifecycle::trace::SessionTrace` | struct | Contains the ordered lifecycle records read from a Session trace artifact. | `src/session/lifecycle/trace.rs:255` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecord` | struct | Records one immutable session trace observation. | `src/session/lifecycle/trace.rs:55` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorder` | struct | Collects ordered lifecycle records and writes the trace artifact during Session finalization. | `src/session/lifecycle/trace.rs:152` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderHandle` | struct | Owns bounded access to session trace recorder. | `src/session/lifecycle/trace.rs:108` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderOutcome` | struct | Reports the structured session trace recorder outcome. | `src/session/lifecycle/trace.rs:70` |
| `pocketstation::session::lifecycle::trace::SessionTraceTerminal` | struct | Records the terminal Session disposition and component failures stored in a trace. | `src/session/lifecycle/trace.rs:339` |
| `pocketstation::session::lifecycle::trace::SessionTraceValidation` | struct | Reports the validated identity and record count of a parsed Session trace. | `src/session/lifecycle/trace.rs:348` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecordKind` | enum | Selects the session trace record kind used by PocketStation. | `src/session/lifecycle/trace.rs:27` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderFinishError` | enum | Classifies failures reported as session trace recorder finish error. | `src/session/lifecycle/trace.rs:98` |
| `pocketstation::session::lifecycle::trace::SessionTraceRecorderStartError` | enum | Classifies failures reported as session trace recorder start error. | `src/session/lifecycle/trace.rs:88` |

## Executable evidence

Executable evidence selected for **Session traces** is limited to each test's recorded setup and assertions:

- `given_complete_trace_when_validated_then_lifecycle_and_terminal_match` — given complete trace when validated then lifecycle and terminal match (`src/session/lifecycle/trace.rs:988`; `test-9dd947375f94d2a4f21f`).
- `given_corrupted_record_when_read_then_checksum_is_rejected` — given corrupted record when read then checksum is rejected (`src/session/lifecycle/trace.rs:1012`; `test-9c9f0e0f6d8934622867`).
- `given_dropped_records_when_validated_then_trace_is_incomplete` — given dropped records when validated then trace is incomplete (`src/session/lifecycle/trace.rs:1115`; `test-7647b415f36fc0ddd5a0`).
- `given_existing_output_when_started_then_recorder_fails_closed` — given existing output when started then recorder fails closed (`src/session/lifecycle/trace.rs:1090`; `test-3b4ddbb231cb8f0182e8`).
- `given_invalid_lifecycle_when_validated_then_validation_fails_closed` — given invalid lifecycle when validated then validation fails closed (`src/session/lifecycle/trace.rs:1057`; `test-ad4fc9ea8d172cd4b678`).
- `given_record_after_terminal_when_validated_then_trace_is_rejected` — given record after terminal when validated then trace is rejected (`src/session/lifecycle/trace.rs:1160`; `test-13dde755fb2a02648705`).
- `given_sequence_gap_when_validated_then_trace_is_rejected` — given sequence gap when validated then trace is rejected (`src/session/lifecycle/trace.rs:1136`; `test-5863853c4bbea9e5db76`).
- `given_timestamp_regression_when_validated_then_trace_is_rejected` — given timestamp regression when validated then trace is rejected (`src/session/lifecycle/trace.rs:1148`; `test-3d881d801a5b93012bc6`).
- `given_truncated_trace_when_read_then_truncation_is_rejected` — given truncated trace when read then truncation is rejected (`src/session/lifecycle/trace.rs:1027`; `test-f5a25488cf2eb39c2921`).
- `given_unknown_version_when_read_then_version_is_rejected` — given unknown version when read then version is rejected (`src/session/lifecycle/trace.rs:1042`; `test-8ba8bc1dda2af18cb6c5`).
- `given_zero_capacity_when_started_then_recorder_fails_closed` — given zero capacity when started then recorder fails closed (`src/session/lifecycle/trace.rs:1103`; `test-fcc195708afeec03a930`).
- `given_cloned_stem_when_session_frozen_then_mutation_is_rejected` — given cloned stem when session frozen then mutation is rejected (`src/session/declaration/draft.rs:1263`; `test-1682e00b3166c4846a92`).

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Observation API](/docs/reference/observations.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session trace validation fails](/docs/troubleshooting/session-trace.md)

## Evidence boundary

The claims on **Session traces** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/session/lifecycle/trace.rs:1-1179` (`DIRECT`)

For **Session traces**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
