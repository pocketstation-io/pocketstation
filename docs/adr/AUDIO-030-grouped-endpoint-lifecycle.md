# AUDIO-030 — Explicit grouped endpoint lifecycle

- **Status:** Accepted
- **Date:** 2026-07-26
- **Owners:** `pks-endpoint`, `pks-session`, `pks-nodes`

## Context

The product declares application and microphone recording as separate routes,
but the recording artifact is one Session-scoped multistem directory and one
manifest. Treating each route as an independent `RunningEndpoint` would either
create competing manifests or let one endpoint report finalization before the
other stem had joined.

Grouping every endpoint with the same operator is also unsafe. One Session may
contain multiple intentionally separate instances, and registries may be reused
across Sessions.

## Decision

Endpoint grouping is explicit and setup-time only:

```text
SessionId + OperatorId + NodeTypeId + EndpointGroupId
  -> exact declared EndpointId set
  -> EndpointDriverRegistry::prepare_batch(inputs)
  -> one PreparedEndpoint
  -> one RunningEndpoint
  -> one typed finalization outcome
```

`EndpointDriverRegistry::prepare` remains the one-input convenience path.
`prepare_batch` never discovers related endpoints and never groups by operator
or node type alone. The Session startup transaction constructs batches only
from an exact `(OperatorId, NodeTypeId, EndpointGroupId)` key inside one
`SessionSpec`.

The default `StemHandle::record` declaration writes the stable explicit group
identifier `session.multistem.default.v1` into endpoint configuration. A future
public API may allow another explicit group identifier without changing the
driver lifecycle.

All members prepare together. One prepared group starts while the shared start
gate is closed and becomes ready without consuming a receiver. The Session
opens the gate only after every capture and endpoint resource is ready.
Cancellation drops the whole pending batch. Stop and join finalize the group
once and return one authoritative result.

`MultistemEndpointCoordinator` is Session-scoped configuration passed through
normal ownership. It contains the expected Session, group, endpoint IDs, and
recorder stem configuration. It rejects partial, mixed-session, wrong-group,
duplicate, or undeclared batches. No process-global mutable coordinator is
allowed.

## Consequences

- Two recording routes can truthfully produce one directory and manifest.
- A partial prepare cannot leave a live worker or recording artifact.
- Grouping is visible in `SessionSpec` and independently testable.
- Connectors and browser endpoints continue using one-input preparation unless
  they explicitly define their own group contract.
- Concrete recorder algorithms remain in `pks-nodes`; `pks-endpoint` owns only
  lifecycle and grouping contracts.
- No provider catalog, relay behavior, background scheduler, or production
  no-op driver is introduced.
