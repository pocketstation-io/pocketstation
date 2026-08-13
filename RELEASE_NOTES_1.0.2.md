# PocketStation 1.0.2

PocketStation 1.0.2 completes the non-prompting microphone permission preflight
on supported Windows hosts.

- Windows 10 version 1903 and newer now maps the current process' `Microphone`
  AppCapability state into PocketStation's existing `PermissionObservation`;
- the query never requests access or displays UI, and API/platform failure
  remains `NotObservable`;
- Linux preflight remains explicitly `NotObservable` because there is no single
  process-wide authority shared by PipeWire portals, direct PipeWire/ALSA,
  device ACLs, sandboxes, and containers;
- Session prepare/open remains the authoritative selected-source outcome on
  every platform;
- the Rust API, C ABI, PKSS protocol, realtime callbacks, pools, queues, codec,
  timing, graph, and Session execution semantics remain compatible with 1.0.0.
