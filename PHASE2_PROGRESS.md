# Phase 2 Progress - PocketStation Runtime

## W0 product-proof baseline — 2026-07-19

- Protected the existing graph/runtime, capture-adapter, relay, benchmark, and
  documentation worktrees before the Session façade implementation.
- Recorded repository HEADs, overlap classification, and exact acceptance
  results in the factory-root
  `docs/reports/PHASE2_W0_BASELINE_2026-07-19.md`.
- Baseline result: component tests, format, workspace Clippy, and
  `scripts/check_protocol.sh` pass; the accepted `product_quickstart` build fails
  only because the example does not yet exist.
- Added `PHASE2_QUEUE.md` with W1–W5 and the AUDIO-027/AUDIO-028 dependency
  decisions in execution order.
- Product state remains `PARTIAL`; W0 changed no runtime code and created no
  scaffold, mock, or loopback path.

## W1 Session API and lineage freeze — 2026-07-19

- Added the canonical `pks-audio` Session façade, safe source selectors,
  reusable destination handles, declarative routes, typed lifecycle failures,
  and an idempotent stop handle.
- Added compact `FrameLineage` and route-specific `DeliveryLineage` contracts in
  `pks-frame`, with nanosecond units and epoch semantics.
- Added the authoritative `product_quickstart`; it declares application and
  microphone capture to the same external connector and browser receiver plus
  two recording stems.
- A valid draft returns typed `RuntimeNotIntegrated` until W3 rather than fake
  success. The `PARTIAL` path is in the scaffold inventory.
- Acceptance: quickstart build, 121 tests plus one doc test, format, strict
  workspace Clippy, and `scripts/check_protocol.sh` pass.
- Freeze report:
  `docs/reports/SESSION_API_LINEAGE_FREEZE_2026-07-20.md`.
- Product state remains `PARTIAL`; W2 frame ownership is next.

## W2 frame fan-out ownership decision — 2026-07-19

- Accepted `docs/adr/AUDIO-027-frame-fanout-ownership.md` before changing pooled
  frame behavior.
- Decision: mutable exclusive capture/DSP frames freeze into immutable shared
  frames with per-slot atomic references; mutating branches use explicit
  preallocated copies.
- Frozen edge policies: `MoveExclusive`, `ShareReadOnly`, and
  `CopyToBranchPool`.
- Implemented exclusive-to-immutable freeze, per-slot atomic shared references,
  explicit preallocated branch copies, shutdown-draining shared edge channels,
  and planner ownership validation/memory accounting.
- Acceptance passes: 113 targeted debug tests plus one doc test, 21 release
  frame tests, strict workspace Clippy, `scripts/check_protocol.sh`, and
  `pool_bench` at 8.065–8.628 ns acquire/drop with no statistically detected
  regression.
- Runtime plan execution remains W3.

## W3 RuntimePlan edges and bounded Bridges — 2026-07-19

- Replaced linear-only execution as the canonical path with connected
  `RuntimePlan` node/edge execution in validated topological order.
- Preallocates one bounded edge queue and telemetry object per destination;
  realtime fan-out uses W2 move/share/copy ownership without per-frame heap
  allocation.
- Realtime-to-worker edges are returned as independent bounded partition
  crossings. Receiver shutdown drains queued frame references deterministically,
  and a failed/full destination cannot stop another branch.
- Per-edge observations now expose capacity/depth/peak, enqueue/delivery/drop,
  precise drop reasons, overruns, discontinuities, age, latency percentiles,
  worker failures, and shutdown discards.
- Added zero-allocation dispatch and connected-plan integration tests. Fixed the
  release gate so `assert_no_alloc` remains active instead of compiling out its
  allocator.
- Acceptance passes: 138 targeted debug tests plus graph doc test, 30 runtime
  release tests plus two release allocation tests, workspace format, strict
  Clippy, and `scripts/check_protocol.sh`.
- W3 status is `REAL` for compiled local runtime edges and bounded crossings;
  W4 supplies the first real file-I/O worker destination.

## W4 aligned multistem recording — 2026-07-19

- Accepted `docs/adr/AUDIO-028-multistem-proof-format.md` before recorder code.
- Added immutable source-to-session `TimelineMapping` in `pks-timing`; recorder
  workers consume the mapping and do not become a second clock authority.
