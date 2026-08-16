# AUDIO-035: SDK-neutral native extension library host

**Status:** Accepted correction under AUDIO-034
**Decision date:** 2026-08-16
**Decision owners:** PocketStation runtime maintainers
**Cause:** Unexpressible execution primitive

## Context

The accepted Extension ABI executes C source, operator, and endpoint callback
tables through the canonical Session engine. It did not define how a public
Rust `Session` or a language SDK imports those registrations from a packaged
dynamic library while retaining executable code for the callback lifetime.

Without a Core-owned primitive, Python and JavaScript would have to create a
second engine, invent incompatible loader and lifetime policies, or omit
compiled extensions. Those outcomes violate AUDIO-034.

## Decision

Extension ABI 1.2 adds one append-only, SDK-neutral
`pks_extension_library_v1` entrypoint and
`Session::load_native_extension_library`. This is the unexpressible-execution-
primitive correction permitted by AUDIO-034, not a provider or product-scope
expansion.

The host:

- requires a canonical absolute path and never uses ambient library search;
- validates a bounded library descriptor and registration set;
- reuses the existing descriptor, port, callback, source, operator, endpoint,
  compiler, runtime, and lifecycle authorities;
- copies registration metadata before returning to the provider;
- imports all registrations transactionally and rejects duplicate identifiers;
- retains the loaded code through exact instance and registration destruction;
- contains Rust unwind at entry, acquisition, and callback boundaries; and
- executes foreign callbacks only on existing blocking, async, and external
  partitions, never on capture callbacks or the realtime PCM lane.

Python, JavaScript, Rust clients, and future SDKs bind this Session method.
They do not own another library search policy, registry, callback lifetime, or
execution engine.

## Compatibility and evidence

Extension ABI 1.1 remains the accepted compatibility floor. Its type layouts,
field offsets, and Core-exported symbols are required as a subset by the
compatibility gate; 1.2 only appends library-host records.

A separately compiled conformance library proves source → operator → endpoint
execution in the canonical Session, exactly-once context destruction,
malformed-registration cleanup, duplicate-import atomicity, relative-path
rejection, missing-entrypoint handling, and ABI rejection. The fixture is not a
product mock. This decision does not claim sandboxing of trusted native code,
remote delivery, physical-device proof, language-SDK parity, novelty, or
superiority.
