# PocketStation 1.0.1

PocketStation 1.0.1 is a documentation and packaging correction for Core 1.0.

- docs.rs now builds the contracts-only surface on its native Linux target,
  avoiding unsupported Windows cross-compilation of bundled Opus;
- native capture remains enabled by default and can be disabled explicitly for
  documentation and contract tooling;
- the public README, quickstart, extension guide, architecture links, platform
  prerequisites, and evidence boundaries are rewritten for direct developer
  use;
- the Core runtime, Rust API, C ABI, PKSS protocol, realtime callbacks, pools,
  queues, codec behavior, and Session semantics are unchanged from 1.0.0.
