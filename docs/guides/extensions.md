# Extend PocketStation

Start with the direction media moves through your integration:

- Use a `Source` when your integration brings audio or typed events into a
  Session.
- Use an `Operator` when it transforms Session data, such as audio into a
  transcript.
- Use a `Connector` when it sends Session data to an API, transport, storage
  system, or provider.

These are the normal extension points. They preserve source identity and use
the Session's existing bounded routes, lifecycle, observations, cancellation,
and shutdown.

For example, a destination that only needs PCM can use one function:

```rust,no_run
use pocketstation::connector::Connector;

# fn publish(_: &[f32]) -> Result<(), pocketstation::connector::ConnectorError> { Ok(()) }
let connector = Connector::from_audio_fn(|frame| publish(frame.samples()))?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Use [`AudioConnector`](connectors.md#reuse-a-provider) when the destination
opens and closes a connection. Use the advanced Connector driver API only when
a distributable provider package needs typed configuration, secret fields,
named inputs, readiness reporting, or provider-specific observations.

## Choose how the integration runs

Keep the integration in a normal Rust package whenever the application can
compile it directly. Choose another option only when deployment requires it:

| Deployment requirement | Use |
|---|---|
| A C or C++ host supplies callbacks | Versioned Extension ABI in `pocketstation.h` |
| The application loads a trusted native library at runtime | `pks_extension_library_v1` |
| Provider code needs process isolation or a separately managed runtime | Bounded PKSS sidecar protocol |

A native library runs with the application's privileges. A sidecar exchanges
bounded messages with a separately managed process. Neither option creates
another Session, graph,
scheduler, recorder, or lineage model.

Generated audio returns through the bounded generated-audio Bridge. The Bridge
validates its format and frame size, assigns timing and lineage, and re-enters
the existing audio plan without invoking provider code on a capture callback.

## Connect another language

Rust uses `Stream<T>` for local type checking. C and managed languages bind
their native wrappers to stable `SignalSpec` and schema identifiers. The
sidecar protocol transports those identities and bounded payloads between
processes. Rust `TypeId` and generic parameters are never exposed through the
C API or sidecar protocol.

Test an extension from the packaged crate in a clean project. A same-process or
same-host test verifies that integration only; it does not establish behavior
on a remote network or a different physical device.

## Trusted native dynamic libraries

Extension ABI 1.2 adds a transport for the executable callback tables already
accepted by Core. A trusted library exports one `pks_extension_library_v1`
entrypoint. The entrypoint reports a bounded registration count and an acquire
callback; each acquisition returns one source, operator, or endpoint
descriptor, borrowed port records, and its callback table.

```rust,no_run
use pocketstation::Session;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let session = Session::new();
// SAFETY: deployment policy authenticated this exact library and its publisher;
// the library obeys the PocketStation Extension ABI and callback lifetimes.
let loaded = unsafe {
    session.load_native_extension_library("/opt/pocketstation/extensions/example.so")?
};
for registration in loaded.registrations() {
    println!("{} {:?}", registration.id(), registration.kind());
}
# Ok(())
# }
```

This API accepts a raw `.dylib`, `.so`, or `.dll`; it does not authenticate a
package, verify a publisher or signature, or sandbox native code. The caller's
deployment code or package installer makes that trust decision. The file
location must be absolute and is resolved before loading. PocketStation does
not search ambient DLL or shared-library directories. Every library and registration record is
ABI-checked, every descriptor and port is copied, and duplicate identifiers are
rejected before the `Session` registries change. Failure is all-or-none.

Extension ABI v1 registrations execute only on supported non-realtime
partitions. They do not provide arbitrary realtime PCM callbacks or a universal
audio Connector loader.

An OK acquisition transfers the returned `registration_context` to
PocketStation. `destroy_instance` and `destroy_registration` remain
exactly-once terminal callbacks. Core retains the dynamic-library handle in
every acquired callback adapter, so executable code cannot unload before those
terminal calls complete. Entry, acquisition, lifecycle, and payload callbacks
are unwind-contained and execute only during setup or on the existing
blocking, async, and external partitions—never on capture callbacks or the
realtime PCM lane.

Python, JavaScript, Rust clients, and future SDKs should bind the same Session
method. They must not implement their own loader search rules, callback
lifetime owner, extension registry, or execution engine. Platform wheels or
native packages remain responsible for distributing compatible library bytes.
