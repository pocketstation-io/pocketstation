# PocketStation Core 1.0 Rust API allowlist

Status: Core 1.0 reviewed baseline. This file defines the intentional normal-build
Rust surface. `internal-testing` is not a product API and is excluded.

## One-engine rule

`Session` owns declaration, compilation, preparation, execution, observations,
and shutdown. Rust's `Stream<T>` is compile-time façade metadata; `T` never
becomes runtime storage or ABI identity. C, managed SDKs, and sidecars project
the same engine through stable signal/schema identities.

## Allowed root authorities

- Lifecycle: `Session`, `SessionBuilder`, `RunningSession`, start/cancel/stop
  results, stable error codes, events, observations, and recording outcomes.
- Declaration: selectors, sources, stream handles, `Stream<T>`, operators,
  endpoint descriptors, named ports/connections, and checked identity newtypes.
  Numeric engine identities expose `new`/`get`; their tuple representation is
  private so external code cannot depend on storage fields.
- Signal contract: `SignalSpec`, `SignalEnvelope`, payload, timing, lineage,
  derivation, media capabilities, ports, edge policy, partitions, and safety.
- Source extension: `SourceTypeId`, `SourceConfiguration`, `SourceManifest`,
  `SourceFactory`, `SourceDriver`, prepare/session context, cancellation,
  emission, and their errors.
- Operator extension: `AsyncOperatorManifest`, `AsyncOperatorFactory`,
  `AsyncNode`, async preparation, policies, configuration, and their errors.
- Endpoint extension: `EndpointDriverFactory`, `EndpointPortInput`, opaque
  `EndpointAudioReceiver`, `EndpointAudioFrame`, opaque
  `EndpointSignalReceiver`, endpoint context, start gate, prepared/running
  driver traits, observations, finalization records, and errors.
- Capture extension: callback/prepared/active backend traits, bounded delivery
  producers, exact capture selectors/identity, observations, runtime events,
  and capture errors. Capture consumers, Session capture owners, and platform
  backend implementations are not public.
- Realtime audio ownership: sample format/specification, `AudioBufferPool`,
  exclusive/shared buffer handles, exclusive/shared audio frames, and frame
  lineage. These are the explicit fixed-capacity ownership contract, not a
  generic signal queue.
- Observations: immutable snapshots for capture, audio edges, signal edges,
  source ingress, operators, endpoints, generated-audio reentry, and Session
  aggregate metrics. Counter owners and worker handles are not public.
- Capability namespaces: `codec` for checked Opus encode/decode/PLC and
  `timing` for monotonic time, drift estimation, correction, and timeline
  mapping.

## Public namespaces

- `graph` is the canonical namespace for `SignalSpec`, signal/media/port
  contracts, execution partitions, safety contracts, and extension authoring
  contracts. Compiler IR, registries, runtime plans, and workers are private.
- `codec` and `timing` are canonical capability namespaces.
- Endpoint authoring contracts use their canonical crate-root names. The
  implementation namespace, registry, prepared/running engine owners, and
  start-controller internals remain private.
- `conformance` exists only with the explicit `conformance-fixtures` feature.
- `internal` exists only with `internal-testing`, is documentation-hidden, and
  is forbidden to external product consumers.

## Explicitly forbidden normal-build exports

The crate root and public namespaces must not expose:

```text
AsyncOperatorWorker / AsyncOperatorInput / named runtime outputs
RealtimePlanRunner / RealtimePlanExecutor / PlanEdgeRouter
PlanSourceSender or channel constructors
TypedEdgeFanout / TypedEdgeReceiver / typed-edge constructors
SessionEngineHost / SessionEngineBuilder / PreparedSession
SourceRegistry / SourceRuntime / PreparedSourceRuntime
EndpointDriverRegistry / PreparedEndpoint / RunningEndpoint
compiler passes, RuntimePlan builders, structural-node registration
sidecar process-host workers, channels, child-process handles, or framing I/O
capture consumers, CaptureOwner, or platform backend implementations
```

The bounded PKSS wire projection is public compatibility surface:
`SidecarMessage`, `SidecarMessageKind`, `SidecarProtocolLimits`, protocol
errors, process declaration, deadlines, state, and observations. Public
Session methods use those values; callers cannot access the process host,
queues, reader/writer workers, or lifecycle implementation. Control-message
validity remains enforced by the Session-owned host.

The source contract in `tests/public_api_boundary.rs`, external consumer
fixture, rustdoc gate, and SemVer report enforce this allowlist.

## Compatibility policy

No silent duplicate alias or compatibility namespace is retained. Operator
contracts use their canonical crate-root names. Removed pre-1.0 runtime exports
have no compatibility alias because they were implementation machinery, not
supported extension contracts.
