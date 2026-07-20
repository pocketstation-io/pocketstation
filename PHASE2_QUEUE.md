# Phase 2 Queue — Narrow Product Proof

Binding scope and dates come from the factory-root
`PRODUCT_OPERATING_CONTRACT.md`. Complete these tasks in order. Do not start a
downstream wave because its code is easier.

## W1 — Minimal Session API and lineage — due 2026-07-20

- [x] Export the canonical `Session`, `Source`, selector, stem, endpoint, and
  lifecycle vocabulary from `pks-audio`.
- [x] Compile `crates/pks-audio/examples/product_quickstart.rs` in the protected
  working tree; clean-checkout reproduction remains a W7 release gate.
- [x] Define source/frame/delivery lineage fields with units and epoch semantics.
- [x] Test declarative route validation, idempotent stop semantics, and visible
  failure behavior.
- [x] Keep `through`, providers, actions, translation, and SDK parity deferred.

## W2 — Frame fan-out ownership

- [x] Accept AUDIO-027 before changing shared-frame behavior.
- [x] Resolve the planner's shared-handle contract versus the current exclusive
  pool handle.
- [x] Prove bounded immutable fan-out and explicit preallocated copies for
  mutating branches under contention.

## W3 — Runtime edges and Bridges

- [x] Execute `RuntimePlan` edges rather than a linear topological chain.
- [x] Give every destination an independent bounded queue and explicit overflow
  policy.
- [x] Schedule realtime-to-async `Bridge` crossings without callback allocation,
  locks, blocking, async, logging, or panics.
- [x] Expose queue depth, drops, discontinuities, and latency per destination.

## W4 — Multistem recording

- [x] Accept AUDIO-028 for proof artifact layout and finalization semantics.
- [x] Record application and microphone stems independently with aligned clock
  and lineage sidecars.
- [x] Make recorder slowdown/failure unable to block capture or another route.
- [x] Test deterministic finalization and discontinuity reporting.

## W5 — Local isolated vertical slice

- [x] Route two deterministic source-aware stems concurrently to example
  connector, browser/remote receiver, and recorder doubles.
- [x] Prove one slow/failed destination does not block another destination or
  capture.
- [x] Produce a local isolation artifact; retain `LOOPBACK-ONLY` status until W6.

## W6 — Real application + microphone proof — due 2026-08-03

- [x] Implement typed stable physical-input selection and a real macOS capture
  adapter with bounded callback delivery and explicit observations.
- [x] Preserve source/stream identity, timestamps, sequences, and drop accounting
  across profile normalization.
- [x] Add a fail-closed exact app+mic source/recording proof command using two
  independent compiled edges and the real multistem recorder.
- [x] Refactor the real WebRTC publisher to consume externally supplied frames
  and attach two named relay-publish edges without recapturing either source.
- [x] Reproduce that source/recording command with Spotify's exact application
  tap and the built-in physical microphone; retain the five-minute artifact.
- [x] Attach both live stems to independent bounded example-owned connector
  edges, retain source/stem/timestamp lineage, and report receipt, inference
  latency, drops, and failures. Real-device/model reproduction remains part of
  the open hardware artifact item above.
- [x] Extend the real Chromium collector to subscribe to `application` and
  `microphone` independently in one browser context and collect per-bus
  RTCStats plus matching relay downlink telemetry. The 10-second local
  component proof is `LOOPBACK-ONLY`; exact app+mic reproduction is still open.
- [x] Add `pocketstation-lab/e2e/product-proof-smoke.sh` with fail-closed
  process and artifact validation.
- [x] Pass that runner's binding five-minute exact-app + physical-mic,
  three-destination artifact gate.

## W7 — Reliability, authorization, and soak — due 2026-08-10

- [x] Serialize successful-open authorization truth after the real open attempt,
  including exact scope, identity strength, permission epoch, and explicit
  `not-observable` states rather than inferred permission claims.
- [x] Emit periodic observations for both capture Bridges, their fixed pools,
  and all six bounded destination edges on the normal real-device path.
- [x] Fail the proof when final router telemetry disagrees with worker-local
  delivery or RTP counts; drain remote edges before freezing observations.
- [ ] Prove denied, revoked, changed-permission, disappeared-source, and reopen
  epochs on real platform paths.
- [ ] Complete W7 metric semantics, including a trustworthy common-clock
  source-to-receive boundary and an explicit drop-rate field.
- [ ] Pass the destination failure, disconnect/reconnect, and restart fault
  matrix without cross-branch blocking.
- [ ] Pass the binding 60-minute real-device soak with RSS, observation-gap,
  pool, edge, browser, and artifact-integrity gates.
- [ ] Reproduce the accepted proof from committed clean checkouts.

## Decisions required before dependent work

- [x] AUDIO-027: frame fan-out ownership.
- [x] AUDIO-028: multistem proof format and finalization.

## Deferred until their entry gates

- Remaining W7 permission transitions, metric semantics, faults, 60-minute
  soak, and clean-checkout reproduction; W8 independent reproduction.
- Rust/Python/Node parity only after W8 PASS/pivot and a stable SDK boundary.
- iOS/Android adapters only after the desktop proof and language contract.
- Generic `through`, merge, fallback, actions, and provider catalogs only after
  a single evidence-backed asynchronous operator slice.