- Added one independent `MultistemRecording` worker per compiled stem edge. File
  allocation, F32 WAV writes, checksums, event sidecars, metric sidecars, and
  finalization stay off realtime partitions.
- The proof directory contains `manifest.json`, independent stem WAVs,
  discontinuity/permission JSONL, and destination/summary metrics. Timestamp
  gaps receive silence and explicit events; overlaps are rejected visibly.
- Clean finish and explicit cancellation both drain bounded queues and finalize
  playable WAV headers. Worker errors produce `incomplete` state, exact errors,
  and `worker_failures_total` without stopping a healthy branch.
- Removed the registered atomic-counter `sink.recording` scaffold and its
  inventory row; recording now means file evidence, not a tally.
- Acceptance passes: 40 `pks-nodes` tests, 30 `pks-runtime` tests, two runtime
  allocation tests, the `product_quickstart` build, workspace strict Clippy,
  format, and `scripts/check_protocol.sh`.
- W4 status is `REAL` for local component recording. W5 now integrates two
  deterministic source-aware stems with connector/browser doubles and this real
  recorder; no remote/device claim is made.

## W5 local isolated vertical slice — 2026-07-19

- Added `pocketstation-lab/e2e/product-proof-local.sh` and the central
  `product_proof_local` example. The runner executes normal, slow connector,
  slow recorder, connector failure, and recorder failure cells concurrently.
- Found and fixed a real isolation defect: non-realtime edges retained shared
  capture-pool frames. They now default to preallocated branch-pool copies, with
  a planner regression test proving capture ownership isolation.
- The binding five-minute run generated 15,000 frames per normal stem. Every
  healthy browser and connector branch delivered every frame with zero drops;
  only the intentionally slow or failed branch reported drops/failure.
- Every cell produced two playable mono 48 kHz WAVs, manifests, integrity
  checksums, permission/discontinuity sidecars, and per-destination metrics.
- Evidence:
  `docs/reports/PHASE2_BOUNDED_EXECUTION_2026-07-27.md` and
  `pocketstation-lab/artifacts/product-proof/local-2026-07-19`.
- Acceptance passes: workspace tests, format, strict Clippy,
  `scripts/check_protocol.sh`, and the 300-second parallel proof runner.
- W5 is complete. Product status remains `PARTIAL` and `LOOPBACK-ONLY` because
  the integrated sources and connector/browser destinations are deterministic
  doubles. W6 real application plus physical microphone is next.

## W6.1 physical microphone capture checkpoint — 2026-07-19

- Found an exposed-but-nonfunctional `mic` path: `pks` parsed and queried an
  input device, but the shared `CaptureMode` could only represent system,
  application, and process output capture.
- Added typed `InputDeviceSelector::{Default, StableId}` and
  `CaptureMode::InputDevice`. macOS input discovery now uses CPAL's stable
  device identifiers and selects a concrete device rather than a PID/name
  approximation.
- Added `MacosInputSource`: CPAL's CoreAudio callback writes f32 frames into a
  preallocated pool and bounded `rtrb` queue; a worker invokes the caller. The
  callback allocates, locks, blocks, logs, awaits, and panics nowhere. Pool,
  queue, oversized callback, and stream failures are atomic observations.
- Added `DesktopCaptureSource` so application/system loopback and physical input
  dispatch remain explicit; `SystemLoopbackSource` was not made semantically
  dishonest by teaching it to open microphones.
- `pks capture from mic` and `pks publish mic` now resolve and open the physical
  input path when a device exists. Stable-selector preservation is tested, and
  CLI IDs retain the device UID instead of collapsing to `mic:0`.
- Fixed the existing profile normalization boundary: normalized frames retain
  source/stream identity, monotonic timestamps, sequence continuity, source tag,
  and encryption state. Standard ring and normalized-output drops are explicit
  counters instead of silent loss.
- Added the real `pks proof sources` W6.1 executable slice. It requires one exact
  discovered application and one exact physical microphone, opens each once,
  routes canonical frames over independent bounded edges, and produces the real
  multistem recorder artifact plus source/drop/queue evidence. It does not claim
  connector or browser delivery.
