# AUDIO-022 — macOS AudioServerPlugin (ASP) for System Audio Capture

**Status:** Superseded in part by the direct Core Audio implementation
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

The first proposal used libASPL. The shipping package no longer has that
dependency: it compiles a direct `AudioServerPlugIn.h` implementation and uses
the public Core Audio process-tap API on macOS 14.4 and later.

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
5. Compile the direct process-tap bridge, authorization query and ASP shared
   memory reader on macOS.
6. Compile `Plugin.cpp` as an unsigned driver bundle build artifact without a
   libASPL submodule only when the explicit `macos-asp-driver-artifact` feature
   is enabled.
7. Keep installation, signing and `coreaudiod` restart outside the SDK.
8. `macos_asp::asp_is_installed()` reports only a running, ABI-compatible
   shared-memory producer; no stub reports synthetic availability.

No vendor submodule is required. All normal SDK builds compile and link the
process-tap, authorization and ASP-reader boundaries without producing a
driver bundle.

## Alternatives considered

| Option | Rejected because |
|--------|-----------------|
| CoreAudio tap API (macOS 14.2+) | No Rust crate; requires Swift bridging; macOS 14.2 minimum deployment target not yet set |
| Soundflower / BlackHole dependency | Third-party kernel extension; not suitable for embedded distribution |
| Only ScreenCaptureKit | 10 ms floor unacceptable for sub-5 ms targets; Application-mode capture needed for per-app routing |

## Consequences

- CI does not require an installed ASP and the crate has no libASPL dependency.
- Wave B/C (Windows WASAPI, Linux PipeWire) use `CaptureMode` from this ADR.
- macOS 14.4+ uses the process-tap path first; older systems may use an
  externally installed compatible ASP for system-mix capture.
- Deployment of the ASP plugin requires a signed HAL plugin + `coreaudiod`
  restart; this is a separate operational procedure not automated here.

## Phase

Phase 3 (Wave A of loopback improvement, branch `feat/loopback-macos-asp`).
