# AUDIO-036: Provider-neutral connector authoring contract

**Status:** Accepted correction under AUDIO-034  
**Decision date:** 2026-08-17  
**Decision owners:** PocketStation runtime maintainers  
**Cause:** Missing authoring convention over the existing Endpoint SPI

## Context

Session, Graph, and Endpoint already own the runtime contract:

```text
Session / Graph
  declaration · identity · EdgeContract · routing · compilation

Endpoint
  prepare · rollback · closed start gate · stop request · joined finalization

Session extensions
  external Source · Operator · Endpoint registration and loading
```

Provider packages could implement `EndpointDriverFactory`, but every package
had to reconstruct configuration, readiness, error classification,
observations, worker containment, and conformance conventions. The first
Connector draft corrected discovery but went too far: it duplicated queue,
retry, lifecycle, delivery counters, and Session registration concepts that
Core already owned.

## Decision

`pocketstation::connector` is a thin provider-integration authoring layer. It
does not define another execution engine:

```text
ConnectorManifest + ConnectorDriverFactory + ConnectorDriver
                              ↓
                    private Endpoint adapter
                              ↓
                   Session::register_endpoint
                              ↓
            canonical prepare / gate / stop / join transaction
```

Connector owns only:

- typed finite provider configuration and redacted secrets;
- a focused manifest built from the existing `NodeDescriptor`, `PortSpec`,
  `SignalSpec`, `MediaCaps`, and `EdgeContract`;
- stable connector error codes, provider stage, and retryability
  classification;
- orthogonal provider-service facts: delivery readiness, health, and recovery;
- connector-specific retry, reconnect, failure, and last-error observations;
- a connector driver adapter that owns bounded receiver polling, fair delivery,
  drain versus abort, accounting, panic containment, startup-readiness
  supervision, and joined finalization through the Endpoint SPI;
- a lower-level `ConnectorFactory` escape hatch for integrations that must own
  a specialized off-realtime worker; and
- feature-gated executable Session fixtures and deterministic Core tests.

Connector explicitly does not own:

- Session registration or registry locks;
- route capacity, loss, or backpressure policy beyond the existing
  `EdgeContract`;
- generic received, delivered, dropped, discontinuity, or finalization
  counters already owned by `EndpointDriverObservations` and Session metrics;
- a second lifecycle enum;
- a declarative retry policy that Core does not execute; or
- provider protocols, codecs, credentials, network clients, or reconnection
  decisions.

`ConnectorPackage` composes existing Source, Operator, and Connector
registrations under one inspectable package identity. Installation preflights
all existing Session registration authorities before committing any component.
It owns no registry, graph, queue, or lifecycle of its own. Component manifests
remain authoritative for configuration; there is no unused package-level
configuration schema.

`ConnectorServiceStatus` is not a process lifecycle. Its three axes answer
separate operational questions:

```text
delivery_readiness  NotReady | Ready
health              Healthy  | Degraded
recovery            Idle     | Reconnecting
```

Endpoint finalization is the authority for terminal success or failure.
Session is the authority for starting and stopping. A connector therefore does
not expose `Starting`, `Stopping`, `Stopped`, or `Failed` as competing state.
Session conveys endpoint shutdown intent through the existing Endpoint
lifecycle: `stop` requests `EndpointShutdownMode::Drain`, while `cancel`
requests `EndpointShutdownMode::Abort`. The connector stop token preserves
that distinction, permits abort to upgrade an earlier drain request, and never
downgrades abort to drain. Existing Endpoint drivers that implement only
`request_stop` retain their 1.x behavior through the default adapter.

The old public `Session::connector` plus `register_connector_driver` path is
retained only for 1.x source compatibility and is deprecated. New connector
packages use `Connector` and `Session::register_connector`; advanced generic
Endpoint extensions use `Session::register_endpoint` directly.

## Package ownership

Core defines what a Connector means. Protocol defines language-neutral Relay
wire semantics and conformance vectors. One externally packaged Rust
implementation owns the PocketStation Relay protocol behavior:

```text
pocketstation-io/connectors/relay  authoritative pocketstation-relay package
pks                              thin CLI consumer
Python SDK                       PyO3 projection and Python API
JavaScript SDK                   Node-API projection and TypeScript API
benchmark                        measurement consumer only
```

The shared implementation is not provider code inside Core and is not owned by
the CLI. SDKs own their public APIs, native packaging, cancellation, typing,
and installed-consumer proof while projecting the same qualified native Relay
implementation. They do not duplicate signaling, ICE/DTLS, packetization, or
media delivery engines. Cross-language protocol conformance remains mandatory
for any future independent implementation.

Rust implementations normally use `ConnectorDriverFactory` and
`ConnectorDriver`; advanced integrations may use `ConnectorFactory` and
`ConnectorWorker`. Connector driver callbacks never run on capture callbacks or
realtime partitions. The current
native extension ABI remains typed-signal-only; this decision does not claim
arbitrary dynamic raw-PCM connector authoring from Python or JavaScript.
Third-party any-language audio connector authoring requires a separately
versioned native-audio or managed-audio boundary and conformance proof.

## Evidence and boundaries

Core tests execute the public registration and declaration path through the
canonical deterministic Session. They cover configuration rejection, secret
redaction and sensitive-value destruction, duplicate identity, preparation
rollback and cancellation, grouped
application-plus-microphone ownership, service-status separation, startup
readiness, Core-owned driver delivery, per-route edge authority, saturation
accounting, structured terminal errors, stop and joined shutdown, terminal
worker failure, panic containment, and distinct drain/abort delivery. Source-boundary
tests reject downward Endpoint dependencies and reintroduction of duplicated
Connector policy.

The design follows inspect-before-run and capability principles from
GStreamer, typed sensitive configuration principles from Kafka `ConfigDef`,
finite queue/timeout principles from OpenTelemetry Collector, separate
readiness and liveness concepts from Kubernetes, joined task ownership from
Tokio, C-compatible dynamic-library boundaries from the Rust Reference, and
provider-owned transport setup from WHIP RFC 9725.

This decision does not implement Relay, LiveKit, WHIP, or another provider in
Core. It does not claim universal connector compatibility, remote delivery,
physical-device proof, or performance superiority.