- Refactored the existing WebRTC publisher at the frame-receiver boundary and
  wired optional proof-session credentials to two independent remote edges.
  With `--session` plus `--token`, the same captured application and microphone
  frames feed the recorder and real publishers named `application` and
  `microphone`; no destination recaptures its source. RTP/drop/edge evidence is
  emitted. Browser-side receipt and RTCStats are still open, so W6.3 is
  `PARTIAL`, not passed.
- Extended the existing real Chromium RTCStats collector to accept repeated
  named `--bus` selections. It opens independent peer connections inside one
  browser context, labels every sample with its AudioBus, matches relay
  downlink/source-clock telemetry by bus, and fails unless every requested bus
  receives packets. A 10-second local component run received 406 application
  and 402 microphone packets with zero reported packet loss and exact relay bus
  matches. This is `LOOPBACK-ONLY` source evidence, not the missing physical
  app+mic artifact.
- Added `pocketstation-lab/e2e/product-proof-smoke.sh`. It creates relay
  credentials, runs the capture-once CLI proof and dual-bus browser collector
  concurrently, preserves logs/process status, validates all three
  destinations, and rejects incomplete/zero/drop/failure evidence. Its
  fail-closed unavailable-source cell creates no artifact. The binding
  five-minute real-device execution remains open.
- Wired optional example-owned Whisper delivery into the same capture-once
  topology. The application and microphone each have a separate compiled,
  bounded connector edge and worker. Workers consume live normalized frames,
  preserve source/stem/timestamp-range evidence, create independent 16 kHz mono
  WAV inputs, then run the existing `whisper-transcribe-example` processes in
  parallel after capture finalization. Queue, drop, failure, inference-latency,
  transcript, and input-path evidence enter the proof summary. This is W6.2
  delivery code, not a public `through()` API or a streaming-incremental STT
  claim.
- A later elevated real-device discovery exposed Spotify's application tap and
  the built-in CoreAudio microphone. The earlier sandbox-limited empty-device
  observation is retained only as a fail-closed diagnostic, not current host
  status.
- Acceptance passes: central workspace tests, strict Clippy, all 124 `pks`
  tests, format/diff checks, and `scripts/check_protocol.sh`. The unavailable
  source execution creates no false artifact.

## W6 real application + microphone proof complete — 2026-07-19

- A first 300-second execution failed visibly instead of producing a false
  PASS. It exposed callback-arrival timestamps being mistaken for source media
  time and a stereo initial-silence rounding defect in the recorder.
- Added `CaptureSampleTimeline`: macOS application, input, and ASP capture now
  anchor source time once and derive subsequent timestamps from cumulative
  device sample frames. Dropped observed buffers still advance both source time
  and sequence so real discontinuities remain visible. Small-chunk drift and
  callback-arrival-jitter regressions are covered by tests.
- Recorder silence sizing now rounds in sample frames before multiplying by the
  channel count, so every stereo gap remains interleaved-channel aligned and
  WAV finalization cannot fail on an odd sample count.
- Capture starts only after recorder, connector, and remote consumers are
  active. The smoke runner and Chromium collector use an explicit readiness
  file; the proof clock starts only after both named browser subscriptions are
  connected. This removed startup edge drops and end-of-run concealment caused
  by mismatched measurement windows.
- Connector acceptance now follows the binding W6 contract: nonzero delivered
  frames, successful real inference execution, zero edge drops/failures, and
  preserved lineage. An empty STT result is valid for a silent or non-speech
  stem and is no longer mislabeled as delivery failure.
- The binding 300-second real run used `app:com.spotify.client` and
  `mic:coreaudio:BuiltInMicrophoneDevice`. Application/microphone delivered
  14,994/14,987 frames to recorder, connector, and relay branches with zero
  capture, normalization, edge, encoder, or stale drops.
- Chromium received 14,948/14,941 packets on the exact `application` and
  `microphone` buses with zero packet loss and zero discarded packets. Final
  cumulative concealment was 0.002%/0.032%; maximum observed cumulative
  concealment was 0.004%/0.271%. Relay output sequence/timestamp
  discontinuities and pacer queue/stale/late drops were all zero.
- Both multistem WAVs finalized `complete`, with zero stale frames and one
  expected initial alignment range per stem. Checksums are
  `a28b17407b99326b` (application) and `526e2fa079144a63` (microphone).
