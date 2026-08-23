# Glossary

<!-- claims: CLM-DOC-055-CAP-001,CLM-DOC-055-CAP-002,CLM-DOC-055-CAP-003,CLM-DOC-055-CAP-004,CLM-DOC-055-CAP-005,CLM-DOC-055-CAP-006,CLM-DOC-055-CAP-007,CLM-DOC-055-CAP-008,CLM-DOC-055-CAP-009,CLM-DOC-055-CAP-010,CLM-DOC-055-CAP-011,CLM-DOC-055-CAP-012,CLM-DOC-055-CAP-013,CLM-DOC-055-CAP-014,CLM-DOC-055-CAP-015,CLM-DOC-055-CAP-016,CLM-DOC-055-CAP-017,CLM-DOC-055-CAP-018,CLM-DOC-055-CAP-019,CLM-DOC-055-CAP-020,CLM-DOC-055-CAP-021,CLM-DOC-055-CAP-022,CLM-DOC-055-CAP-023,CLM-DOC-055-CAP-024,CLM-DOC-055-CAP-025,CLM-DOC-055-CAP-026,CLM-DOC-055-CAP-027,CLM-DOC-055-CAP-028,CLM-DOC-055-CAP-029,CLM-DOC-055-CAP-030,CLM-DOC-055-CAP-031,CLM-DOC-055-CAP-032,CLM-DOC-055-CAP-033,CLM-DOC-055-SOURCE-001 -->

Use these terms consistently. Code spelling anchors a term to a compiler symbol; it does not imply that every related type uses that name.

## Terms

| Term | Code spelling | Definition | Aliases | Avoid |
|---|---|---|---|---|
| **Session** | `Session` | A declaration-time owner for sources, routes, operators, endpoints, and recording intent. | — | session runtime, context |
| **source** | `Source` | A declaration that identifies where a Session obtains a signal. | capture source | input thing |
| **stem** | `StemId` | A source-aware branch identity retained through routing and recording. | — | channel |
| **stream** | `Stream` | A declaration-time typed connection origin used to compose Session routes. | — | track |
| **lineage** | `FrameLineage` | Immutable identity and sequence metadata carried with a frame or signal. | provenance | — |
| **clock domain** | `ClockDomainId` | An identity that states which clock produced a timestamp. | — | — |
| **Session timeline** | `SessionTimelineOrigin` | The common monotonic time origin used to correlate Session work. | — | wall clock |
| **route** | `RouteId` | A compiled delivery path from an output to an independent consumer. | edge | — |
| **backpressure** | `BackpressurePolicy` | The declared response when a bounded route cannot accept work immediately. | — | — |
| **loss policy** | `LossPolicy` | The declared rule for data loss at a constrained edge. | drop policy | — |
| **copy policy** | `CopyPolicy` | The declared ownership or copying rule for an edge crossing. | — | — |
| **operator** | `AsyncOperatorFactory` | An extension that transforms or produces typed signals on the asynchronous lane. | processing stage | filter |
| **endpoint** | `EndpointDriverFactory` | A destination driver prepared and finalized through the Session lifecycle. | destination | — |
| **connector** | `ConnectorManifest` | A typed endpoint integration contract with configuration, delivery, readiness, and failure policy. | — | provider plugin |
| **native extension** | `NativeExtensionLibrary` | A compiled dynamic library that contributes versioned source, operator, or endpoint registrations. | — | plugin |
| **sidecar** | `SidecarProcessSpec` | A managed child process participating through a bounded lifecycle protocol. | — | service |
| **signal** | `SignalEnvelope` | A typed asynchronous payload with specification, timing, and lineage. | — | message |
| **audio frame** | `AudioFrame` | A pooled PCM buffer paired with sample specification and frame lineage. | frame | — |
| **buffer pool** | `AudioBufferPool` | A fixed-capacity owner that supplies reusable audio storage. | — | — |
| **audio reentry** | `SessionAudioReentryMetrics` | The bounded crossing that returns asynchronously generated PCM to the audio lane. | Bridge | — |
| **Session declaration** | `SessionSpec` | The immutable specification produced before compilation and runtime preparation. | Session spec | — |
| **compiled Session** | `CompiledSession` | A validated and resolved Session plan ready for resource preparation. | — | — |
| **prepared Session** | `PreparedSession` | A Session whose source and endpoint resources have been prepared but not started. | — | — |
| **running Session** | `RunningSession` | The runtime owner used to observe and stop active Session execution. | — | — |
| **terminal outcome** | `SessionTerminalOutcome` | The structured result of Session stop and component finalization. | — | exit status |
| **observation** | `SessionMetricsSnapshot` | A metrics, event, or trace value exposed without redefining runtime behavior. | metric | telemetry guarantee |
| **Session trace** | `SessionTrace` | A persisted sequence of lifecycle trace records with validation support. | — | log file |
| **source generation** | `SourceGeneration` | A monotonic identity revision used when an observed source changes incarnation. | — | — |
| **permission epoch** | `PermissionEpoch` | A revision associated with authorization state observation. | — | — |
| **readiness** | `ConnectorReadinessPolicy` | The connector policy governing readiness observations before delivery. | — | health |
| **retryability** | `ConnectorRetryability` | The connector failure classification that tells application policy whether the same operation may be attempted again, must wait for reconfiguration, or must not be retried. | — | retry budget, infinite retry |
| **C ABI** | `PksSessionStatus` | The versioned C representation of Session and extension operations. | — | Rust API |
| **conformance** | `conformance` | Executable evidence that a boundary matches a versioned contract under stated conditions. | — | qualification |

## Terminology conflicts

Aliases are permitted only where listed. A forbidden alias usually collapses a distinction such as source versus stem, endpoint versus connector, or wall clock versus clock domain.

## Evidence boundary

The claims on **Glossary** are anchored to Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `src/lib.rs:1-1129` (`DIRECT`)

For **Glossary**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
