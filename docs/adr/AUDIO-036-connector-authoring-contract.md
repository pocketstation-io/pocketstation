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
ConnectorManifest + ConnectorFactory + ConnectorWorker
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
- a worker adapter that contains panics, supervises the startup-readiness
  deadline, provides a stop token, and reports through the Endpoint SPI; and
- a feature-gated connector conformance protocol and deterministic Core tests.

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

The old public `Session::connector` plus `register_connector_driver` path is
retained only for 1.x source compatibility and is deprecated. New connector
packages use `Connector` and `Session::register_connector`; advanced generic
Endpoint extensions use `Session::register_endpoint` directly.

## Package ownership

Core defines what a Connector means. Concrete providers remain separate.
PocketStation Relay is the `pocketstation-relay` crate in the dedicated
`pocketstation-io/connectors` repository. Future LiveKit, WHIP, process, or
customer implementations use the same contract without adding provider enums
or provider dependencies to Core.

Rust packages implement `ConnectorFactory` and `ConnectorWorker`. Python,
JavaScript, and other managed SDKs consume packaged native connectors or use a
supported bounded sidecar/native-extension boundary. Managed callbacks never
run on capture callbacks or realtime partitions. The current native extension
ABI remains typed-signal-only; this decision does not claim arbitrary dynamic
raw-PCM connector authoring from Python or JavaScript.

## Evidence and boundaries

Core tests execute the public registration and declaration path through the
canonical deterministic Session. They cover configuration rejection, secret
redaction, duplicate identity, preparation rollback and cancellation, grouped
application-plus-microphone ownership, service-status separation, startup
readiness, saturation accounting, stop and joined shutdown, terminal worker
failure, and panic containment. Source-boundary tests reject downward Endpoint
dependencies and reintroduction of duplicated Connector policy.

The design follows inspect-before-run and capability principles from
GStreamer, typed sensitive configuration principles from Kafka `ConfigDef`,
finite queue/timeout principles from OpenTelemetry Collector, separate
readiness and liveness concepts from Kubernetes, joined task ownership from
Tokio, C-compatible dynamic-library boundaries from the Rust Reference, and
provider-owned transport setup from WHIP RFC 9725.

This decision does not implement Relay, LiveKit, WHIP, or another provider in
Core. It does not claim universal connector compatibility, remote delivery,
physical-device proof, or performance superiority.