- Evidence:
  `pocketstation-lab/artifacts/product-proof/real-app-mic-w6-pass-2026-07-19`.
  Status: `REAL-DEVICE-PROVEN`. W6 is complete; W7 reliability, permission,
  observation, and 60-minute soak work is next.
- Post-fix acceptance passes: canonical `cargo test --workspace`, strict
  workspace Clippy, all CODE_PROTOCOL laws, 126 `pks` tests, strict `pks`
  Clippy, Rust format/diff checks, lab TypeScript checking, shell syntax, and
  lab protocol lint with zero hard failures. The finalized W6 manifest passes
  the machine continuity predicate added after the run.

## W7.1/W7.2 normal-path checkpoint — 2026-07-19

- Status: `REAL-DEVICE-PROVEN` 30-second checkpoint; W7 remains `PARTIAL`.
- `pks-capture` now models authorization after a real open attempt: capability,
  OS permission observation, application policy observation, explicit Session
  grant, exact capture scope, identity strength, permission epoch, and open
  outcome. Unknown OS state is serialized as `not-observable`; backend failure
  is not guessed to mean permission denial.
- Runtime edge observations now expose capacity/depth/peak, every typed drop
  reason, typed continuity events, enqueue-to-receive latency with sample
  coverage, source-timestamp validity counts, worker failure, and shutdown
  discard. Attempted-frame counts and drop percentage use enqueued plus dropped
  dispatches as their explicit denominator. Receive-before-enqueue samples are
  counted as invalid rather than silently becoming zero latency. Both capture
  Bridges expose source/sink counters and executor and accumulator pool
  availability/failures.
- The first long soak exposed a receiver-instrumentation race without media
  loss: a destination worker sampled its receive timestamp before attempting an
  empty-queue pop, allowing a producer enqueue between those operations. The
  router now owns `PlanEdgeReceiver::try_recv`, which pops first and samples the
  canonical `pks-timing` process clock second. Explicit `recv_at` remains for
  deterministic runtime schedulers/tests and is crate-private so destination
  workers cannot repeat the race. A second candidate identified and removed the
  recorder's remaining pre-pop call. Both rejected attempts remain negative
  evidence; no tolerance or metric was relaxed.
- Physical microphone timestamps now use CPAL's authoritative relative capture-
  to-callback duration in the shared process clock. The CLI normalization
  Bridge derives output timestamps from cumulative normalized samples and no
  longer reanchors each full frame to callback jitter. A failing intermediate
  run proved why this matters: 1,500 microphone frames reached the remote and
  connector with no drops, but recorder rejection converted benign timestamp
  jitter into 1,024 synthetic gap events and failed halfway. The corrected run
  delivered and recorded all 1,500 frames with zero continuity events and zero
  future timestamps on every microphone edge.
- Runtime observations are written at start, every ten seconds, and finalization.
  The final proof decision uses router-owned edge telemetry after consumers have
  stopped, requires enqueue/delivery parity, and reconciles remote delivered,
  encoded, and RTP frame counts with source dispatch.
- A synchronized macOS run opened exact Spotify application capture and the
  built-in physical microphone. Each application edge enqueued and delivered
  1,499 frames and each microphone edge delivered 1,500, with zero drops,
  overruns, discontinuities, worker failures, or shutdown discards. Both
  recordings and connector branches completed; the remote publishers sent
  1,499/1,500 RTP packets with exact delivery/encode/RTP parity.
- Browser receipt had zero packet loss on both buses. The example connector used
  an English-only tiny Whisper model while Spotify content was uncontrolled and
  could use any language, so this proves connector delivery/execution, not STT
  recognition accuracy.
- Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-normalized-clock-30s-pass4-2026-07-19`.
  The browser ended with zero packet loss, 0.141% cumulative application
  concealment, and 0% microphone concealment. These are same-host receiver
  observations, not competitive transport claims.
- The real browser-disconnect and connector-process-failure cells pass. In
  both cases the named failed branch was observed and every unrelated branch
  completed with exact frame delivery, zero drops, and complete recordings.
  Proof finalization now joins all destination workers and writes failure
  outcomes even if recorder finalization is incomplete or errors. Evidence:
  `w7-fault-browser-disconnect-30s-2026-07-19` and
  `w7-fault-connector-failure-30s-2026-07-19` under the lab product artifacts.
  The new soak wrapper also passed a 30-second acceptance execution with five
  runtime batches and nine RSS samples at
  `pocketstation-lab/artifacts/product-proof/w7-soak-30s-pass3-2026-07-19`;
  it is correctly labeled `SAFE-TO-TEST`, not the W7 60-minute soak.
  Open W7 gates are real permission transitions, source/relay/recorder restart
  and recovery paths, the 60-minute soak, and clean-checkout proof.

## Repository and timing partition - 2026-07-16

- Renamed the local central workspace from `pocketstation` to `pocketstation`; the
  workspace is the product center while `pks-*` crates keep narrow ownership.
- Removed `media-clock` from the central workspace dependency graph.
- Removed the unrelated `StreamProfile -> media_clock::Contract` mapping from
  `pks-codec`; codec profiles now own codec configuration only.
- Kept drift/correction and the compiled, tested experimental `SegmentGate` in
  `pks-timing`.
- Confirmed network pacing, RTP sequence/timestamp continuity, repair, and RTCP
  clock lineage remain relay media-plane responsibilities; no `pks-playout`
  crate was added.
- Added allocation-stable Opus PLC decoding to `pks-codec` so benchmark and
  receiver code do not need a separate codec wrapper.
- Acceptance: `cargo test -p pks-codec -p pks-timing` passes (32 tests total).
- The neutral benchmark's final `media-clock` dependency was removed: benchmark
  drift uses `pks-timing`, Opus PLC uses `pks-codec`, and reproducibility-only
  reorder/holdback stays private to the harness. Product docs now mark the old
  workspace archived rather than compatibility-active; the remote archive was
  verified 2026-07-16.
- Linux capture now uses the canonical `sample_rate_hz` field on `AudioFrame`
  and `CaptureSource`, closing the cross-platform strict-clippy failure without
  changing capture behavior.
- CI benchmark compilation and the allocation-free integration gate now target
  the canonical `pks-audio` package name instead of the retired
  `pocketstation-audio` name.

## Runtime timing ownership - 2026-07-16

- Added `pks-timing` as the single owner of clock drift estimation and PI clock
  correction.
- Replaced `pks-pipeline`'s duplicate `ClockSync` implementation with the
  runtime-owned controller while retaining a compatibility alias.
- Stopped treating an absolute frame timestamp as a measured clock offset in
  `ResampleNode`; correction now requires an explicit inter-clock observation.
- Preserved the future voice-output interruption state machine as compiled,
  tested `pks_timing::experimental::SegmentGate` code without exposing it as a
  current product feature.
- `media-clock` compatibility wrappers delegate to the new owner; the live
  CLI/codec path has since been decoupled from that workspace.
## Local Whisper connector example - 2026-07-13

- Added `examples/whisper-transcribe` as an example-owned `AsyncNode`; no provider dependency entered first-party crates.
- Binary WAV input crosses the async boundary and text output preserves sequence/timestamp lineage.
- Missing process/model and subprocess crashes fail visibly.
- Real whisper.cpp tiny English E2E passed in CPU mode with a 3.84-second spoken fixture; measured wall time was 1.08 seconds.
- GPU remains explicit opt-in because Homebrew whisper.cpp 1.9.1 crashed in the Metal backend on this machine.
## Bounded captured-frame stream - 2026-07-13

- Added a stable `FnMut(AudioFrame)` capture callback contract across the platform adapters.
- Added a bounded, non-blocking SPSC `CapturedFrameStream` with explicit delivered/drop counters and no hidden runtime.
- Unit tests pass for delivery, overflow, closure, callback adaptation, and invalid capacity.
- Real macOS exact-process capture passed with 281 consumed frames, 287,744 samples, RMS 0.141005, and zero dropped frames.
- All 112 CLI tests pass against the updated capture API.
- The capture-stream example is target-gated so Linux and Windows all-targets
  checks compile without pretending the macOS system-loopback endpoint exists.
- Linux capture tests explicitly reject the stream-capacity setup error so the
  cross-platform `CaptureError` contract remains exhaustively checked.
