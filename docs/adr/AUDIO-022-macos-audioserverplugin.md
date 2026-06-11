# AUDIO-022 — macOS AudioServerPlugin (ASP) for System Audio Capture

**Status:** Accepted  
**Date:** 2026-06-10  
**Deciders:** Raphael Avocegamou  
**Related:** AUDIO-011 (SPSC ring), AUDIO-020 (capture benchmark)

---

## Context

PocketStation's macOS loopback backend uses ScreenCaptureKit (SCKit) for
system audio capture.  SCKit has a ~10 ms scheduling floor imposed by
WindowServer's display refresh cycle, even for audio-only streams.

For professional use cases requiring sub-5 ms capture latency, macOS
provides the AudioServerPlugin (ASP) API.  An ASP is a HAL plug-in loaded
by `coreaudiod`.  It installs a virtual output device; audio written to that
device is available to the host process at CoreAudio buffer sizes (typically
256 or 512 frames at 48 kHz = 5–10 ms, configurable down to 64 frames
= ~1.3 ms).

libASPL (<https://github.com/appleasp/libASPL>) is a C++17 helper library
that wraps the low-level `AudioServerPlugIn.h` interface.

## Decision

Wave A of the loopback improvement ships:

1. A `CaptureMode` enum (`SystemMix`, `Application(bundle_id)`, `Process(u32)`)
   that selects the capture source at the API level.
2. `capture_with_mode()` as the primary entry point; `capture_system_audio()`
   becomes a thin wrapper for `SystemMix`.
3. A per-app SCKit path using `SCContentFilter.with_including_applications`
   for `Application(bundle_id)` on macOS.
4. `with_excludes_current_process_audio(true)` on all SCKit streams to
   prevent the CLI capture loop.
5. An `asp` Cargo feature (off by default) that compiles `asp/Plugin.cpp`
   with libASPL when the `vendor/libASPL` submodule is present.
6. A `pks_asp_is_installed()` C bridge stub compiled unconditionally so
   `asp_is_installed()` is always callable regardless of the feature flag.
7. `macos_asp::asp_is_installed()` as a public Rust API for runtime
   plugin presence detection.

The libASPL **submodule add is a human operator step** and is not performed
by the agent.  All code compiles and all tests pass without it.

## Alternatives considered

| Option | Rejected because |
|--------|-----------------|
| CoreAudio tap API (macOS 14.2+) | No Rust crate; requires Swift bridging; macOS 14.2 minimum deployment target not yet set |
| Soundflower / BlackHole dependency | Third-party kernel extension; not suitable for embedded distribution |
| Only ScreenCaptureKit | 10 ms floor unacceptable for sub-5 ms targets; Application-mode capture needed for per-app routing |

## Consequences

- The `asp` feature is permanently off by default; CI does not require libASPL.
- Wave B/C (Windows WASAPI, Linux PipeWire) use `CaptureMode` from this ADR.
- When the ASP plugin is installed, callers can query `asp_is_installed()`
  and route capture accordingly (future Wave D work).
- Deployment of the ASP plugin requires a signed HAL plugin + `coreaudiod`
  restart; this is a separate operational procedure not automated here.

## Phase

Phase 3 (Wave A of loopback improvement, branch `feat/loopback-macos-asp`).
