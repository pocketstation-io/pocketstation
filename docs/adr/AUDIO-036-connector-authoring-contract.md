# AUDIO-036: Provider-neutral connector authoring contract

**Status:** Accepted correction under AUDIO-034  
**Decision date:** 2026-08-17  
**Decision owners:** PocketStation runtime maintainers  
**Cause:** Missing execution and distribution convention

## Context

Core already owns the strong endpoint transaction:

```text
compile and validate
  → prepare
  → rollback on failure
  → start behind a closed Session gate
  → deliver through bounded routes
  → request stop
  → join and finalize
```

Concrete endpoint packages could use that lifecycle, but they had no common
connector manifest, typed configuration, secret classification, delivery and
retry policy, readiness model, stable error identity, observation model, or
connector-specific conformance surface. The PocketStation Relay integration
therefore repeated integration policy that another transport would also need.

## Decision

Core adds `pocketstation::connector` as an authoring and inspection layer over
the existing endpoint runtime:

```text
ConnectorManifest + EndpointDriverFactory
                  ↓
        Session::register_connector
                  ↓
          RegisteredConnector::declare
                  ↓
  existing NodeDefinition + endpoint registry
                  ↓
 existing prepare/start-gate/stop/join transaction
```

The module provides:

- open connector identity through `OperatorId` and `NodeTypeId`;
- the existing named `PortSpec`, `SignalSpec`, and `MediaCaps` contracts;
- a versioned, typed and finite configuration schema;
- `ConnectorSecret`, whose normal debug representation is redacted;
- finite edge, worker-queue, retry, deadline and readiness policies;
- explicit `Starting`, `Ready`, `Degraded`, `Reconnecting`, `Stopping`,
  `Stopped`, and `Failed` states;
- stable open error codes, stage and retryability classification;
- saturating counters and a typed observation snapshot; and
- feature-gated deterministic Session conformance for rollback, gate
  isolation, saturation, cancellation, finalization failure and worker panic.

`Connector` does not create a worker, queue, retry loop, protocol client, or
second execution engine. The concrete package implements
`EndpointDriverFactory`; Core validates the manifest and configuration and
then uses the existing endpoint transaction unchanged.

## Package ownership

Core defines what a connector means. Provider and protocol packages remain
outside the Core crate. The first concrete package is owned by the dedicated
`pocketstation-io/connectors` repository and implements PocketStation Relay.
Future LiveKit, WHIP or customer transports use the same Core contract without
adding provider enums or dependencies to Core.

Rust packages implement the in-process factory directly. Python, JavaScript
and other managed SDKs consume a packaged native connector or configure a
bounded sidecar/native extension where its signal contract permits it. Pure
managed callbacks never execute on capture callbacks or realtime partitions.
The current native extension ABI remains typed-signal-only; this decision does
not claim that arbitrary raw-PCM connectors can be authored entirely in
Python or JavaScript.

## Evidence and boundaries

The connector tests run the public registration and declaration path through
the canonical deterministic Session. They cover typed configuration rejection,
secret redaction, duplicate identity, Session ownership, preparation rollback,
start failure, readiness transitions, route saturation, cancellation, stop,
join/finalization failure and worker-panic containment.

The design follows the inspect-before-run and capability-negotiation principles
in GStreamer, typed and sensitive configuration principles in Kafka
`ConfigDef`, finite queue/retry/timeout principles in OpenTelemetry Collector,
separate readiness and liveness semantics in Kubernetes, readiness-driven
backpressure in Tower, joined shutdown in Tokio, C-compatible dynamic-library
boundaries in the Rust Reference, and provider-owned transport setup in WHIP
RFC 9725.

This decision does not implement Relay, LiveKit, WHIP or another provider in
Core. It does not claim universal connector compatibility, remote delivery,
physical-device proof, or performance superiority.

