# ADR-020-capture-to-cloud-benchmark — Capture-to-Cloud Latency Benchmark

## Status

Accepted for v2.3. Ongoing — gate applies from Phase 4 onward.

## Context

No mobile audio SDK publishes a documented, reproducible capture-to-relay latency number. Published voice agent papers (arXiv 2508.04721) treat audio transport as zero overhead. LiveKit publishes "< 50ms" as a target, not a measured guarantee. The single most powerful competitive differentiator for PocketStation is a measured, reproducible, CI-gated, published capture-to-cloud latency number. This ADR defines the methodology, tooling, CI gate, and publication format so results can be cited and compared across releases.

## Decision

Implement a benchmark tool (relay/cmd/benchmark/) that measures mic-capture-to-first-relay-packet latency using embedded send timestamps in Opus payloads. Methodology: source embeds send_timestamp_ns in Opus payload; relay echo endpoint reflects the timestamp; client computes delta. 1000-sample mean, P50/P95/P99 reported. Reference hardware: iPhone 16 (iOS), Pixel 9 (Android). Two frame sizes: 10ms and 20ms Opus. VAD gating on/off. Results committed to relay/benches/results/capture-to-cloud-<version>.json. CI gate (in-process simulation): P95 <= 120ms at 20ms frames, P95 <= 80ms at 10ms frames. README badge published per release.

## Options considered

A) No published benchmark — current state, no competitive claim possible
B) Informal one-time measurement, no CI — not reproducible, not citable
C) Formal methodology with CI gate, committed results, README badge — reproducible, citable, version-tracked

Chosen: C

## Consequences

- Relay gains /v1/echo WebSocket endpoint (timestamp reflection, benchmark use only)
- cmd/benchmark/main.go: CLI tool with --relay-url, --duration, --frame-size, --vad flags
- JSON output: {p50_ms, p95_ms, p99_ms, frame_size_ms, vad_enabled, device, version}
- CI gate fails build if P95 regression > 20% from previous committed result
- README shows badge: "Capture-to-cloud P95: Xms @ 20ms frames"

## Test / measurement plan

- Benchmark runs in CI against in-process relay (no external network required)
- P95 <= 120ms at 20ms frames (in-process; real device numbers will be lower bound)
- P95 <= 80ms at 10ms frames (in-process)
- Regression test: if P95 increases > 20% vs baseline, CI fails
- Result JSON committed on every release tag

## Reversal trigger

Reference hardware changes make the benchmark non-reproducible (update methodology rather than remove).
