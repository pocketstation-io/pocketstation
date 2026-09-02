# macOS fallback capture plugin

## Ownership

PocketStation uses the public Core Audio process-tap backend on macOS 14.4 and
later. The AudioServerPlugin files in this directory provide an optional
fallback for older systems and for a separately provisioned virtual device.

The crate build compiles:

- `source_discovery.m` for process-tap discovery/capture;
- `authorization.m` for microphone authorization state;
- `shm_reader.c` for the versioned, single-consumer ASP ring;
- `Plugin.cpp` into an unsigned `PocketStationLoopback.driver` build artifact
  only when `macos-asp-driver-artifact` is explicitly enabled.

There is no legacy `asp` Cargo feature and no compiled fallback stub. The SDK never
installs the driver, restarts `coreaudiod`, or silently reports a missing
driver as available. Signing and installation remain explicit release or
operator actions outside the SDK.

To produce the unsigned bundle for an explicit provisioning workflow:

```sh
cargo build --release --features macos-asp-driver-artifact
```

## Realtime rules

`Plugin.cpp::pks_DoIOOperation` and the ring write are allocation,
lock, blocking, async, logging, and panic free. Reader allocation and native
source discovery occur during setup, not on Core Audio callbacks. Ring
saturation and invalid native timelines are rejected and counted explicitly.

## Files

| File | Purpose |
|------|---------|
| `bridge.h` | Stable C API for the shared-memory reader |
| `shm_reader.c` | Versioned single-consumer bounded ring reader |
| `SharedRing.h` | Shared ABI and lock-free bounded publication rules |
| `source_discovery.m` | Core Audio process-tap discovery and lifecycle |
| `authorization.m` | Native authorization query |
| `Plugin.cpp` | Direct AudioServerPlugin implementation; no libASPL dependency |
| `Info.plist` | Driver bundle metadata |
| `README.md` | This file |
