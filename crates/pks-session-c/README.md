# pks-session-c

Portable C projection for the canonical `pks-session` engine.

This crate is a sibling adapter. It does not own Session semantics, media
runtime behavior, capture implementations, or endpoint workers. It owns only
the versioned C ABI records, opaque generational handles, panic containment,
header packaging, and C-facing conformance fixtures.

Current status:

- the checked-in ABI and header expose versioned records, opaque generational
  engine, Session, and audio-batch handles, and the narrow application plus
  microphone declaration;
- the adapter drives the real native `SessionEngineHost` through declaration,
  compile, start, stop, event polling, metric polling, bounded audio leases,
  release, and destruction;
- versioned Rust-record and C-header layout parity tests cover every public
  record, and output pointers are validated before an operation can acquire a
  runtime resource;
- the default C executable proves the typed missing-source failure path,
  foreign and stale handle rejection, and engine recovery;
- `scripts/test-session-c-conformance.sh` builds the crate with the
  `conformance-fixtures` feature and compiles a separate C executable. That
  executable proves a successful application-plus-microphone canonical
  runtime, two distinct source/stem lineages, bounded lease-exhaustion
  metrics, stable leased samples across Session stop, stale double-release
  rejection, and recovery after a contained panic;
- fixture-only exports are feature-gated. The default library and public
  header do not expose conformance symbols.

ABI version 1 intentionally supports exactly one Session during an engine's
lifetime. The engine-scoped polled-audio receipt is therefore isolated by the
ABI contract rather than shared across concurrently reusable Session slots.

W11 remains `PARTIAL` and this boundary is `SAFE-TO-TEST`, not accepted. A
foreign stop can now cancel a concurrently starting Session through a shared
token that does not wait for the start-held runtime mutex. The C lifecycle
reports `Starting` and `Stopping` explicitly, and the bounded cancellation
test requires both calls to finish after the blocked backend open is released.

The C metrics projection delegates to the authoritative
`pks_session::SessionMetricsSnapshot`. The aggregate record reports the
bounded Session event queue, the ABI's one engine-scoped polled-audio
endpoint. ABI 1.1 keeps that aggregate record byte-compatible with ABI 1.0;
the new `pks_session_source_metrics_count` and
`pks_session_route_metrics_count` functions expose bounded record counts
without extending the old output buffer. A compiled ABI 1.0 C canary proves
the unchanged 160-byte record is not overrun. Callers copy an indexed source
or route record with `pks_session_source_metrics_at` and
`pks_session_route_metrics_at`; an invalid index returns
`PKS_SESSION_STATUS_INDEX_OUT_OF_RANGE`.

Each source record carries its stable stem ID, backend-owned capture
observations, captured-stream delivery/drop observations, runtime-event
observations, and source-ingress queue capacity/depth/peak plus delivery and
discard totals. Each route record carries stable route and endpoint IDs, the
complete authoritative runtime-edge snapshot, endpoint delivery/failure
observations, an explicit `UNAVAILABLE`, `LIVE`, or `FINALIZED` stage, and
endpoint-finalization failure count. Endpoint counters are authoritative only
for `LIVE` or `FINALIZED`; `UNAVAILABLE` prevents a defensive lookup miss from
masquerading as synthetic live zeroes.

`RunningSession` retains only cloneable read-only receipts established during
setup. The adapter never polls moved owners, reaches into capture callbacks,
duplicates counter ownership, or exposes mutation. Final endpoint
observations remain readable until the Session handle is destroyed; afterward
indexed access returns `PKS_SESSION_STATUS_STALE_HANDLE`.
