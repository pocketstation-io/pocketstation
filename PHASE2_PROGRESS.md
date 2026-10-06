# Phase 2 Progress - PocketStation Runtime

## 2026-10-02 — Windows same-open AEC route, candidate145

Status: PARTIAL. The Windows WASAPI capture worker now owns one opened
microphone stream for PCM and effect inspection. A native request binds exact
active microphone and render endpoints, requests that render endpoint through
Windows AEC control, and admits the route only if the same client reports an
active AEC effect. Ordinary microphone capture observes effect state on that
same stream when Windows exposes the effect manager. Effect, selected-endpoint
and default-device notifications invalidate the route and processing facts;
unsupported native routes reject startup rather than delivering raw audio.
Default Core builds still omit the portable WebRTC engine.

The Windows 11 ARM64 VM executed two real Session probes: ordinary microphone
open/stop passed, and its virtual line-in rejected native AEC because it has
no controllable effect. The virtual line-in delivered zero physical PCM frames,
so this is an endpoint/lifecycle result, not an acoustic result. Cross-target
Windows ARM64 and x64 lean builds, 722 Mac all-feature tests across 37 suites,
protocol checks,
and lean/enabled quickstart builds are recorded in the task evidence. Native
macOS and Linux routes remain unsupported. Windows physical echo reduction,
double-talk, callback transition timing and route recovery remain unqualified.
No SDK release, publication, new package or fork change accompanies this step.

## 2026-10-02 — opened-input AEC admission, candidate144

Status: PARTIAL. Core observes processing facts supplied by the opened capture,
bound to the attached source and continuity. Support, active processing and raw
availability are distinct optional facts. Built-in OS effect queries and suitable
native route selection remain unfinished; unknown is not a raw/native claim.

Processing reports use one coherent atomic snapshot and sticky invalidation
history. Session checks known processed microphone ancestry before delivery,
then privately decorates its AEC worker to check before/after preparation,
processing and flush. Rejection is terminal while cleanup always delegates.
Processed replacement is rejected before attachment; old reporter state cannot
change a replacement's observations. Automatic capture failure clears attachment
facts. Per-open attachment counting handles saturated public generations.

New tests simulate backend facts but exercise actual Session lifecycle and
prepared PCM. Final all-target/all-feature regression passes 709 tests (33
nonempty suites). Five lean Session checks, final protocol/strict Clippy and 14
hot-path checks pass. API compatibility passes 196 checks against 1.1.9;
lean and enabled release quickstarts compile. No callback changes,
default-engine inclusion, live stub,
new package/repository, native fork changes, version or publication.

## 2026-10-01 — known AEC input provenance, candidate143

Status: SAFE-TO-TEST for Session declarations and prepared PCM, accepted by the bounded gates. Core tracks its cancellation ancestry privately; external Operator
IDs cannot forge it. Cloned, transformed and reentered streams retain that fact.
`echo_input_processing` remains available without the engine and distinguishes
unknown, explicit caller declarations and Core evidence. Caller assertions do
not erase known Core ancestry or establish native-device qualification.

`echo_cancel` rejects known processed microphone inputs; graph insertion repeats
the check under the declaration lock before allocating IDs. Freeze validates
late generic connections. Iterative traversal terminates on cycles without
external callbacks or recursion. Rejected declarations leave first-stage and raw
prepared-PCM delivery usable. Capture callbacks, source selection, provider
ownership and default dependencies remain unchanged.

Final all-target/all-feature regression: 688 tests across 35 suites pass.
Protocol/strict Clippy, no-default compilation, doctests, API comparison,
lean/enabled release quickstart and fresh lean archive gates are recorded in
factory evidence `docs/execution/evidence/W21-AEC-INPUT-PROVENANCE/`.
Native discovery/selection, OS backends, SDK projection of new input facts,
physical double-talk and device-clock/recovery qualification remain separate.
No live scaffold; unregistered generic operators occur only in declaration tests.
No version, push, tag, publication or deployment is authorized.

## macOS microphone sample-clock continuity — 2026-09-26

- Status: `SAFE-TO-TEST`. Installed SDK qualification found 37 timestamp-only
  discontinuities in six seconds with zero sequence gaps or capture/route loss.
  The native-format reader had stopped using the existing sample-count clock;
  every aligned packet boundary could reintroduce callback scheduling jitter.
- The reader now composes fixed product framing with `CaptureSampleTimeline`,
  anchored to the first represented capture sample. The clock advances with
  canonical samples, including frames later rejected by bounded delivery. An
  explicit capture/conversion gap still resets partial framing, reanchors the
  clock and advances the existing sequence discontinuity.
- The new 512-sample callback regression fails before the repair and passes
  for both 480- and 960-sample output. Reset and empty-output tests preserve the
  real-gap and partial-buffer contract.
- All 530 library tests and all targets/features, strict Clippy, formatting,
  release quickstart and CODE_PROTOCOL pass. Exact clean physical continuity
  qualification follows in Lab before release acceptance.
- This restores source timing in the off-callback reader. No public API,
  callback operation, capacity, loss threshold, provider, scaffold or mock
  product path changed. Historical 10 ms source-age limits remain unchanged.


## Minutes HFP release prerequisite — 2026-09-24

- Status: the Rust candidate required by the opt-in Minutes microphone
  supervisor is qualified locally. Exact Core 1.1.11 versioning, push, tag,
  crates.io publication, and GitHub release were authorized on 2026-09-25;
  registry and installed-consumer verification remain before Minutes can
  consume it.
- Public release notes and guides now cover opened native format,
  first/latest-frame activity, delivered-PCM signal measurements, caller-owned
  evaluation windows, explicit same-device reopen, host-selected replacement,
  response-timeout uncertainty, generation/discontinuity changes, and survival
  of unrelated stems.
- The documentation explicitly does not infer speech, audibility, correct
  routing, or repair of a Bluetooth profile that already supplies zero-valued
  samples.
- Target-specific timeline helpers are compiled only on the Linux and test
  paths that use them, leaving the default macOS release quickstart free of
  dead-code warnings.
- Acceptance passed: formatting, strict all-target/all-feature Clippy, 528
  library tests plus every integration/example/benchmark target, release
  quickstart build, CODE_PROTOCOL, single-package archive checks, public API
  semver comparison against 1.1.10, and five packaged documentation tests.
- This work adds no provider, fallback-selection policy, mock, scaffold, or
  loopback product path. The Logi HFP plus Teams case remains unproved pending
  the reporter's physical-device comparison.

## Per-source delivered-PCM signal truth — 2026-09-24

- Status: implementation complete and `SAFE-TO-TEST` for
  `W21-CORE-SOURCE-SIGNAL-OBSERVATIONS`; full release gates and acceptance
  evidence remain pending.
- Every built-in Source now measures canonical PCM after capture dequeue on
  the Session runtime worker. Native capture callbacks are unchanged.
- Observations expose cumulative sample, exact-zero, nonzero, and non-finite
  totals plus one latest-frame window with source time, process observation
  time, duration, sequence, source generation, discontinuity epoch, peak, RMS,
  exact-zero ratio, and consecutive exact-zero duration.
- One-writer atomic snapshots use a sequence counter so control-path readers do
  not combine fields from different frames.
- A caller-owned policy distinguishes no samples, pending exact zero, sustained
  exact zero, below-threshold numeric signal, signal meeting caller thresholds,
  and non-finite PCM. Core provides no threshold default and does not infer
  speech, audibility, permission state, route correctness, or fallback policy.
- Focused worker tests prove exact-zero accumulation, continuity reset,
  non-finite accounting, peak and RMS. A full Session test proves an
  application stem with 0.25 linear signal and an exact-zero microphone stem
  remain independently observable while both reach all configured routes.
- This adds no reopen, automatic fallback, provider, mock, scaffold, or
  loopback product path. Bluetooth HFP and Teams remain unproved until the
  affected physical route runs the diagnostic.

## macOS microphone native-format normalization — 2026-09-24

- Status: `SAFE-TO-TEST` for
  `W21-CORE-MICROPHONE-FORMAT-NEGOTIATION`; the affected Logi HFP and Teams
  route remains unproved.
- The input owner now selects one PCM format the device actually advertises.
  Selection covers signed, unsigned, and floating-point PCM, prefers 48 kHz
  mono `f32`, and otherwise uses the advertised rate nearest 48 kHz. DSD-only
  inputs fail explicitly instead of being mislabeled as PCM.
- The CoreAudio callback now copies raw native bytes into a preallocated
  bounded packet pool and performs a nonblocking queue send. Native sample
  conversion, multichannel downmix, streaming linear rate conversion, product
  frame normalization, and Session delivery run on the owned reader worker.
  The callback has no conversion, resampling, allocation, locking, blocking,
  async work, logging, or panic-based control flow.
- Session frames remain canonical mono `f32` at 48 kHz. Resampling phase
  crosses native packet boundaries; source timestamps remain anchored to the
  first represented native sample; conversion failures and queue loss remain
  observable and force the next packet through a discontinuity reset.
- `CaptureNativeFormat`, `CaptureSampleRepresentation`, and
  `SessionSourceNativeFormatObservation` expose the exact opened device rate,
  channel count, and sample representation separately from canonical Session
  media.
- Direct tests cover advertised 8/16/24/32/44.1/48/96 kHz rates, all supported
  integer and floating-point representations, multichannel downmix, DSD
  rejection, insufficient capacity, and cross-packet 16→48 kHz phase.
- Passed so far: focused macOS input tests, 517 all-feature library tests plus
  every integration test/example/benchmark under `cargo test --all-targets
  --all-features --locked`, the callback source-contract test, and strict
  all-target/all-feature Clippy. Release quickstart, CODE_PROTOCOL, exact clean
  physical built-in-microphone evidence, and final execution validation remain
  before acceptance.
- This adds no automatic fallback, reopen policy, signal/speech threshold,
  provider, mock, scaffold, or loopback product path.
- Implementation review is complete for the advertised-format selector,
  bounded callback packet handoff, worker-side conversion, Session observation,
  and source-contract regression. The physical built-in-microphone control and
  release gates remain the acceptance boundary before this status can advance.

## Source activity truth — 2026-09-18

- Status: `SAFE-TO-MERGE` for `W21-CORE-SOURCE-ACTIVITY-TRUTH`; no release or
  physical-platform claim.
- Every built-in Source now retains Session-start, first-frame, latest-frame,
  snapshot, and frame-count observations after the bounded capture queue.
  `SessionMetricsSnapshot::source_activity(index)` exposes them in the same
  stable declaration order as existing Source metrics without changing the
  public `SessionSourceMetrics` record or its fixed C projection.
- C consumers use the separate 56-byte `PksSessionSourceActivity` record and
  `pks_session_source_activity_at`; the legacy 176-byte source metrics record
  and its tail canary remain unchanged.
- `SessionSourceActivityPolicy` applies caller-supplied non-zero first-frame
  and stall durations and returns distinct awaiting, active, first-frame
  timeout, and stalled states.
- Session `Running` remains lifecycle readiness. Core does not inspect sample
  energy, infer permission, choose a fallback, replace a Source, or restart
  capture.
- The implementation changes no native callback and adds no queue, lock,
  provider, scaffold, mock, or loopback product path.
- Acceptance passed: `cargo fmt --all -- --check`, 509 library tests plus all
  integration targets/examples/benches under `cargo test --all-targets
  --all-features --locked`, strict all-target clippy, release quickstart build,
  and the complete CODE_PROTOCOL check.

## macOS input failure diagnosis — 2026-09-10

The installed JavaScript 10 ms proof repeatedly opened an explicitly selected
built-in microphone, received one native callback, and then received a CPAL
stream error. The earlier Core proof used the default microphone selector and
did not exercise this exact case.

Core now preserves CPAL's stable error category in the Session failure instead
of reducing every failure to `cpal-stream-error`. The error callback reuses a
preallocated string, so it does not allocate, block, log, or run asynchronous
work. This is diagnosis only until the explicit-device physical test passes.

The diagnosis identified an xrun. Core no longer changes the physical device's
global callback size to match the public 10 ms frame profile; the existing
normalizer accepts the device's native callback cadence and emits exact Session
frames. CPAL reports xrun, route-change, and realtime-scheduling notifications
while the stream remains usable. Core now keeps that stream alive. After an
xrun or route change, it discards incomplete buffered samples, establishes a
new timestamp anchor, and advances the frame sequence so the next delivered
frame reports the gap.

## CoreAudio process-tap normalization — 2026-09-04

The installed JavaScript Session proof requested 10 ms application frames and
observed normal 480-frame stereo batches followed by a 32-frame stereo batch
while the source was still active. The macOS process-tap reader requested the
configured size but forwarded whatever CoreAudio returned. The microphone,
Windows, and Linux implementations already assemble variable native callbacks
into the selected 10 ms or 20 ms frame size.

The process-tap reader now uses the same preallocated
`CaptureFrameNormalizer`. The audio thread still performs no allocation,
locking, blocking, logging, or asynchronous work. Partial native batches remain
inside the fixed buffer until a complete PocketStation frame is available.

Acceptance:

```bash
cargo test capture::frame_normalizer
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

The original installed-package proof is rerun with a physical `afplay` source
and must report only 10,000,000 ns frames. This is macOS application-capture
evidence only; it does not upgrade Windows, Linux, microphone, or latency
claims.

### Staff Bar Self-Check — CoreAudio process-tap normalization

- Smallest correct design: yes — reuse the existing capture normalizer in the
  one backend that did not use it.
- Tests added or updated: yes — existing normalizer tests plus the installed
  physical regression that exposed the defect.
- Hot-path safe: yes — storage is allocated before the reader starts; `push`
  only copies into that storage and invokes the existing delivery closure.
- Public API changed: no.
- New dependency: no.
- Phase scope respected: yes — required by the active installed JavaScript
  Session proof and recorded in its execution envelope.
- Unsafe added: no.
- Remaining risk: a final incomplete native batch is intentionally not emitted
  as a shorter public frame when a source stops.

## Route depth observation — 2026-09-03

- A Windows installed-wheel test exposed a transient observation race: a
  producer could sample the delivered-frame counter before the consumer's
  update became visible and report a queue peak above the route's capacity.
- The ring buffer already enforced its configured capacity and did not accept
  extra frames. Route metrics now cap sampled depth at that capacity, so the
  observation cannot report a state the queue cannot hold.
- A focused regression reproduces delayed delivery-counter visibility and
  verifies that current and peak depth remain at or below eight frames.
- This changes observation reporting only. Queue storage, routing, capture
  callbacks, and delivery policy are unchanged.

## Public developer documentation — 2026-09-02

- The packaged documentation now starts with capture, then covers Session
  lifecycle, finite route delivery, source identity, Connectors, extensions,
  events, metrics, errors, platform preparation, and troubleshooting.
- New concept and reference pages are included in the crate archive and checked
  by the single-package publication test. Internal ADRs, reports, and execution
  records remain outside the published crate.
- Normal guides describe route behavior in developer terms. `RouteSettings`
  holds the media and delivery choices for a compiled route; it is not required
  by the quickstart or normal Connector API.
- The complete all-target, all-feature Rust suite and compiling examples pass.
  This documentation work adds no runtime behavior, scaffold, mock, loopback
  result, or new product claim.

## W21 concise Connector authoring — 2026-09-01

- `Connector::from_audio_fn` creates an application-local audio destination
  from one delivery function. `AudioConnector` provides the reusable
  `start`/`send`/`stop` form without exposing manifests or driver factories.
- `Session::destination` registers and declares a default realtime-audio
  destination. Typed configuration and custom edge contracts remain available
  through `RegisteredConnector::declare`.
- Core continues to own the existing bounded routes, source-aware frames,
  grouped preparation, readiness, delivery accounting, drain or abort, panic
  containment, and joined shutdown. No provider implementation or second
  runtime was added.
- Focused Session tests prove the function form and prove two independent
  sources share exactly one reusable provider lifecycle. They also prove that
  two Connector values remain independent and that startup failure closes the
  provider exactly once. The public guide now explains the architecture path,
  lifecycle, use cases, failure behavior, and boundary with Sources and
  Operators. Full Core release gates and installed-package qualification remain
  required before release.

## W21 polled-audio frame negotiation — 2026-08-31

- The installed Python qualification exposed one remaining fixed-frame
  assumption: the managed polling endpoint advertised the Session capture
  cadence even when an application-owned or Operator-generated stream declared
  another finite PCM frame size.
- The polling endpoint now accepts the connected stream's declared frame size.
  Capture, recording, and transport endpoints retain their explicit 10 ms or
  20 ms cadence.
- This changes only compile-time media negotiation. It adds no hot-path
  allocation, queue, resampling, buffering, mock, or loopback path.

## W21 frame-duration release qualification — 2026-08-31

- The exact Core candidate passes the complete seven-mode 10 ms application
  selector matrix on macOS, Ubuntu, and Windows 11 ARM64. The existing 20 ms
  profile remains the default.
- An installed Python wheel built from the exact Core candidate passed the
  physical microphone, OpenAI Realtime, Relay, Chromium, output-cancellation,
  and three-stem recording gate with zero active route loss.
- The freeze protocol identified a PipeWire clock-rate parameter without an
  explicit unit. It now describes the ratio as seconds per tick with separate
  numerator-seconds and denominator-ticks names; runtime behavior is unchanged.
- The published `RecordingErrorCode` discriminants remain unchanged. The new
  missing-initial-frame code is appended after every 1.1.3 variant, and all 196
  applicable SemVer checks now pass against the published 1.1.3 tag.
- The compatible release line remains 1.1.x. Public application selectors and
  the optional 10 ms capture profile are qualified without changing the major
  or minor version.
- Windows VM scheduling remains correctness-only evidence. Receiver playout,
  acoustic hearing, echo cancellation, and WAN/TURN remain separate gates.
- No mock, scaffold, provider implementation, or second media runtime was
  added.

## W21 independent multistem activation — 2026-08-30

- Status: `SAFE-TO-TEST`; focused recording tests and strict all-target,
  all-feature Clippy pass. The installed physical-device voice gate is still
  required before this change is `REAL` product evidence.
- Each recording stem now starts its worker when that stem delivers its own
  first frame. A silent generated-audio stem no longer delays active
  application and microphone writers or causes their bounded recording edges
  to overflow.
- Recording manifest schema 2 writes the complete declared stem set before
  workers start. Source, clock, generation, permission, and source-timeline
  fields remain explicitly unavailable until the first authoritative frame
  establishes them. Finalization fails visibly if a declared stem never
  establishes lineage.
- A three-stem regression keeps application and microphone active while the
  assistant stem starts later, then proves all 41 frames are written with zero
  edge loss. Recording remains one grouped Endpoint lifecycle, one directory,
  and one terminal outcome.
- This changes no capture callback, realtime route, queue bound, provider,
  scaffold, mock, or loopback path.

## W21 exact application selection — 2026-08-28

- Status: `SAFE-TO-TEST` and `PARTIAL` until the Lab completes the Linux
  current-candidate acceptance. The Windows selector matrix is `VM-PROVEN`
  and macOS application capture is `REAL-DEVICE-PROVEN`; neither result
  qualifies a cross-platform production-latency claim.
- `Source::application` accepts an exact display name or native application
  ID, a typed `ProcessId`, or a discovered `StableSourceId`. Raw integers
  remain unsupported because they do not identify which ID domain they use.
- Display names and native application IDs use exact ASCII-case-insensitive
  matching. Missing and ambiguous selections fail before capture starts;
  backends no longer select the first matching source.
- macOS can reopen a discovered application identity and includes every live
  process with that identity. Windows can reopen the exact discovered process
  incarnation and verifies its creation time. Linux can reopen a discovered
  persistent or live transient PipeWire identity, while multiple matching
  nodes still fail because the current backend opens one target object.
- The first Ubuntu ARM64 Lab run found that PipeWire exposes
  `application.process.id` through the bound node information rather than the
  registry summary. Linux discovery now reads that complete node information
  and waits for a second PipeWire synchronization round before returning.
  The same run exposed a second defect: PipeWire delivered its native 1,024
  sample-frame quantum while the capture callback assumed one fixed 20 ms
  frame. Linux capture now accepts bounded native callback sizes and normalizes
  them through preallocated storage into the Session's 20 ms frames while
  preserving the source timeline.
- The rebuilt Ubuntu ARM64 release fixture captured the controlled PipeWire
  application by exact name, discovered stable identity, and typed process ID.
  Each selector delivered five stereo frames with no capture, ingress, route,
  or polled-Endpoint drops. This remains `VM-PROVEN`; the official clean-commit
  Lab artifact and Windows/macOS acceptance are still required.
- Compilation now asserts that the application route reserves 1,920
  interleaved samples per 20 ms stereo frame while the microphone route
  reserves 960 mono samples. The PipeWire regression test proves two 1,024
  sample-frame callbacks produce contiguous 20 ms frames and retain the exact
  bounded remainder.
- Linux source discovery now uses PipeWire's human `node.description` for
  input and output device names while retaining `node.name` as the stable
  device identifier. This aligns the user-facing microphone name with macOS
  and Windows without weakening exact device selection. The focused test and
  strict Clippy gate pass; the installed-wheel VM rerun remains required.
- A Windows 11 ARM64 Lab run now exercises the complete selector matrix:
  display-name string, application-ID string, explicit name, bundle ID,
  stable ID, process ID, and process instance. All seven modes captured 100
  real WASAPI frames from one packaged Media Player process and retained one
  `SourceId`. Capture, ingress, route, and polled-Endpoint loss counters were
  zero. Native packets were normalized into 20 ms Session frames without an
  oversized-buffer or stream error.
- That run also found and fixed a Windows capacity check that measured the
  size of a `Box` handle instead of the boxed sample slice. The regression
  test now checks the actual slice storage. This changes no callback
  allocation, lock, blocking, logging, or panic behavior.
- Windows VM queue latency included scheduler spikes up to 415,605,334 ns;
  source-timestamp-to-receive latency reached 1,068,964,500 ns. These values
  are retained as measurements and do not qualify a low-latency claim. A 20
  ms media-frame profile also cannot establish first-sample-to-receive below
  20 ms; a separate 10 ms profile would require its own cross-platform and
  browser qualification.
- The exact Core commit `7ac07e493dc742eb9f431b2c72216dce24b41051`
  passed the complete macOS matrix against real Spotify application capture.
  Every selector delivered 100 frames under one source identity with zero
  active-path loss. Queue p95 was 2,097,152 ns and the largest queue maximum
  was 8,174,333 ns; source-timestamp-to-receive maxima ranged from 35,680,896
  ns to 59,625,771 ns. Frames intentionally rejected before the Session start
  gate are reported separately from active delivery loss.
- Formatting, the public façade test, focused selector tests, strict Clippy,
  warnings-as-errors rustdoc, and all 196 API compatibility checks against the
  accepted 1.1.2 baseline pass on macOS.
- This step adds no provider, scaffold, mock, fallback, fuzzy matching, or
  new media engine. Cross-platform and physical-device status remains bound
  to the Lab artifacts, not cross-compilation.

## W21 generation-owned output cancellation — 2026-08-27

- Status: `SAFE-TO-TEST` and `PARTIAL`. This step does not claim remote
  Connector clearing, browser playout acknowledgement, acoustic hearing,
  echo cancellation, or continuous full-duplex completion.
- `OutputGeneration` gives application-owned generated PCM a typed identity.
  Starting or cancelling one output generation does not stop capture,
  recording, transcription, or unrelated AudioBuses.
- Generation identity now survives audio frames, typed signal envelopes,
  Operator audio, generated-audio reentry, routing, and Endpoint delivery.
  Inactive frames are discarded on bounded dequeue paths and counted without
  creating a second queue or runtime.
- The polled Endpoint wake predicate now shares one mutex with its condition
  variable, closing a missed-notification race. Source continuity advances
  before generation filtering, so cancelling output cannot create a false
  source sequence gap.
- Cancellation observations use additive methods instead of adding fields to
  externally constructible structs. Output-specific write failures use a new
  error type; the existing exhaustive audio-input error enums are unchanged.
  The repository compatibility gate now compares with the published 1.1.2
  API and requires a clean result instead of accepting historical breaks.
- Acceptance passes: formatting, 477 library tests, every integration and
  benchmark target, strict all-target/all-feature Clippy, warnings-as-errors
  rustdoc, the release quickstart, and all 196 SemVer checks against 1.1.2.
- The complete protocol script remains blocked before its protocol laws by
  the local freeze-policy gate: the separately edited PR template no longer
  exposes the seven classifications that gate requires. No release, version,
  tag, push, publication, or deployment action is included.

## Core 1.1.2 publication recovery — 2026-08-23

- Status: `SAFE-TO-TEST`; crates.io publication did not run after the first
  release workflow failed during validation.
- The clean GitHub checkout lacked the protocol repository that owns the
  connector v1 conformance corpus. The published package code and local Core
  tests were not the failure.
- The recovery workflow now checks out the exact accepted protocol commit and
  materializes only that corpus at the path expected by the tagged test. The
  package and immutable `pocketstation-v1.1.2` tag remain unchanged.
- The workflow's guarded recovery policy already permits this workflow-only
  correction while rejecting package or product drift.


## Core 1.1.2 release candidate — 2026-08-23

- Status: `SAFE-TO-TEST`. The owner explicitly authorized the exact `1.1.2`
  version edit, main push, crates.io publication, immutable
  `pocketstation-v1.1.2` tag, and GitHub release.
- This compatible patch carries the already accepted Python SDK prerequisites
  without adding a provider, second runtime, system-output Session declaration,
  scaffold, mock, or new platform claim.
- `cargo-semver-checks` passes all 223 applicable checks against published
  Core `1.1.1`. The release commit will be published only after the complete
  release gates pass from the exact versioned tree.


## W21 Core 1.1 compatibility repair — 2026-08-21

- Status: `SAFE-TO-MERGE`; no version, tag, registry, deployment, or
  publication action is included.
- Release qualification found and removed two accidental public compatibility
  breaks relative to published Core `1.1.1`: new fields on externally
  constructible `RecordingOutcome`, and a new variant on the exhaustive
  Session `Source` enum.
- Recording manifest identity remains available through additive Session
  recording constants and the existing running Session identity. The Python
  façade derives the same typed recording result without changing the Rust
  outcome's constructible shape.
- System-mix discovery remains available. Session declaration remains the
  stable application-and-microphone 1.1 contract; a future system-output
  declaration requires a compatibility-safe design instead of an exhaustive
  enum mutation.
- `cargo-semver-checks` now passes all 223 applicable checks against registry
  `1.1.1`. The complete all-target/all-feature suite, strict Clippy,
  warnings-as-errors rustdoc, and formatting pass.
- This repair adds no provider, scaffold, mock, loopback product claim,
  physical-device claim, or remote-network claim.

## W21 public documentation and API-comment correction — 2026-08-21

- Status: `SAFE-TO-MERGE`; this step changes documentation and public Rust
  comments only. It does not change a version, tag, registry, deployment, or
  runtime contract.
- The README and getting-started path now begin with the supported developer
  task, show what runs, state prerequisites, and separate local component
  evidence from physical-device and remote-network evidence.
- Architecture, extension, Connector, signal, compatibility, and example
  pages now use the project vocabulary and identify historical or
  maintainer-only material before presenting it.
- Public Rust comments describe the resource or operation a developer uses,
  its ownership boundary, and its failure behavior without marketing language
  or claims that exceed executable evidence.
- Acceptance passes: Rust formatting, strict Clippy, warnings-as-errors
  rustdoc, every all-target/all-feature test, the release quickstart build,
  the complete `CODE_PROTOCOL` gate, Markdown link validation, and
  `git diff --check`.
- No provider, scaffold, mock, loopback-only runtime, physical-device claim,
  remote-network claim, or new product capability is introduced.

## W21 pks consumer boundary and Whisper example lock — 2026-08-19

- Status: `SAFE-TO-TEST`; no version, tag, push, publication, or release
  mutation is authorized.
- The permanent pks boundary now rejects the retired process-shaped
  transcription connector, requires the shared Relay package and registered
  `WhisperOperatorFactory`, and rejects any pks-owned connector implementation.
- Refreshed the example-owned Whisper package lock against the current Core
  candidate and applied the current formatter. Real `whisper-cli` inference
  with the checked model and an existing 16 kHz microphone fixture completed.
- The pks all-target/all-feature test and strict Clippy suites plus the
  single-engine boundary pass. No Core API, runtime, provider ownership,
  scaffold, mock, or product claim changed.

## W21 caller-owned PCM source — 2026-08-18

- Status: `SAFE-TO-TEST`; no version, tag, push, publication, registry, or
  release mutation is authorized by this task.
- `Session::pcm_source` admits caller-owned F32 PCM through preallocated
  buffers and one bounded nonblocking edge into the existing external Source
  lifecycle. It performs no OS self-capture and creates no second runtime.
- The public writer returns explicit full, closed, cancelled, wrong-writer,
  empty, channel-misaligned, wrong-frame-length, and capacity failures while
  preserving rejected-buffer ownership. Sequence, source timing, and declared
  discontinuities become ordinary Session lineage.
- `io.pocketstation.source.pcm.v1` is one stable source contract. Private
  writer ownership does not enter `SourceConfiguration`; each declaration is
  bound to Session-assigned source, stream, and stem identities. One Session
  requires one concrete sample/frame contract while each source may choose its
  own bounded capacity.
- First-party contract IDs, structural node IDs, protocol IDs, and
  configuration keys now have separate audited syntax. Shipped wire values
  remain unchanged, while provider-owned identifiers and configuration keys
  remain open and opaque to Core.
- The real-path test proves two independent PCM source declarations, bounded
  saturation, polling, Operator processing and audio reentry, Connector
  delivery, two-stem recording, cancellation, exact buffer recovery, and that
  OS capture is never opened.
- Accepted frames now drain when their writer or generated-audio producer
  closes. SPSC depth accounting cannot underflow when a consumer wins the
  publish race; `Closed` and `Cancelled` remain distinct terminal outcomes.
- Standalone declaration, lifecycle, and extension suites now live under
  scoped `tests/` directories. Built-in lowering remains extension-owned;
  the generic compiler contains no product-node switch.
- Acceptance passes: 455 Core unit tests and every all-feature target, three
  caller-owned PCM integration tests, strict
  Clippy, warnings-as-errors rustdoc, release quickstart, architecture and
  built-in-lowering boundaries, and the complete `CODE_PROTOCOL` gate.
- No provider, scaffold, mock, loopback product claim, physical-device claim,
  remote claim, or performance claim is introduced.

## W21 Core Connector maturity 2 — 2026-08-18

- Status: `SAFE-TO-TEST`; no version, tag, push, publication, registry, or
  release mutation is authorized by this task.
- The provider authoring surface is now `Connector::with_driver`,
  `ConnectorDriverFactory`, and `ConnectorDriver`. The abandoned managed-name
  API was removed.
- The unused `ConnectorPackage` composition framework was removed after a
  workspace-wide consumer audit found no pks, Relay, SDK, Lab, or Bench user.
  Sources, Operators, and Connectors retain their existing distinct Session
  registration authorities; Core does not expose a fixture-only package API.
- The self-attested conformance checklist was removed. Feature-gated
  conformance now exposes executable deterministic Session fixtures, while
  provider packages remain responsible for real protocol and receiver proof.
- Focused acceptance passes: 13 Connector contract tests and the external
  grouping test. The complete all-target/all-feature workspace suite, strict
  Clippy, warnings-as-errors
  rustdoc, release quickstart build, architecture constraints, `CODE_PROTOCOL`,
  and Core freeze-policy checks pass.
- The shared `pocketstation-relay` package and the pks, Python-native, and Bench
  consumers compile against the corrected explicit-edge declaration API. Those
  consumer checks establish source compatibility only; they do not add a Relay,
  remote, or performance claim.
- This task adds no provider implementation, competing Graph/Endpoint/Session
  authority, scaffold, mock, loopback path, remote claim, or performance claim.

## Core 1.1 connector-maturity release candidate — 2026-08-17

- Status: `SAFE-TO-TEST`; publication remains pending exact archive and
  isolated-consumer verification.
- The public 1.0.0 release and historical 1.0.x tags are immutable. The
  connector-maturity candidate is therefore versioned 1.1.0; no existing tag
  or registry version will be deleted, moved, or reused.
- `cargo-semver-checks` compared the candidate with registry 1.0.0 and passed
  all 223 applicable checks with no breaking API change. The minor version
  truthfully identifies the additive connector-authoring surface.
- The shared `pocketstation-relay` package passes 27 tests, strict Clippy,
  warning-free rustdoc, and isolated `cargo package` verification. The current
  pks workspace passes its full test suite against the candidate. Relay Go
  race gates pass for command, internal, integration, stress, and bounded
  AudioBus-subscription soak packages.
- No new endurance, remote, physical-device, cross-platform, or superiority
  claim is introduced.

## W21 Connector driver-authoring maturity correction — 2026-08-17

- Status: `SAFE-TO-TEST`; complete Core freeze qualification and legal release
  remain pending.
- `Connector::with_driver` provides the preferred provider-authoring path.
  Core owns bounded receiver polling, fair grouped delivery, delivery/drop and
  discontinuity accounting, drain/abort, startup supervision, panic
  containment, and joined Endpoint finalization. Provider code handles typed
  items and provider state instead of reconstructing an Endpoint worker loop.
- Driver factories receive `ResolvedConnectorConfiguration`; secret values do
  not regress to an author-facing string map. Sensitive Connector,
  EndpointConfiguration, and NodeConfig strings are overwritten on destruction
  and remain redacted in debug output.
- `RegisteredConnector::declare` receives the exact Session `RouteSettings`.
  Connector manifests no longer own a default route capacity, loss,
  backpressure, copy, or latency policy.
- Connector error code and retryability now survive as structured
  `EndpointFailure` details in final Session outcomes.
- The compiling authoring example no longer contains a manual receiver polling
  loop. The focused connector suite now passes 12 tests, including a real
  deterministic two-stem driver delivery and graceful-drain case.
- AUDIO-036 and the connector guide now enforce one authoritative shared
  `pocketstation-relay` package outside Core, with pks/Python/JavaScript as
  consumers or projections rather than duplicated Relay engines.
- This work adds no provider, protocol, scaffold, mock, loopback path, remote
  claim, or competitive-performance claim.

## W21 Connector Core authority requalification — 2026-08-17

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260817-w21-connector-core-authority-requalification-1`; the separate
  exact-package release/refreeze task remains pending.
- Source audit confirms `pocketstation::connector` is a thin authoring layer:
  Graph and Session own declarations and bounded routing, Endpoint owns the
  prepare/start/stop/join transaction and delivery observations, and Session
  extensions own registration. Connector adds typed provider configuration,
  redacted secrets, a focused manifest, classified failures, orthogonal
  provider-service status, an Endpoint-backed worker adapter, and conformance.
- The ownership conclusion recorded by this historical candidate was
  superseded by the later shared-connector-authority correction. The accepted
  model keeps one provider package, `pocketstation-relay`, outside Core and
  projects it through pks and language SDKs without duplicating media engines.
- This task adds no provider implementation, protocol, queue, runtime engine,
  scaffold, mock, loopback path, remote claim, or performance claim.
- Acceptance passes: 11 focused connector contract tests, the external
  grouping surface, 446 library tests plus every integration/example/benchmark
  target, strict all-feature Clippy, warnings-as-errors rustdoc, release
  quickstart, architecture constraints, `CODE_PROTOCOL`, Core freeze policy,
  and executor schema/state validation.

## W21 Endpoint shutdown-intent correction — 2026-08-17

- Status: `SAFE-TO-TEST` inside active candidate
  `pks-20260817-w21-relay-connector-package-7`; full repository and real Relay
  gates remain pending.
- The canonical Endpoint lifecycle now distinguishes graceful drain from
  cancellation abort. `RunningSession::stop` requests
  `EndpointShutdownMode::Drain`; `RunningSession::cancel` requests
  `EndpointShutdownMode::Abort`.
- The Connector worker token preserves that intent without introducing a
  Connector lifecycle. Abort monotonically upgrades drain and cannot be
  downgraded. Existing Endpoint drivers remain source-compatible through the
  default `request_shutdown` adapter to `request_stop`.
- Public Session tests prove drain and abort reach a grouped connector worker.
  The canonical Relay package separately proves graceful queue drain and fast
  abort semantics. No capture callback, realtime queue, graph compiler,
  provider protocol, or platform backend changed.

## W21 connector runtime authoring correction — 2026-08-17

- Status: `SAFE-TO-TEST` inside active candidate
  `pks-20260817-w21-relay-connector-package-7`; full repository and real Relay
  gates remain pending.
- Connector no longer exposes a competing lifecycle, delivery policy, retry
  policy, worker-queue capacity, delivery counters, or Session registry path.
  Delivery readiness, health, and recovery remain separate provider-service
  facts; Endpoint finalization remains the terminal authority.
- `Connector::new` lowers `ConnectorFactory` and `ConnectorWorker` into the
  canonical Endpoint lifecycle. Core owns the worker thread, closed start gate,
  stop token, startup-readiness deadline, joined shutdown, panic containment,
  terminal error propagation, and canonical Endpoint observations.
- `Session::register_connector` delegates to the existing atomic
  `Session::register_endpoint` extension authority.
- Ten public Connector tests and four API-boundary tests pass. They prove
  typed configuration and redaction, duplicate rejection, preparation
  rollback/cancellation, grouped application-plus-microphone ownership,
  orthogonal provider status, startup-readiness timeout, canonical delivery
  observations, saturation, joined stop, terminal failure, panic containment,
  and dependency direction.
- The exact Core candidate passes 446 library tests plus every integration,
  example, and benchmark target; strict all-target/all-feature Clippy;
  warnings-as-errors contracts-only rustdoc; the release quickstart;
  architecture constraints; `CODE_PROTOCOL`; and the Core freeze-policy gate.
- Provider protocols, network clients, codecs, queues, retry decisions, and
  credentials remain outside Core. No callback, realtime queue, capture path,
  platform, soak, remote, or competitive-performance claim is changed.

## Core 1.0 extension-first freeze activation — 2026-08-13

- Status: `REAL` governance boundary after immutable Core 1.0 publication and
  independent registry consumption; active through 2028-08-13.
- AUDIO-034 is accepted. Provider, customer, model, exporter, storage and
  application-policy behavior must remain outside Core whenever a source,
  operator, endpoint/connector, transport, SDK projection or sidecar can
  express it.
- The pull-request template requires one approved Core-change category plus
  extension-model, compatibility and realtime analysis. CI and
  `CODE_PROTOCOL` run `tools/check-core-freeze-policy.sh` so the dated
  authority and review contract cannot silently disappear.
- This activates an ownership rule, not a new platform or performance claim.
  Windows/Linux evidence keeps its existing classification, and overall
  novelty or superiority still requires separate evidence.
- The first remote CI run found a stale public-boundary assertion that still
  classified `SidecarMessage` as private even though public `RunningSession`
  signal methods and every immutable 1.0 release expose the bounded PKSS
  projection. The allowlist/test now preserve that shipped SemVer contract
  while keeping process-host workers, queues and framing I/O private. No
  runtime, callback, pool, queue or protocol byte changed.
- The next remote run reached the new README doctest and rejected one stale
  `PolledAudioFrame::sequence_number()` call. The example now reads sequence
  from `FrameLineage`, matching the published API. This is documentation-only
  and changes no runtime contract.
- The following Linux CI run passed doctests and then exposed an existing
  recording-test scheduling race: its 200 one-millisecond polls could expire
  before a loaded runner scheduled the nested recorder failure. The test now
  uses a monotonic two-second deadline and retains the exact fail-closed and
  final-accounting assertions. Production timeout, worker, queue and recording
  behavior are unchanged.

## W20 final performance qualification — 2026-08-13

- Status: `SAFE-TO-MERGE` component gate within hash-accepted candidate
  `pks-20260813-final-performance-16`; competitive classification is
  `LOOPBACK-ONLY`.
- The exact candidate passes nine callback/codec/router no-allocation cells,
  the complete boundedness/recovery unit suite, and 20 Criterion cases with
  nanosecond distributions, throughput, process CPU and accepted-baseline
  deltas. The largest non-lifecycle component p99 is 106,499 ns against the
  20,000,000 ns audio-frame budget.
- The protocol naming gate exposed three compatibility/profile tests that did
  not use mandatory `given_when_then` names. Only those test names changed;
  public API, ABI, callbacks, pools, queues and runtime behavior did not.
- No long soak, physical-device rerun, Windows/Linux rerun, remote transport,
  publication, fastest claim, or Core-freeze claim is added.

## W20 API/ABI/package compatibility — 2026-08-13

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260813-w20-api-abi-freeze-15`; all three executor predicates are
  hash-accepted.
- The default Rust surface passes 196 pinned SemVer checks against the accepted
  0.1.2 baseline. The accepted C header remains byte-identical; public type
  layouts, exported symbols, callback-table prefix compatibility and the PKSS
  1.0 golden wire vector pass.
- Previous Session C, Extension ABI 1.0 C, executable Extension ABI 1.1 C and
  codec C++ consumer paths compile against their preserved headers and execute
  against the current library.
- Cargo created and verified the locked crate from an exact clean committed
  snapshot without `--allow-dirty`, `--no-verify` or source patches. This gate
  changes no callback, pool, bounded queue or realtime executor and makes no
  publication, comparative-performance or Core-freeze claim.

## W20 public single-engine CLI projection — 2026-08-13

- Status: `SAFE-TO-TEST` inside active candidate 12; physical acceptance is
  pending.
- Central `CaptureSource` now owns selector-persistence and process-tree scope
  observations, so the CLI no longer reconstructs private capture modes.
- `EndpointAudioReceiver` exposes immutable public edge observations, allowing
  external process and relay connector packages to receive bounded audio
  without importing `PlanEdge*` runtime machinery.
- The protocol gate now runs `scripts/check_pks_single_engine_boundary.sh`,
  which rejects any return of a CLI-owned capture/compiler/runtime/pool/queue
  authority or private connector access.
- The realtime callback, fixed pools, bounded SPSC edges, saturation policy and
  audio execution path are unchanged. No soak or comparative claim was added.

## W20 public source-discovery ownership — 2026-08-13

- Status: `SAFE-TO-TEST` inside the active pks single-engine closure.
- Canonical capture now exposes platform-neutral source discovery and query
  contracts through the public SDK. macOS application/input discovery,
  Windows discovery and Linux discovery are merged and deduplicated in the
  capture owner; the CLI no longer requires `internal-testing` merely to list
  or resolve sources.
- `application_capture_available()` is a control-plane capability query only:
  it opens no capture source and creates no callback, pool, queue, compiler or
  runtime owner.
- The realtime capture implementation, pool sizes, bounded edges, saturation
  semantics and callback code are unchanged. No scaffold, mock, provider path,
  physical claim or soak was added.

## W20 Session-owned sidecar host — 2026-08-12

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260812-w20-sidecar-host-6`; acceptance is executor-manifest bound.
- Public `Session::register_sidecar` retains a versioned process contract and
  the canonical engine transactionally spawns and attaches every child.
- The bounded PKSS host owns separate data and reserved control queues, the
  `Spawned -> Hello -> Manifest -> Configure -> Ready -> Running` handshake,
  close/cancel acknowledgement, deadlines, typed failures, kill/wait/reap and
  final Session observations. Foreign work stays off realtime callbacks.
- The external Python fixture proves typed signal echo, close and cancel state
  transitions, data saturation with control delivery, crash isolation,
  hung-child deadline/kill/reap and malformed-frame failure during Session
  start. Exact acceptance, strict Clippy and release quickstart gates pass.
- No callback, audio pool or hot executor changed, so no endurance gate was
  triggered. Cross-language conformance and Core 1.0 freeze remain pending.

## W20 registered built-in lowering closure — 2026-08-12

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260812-w20-builtin-lowering-closure-5`; acceptance remains bound to
  the executor manifest rather than inferred from code existence.
- Generic Session compilation no longer matches application/microphone source
  variants or owns built-in/external-audio node type IDs. One typed
  `SessionSourceLoweringContext` carries the pipeline, source registry, typed
  node maps, and compiled bindings to component-owned lowerers.
- Session engine bootstrap no longer constructs a fixed structural node list or
  a separate lowerer list. Component registration derives collision checks from
  the registered descriptors and returns the exact lowerers installed for that
  engine.
- Built-in application/microphone capture, registered custom sources, registered
  PCM sources, and generated-audio reentry now lower through the same compiler
  extension seam while retaining their specialized runtime execution paths.
- Gates pass: the static boundary predicate, 443 unit tests plus all targets,
  ABI/integration and allocation tests, every benchmark target, strict Clippy,
  default-feature check, release quickstart, and the full protocol check.
- No callback, frame pool, `rtrb` edge, saturation policy, hot-path `Drop`,
  realtime executor, provider/domain type, physical claim, or soak changed.

## W19 composition fault and bounded-scaling closure — 2026-08-08

- Status: `SAFE-TO-MERGE`; all W19 executor tasks and mandatory predicates are
  `DONE`/`PASS` and execution is deliberately `PAUSED` before W20.
- Ten exact deterministic cells prove Session branch saturation isolation,
  independent source survival after another source fails, operator timeout and
  bounded cancellation, typed-edge branch isolation, recorder-branch failure
  isolation, generated-audio pool/ingress exhaustion accounting, composed
  Session ownership, and transactional endpoint-start rollback.
- The hash-verified artifact binds capacity 8, peak depth 4, final depth 0,
  eight pool slots, 960 samples per frame, and the exact 61,440-byte queued
  audio upper bound. Four signals were enqueued and received with zero
  unexplained loss.
- W19 starts no sidecar child process; process lifecycle remains a gated W20
  concern. No physical-device, remote-network, endurance, or product claim was
  added, and no soak ran.

## W19 Session-owned generated-audio reentry — 2026-08-08

- Status: `SAFE-TO-MERGE` for task `W19-GENERATED-AUDIO-REENTRY`; its three
  mandatory predicates are bound to the hashed acceptance manifest. The later
  deterministic fault/scaling task also passed, so W19 is now `DONE`.
- Public `Session` owns an exclusive bounded typed-PCM receiver, normalization
  bridge, dedicated pool, authoritative lineage projection, existing
  plan-source ingress, cancellation, graceful finish, join, and final metrics.
  Operator PCM output stays on the typed async lane until this explicit bridge;
  captured realtime audio stays on the specialized audio lane.
- The generic typed edge now reports exact capacity, current/peak depth,
  enqueue, receive, and declared-drop counters. Its final fan-out branch moves
  the sole `Arc` rather than retaining a racing publisher reference, making
  exclusive generated-audio ownership deterministic.
- Compilation rejects a second consumer of the same named PCM output, including
  a second reentry. Deterministic tests cover ingress saturation, pool
  exhaustion, graceful drain, source/operator/bridge close, and a downstream
  endpoint-delivery scheduling race; the public Session test passes 25 repeated
  runs.
- Final gates pass: 390 unit tests plus every target/integration, strict
  all-target/all-feature Clippy, no-default check, quickstart, hot-path
  allocation tests, full code protocol, and the external public-API consumer.
- No callback, pool ownership, hot executor, provider/domain type, mock,
  physical-device claim, product-claim upgrade, or soak was introduced.

## W17 central Core 1.0 boundary corrections — 2026-08-08

- Status: `SAFE-TO-MERGE` for task
  `W17-CENTRAL-BOUNDARY-CORRECTIONS`; executor completion is bound to the
  hashed acceptance manifest, not inferred from component test success.
- Public `SignalSpec`, `PortSpec`, `RouteSettings`, `NodeDescriptor`, operator
  manifests, source manifests, envelopes, frames, and lineage records expose
  checked construction and read-only access rather than mutable public
  representations.
- `SessionSpec` owns one `ConnectionSpec` collection. Captured stems, external
  source outputs, and operator outputs delegate through one internal stream
  handle; Rust `Stream<T>` remains declaration-time typing over stable
  `SignalSpec`, not a second runtime or an ABI type.
- Graph preparation has one port-aware record. The asynchronous bridge and
  typed fan-out now use one bounded `SignalEdge` implementation with explicit
  owned versus shared payload ownership; the specialized audio edge remains
  separate.
- The generic Session compiler no longer switches on connector, browser,
  recorder, or generated-audio identities. Endpoint-owned configuration is
  copied generically and the audio-reentry package enters through the
  registered graph-lowering seam.
- Unaccepted DSP, codec profile/mock, capture compatibility, recording
  coordination, runtime-node, and experimental timing scaffolds are absent
  from the shipping package. The empty experimental directory was removed and
  is not recoverable except from version control.
- Acceptance passes: strict no-default dead/unused check, 385 central unit
  tests plus all targets and integrations, strict Clippy, release quickstart,
  protocol/hot-path laws, and the external public-consumer artifact with an
  adversarial verifier self-test.
- No audio callback execution, buffer-pool ownership, or hot executor changed;
  W10 endurance evidence remains applicable and no soak was run.

## W18 Session-owned external source lifecycle — 2026-08-08

- Status: `SAFE-TO-MERGE` for task `W18-SOURCE-LIFECYCLE-EVIDENCE`.
  Central gates and the external public-Session artifact pass; executor
  completion remains hash-binding work and is not inferred from this label.
- `PreparedSession` now owns external source branch mappings. Startup prepares
  every source driver, bounded typed branch, endpoint, and PCM ingress bridge
  behind the common closed start gate; only a fully prepared transaction
  starts workers. `RunningSession` owns cancellation, joins, final source
  observations, and failure accounting.
- Custom typed signals use the shared bounded typed-edge runtime without an
  invented audio `SampleSpec`. External PCM crosses one bounded generated-audio
  ingress boundary into the existing specialized audio plan. The realtime
  callback, audio pool owner, and realtime executor are unchanged.
- `SourcePrepareContext` supplies the exact Session, source, and named-output
  stream identities. Runtime emissions fail closed on identity mismatch and
  observe generation, discontinuity, recovery, and policy transitions.
  Endpoint preparation receives a typed source-route identity; non-audio
  endpoint preparation no longer manufactures an audio context.
- Deterministic tests prove a typed-only Session, external PCM, saturated-branch
  isolation, generation/discontinuity transition, independent-source survival
  after another source fails, transactional gate ordering, cancellation, join,
  and clean driver close. The existing application/microphone suite remains
  green.
- Acceptance passes: 506 central unit tests plus all ABI, allocation,
  conformance, facade, integration, and quickstart targets; strict all-target/
  all-feature Clippy; release `quickstart`; and the external Lab
  verifier/artifact using public `Session` only.
- No scaffold, mock, provider/customer/domain type, direct-registry acceptance
  fixture, realtime hot-path change, physical-device claim, product-claim
  upgrade, or soak was introduced.

## W18 Session source declaration and compiler lowering — 2026-08-08

- Status: `SAFE-TO-MERGE` for task `W18-SESSION-SOURCE-COMPILER`; W18 as a
  whole remains `PARTIAL` until the next executor task proves source worker
  lifecycle, cancellation, replacement, failure, saturation, observations,
  and an external public-API consumer.
- The shipping public `Session` now declares an external source by open
  `SourceTypeId`, retains caller-owned `SourceFactory` registrations, selects
  named manifest outputs, and exposes stable Session-assigned source, stream,
  and instance identities. The internal declaration owner freezes those values
  into Session schema 1.3 without adding a closed `Source` variant.
- `SessionEngineBuilder` validates and registers each source manifest as a
  zero-input graph definition. Session compilation resolves the exact source
  factory and configuration, validates every selected output, lowers direct
  and operator-bound routes through the normal graph compiler, and records
  connected root outputs in `RuntimePlan`.
- Audio outputs use the existing bounded audio memory plan; custom typed
  outputs use the existing bounded typed-edge plan. No signal-specific queue,
  invented non-audio `SampleSpec`, worker lifecycle, or alternate graph/runtime
  was introduced. Application and microphone declarations retain their
  optimized path.
- Acceptance passes: 503 unit tests plus every ABI, allocation, conformance,
  façade, and integration target; 8 focused public/Session source tests; the
  neutral graph-root boundedness predicate; strict all-target/all-feature
  Clippy; release `quickstart`; formatting; and
  `scripts/check_protocol.sh`.
- No scaffold, mock, loopback path, provider/customer/industrial type,
  realtime callback change, physical-device claim, product-claim upgrade, or
  soak was introduced.

## W17 final signal/API hardening — 2026-08-08

- Status: `SAFE-TO-MERGE`; all central gates and the clean external-contract
  proof pass. Final executor `DONE` remains commit- and hash-binding work, not
  an inferred result from these commands.
- `SignalEnvelope` now has exactly five authorities: `payload`, `spec`,
  `timing`, optional source-independent `lineage`, and optional generic
  `derivation`. The former mutable mirror fields and audio-origin-only derived
  lineage contract are removed.
- `SignalDerivation` references generic upstream `SignalLineage` and
  `SignalTiming`. `FrameLineage` is projected once when specialized
  `AudioFrame` crosses into the typed lane; the realtime audio executor and
  pooled frame representation remain specialized.
- Transcript role constructors/constants moved to the external Whisper example.
  The core event payload is now an open type-id/bytes record, generated audio
  uses the generic `Generated` provenance tag, and the future diarization-only
  frame field is removed. No provider, customer, industrial, `Flight*`, or
  transcript-policy authority remains in the public core implementation.
- The external Whisper package compiles warning-free and passes all 15 tests on
  the canonical contract. The Lab hardening proof compiles a separate public-
  API consumer, checks exact canonical fields and payload coverage, and proves
  the removed duplicate API fails compilation for the intended reason.
- Acceptance passes: 492 central unit tests plus every ABI/allocation/facade
  target, strict all-target/all-feature Clippy, release `quickstart`,
  `scripts/check_protocol.sh`, the Lab adversarial verifier, and the exact
  candidate artifact verifier. No mock, scaffold, physical-device claim,
  product-claim upgrade, or soak was introduced.

## W20 Core 1.0 extension completeness — 2026-08-08

- 2026-08-09 ownership-remediation slice: `RUNNING`, not accepted. The central
  package is being reorganized around explicit authorities:
  `session/{declaration,compile,prepare,lifecycle,extensions}`,
  `runtime/{audio,signal,bridge,lifecycle}`, split frame/capture/graph/endpoint/
  recording modules, and restored codec/timing owners. Empty legacy module
  directories are gone; no DSP placeholder was restored.
- `CompiledSessionBindings` is now the sole typed bridge from lowered graph
  nodes to Session declarations. Built-in application/microphone selectors are
  no longer serialized into `NodeConfig`; structural ingress nodes receive an
  empty configuration and runtime preparation consumes the typed Session
  declaration. External source, operator, and endpoint configuration remains
  extension-owned and opaque.
- Connector identity now follows the same typed path: `ConnectorId` is retained
  by the endpoint declaration, frozen into `EndpointSpec`, lowered into the
  compiled endpoint binding, and supplied through `EndpointPrepareContext`.
  The former `"connector_id"` configuration injection and endpoint-side
  string parser are removed.
- Endpoint authoring contracts now have one public Rust location at the crate
  root. The endpoint implementation namespace is private, and root contracts
  are re-exported from their actual graph, frame, declaration, extension, and
  lifecycle owners instead of being presented as Session-owned machinery.
- Realtime/session numeric identities retain their compact scalar layout and
  internal zero-cost field access, but the tuple field is no longer public.
  External consumers use `Id::new(...)` and `.get()`, preventing the scalar
  representation from becoming a frozen public-field contract. The central
  examples, integration targets, active CLI, and Lab fixtures were migrated;
  archived `audio-ml` remains archived rather than being pulled into Core.
- The former mixed graph signal file is now separated into payload,
  timing, lineage/derivation, envelope validation, continuity, asynchronous
  preparation, and operator-contract modules. `SignalTiming` and
  `SignalLineage` now have checked constructors and read-only accessors;
  `SignalDerivation` is read-only after construction. Current external examples
  and Lab fixtures were migrated away from writable representation fields.
- Capture remains an explicit owner in this cleanup. Platform adapters retain
  fixed-capacity audio pools, bounded `rtrb` crossings, nonblocking saturation
  accounting, and the callback prohibition on allocation, locking, blocking,
  async work, logging, and panic. Existing capture/pool Criterion targets are
  preserved. This slice changes no callback, pool, queue, or drop semantics.
- The misleading `capture/source/` namespace was removed. Capture selection,
  authorization, stable native identity, callback observations, runtime events,
  and sample-time mapping now have direct modules under `capture/`; the generic
  extensible Source contract remains exclusively under `session/extensions/`.
- The lifecycle start/stop contract has been separated from the Session
  runtime orchestrator, and language-neutral signal identifiers are opaque
  rather than public tuple representations. No provider/customer/domain type,
  second engine, mock, fallback, or loopback path was introduced.
- Verification is deliberately deferred by user direction. This entry records
  implementation state only and does not upgrade W20, Core 1.0, performance,
  platform, or release acceptance.

- Status: `PARTIAL`; the component/package candidate passes, but Core 1.0 is
  not frozen. The preserved artifact is package `0.1.2`, `LOOPBACK-ONLY`,
  produced with `--allow-dirty --no-verify`, and explicitly records
  `clean_worktree_claimed=false`.
- The public Rust `Stream<T>` façade provides compile-time composition without
  creating a generic runtime. External packages define marker types through
  `StreamSignal`; runtime and cross-language identity remains the stable
  `SignalSpec`, schema, named-port, edge-contract, and plan representation.
- `pocketstation.h` now exposes a versioned source/operator/endpoint descriptor
  ABI. Every record validates ABI version, struct size, pointer alignment,
  lengths, UTF-8, port direction, unique names, and extension shape before any
  caller memory can be retained.
- The versioned `PKSS` sidecar frame carries stable signal identity, role,
  schema, sequence, timestamp, bounded payload, terminal state, and control
  kind. Decode rejects unknown versions, invalid flags, oversized fields,
  invalid UTF-8, truncation, and trailing bytes. It performs no callback work.
- The packaged `pocketstation 0.1.2` proof builds an exact-version external Rust
  consumer, replays W18 open-source and W19 composition contracts, links a C
  descriptor consumer to `libpocketstation`, and round-trips a Python sidecar.
  The independently verified artifact remains `LOOPBACK-ONLY`; it upgrades no
  platform or product claim.
- Full central tests (494 unit tests plus ABI, allocation, façade, and
  integration gates), strict all-target/all-feature Clippy, release quickstart,
  Bench (36 tests), Python SDK (19 tests), and Node SDK (14 tests) pass.
- No provider, customer, industrial/domain payload, second engine, scaffold,
  mock, hot-path change, physical-device claim, or soak was introduced.
- Remaining exit: executable C source/operator/endpoint registration,
  Session-owned bounded sidecar lifecycle, completed W18/W19 Session paths,
  clean installed or published external consumption, compatibility gates, and
  a real `1.0.0` release.

## W19 operator composition and generated-audio reentry — 2026-08-08

- Status: `PARTIAL`; focused low-level composition and generated-audio gates
  pass, but the execution fixture manually assembles workers, fanout,
  plan-source ingress, and bridge outside Session.
- `DerivedStreamHandle` can declare another `through(...)` stage. Explicit
  `through_ports(...)` and `output(...)` selection lower named operator ports
  into the Session graph; simple `through(...)` remains fail-closed 1x1 sugar.
- `AsyncOperatorWorker::spawn_composed(...)` executes multiple named bounded
  typed inputs and outputs. Three independently registered external operators
  execute in sequence through the shared `TypedEdgeFanout` runtime, and
  multi-input/multi-output manifests route by declared `SignalSpec`, schema,
  media, and semantic role.
- Asynchronously produced PCM is accepted only through
  `GeneratedAudioBridge`: an exclusive typed branch validates format and frame
  size, copies into a dedicated bounded pool, restores authoritative timing and
  lineage, then performs a nonblocking send into the existing plan-source
  ingress. The realtime callback/executor contract is unchanged.
- Runtime policy and observations remain signal-generic. No transcript,
  provider, customer, telemetry, industrial, or other domain payload type was
  added; no scaffold, mock, physical-device claim, or soak was introduced.
- Remaining exit: Session-owned execution of derived non-audio chains, one
  operator instance with multiple named upstream inputs/outputs, and bounded
  generated-audio route lifecycle with fault isolation.

## W18 open source registration and bounded typed ingress — 2026-08-08

- Status: `PARTIAL`; focused low-level central and external-consumer gates
  pass, but the fixture constructs `SourceRegistry` directly and does not
  register or declare the external source through Session.
- Public `SourceManifest`, `SourceTypeId`, `SourceFactory`, `SourceDriver`,
  `SourceRegistry`, configuration, emission, lifecycle, cancellation, and
  observation contracts accept externally owned source implementations without
  provider or domain enums in core.
- `TypedEdgeFanout` is the one bounded source-independent fan-out owner for
  external typed sources and async operator outputs. Each branch has explicit
  capacity, delivery/drop counters, and fail-closed `MustDeliverOrFail`
  terminal behavior.
- Source outputs validate declared port, `SignalSpec`, schema, `MediaCaps`,
  lineage, sequence, time, discontinuity, generation, and replacement before
  delivery. External audio remains on the non-callback source boundary and
  retains authoritative `FrameLineage` projection.
- The specialized realtime `AudioFrame` executor and native application and
  microphone capture remain unchanged. No named-port composition, operator
  chaining, generated-audio reentry, sidecar, provider behavior, scaffold,
  mock, or product-claim upgrade is introduced.
- Remaining exit: public Session factory registration and external-source
  declaration compiled into Session start/stop/cancel/replacement/fault and
  observation semantics. Current Session still requires exactly one
  application and one microphone source.

## W17 source-independent signal contract — 2026-08-08

- Historical status: `DONE` at the earlier named `LOOPBACK-ONLY` contract
  boundary; superseded for Core 1.0 acceptance by the final W17 hardening task
  above.
- The public async boundary is now `SignalEnvelope` plus `SignalPayload`, with
  source-independent `SignalLineage` and `SignalTiming`. The optimized
  `AudioFrame` realtime lane is unchanged.
- Schema-backed custom signals, encoded audio, text, events, metrics, control,
  and binary payloads have explicit `SignalSpec`/`MediaCaps` symmetry. Invalid
  payload/specification pairs fail closed.
- `SignalContinuityTracker` validates stable identity, monotonic timestamps,
  sequence continuity, declared discontinuities, source generations, and
  policy epochs deterministically. Generic terminal delivery is
  `MustDeliverOrFail`; the runtime no longer owns transcript-named policy or
  observation concepts.
- The misleading public `SessionFlight*` vocabulary is removed before API
  freeze. Session lifecycle diagnostics now use `SessionTrace*`,
  `session_trace(...)`, `.pkstrace`, and `trace.validate()` consistently.
- Full central acceptance passes: 482 unit tests plus C/C++ ABI, allocation,
  conformance and façade tests; strict all-target/all-feature Clippy; and the
  release `quickstart` build.
- No source factory, typed source ingress, operator chaining, named-port API,
  generated-audio bridge, sidecar, provider integration, scaffold, mock, or
  product-claim upgrade is introduced in this step.

## W13 recorder initialization observation repair — 2026-08-02

- Status: `SAFE-TO-TEST`; candidate
  `pks-20260802-w13-operational-trust-3` is frozen as
  `pocketstation-0.1.2.crate` with SHA-256
  `e6943cad4c16af22492d880a28e0e6c10b957d031352c675dd405efd41c42657`.
- The Session multistem recorder now publishes received-frame progress as soon
  as it accepts each authoritative first frame. Previously the outer endpoint
  telemetry remained at zero while filesystem-backed recorder initialization
  ran, so a valid received frame could be temporarily invisible under load.
- The permission-epoch fail-closed test exposed the race during the full
  all-target gate. The focused regression now passes without weakening the
  permission lineage assertion or recording finalization behavior.
- The complete central gate passes: 470 unit tests plus C/C++ ABI,
  allocation, integration, façade, quickstart, package verification, strict
  Clippy, CODE_PROTOCOL, and architecture constraints.
- This changes observation timing only. It adds no queue, sleep, capture
  fallback, scaffold, mock, or product claim.

## Historical W13 `-2` Core Audio timeline candidate — 2026-08-02

- Historical status: `PARTIAL`; candidate
  `pks-20260802-w13-operational-trust-2` is frozen as
  `pocketstation-0.1.2.crate` with SHA-256
  `6d4de7597af33d5ceaba0724ba30420b2f3c170691f769018ba4e59ea01ae5bf`.
- The macOS process-tap callback now publishes the native Core Audio host time
  together with its absolute sample-frame position through a lock-free
  seqlock snapshot. Rust converts that native sample timeline into the shared
  PocketStation process-monotonic clock. It fails closed if the native
  timeline is unavailable instead of anchoring time when Rust first polls.
- The callback remains allocation-free, lock-free, blocking-free, async-free,
  log-free, and panic-free. Focused timeline tests cover reader positions on
  both sides of the callback anchor and native-host to process-clock mapping.
- Central formatting, 470 unit tests plus C/C++ ABI, allocation and façade
  tests, all-target/all-feature strict Clippy, CODE_PROTOCOL, architecture
  constraints, package verification, CLI 195 tests plus strict Clippy,
  neutral Bench 36 tests plus strict Clippy, and relay race tests pass.
- Physical macOS evidence for the exact rebuilt CLI binary proves two distinct
  application process incarnations, disappearance and explicit reselection,
  two complete Sessions, independent application and microphone recording,
  connector and same-host browser routes, browser reconnect, zero route drops,
  zero continuity gaps, and complete common-clock latency sample coverage.
- This does not prove denied/revoked permission transitions, Windows or Linux
  native behavior, the final 3,600-second soak, or clean-source reproduction.
  W13 remains `PARTIAL`; no scaffold, mock, fallback, or loopback-only product
  path was added.

## W16 single-package consolidation — 2026-08-02

- Status: `SAFE-TO-MERGE`; W16 is `DONE` at its local consolidation boundary.
  The central implementation is exactly one Cargo
  package named `pocketstation`, with internal frame, timing, graph, runtime,
  capture, endpoint, recording, codec, DSP, Session, observation, and ABI
  modules. The old `crates/` package tree is removed.
- The package emits Rust, static-library, and dynamic-library forms. Native
  consumers include `pocketstation.h` and link `libpocketstation`; the former
  Session and codec C packages are unified ABI modules. Retained `pks_*`
  symbols are compatibility only.
- The CLI, Python and Node native adapters, external examples, Lab fixtures,
  and neutral benchmark now consume the root package. The CLI's 195 tests,
  benchmark's 36 tests, strict Clippy, relay race tests, and central 463 unit
  tests plus ABI/allocation/integration tests pass locally.
- Fresh independent W12 Rust, W12 Python/Node, W15 real-whisper, and
  session trace artifact verifiers pass after consolidation. Their
  classifications remain `LOOPBACK-ONLY` or `PARTIAL`; this work creates no
  new physical-device claim.
- AUDIO-033 supersedes the historical multi-package topology. `runtime::metrics`
  is only the runtime observation implementation, not a separate product or
  package. New Cargo packages require an independently consumed/shipped or
  unavoidable toolchain boundary.
- The packaged crate hash and W16 evidence events are recorded in the
  workspace `docs/execution/evidence/`. W13 now owns fresh operational
  requalification of this exact candidate.

## W15 typed asynchronous STT local acceptance — 2026-07-30

- Status: `DONE`, `LOOPBACK-ONLY`; the central implementation and strict live
  Lab artifact are accepted at their named local evidence boundary.
- Public `pocketstation::Session` owns typed operator registration,
  `through(operator)`, terminal endpoints, 16 kHz signal propagation,
  graceful finish, explicit cancellation, compiled-input observations, and
  derived-route observations without a duplicate scheduler or counter owner.
- The example-owned Whisper operator runs real whisper.cpp CPU processes and
  records actual argv, PID, timestamps, logs, content hashes, transcript,
  timeout/cancellation, and killed/waited/reaped outcomes.
- The isolated external consumer proves typed partial/final lineage, bounded
  derived pressure, and a healthy raw browser/recording branch. The success raw
  branch delivered 751/751 frames with zero drops.
- Central graph, runtime, endpoint, Session, portable C, façade, Whisper,
  formatting, protocol, and strict Clippy gates pass. The embedded and
  independent Lab verifiers pass against `/private/tmp/pks-w15-live-final`.
- Capture uses a Lab speech fixture. This does not upgrade W15 to
  `REAL-DEVICE-PROVEN`, introduce provider code in core, or start W16/mobile.

## W12 focused recording and foreign-audio ownership — 2026-07-29

- Status: `SAFE-TO-TEST`; concrete multistem WAV implementation now lives in
  `pks-recording`, behind the unchanged `pks-endpoint` lifecycle. The generic
  endpoint contract, graph declarations, runtime delivery, and Session
  lifecycle remain in their existing owners.
- The bounded polled-audio endpoint moved from the grab-bag `pks-nodes`
  package into `pks-session`, which already owns its queue, batch, lease,
  observation, and cross-language projection semantics.
- `pks-session` no longer depends on `pks-nodes`. It registers the canonical
  grouped recorder through `register_multistem_recording`, retains a safe
  `SessionRecordingReceipt`, and exposes only terminal recording outcomes.
  Callers supply an artifact root; Session route context and capture-owned
  frame lineage remain authoritative.
- `pks-nodes` and `pks-dsp` are deferred non-registry packages.
  The supported `pocketstation` publication closure derives to 14 packages
  instead of 15 and contains `pks-recording`, not `pks-nodes` or `pks-dsp`.
- Focused acceptance passes: 36 transitional-node tests, 15 recording tests,
  59 Session tests, and three non-empty façade tests. The derived publication
  dry run validates all 14 closure packages in dependency order, and strict
  all-target Clippy passes for all four focused packages.
- No recording behavior, queue bound, lineage rule, hot path, provider,
  scaffold, mock, fallback, loopback classification, or product claim changed.
  Full workspace, quickstart, architecture, and CODE_PROTOCOL gates remain
  before the boundary candidate can pass.

## W12 stable public Session error codes — 2026-07-28

- Status: `SAFE-TO-TEST`; canonical `pks-session` now owns stable, namespaced,
  language-neutral codes for declaration, start, runtime, bounded audio-poll,
  stop status, and every retained stop-failure cause. The public Rust façade
  re-exports this contract and owns only mappings from its wrapper errors.
- The codes are additive to the published typed Rust errors. Existing error
  enums and method signatures remain unchanged, while Python and Node can
  normalize against values such as `session.start_cancelled`,
  `session.invalid_selector`, and `audio.lease_capacity_exhausted`.
- The string returned by `as_str()` is the compatibility contract; Rust enum
  variant names and discriminants are not. Exhaustive tables pin every current
  string and validate namespace syntax and uniqueness. Declaration variants,
  nested capture/start classes, audio-poll cases, and stop-cause projections
  have focused mapping coverage.
- AUDIO-032 records the measured package-boundary repair: concrete multistem
  recording moves from the broad `pks-nodes` package to `pks-recording`, while
  bounded foreign-audio projection moves to its canonical Session owner.
  Graph/caps/metrics and native-capture package convergence remain W15 work.
- All 53 `pks-session` tests and three non-empty `pocketstation` façade tests
  pass. Strict focused Clippy, CODE_PROTOCOL, and the recording extraction
  remain the next acceptance steps for the active boundary-repair candidate.
- This changes no engine, capture, recorder, queue, hot path, provider,
  scaffold, mock, fallback, or product claim.

## W12 Session-owned endpoint route and timeline context — 2026-07-28

- Status: `SAFE-TO-TEST`; this is the additive setup-context prerequisite for
  the Session-owned recording reference. Recorder composition and outcome
  projection remain separate follow-up work.
- `pks-endpoint` now defines typed `EndpointRouteContext` and
  `SessionTimelineOrigin` values. Canonical endpoint preparation can consume
  exact stem, route, and monotonic-origin identity without parsing reserved
  string configuration.
- The published `EndpointPrepareContext::new` signature remains unchanged.
  Its additive Session-route context is absent for legacy callers and attached
  explicitly by canonical `pks-session` startup.
- `pks-session` samples the shared monotonic clock exactly once after the
  initial cancellation gate and supplies that same origin to every endpoint
  input in the startup transaction. Each input also receives its compiler-owned
  stem and route identity.
- Focused evidence passes: eight `pks-endpoint` tests and 49 `pks-session`
  tests. The Session regression observes all six product routes, two distinct
  stems, unique route IDs, and one identical nonzero timeline origin.
- This changes no capture callback, realtime router, queue capacity, endpoint
  worker, recorder behavior, public provider surface, scaffold, mock,
  fallback, or loopback-only product claim.

## W12 language-owned Rust Session façade — 2026-07-28

- Status: `PARTIAL`; the central Rust façade and canonical-engine fixture pass,
  while the independent Lab clean-consumer artifact remains the W12 acceptance
  owner.
- Added one Cargo package/library named `pocketstation`. Its public `Session`
  and `RunningSession` are thin owners over the canonical `pks-session`
  declaration, native host, capture/runtime transaction, bounded polled-audio
  endpoint, events, metrics, cancellation, and idempotent stop. No scheduler,
  capture backend, counter, or lifecycle rule was duplicated.
- `Session::new()` is infallible and keeps engine setup internals out of the
  developer declaration. `start()` builds the native host with bounded defaults
  and reports host, compile, startup, missing-receipt, and missing-event states
  through typed errors. The public quickstart contains no `PrepareContext`,
  queue capacity, node ID, `CaptureBackendSet`, or CLI/subprocess delegation.
- The default-disabled `conformance-fixtures` feature supplies deterministic,
  distinct application and microphone frames to the same canonical host. It is
  explicitly `LOOPBACK-ONLY`, inventoried, and cannot upgrade a product claim.
  Three focused tests prove two independent stems cross bounded destinations,
  lifecycle/events/metrics are observable, stop is idempotent, cancellation is
  typed, and an invalid selector fails rather than reporting success.
- The unconsumed `pks-audio` compatibility package is retired. Its allocation
  gate moved to the codec owner, its executable graph example moved to the node
  owner, and its obsolete duplicate quickstart/local-proof/soak helpers were
  deleted rather than preserving a second public identity.
- Every internal path dependency in the public façade's Cargo closure now also
  carries its exact compatible version, and previously anonymous closure crates
  have truthful package descriptions. This is package readiness only; no crate
  was published and no registry-consumer claim is made before the dependency
  closure exists in the registry.
- All 19 workspace packages declare an explicit registry role. The release
  dry-run derives the exact 15-package normal/target dependency closure of the
  public façade, validates it in Cargo dependency order, and keeps
  `pocketstation` last. The codec C ABI, Session C ABI, and Whisper example
  remain explicitly non-publishable rather than leaking into that closure.
- Main/PR CI now runs workspace tests, strict all-target/all-feature Clippy,
  the release quickstart build, architecture and CODE_PROTOCOL checks, and the
  exact 15-package publish dry-run. The actual crates.io job is release-only:
  it rejects prereleases, non-`pocketstation-v<workspace-version>` tags,
  commits outside `main`, dirty checkouts, and any failed validation. It reads
  the scoped token from the job environment and does not run `cargo login`.
  GitHub's `crates-io` environment is the deployment boundary where repository
  owners can require approval and scope the token. The token is explicitly a
  first-release bootstrap: after `0.1.0`, each crate can authorize the
  repository through crates.io trusted-publishing OIDC and the long-lived
  secret must be retired. No crate was published by this work.
- No provider code, first-party connector catalog, browser/relay implementation,
  recording implementation, mock product path, per-frame foreign callback,
  unbounded queue, or new device claim was introduced. Connector/browser/
  recording parity remains outside this central façade slice until their
  reusable owners are extracted from callers.

## W11 transactional capture-delivery start boundary — 2026-07-28

- Status: `SAFE-TO-TEST`; the component correction is complete and the
  product-path lab rerun remains the acceptance gate.
- The W11 product proof exposed a real startup defect: capture backends could
  publish into their bounded streams while endpoint workers were still held
  behind the Session start gate. The runtime then admitted that backlog as one
  burst, overflowing otherwise healthy route edges.
- `pks-capture` now owns a one-way, atomic capture-delivery start gate.
  `pks-session` is its sole controller and opens endpoint workers first, then
  capture delivery, immediately before publishing `Running`. Frames produced
  before that transaction boundary are not admitted as product frames and are
  counted explicitly as
  `frames_discarded_before_start_total`.
- The callback-side check remains allocation-free, lock-free, blocking-free,
  async-free, log-free, and panic-free. No capacity changed; no sleep, retry,
  pacing, endpoint dependency, or consumer coordination entered production
  code.
- A deterministic regression emits sixteen frames per source during backend
  open and delays endpoint consumption. It proves those pre-`Running` frames
  are explicitly accounted, each source delivers its first post-start frame,
  every source-ingress rejection/discard counter stays zero, and every
  destination-edge drop counter stays zero.
- The C conformance fixture now delivers its observable audio after the
  Session start boundary. The pre-start counter is deliberately not appended
  to the ABI 1.1 source record: that output function has no caller-size
  negotiation, so growing its 176-byte record would be unsafe for an older
  compiled caller. The README names this projection gap instead of claiming
  complete captured-stream observations. The ABI 1.0 aggregate canary remains
  unchanged.
- Focused acceptance passes: 51 `pks-capture` tests, 49 `pks-session` tests,
  12 `pks-session-c` tests, two executable C harness tests, the standalone C
  conformance script, formatting, and strict all-target/all-feature Clippy for
  all three owners.
- No production scaffold, mock, fallback, loopback-only path, queue inflation,
  or duplicate lifecycle implementation was introduced.

## W11 codec C ABI ownership extraction — 2026-07-28

- Status: `PARTIAL`, `SAFE-TO-TEST`; the central Rust ownership correction is
  complete. Mobile consumption remains source-level compatibility only until
  the separate iOS and Android package/link migrations and native link tests
  pass in their owning repositories.
- Added sibling `pks-codec-c`, depending only on `pks-codec`, as the sole owner
  of the retained Opus C compatibility ABI. The dynamic/static library retains
  the existing `pks_encode_opus`, `pks_opus_encoder_create`,
  `pks_opus_encoder_destroy`, and `pks_opus_encoder_set_bitrate` symbols.
- Moved all ten ABI behavior tests to the new owner. `pks-audio` is now a Rust
  façade only: it no longer owns C exports, a build script, cbindgen, or
  `cdylib`/`staticlib` artifact types.
- The checked header now lives at `crates/pks-codec-c/include/pks_codec.h`.
  Builds generate only into Cargo `OUT_DIR` unless the caller explicitly sets
  `PKS_CODEC_C_HEADER_OUTPUT`. The repository no longer has an ambiguous root
  `ffi/` directory.
- Replaced `scripts/sync-ffi-header.sh` with
  `scripts/sync-codec-c-header.sh`. It generates outside the source tree and
  targets the canonical SDK paths
  `Sources/PocketStationCodecFFI/pks_codec.h` and
  `sdk/src/main/cpp/pks_codec.h`.
- `scripts/sync-codec-c-header.sh --check` is non-mutating and fails closed
  when the generated header differs from the central checked copy or either
  SDK compatibility copy. Unknown arguments fail instead of silently
  triggering synchronization.
- Focused evidence: all ten `pks-codec-c` tests pass; the checked header is
  byte-identical to a fresh explicit-output build; and the debug dynamic
  library exports exactly the four retained `pks_*` codec symbols.
- The generated header is C++ compatible. A separate C++17 executable now
  includes the checked header, links the actual `pks-codec-c` library, creates
  an encoder, encodes one 20 ms frame, and destroys it. This prevents JNI
  consumers from silently compiling against mangled C symbols.
- This introduces no Session ABI, runtime, provider, scaffold, mock, fallback,
  or loopback-only product path.

## W11 portable Session C lifecycle and conformance — 2026-07-28

- Status: `PARTIAL`, `SAFE-TO-TEST`; this checkpoint supersedes the current-
  state claims in the earlier W11 Session-ownership, host-foundation, and
  initial-C-boundary entries below. Those entries remain as historical
  evidence and must not be read as the current implementation inventory.
- `pks-session-c` now projects the real native `SessionEngineHost` through
  versioned records and opaque generational engine, Session, and audio-batch
  handles. The exported boundary covers the narrow application-plus-default-
  microphone declaration, compile, start, stop, state and event polling,
  bounded metric and audio polling, immutable frame access, explicit batch
  release, Session destruction, and engine destruction.
- Rust-record and checked-in C-header layout parity tests cover the complete
  public record surface. ABI entry points validate output pointers before
  acquiring engine, Session, or lease resources, and panic containment maps an
  unwind to a typed status without crossing the foreign boundary.
- The default executable C harness proves the real native-host failure path
  with a deliberately missing application source. It verifies compiled-to-
  failed lifecycle truth, foreign-handle rejection, stale engine rejection,
  bounded event and metric access, and recovery after the failed Session. It
  does not claim successful capture or audio delivery.
- The `conformance-fixtures` feature is test-only. With that feature,
  `scripts/test-session-c-conformance.sh` builds the adapter and compiles and
  runs a separate C executable against it. The executable proves successful
  application-plus-microphone execution through the canonical runtime, two
  distinct source and stem lineages, bounded lease-exhaustion observations,
  sample pointer and value stability while a lease is retained across Session
  stop, stale double-release rejection, and usable ABI recovery after an
  intentionally contained panic.
- The fixture exports are absent from the default library and public header.
  No synthetic capture symbol, test control, or panic trigger enters the
  production ABI.
- ABI version 1 intentionally permits exactly one Session for an engine's
  lifetime. The engine-scoped polled-audio receipt is consequently isolated by
  the ABI contract; concurrent or sequential Session reuse is not implied.
- A concurrent foreign stop no longer waits for the global engine table or the
  start-held runtime mutex before publishing its request. Each engine owns a
  shared `SessionStartCancellation` token plus atomic C lifecycle state. A
  blocking-open test observes `Starting`, requests stop from another thread,
  observes `Stopping` while the open remains blocked, then requires the start
  call to return `Cancelled`, the stop call to return success, and the terminal
  C state to become `Stopped` within the bounded post-release window. A
  compare-exchange owns the `Compiled` to `Starting` transition; a second
  concurrent start fails typed and cannot overwrite a live transition or
  running state.
- `pks-session` now owns `SessionMetricsSnapshot`; `SessionEngineHost`
  composes it from the authoritative bounded event queue, selected polled-audio
  receipt, and setup-time read-only receipts retained by `RunningSession`.
  Indexed source records expose stable stem identity, capture-owner,
  captured-stream, runtime-event, and source-ingress observations. Indexed
  route records expose stable route and endpoint identities, every
  authoritative runtime-edge observation, endpoint observations, an explicit
  unavailable/live/finalized stage, and endpoint-finalization failure count.
  A defensive endpoint lookup miss is typed `Unavailable`; it cannot
  masquerade as synthetic live zero observations.
- C ABI 1.1 adds source/route count functions and count-indexed source and
  route records while preserving the ABI 1.0 aggregate metrics record at
  exactly 160 bytes. A compiled ABI 1.0 C canary requests minor version 0,
  places a guard immediately after that record, polls metrics, and proves the
  guard is unchanged. Invalid indexes fail with `IndexOutOfRange`; final route
  observations remain readable after stop; destroyed Session handles fail
  indexed access with `StaleHandle`. The integer stage field is safe for
  normal C zero initialization and has named `UNAVAILABLE`, `LIVE`, and
  `FINALIZED` values.
- Focused acceptance passes: 48 `pks-session` tests, 12 `pks-session-c` tests,
  the default executable C harness, the feature-gated successful C conformance
  executable, and strict all-target Clippy for both crates.
- W11 is not accepted by this component checkpoint. ABI v1 metrics deliberately
  retain lower-layer counter ownership: the Session snapshot holds only
  read-only capture, ingress, edge, and endpoint receipts. The frozen W11
  acceptance matrix and evidence hashes remain open; this checkpoint does not
  claim language-package consumption or a real-device product proof.

## W11 bounded application-polled audio endpoint — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-nodes` now owns a concrete external endpoint
  worker that consumes the canonical compiled `PlanEdgeReceiver` path and
  publishes immutable audio through fixed-capacity queues and preallocated
  batch-lease ownership. `pks-session` exposes only the safe composition and
  receipt projection needed by later language adapters.
- The worker accepts only `LineagedAudioFrame` values delivered as
  `PlanEdgeFrame::LineagedExclusive`. Raw exclusive, shared-reference, and
  shared-lineaged variants fail closed and increment explicit ownership-drop
  and endpoint-failure observations. This makes branch-copy plus lineage a
  stored type invariant rather than a getter assertion.
- One factory supports independent application and microphone endpoints.
  Every leased frame retains Session, source, stem, clock, sequence,
  timestamp, permission, endpoint, connector, and route identity. Samples
  remain pool-owned and stable until the bounded lease is dropped.
- Endpoint workers perform only bounded receiver pops, SPSC pushes, and atomic
  observations per frame. Queue saturation drops the newest branch copy and
  counts it. Queue depth is reserved before publication, checked on dequeue,
  bounded under concurrent publish/poll, and reports any impossible underflow
  without wrapping. Untrusted queue, batch, and lease capacities are capped
  before allocation. Foreign polling and lease recycling may lock only on the
  control thread; capture callbacks and realtime routing never call foreign
  code, allocate, lock, block, or log.
- A deterministic capture test proves the real
  `CaptureDelivery → RunningSession → compiled runtime → endpoint worker →
  receipt` path for the required application-plus-microphone topology. Focused
  tests also prove all invalid ownership variants are counted, a held lease
  preserves sample address and data through Session stop, and exhausted lease
  capacity returns a typed result and observation.
- All 52 `pks-nodes` tests and 44 `pks-session` tests pass, as does strict
  all-target Clippy for both packages. No adapter-local injection queue,
  provider implementation, production mock, fallback, hidden scaffold, or
  loopback-only product claim remains in this step. The versioned C projection
  and non-Rust conformance harness remain the next gated W11 task.

## W11 source-failure branch isolation — 2026-07-26

- Status: `SAFE-TO-TEST`; the canonical Session runtime now stops and
  finalizes only the capture owner that emits a typed source failure. Other
  source stems continue through their independent bounded routes until the
  Session owner requests stop.
- The failed source remains represented in terminal source failures and makes
  the final Session outcome failed. Branch isolation does not hide or
  downgrade the fault.
- A focused two-source/six-route test injects an authoritative application
  disappearance before runtime polling and proves the microphone frame still
  reaches all three destinations, exactly one source failure is emitted, all
  owners finalize, and the Session outcome remains failed.
- No retry, selector fallback, source replacement, unbounded queue, mock
  product path, or loopback-only behavior was introduced.

## W11 declared connector identity handoff — 2026-07-26

- Status: `SAFE-TO-TEST`; the `EndpointHandle` returned by
  `Session::connector` now exposes its Session-allocated `ConnectorId`.
  Concrete open connector factories can configure exact endpoint/route
  receipts without guessing allocation order or freezing a second Session
  copy.
- Non-connector endpoints return no connector identity. The ID remains
  Session-owned and is still serialized in the canonical `SessionSpec`.
- All 41 `pks-session` tests and strict all-target Clippy pass, including a
  focused allocation test.
- No compatibility alias, provider enum, global registry, scaffold, fallback,
  or loopback-only path was introduced.

## W11 multistem endpoint completion receipt — 2026-07-26

- Status: `SAFE-TO-TEST`; `MultistemEndpointCoordinator` now exposes a
  cloneable, read-only completion receipt owned by the recording endpoint
  boundary. The receipt publishes the exact finalized `RecordingOutcome`,
  including each stem's written-frame count, discontinuities, error, and
  authoritative edge observations.
- Finalization installs the outcome once. A duplicate installation fails the
  endpoint finalization explicitly instead of replacing evidence or returning
  a false success.
- The receipt does not start, stop, poll, or configure recording and adds no
  process-global registry. `MultistemRecording` remains the sole recording
  lifecycle owner.
- Five focused multistem endpoint tests pass, including a two-stem gated run
  that proves the receipt remains readable after endpoint finalization. Strict
  all-target Clippy for `pks-nodes` passes.
- No scaffold, fallback, unbounded storage, connector/provider behavior, or
  loopback-only product path was introduced.

## W11 per-stem Session media preparation — 2026-07-26

- Status: `SAFE-TO-TEST`; the canonical Session structural graph now declares
  the product formats explicitly: 48 kHz stereo for application capture and
  48 kHz mono for microphone capture. Negotiated edge media therefore sizes
  each bounded fan-out branch pool for its real channel count instead of
  treating an `Any` layout as mono.
- Runtime preparation now derives each endpoint input's `PrepareContext` from
  that route's negotiated graph edge. Connector, relay/browser, and grouped
  recorder drivers receive the application and microphone formats separately;
  one global Session sample format no longer misrepresents both stems.
- The obsolete endpoint-preparation context argument was removed from
  `start_prepared_session`. The engine's setup context remains solely with
  realtime-node preparation, while route-specific endpoint contexts travel
  with the owned `PreparedWorkerMapping`.
- A focused canonical-engine test proves all three application destinations
  prepare as stereo and all three microphone destinations prepare as mono.
  All 40 `pks-session` tests and strict all-target Clippy pass.
- No converter, fallback, unbounded queue, provider behavior, capture
  implementation, or loopback-only path was introduced.

## W11 pks-audio canonical Session facade — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-audio` now re-exports the authoritative
  `pks-session` surface instead of owning a second Session declaration,
  lifecycle state machine, stop handle, and `RuntimeNotIntegrated` result.
- A complete repository search found no internal consumer of the removed
  `ConnectorKey`, `ConnectorHandle`, `SessionState`, or `StopHandle` surface,
  so no speculative compatibility aliases remain. The removed implementation
  and its self-tests were the only consumers.
- The `quickstart` compile target now declares the application and
  microphone stems with canonical fallible handles, routes them to open
  connector, browser, and grouped recording boundaries, and calls
  `SessionEngine::start` with host-owned capture backends. It does not construct
  a no-op endpoint, select a provider, or move transport policy into the
  facade.
- A focused facade test proves `pks_audio::Session` and
  `pks_audio::SessionEngine` are the exact canonical types. All ten
  `pks-audio` unit tests, its allocation test, pipeline integration test,
  facade test, all 40 `pks-session` tests, strict all-target Clippy,
  architecture constraints, the full CODE_PROTOCOL gate, and the release
  quickstart build pass.
- No scaffold, mock product path, fallback, helper process, provider
  implementation, or loopback-only behavior was introduced.

## W11 canonical Session engine bootstrap — 2026-07-26

- Status: `SAFE-TO-TEST`; `SessionEngineBuilder` now installs the fixed
  structural node set once, validates open operator-to-node registrations, and
  consumes all setup state before constructing the paired operator and
  endpoint-driver registries. A failed registration or build cannot expose a
  partially usable engine.
- `SessionEngine::start` is the one reusable setup-time composition path from
  a public `Session` declaration to the existing freeze, graph compile,
  bounded runtime preparation, and transactional start owners. Freeze,
  compile, prepare, and start failures remain separate typed variants;
  `SessionStartFailure` remains available with its rollback failures and event
  receiver instead of being converted to text.
- Concrete callback capture backends and endpoint-driver factories remain
  injected through the existing `pks-capture` and `pks-endpoint` contracts.
  No platform capture, relay, recorder, connector, provider, artifact, or
  proof-policy implementation moved into `pks-session`.
- Four focused engine tests prove the complete application plus microphone
  declaration with connector, browser, and grouped recording boundaries
  reaches `Running`, all five prepared endpoint instances start behind the
  closed Session gate, repeated stop is idempotent, duplicate and conflicting
  registrations fail typed, unknown operators remain compile failures, and a
  capture-open failure preserves the transactional start error.
- All 40 `pks-session` tests, strict all-target Clippy, workspace formatting,
  architecture constraints, the full CODE_PROTOCOL gate, and the release
  `quickstart` example build pass. The initial architecture acceptance
  command in the execution envelope named a nonexistent legacy path; execution
  stopped, the envelope was corrected to the CI-authoritative
  `scripts/lint/check-architecture-constraints.sh`, and the corrected gate
  passed.
- No scaffold, mock product path, fallback, helper process, provider
  implementation, or loopback-only behavior was introduced. Test-only capture
  and endpoint contract doubles remain under `cfg(test)`.

## W11 local CI correction and candidate gate — 2026-07-26

- Status: `SAFE-TO-TEST`; GitHub PR #43 remains unmerged and must not be
  described as `SAFE-TO-MERGE` until its exact pushed head has every required
  check green.
- Rust 1.97 strict all-target Clippy exposed manual no-op `Wake`
  implementations in two test poll helpers. Both now use the standard
  `Waker::noop()` without changing runtime scheduling, capture, endpoint,
  allocation, or hot-path behavior.
- The exact local Linux candidate passes workspace formatting, strict
  all-target Clippy, all workspace tests and doc tests, allocation tests,
  architecture constraints, quickstart compilation, CODE_PROTOCOL, and the
  complete `pks-audio` benchmark build. The benchmark link was repeated with
  one Cargo job and a 6 GiB ceiling after the initial 4 GiB container was
  killed by its memory limit; the source, locked dependencies, release
  profile, Rust 1.97 toolchain, and benchmark executables were unchanged.
- The native macOS candidate independently passes workspace formatting, strict
  all-target Clippy, all workspace tests and doc tests, allocation tests,
  architecture constraints, quickstart compilation, CODE_PROTOCOL, and the
  complete `pks-audio` benchmark build.
- No CI check was disabled, made advisory, skipped, or wrapped in a success
  override. No scaffold, mock, fallback, provider implementation, or
  loopback-only product path was introduced.

## Linux runtime-event CI compilation — 2026-07-26

- Status: `SAFE-TO-TEST`; the Linux capture module now imports the shared
  `Platform`, `SourceKind`, and `StableSourceId` types used by its typed
  runtime-failure events.
- GitHub's Linux all-target Clippy gate found the missing imports after the
  macOS host gates passed. A current Linux Clippy run also removed an
  immediately dereferenced name borrow in exact application matching. These
  corrections change no capture behavior, queue, callback work, fallback,
  selector, or product claim.

## W11 reusable Session structural node registration — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-session::register_session_structural_nodes`
  installs the fixed application/microphone ingress and
  connector/browser/recording boundary descriptors required by the canonical
  Session compiler and runtime. CLI and SDK adapters no longer need to
  recreate compile-only placeholder factories.
- Application and microphone structural nodes are real allocation-free
  realtime ingress forwarders. Their configuration validation requires exact
  Session/stem identity plus a selector form valid for that source kind,
  including Windows process-instance PID and stable identity fields.
- External destination descriptors remain `AsyncWorker` boundaries. Their
  factories validate route and endpoint metadata, but any accidental
  `RuntimeNode` instantiation returns the dedicated typed
  `ExternalBoundaryExecution` error. Connector, relay/browser, and recording
  work remains in `EndpointDriverFactory` implementations; no no-op endpoint
  can report success.
- Registration preflights all five stable node type IDs before mutation and
  returns a typed duplicate error without partially changing the registry.
  Compiler tests now consume the production registration seam instead of local
  compile-only factories.
- All 36 `pks-session` tests pass. Strict all-target Clippy for `pks-session`
  and `pks-graph` and workspace formatting pass.
- No provider implementation, relay algorithm, product policy, scaffold,
  mock, fallback, helper process, or loopback-only path was introduced.

## W11 endpoint-driver lifecycle contract — 2026-07-26

- Status: `SAFE-TO-TEST`; the new acyclic `pks-endpoint` crate owns the open
  endpoint-driver registry and setup-time lifecycle contract shared by
  `pks-session` orchestration and concrete destination packages.
- `EndpointDriverRegistry` resolves only an exact open `OperatorId` plus
  `NodeTypeId` pair and transfers the route's existing bounded
  `PlanEdgeReceiver` into the selected factory. Unknown, empty, and duplicate
  registrations and driver preparation failures remain typed.
- Prepared endpoints may start only while the shared gate is closed. Starting
  makes the driver ready but does not authorize delivery; only the
  Session-owned `EndpointStartGateController` can open the one-way gate after
  every startup resource is ready. An already-open gate fails start and returns
  the prepared endpoint for rollback.
- Preparation cancellation, idempotent stop request, and join/finalize return
  authoritative endpoint observations and preserve stop and finalization
  failures independently. The contract creates no worker thread and contains
  no concrete connector, relay, recorder, provider, or production no-op
  implementation.
- `OperatorId` moved to this lower contract crate and remains re-exported from
  `pks-session`. Its version-one serialized form is explicitly one transparent
  UTF-8 string; `SessionSpec` retains document migration authority.
- Six contract tests prove exact registry resolution, multi-endpoint prepare
  rollback, closed-gate readiness with no pre-open delivery, fail-closed
  already-open start, and truthful stop/join failure reporting. All 16
  `pks-session` tests, strict Clippy for both packages, focused formatting, the
  architecture dependency lint, and full CODE_PROTOCOL pass.
- No scaffold, mock product path, fallback, provider implementation, helper
  process, or loopback-only product behavior was introduced. The only driver
  implementation is a `cfg(test)` contract double.

## W11 grouped multistem recording endpoint — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-nodes::MultistemEndpointCoordinator` is one
  Session-scoped concrete endpoint driver over the accepted
  `MultistemRecording`. It does not duplicate WAV, timeline, discontinuity,
  manifest, metric, checksum, or finalization algorithms.
- AUDIO-030 adds explicit batch preparation to `pks-endpoint`. Grouping requires
  one Session and an exact `OperatorId`, `NodeTypeId`, `EndpointGroupId`, and
  declared endpoint set. `StemHandle::record` now persists the stable default
  `recording_group_id` `session.multistem.default.v1`; sharing only an operator
  or node type never groups endpoints.
- Preparation validates the complete application/microphone batch, exact
  endpoint IDs, Session, group, stem labels, and sample specifications before
  creating an artifact or worker. Cancellation drops the pending receivers and
  leaves no Session directory.
- One prepared group starts one `MultistemRecording` while the shared gate is
  closed. Setup uses an unpublished pending directory; workers consume zero
  queued frames and no final Session directory or manifest exists before the
  Session opens the gate. Opening publishes the staged directory atomically.
  One `RunningEndpoint` owns both stems and therefore requests stop, joins
  workers, writes one final manifest, and reports one typed outcome exactly
  once. Pre-open rollback removes staging and reports cleanup failure rather
  than hiding it; all workers are joined and typed worker plus cleanup failures
  are preserved together when both occur.
- Recorder observations now expose frames received/written/rejected,
  discontinuities, and failures while running; final endpoint observations also
  preserve each edge's authoritative delivery/drop counters. Incomplete worker
  finalization remains a failed endpoint outcome with an incomplete manifest.
- Five grouped-driver tests prove two stems in one directory/manifest,
  pre-gate zero consumption and zero published artifacts, partial-batch and
  ready-group rollback without artifacts, finalization failure truth, and
  failed-branch isolation. A recorder-level test separately proves aggregate
  pre-gate worker and cleanup failure truth. All 47 `pks-nodes` tests and all
  six `pks-endpoint` tests pass; strict `pks-nodes` Clippy passes.
- No process-global registry, concrete relay/provider logic, production fake,
  fallback, helper process, or loopback-only product behavior was introduced.

## W11 Session compiler and RuntimePlan ownership — 2026-07-26

- Status: `PARTIAL`; immutable Session declarations now lower through the real
  `pks-graph` compiler and runtime planner, while runtime start, capture and
  endpoint ownership, transactional rollback, stopping, and finalization
  remain open.
- `SessionCompiler` consumes a validated `SessionSpec`, the existing
  `NodeRegistry`, and an open `OperatorRegistry` mapping `OperatorId` to
  `NodeTypeId`. Unknown operators, missing source node types, mismatched
  operator/node registrations, reserved configuration keys, graph compile
  errors, and planner errors remain typed.
- Each captured stem lowers to one source node. Each route lowers to its own
  endpoint node and edge, so two stems sharing connector/browser declarations
  still receive independent edge queues and memory plans.
- `CompiledSession` privately owns the immutable specification, verified
  `GraphIr`, and `RuntimePlan`. Its public surface exposes declarations and
  summary counts, not graph IR, runtime plans, factories, pools, or executor
  internals.
- The focused product topology compiles two sources plus six route endpoints
  into eight nodes and six independent graph/planned edges. Test-only
  descriptors are registry-backed and their compile-only factories return a
  typed error if execution is attempted; no no-op endpoint success exists.
- Eleven focused tests, package strict Clippy, and format pass. This step adds no
  `start`, `Running`, runtime-success, endpoint worker, capture backend,
  provider implementation, mock product path, fallback, or loopback-only
  claim.

## W11 callback capture ownership contract — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-capture` now owns a platform-neutral
  prepare/open/stop-and-join contract for callback-oriented capture.
- A `CaptureOwner` retains the native backend, bounded captured-frame stream,
  typed runtime-event channel, and their authoritative observations. Prepared
  and active owners are distinct, and a prepared backend can open only once.
- Dropping the owner reclaims the backend through its RAII contract; explicit
  `stop_and_join` joins every native worker before returning final observations
  and maps a worker panic to typed `CaptureWorkerPanicked`. Drop performs the
  same reclamation best-effort without propagating failure. The existing
  pull-oriented `PlatformAdapter` remains a documented legacy compatibility
  path.
- Fifty package tests, strict clippy, focused format, and the full
  CODE_PROTOCOL gate pass.
- Thin target adapters now move the platform-neutral bounded delivery
  endpoints into the existing macOS, Windows, and Linux
  `DesktopCaptureSource` owners. macOS native check, tests, and strict Clippy
  pass. Windows ARM64 MSVC check and strict Clippy pass as cross-compilation
  evidence only; its existing typed runtime events now publish directly into
  the supplied Session channel with no forwarding thread or duplicate queue.
  macOS and Linux likewise move the supplied sender into their native
  callback/worker owner: CPAL and PipeWire callbacks publish a prebuilt,
  allocation-free one-shot failure event, and reader failure ownership closes
  when the native producer exits.
  Linux source integration is implemented, but macOS cross-compilation stops
  in `alsa-sys` before the crate builds because no ARM64 Linux ALSA/PipeWire
  pkg-config sysroot is installed. Linux therefore still requires a native VM
  check, and the target adapters are not yet real Session-path evidence.
- No live scaffold, mock, fallback, helper process, provider implementation, or
  loopback-only product path was introduced. Contract doubles are test-only.

## W11 immutable Session declaration foundation — 2026-07-26

- Status: `PARTIAL`; `pks-session` now owns a safe Rust declaration/freeze
  foundation, while runtime compilation, startup, endpoint ownership, stopping,
  the C conformance surface, and real-path migration remain open W11 work.
- `Session` builds one versioned `SessionSpec` from application/microphone
  selectors, open `OperatorId` plus `NodeTypeId` endpoint descriptors,
  configuration values, stems, endpoints, and routes. No closed
  provider/model/policy enum entered the crate.
- Freezing consumes the public Session builder and closes the shared draft
  before validation. Cloned stem handles cannot mutate a frozen draft; foreign
  endpoint handles fail immediately and create no route.
- The specification exposes immutable slices and typed identifiers. It exports
  no graph IR, runtime plan, pool, Tokio type, Rust trait object, provider
  client, platform object, or raw foreign handle.
- Six focused tests pass for distinct routes, foreign endpoints, cloned/stale
  post-freeze mutation, invalid open operator identity, and fail-closed
  unrouted stems. Focused strict Clippy and format pass.
- This step adds no `run`, `start`, `Running`, stop, runtime-success,
  `RuntimeNotIntegrated`, C ABI, mock, scaffold, fallback, or loopback-only
  path.

## W11 Session runtime preparation — 2026-07-26

- Status: `SAFE-TO-TEST`; consuming a `CompiledSession` now instantiates the
  real `RealtimePlanExecutor`, creates one bounded `PlanSource` channel for
  each declared stem, and retains each non-realtime edge receiver with its
  exact route, stem, and endpoint identity.
- Preparation validates that the compiled plan produces exactly one worker
  receiver per route, rejects missing, invalid, unknown, duplicate, or
  mismatched route metadata with typed errors, and rolls all instantiated
  nodes and bounded channels back through ownership drop on any failure.
- The public prepared surface exposes Session, stem, route, and endpoint
  identities plus counts and observations. Graph node IDs, the executor,
  source consumers, worker receivers, and cancellation ownership remain
  internal for the later `RunningSession` transition.
- Sixteen focused `pks-session` tests pass. Four preparation tests use explicit
  test factories to prove two independently bounded source channels, six
  independently mapped worker receivers, and zero live nodes after both
  receiver-mapping and source-channel failures. Focused strict Clippy and the
  full workspace CODE_PROTOCOL gate pass.
- This step opens no capture backend, starts no endpoint worker or runtime
  thread, publishes no `Running` state, and adds no production no-op node,
  scaffold, mock, fallback, provider implementation, or loopback-only path.

## W11 lineage-authoritative runtime routing — 2026-07-26

- Status: `SAFE-TO-TEST`; canonical `pks-runtime` source ingress, realtime
  execution, bounded edge fan-out, and worker delivery now retain the exact
  capture-time `FrameLineage`.
- `PlanSourceSender` accepts only `LineagedAudioFrame`. Full or cancelled sends
  return a non-allocating outcome containing a small typed reason and the
  rejected exclusive frame, preserving explicit drop-newest ownership without
  boxing on the hot path.
- `RealtimePlanExecutor` temporarily separates samples from lineage only while
  a registered `RuntimeNode` processes the exclusive `AudioFrame`. It validates
  the returned frame identity and reattaches the same immutable lineage;
  source, sequence, or timestamp mutation fails typed execution instead of
  reconstructing source generation, discontinuity, or permission epochs.
- `PlanEdgeRouter` preserves lineage through both shared-reference fan-out and
  branch-pool copies. Receivers expose the retained lineage and count an
  authoritative discontinuity-epoch transition beside existing sequence and
  timestamp discontinuities.
- Raw `dispatch_from` and `execute_from` remain documented compatibility entry
  points for existing callers only. The canonical runner cannot enter that
  path, and lineaged/raw edge variants fail closed if mixed during execution.
- Focused tests prove exact lineage across three independent copied branches,
  realtime operator processing, discontinuity-epoch observation, independent
  multi-source dispatch, bounded cancellation, and zero-allocation lineaged
  router, executor, and runner processing.
- No frame allocates, no lineage epoch is inferred on the canonical path, and
  no Session lifecycle authority, provider implementation, scaffold, mock,
  fallback, or loopback-only product behavior was introduced.

## W11 bounded multi-source runtime runner — 2026-07-26

- Status: `SAFE-TO-TEST`; this is a reusable runtime component, not a complete
  Session or a real-path W11 acceptance artifact.
- `RealtimePlanRunner` owns one prepared `RealtimePlanExecutor` and drains
  independent preallocated source-input rings with bounded round-robin work.
  It spawns no thread and publishes no Session lifecycle state.
- `PlanRunnerCancellation` prevents new source delivery after cancellation.
  Finalization uses an explicit `DrainQueued` or `DiscardQueued` policy, a
  caller-supplied frame budget, and counted discard observations. The owning
  Session must stop capture producers before final drain so no producer can
  race terminal resource reclamation.
- Each source input exposes capacity, current/peak depth, enqueue/delivery,
  full/cancelled rejection, and shutdown-discard counts. A full source input
  drops only its newest frame.
- Focused tests prove two sources dispatch independently, cancellation drains
  no more than its declared frame budget, remaining frames discard explicitly,
  discard-only finalization executes no frame, and prepared runner processing
  performs zero heap allocation.
- Acceptance passes: scoped format, locked package check, all 43 runtime unit
  tests plus three allocation tests, strict all-target `pks-runtime` Clippy,
  and the full CODE_PROTOCOL gate.
- No Session type, lifecycle authority, worker thread, capture implementation,
  endpoint policy, provider path, scaffold, mock, fallback, or loopback-only
  product behavior was introduced.

## W11 immutable frame-lineage envelope — 2026-07-26

- Status: `PARTIAL`; `pks-frame` now provides exclusive and shared audio
  envelopes that retain the frozen `FrameLineage` snapshot through bounded
  fan-out and `CopyToBranchPool`.
- Construction rejects source, sequence, or timestamp mismatches without
  allocation. Dynamic source-generation, discontinuity, and permission epochs
  therefore remain attached to the samples they describe.
- The focused `pks-frame` tests and clippy gate pass. Runtime edge integration
  is still required before this becomes real Session-path evidence.
- No scaffold, mock, provider implementation, fallback, helper process, or
  loopback-only path was introduced.

## W11 Session ownership correction — 2026-07-26

- Status: `PARTIAL`; the safe engine and portable adapter ownership contracts
  are accepted, while no `pks-session` implementation, C artifact, or
  conformance result exists yet.
- `pks-session` is the thin safe-Rust composition and lifecycle owner. It
  freezes declarations, coordinates real graph compilation and transactional
  startup, owns all running resources, and coordinates bounded stop, join, and
  recording finalization.
- `pks-graph` retains graph compilation and `RuntimePlan`; `pks-runtime`
  retains scheduling, execution, routing, and runtime observations;
  `pks-frame` retains pools and frames; native capture and endpoint
  implementations remain in their existing smallest owners.
- The portable Session C projection remains a separate pending adapter task. It
  does not exist as a checked-in crate today; when introduced it owns ABI
  records, generational handles, marshalling, polling, leases, panic
  containment, reproducible headers, and C conformance. It does not own
  Session semantics.
- Rust uses the future `pocketstation` façade directly over `pks-session`.
  Python and Node may use direct PyO3 and Node-API adapters. Swift and Kotlin
  may use language-owned adapters over the C/control boundary and setup-time
  bounded buffers. Browser JavaScript remains relay-native.
- The reusable lifecycle currently assembled by the CLI proof command is the
  extraction seam. Proof policy, artifact rendering, connector provider code,
  relay mechanics, and product thresholds remain outside the engine.
- The unaccepted standalone `pks-runtime::RuntimeHandle` overlay was rejected:
  it could fabricate lifecycle without owning capture, endpoint workers,
  rollback, or recorder finalization.
- This correction adds no scaffold, mock, provider implementation, fallback,
  helper process, or loopback-only path.

## W11 Session host foundation — 2026-07-27

- Status: `SAFE-TO-TEST`; `pks-session` now exposes `SessionEngineHost` and
  `SessionEngineHostBuilder` as the safe owner for the canonical Session
  engine, the exact application and microphone capture backends, and any
  retained bounded polled-audio receipts used for foreign projection.
- The host builder reuses the real `SessionEngineBuilder` registration seam,
  rejects missing application or microphone backends before a host exists, and
  registers polled-audio endpoints only through the real bounded
  `PolledAudioEndpoint` factory. No side runtime, synthetic queue, or adapter-
  local media path was introduced.
- The host starts only through the canonical engine and therefore preserves the
  real compile, prepare, start, rollback, stop, and bounded lease path already
  proven by `pks-session`.
- Four focused host tests now prove typed missing-backend rejection, retained
  receipt ownership, host-owned polled-audio delivery through the real runtime,
  and typed capture-start failure propagation.
- All 48 `pks-session` tests, strict `cargo clippy -p pks-session --all-targets
  --locked -- -D warnings`, and the full workspace `bash scripts/check_protocol.sh`
  pass on 2026-07-27.
- `pks-session-c` still does not exist. Versioned ABI records, generational
  foreign handles, header packaging, panic containment, and the real C
  conformance harness remain the open W11 portable-binding work.

## W11 initial pks-session-c boundary slice — 2026-07-27

- Status: `SAFE-TO-TEST`; the sibling `pks-session-c` crate now exists and owns
  the first real portable C boundary slice above `pks-session`.
- The crate ships checked-in ABI version/status records, opaque generational
  handle records, an internal handle table, panic-contained exported entry
  points for ABI version query plus runtime open/close/live checks, and the
  packaged public header `include/pks_session.h`.
- Header packaging is non-hermetic: `build.rs` copies the checked-in header into
  `OUT_DIR` only. It does not write back into source.
- The first conformance fixture is compile-level only: a C translation unit
  includes the public header, checks stable layout expectations, and compiles
  against the exported symbol declarations. It does not yet execute the full
  Session lifecycle from C.
- Focused gates pass on 2026-07-27:
  `cargo test -p pks-session-c --offline` and
  `cargo clippy -p pks-session-c --all-targets --offline -- -D warnings`.
- This is still `PARTIAL` W11 portable binding work. Real Session declaration,
  lifecycle, bounded event/metric/audio polling, generational engine/session
  handle ownership, and the executable C conformance harness remain open.

## W11 embedded Session engine boundary decision — 2026-07-25

- Status: `PARTIAL`; W10 PASS opened W11 and the engine deployment/ownership
  decision is accepted, while no W11 engine code, C header, language binding,
  or bindability artifact exists yet.
- AUDIO-029 selects one embedded native Session engine. The embedding
  application remains the visible permission identity; desktop packages may
  load a dynamic library and future mobile adapters may statically link the
  same engine. A signed local helper and process IPC remain evidence-triggered
  alternatives, not parallel implementations.
- The binding contract is a versioned C ABI over opaque generational handles.
  It exposes Session declaration, compile/start/stop, bounded event and metric
  polling, and bounded multi-frame audio leases. It does not expose Rust graph
  IR, runtime/pool layouts, traits, Tokio types, platform objects, provider
  enums, or per-frame foreign-language callbacks.
- Foreign audio retention receives a preallocated `CopyToBranchPool` copy under
  AUDIO-027. Batch leases are immutable, explicitly released, and bounded at
  compilation; queue or lease exhaustion drops the newest delivery and records
  the exact reason.
- `CRATE_OWNERSHIP.md` assigns the authoritative Session specification,
  lifecycle, error/event/metric projections, leases, and C ABI to the precise
  `pks-session` crate. `pks-runtime`, `pks-frame`, capture adapters, and endpoint
  implementations retain their existing lower-layer ownership.
- The existing `pks-audio` façade now re-exports the canonical `pks-session`
  surface. The remaining W11 gap is the portable C adapter and its conformance
  harness, not a second Rust Session runtime.
- This documentation slice adds no code, generated header, mock, scaffold,
  fallback, provider path, helper process, or loopback-only behavior. W11 exit
  still requires a non-Rust lifecycle/lease harness, typed panic containment,
  the quickstart compile gate, and full protocol acceptance.

## macOS native ring-loss telemetry — 2026-07-23

- Status: `SAFE-TO-TEST`; the 11 macOS component tests pass, while no
  post-change real-device artifact yet proves the new counters on an active
  CoreAudio process tap, microphone, or AudioServerPlugin fallback.
- CoreAudio process-tap and AudioServerPlugin readers now convert native ring
  overwrite deltas into `dispatch_queue_full_total`. The microphone callback
  reports bounded Rust-ring rejection through the same canonical counter.
- Process-tap, AudioServerPlugin, and microphone capture expose canonical
  `CaptureObservations` for callback buffers, enqueued frames, pool exhaustion,
  bounded-ring loss, oversize rejection, and stream errors where each backend
  can observe them. `DesktopCaptureSource::observations()` no longer fabricates
  or omits the process-tap boundary.
- Existing macOS real-device artifacts predate this telemetry and therefore do
  not prove zero native-ring loss for the new boundary. A new physical-device
  capture artifact is required before upgrading this section.
- No queue was made unbounded, and no mock, fallback, provider path, or new
  loopback-only behavior was introduced.

## Windows active-source lifecycle events — 2026-07-23

- Status: `SAFE-TO-TEST`; the shared contract and Windows ARM64 compilation
  gates pass, while native active-process-exit and endpoint-invalidation
  artifacts remain open.
- `pks-capture` now owns a typed `SourceRuntimeEvent` control stream. It carries
  the exact stable identity and generation, distinguishes authoritative
  `SourceUnavailable` from `BackendFailure`, and requires explicit rediscovery
  plus a new Session after disappearance.
- The control stream is bounded to eight events in the Windows backend.
  Publication uses nonblocking `try_send`; a full channel drops the newest
  event and exposes exact enqueued/dropped totals. Event creation and
  publication happen only on the capture worker after a terminal condition,
  never in the audio delivery callback.
- Active Windows process-loopback retains a synchronization handle for the
  process incarnation verified at open. A signaled handle emits typed
  `SourceUnavailable` with `SourceInstanceExited`; PID reuse cannot silently
  retarget the active Session.
- WASAPI query, read, and event-wait failures retain their exact Windows
  HRESULT when present. Only `AUDCLNT_E_DEVICE_INVALIDATED` is classified as
  source disappearance. Resource invalidation, process-watch errors, oversize
  packets, and all other WASAPI classes remain typed backend failures rather
  than being guessed as disappearance.
- Acceptance: 44 shared capture tests and 15 Windows host-neutral tests pass;
  `cargo check -p pks-capture-windows --target aarch64-pc-windows-msvc` and
  target strict Clippy pass.
- `pks proof sources` and `pks capture from` now consume the typed Windows
  events and retain exact failure plus event-channel observations. This CLI
  integration has host tests but no native Windows active-invalidation
  artifact.
- Residual boundaries: process-tree capture observes termination of the
  selected root process, not each descendant; the bounded event stream may
  report counted overflow; and native Windows active-invalidation evidence is
  still required.
- No scaffold, mock, automatic restart, replacement following, fallback,
  provider path, or new loopback-only behavior was introduced.

## Windows process-instance identity hardening — 2026-07-23

- Status: `SAFE-TO-TEST`; deterministic identity tests and Windows ARM64
  cross-checks pass, while a native PID-reuse execution artifact remains open.
- Discovered WASAPI application sources now encode the native process creation
  `FILETIME` with the PID in `StableSourceId`. Discovery verifies the creation
  time again after resolving display metadata and omits a session that changed
  or disappeared during enumeration.
- `ExactApplication` parses that fingerprint and verifies PID plus creation time
  immediately before and after `ActivateAudioInterfaceAsync`. A missing,
  replaced, recycled, or malformed exact identity fails closed; no system-mix
  or replacement-process fallback is permitted.
- Direct `Process(pid)` remains a raw process-lifetime selector. It checks that
  the PID is queryable and pins the observed incarnation only across that open,
  but intentionally persists no creation-time identity and makes no
  restart-stability claim.
- Missing/recycled exact instances return typed `SourceUnavailable` with the
  selected stable key. Access-denied, malformed identity, cleanup, and generic
  backend failures remain backend errors rather than being guessed as source
  disappearance or permission denial.
- Four platform-neutral fingerprint parsing/matching tests join the eight
  lifecycle/packet tests. Host tests, host and Windows ARM64 strict Clippy,
  Windows ARM64 check/tests, and scoped format pass. The latest full
  CODE_PROTOCOL rerun passes across 85 central Rust files.
- No public capture-type redesign, scaffold, mock, automatic restart, fallback,
  provider path, or new loopback-only behavior was introduced.

## Windows backend open and packet-delivery hardening — 2026-07-23

- Status: `SAFE-TO-TEST`; Windows target compilation and focused lifecycle
  contracts pass, while a new native Windows execution artifact remains open.
- The five-second backend-open deadline is now caller-bounded even when
  `ActivateAudioInterfaceAsync` does not return. Timeout marks the open
  cancelled, signals stop, drops the unopened worker handle without joining,
  and returns an error. Dispatch starts only after successful backend open.
- A late activation completion observes cancellation before process-loopback
  initialization. Every backend mode also rejects a late success notification,
  stops a stream that raced with timeout, and releases WASAPI/COM objects on
  the capture worker. Exact application capture never falls back to system mix.
- WASAPI packet size is checked with overflow-safe byte accounting before
  reading into the fixed buffer and checked again against the delivered frame
  count. Announced/delivered oversize increments
  `oversized_buffer_total`; query/read failure increments
  `stream_errors_total`; each condition terminates the failed stream instead
  of silently breaking only the packet-drain loop.
- Missing exact process-lifetime and input-device selectors now return
  `SourceUnavailable` with their selected stable key. Access-denied or otherwise
  inaccessible process/device failures remain backend errors rather than being
  guessed as disappearance or permission denial.
- Eight platform-neutral lifecycle/packet tests pass on the host. Targeted
  host tests, host strict Clippy, Windows ARM64 check/tests, Windows ARM64
  strict Clippy, workspace format, and CODE_PROTOCOL checks pass.
- The pinned `wasapi` activation helper has no cancellation API. If Windows
  never completes activation, one detached setup worker can remain until
  process exit; caller latency stays bounded and no capture or dispatch
  success is reported.
- No scaffold, mock capture, fallback, provider path, unbounded queue, or new
  loopback-only behavior was introduced.

## Linux native capture loss telemetry — 2026-07-23

- Status: `SAFE-TO-TEST`; native Linux execution remains required before a new
  `VM-PROVEN` artifact can claim zero loss at this boundary.
- Both PipeWire process callbacks and the ALSA capture loop now route pool
  acquisition through one hot-path helper. Exhaustion increments
  `pool_exhausted_total` with one relaxed atomic operation and drops the newest
  buffer without blocking.
- Both PipeWire producer paths route bounded SPSC pushes through one hot-path
  helper. A rejected push increments `dispatch_queue_full_total`; a successful
  push increments `frames_enqueued_total`. The helpers allocate, lock, block,
  log, await, and panic nowhere.
- Deterministic GWT tests exercise a real exhausted `AudioBufferPool`, a real
  full `rtrb` producer, exact counter snapshots, and pool-slot reclamation.
  All seven Linux crate tests pass in a Linux ARM64 Rust container with the
  PipeWire/ALSA development libraries. Native device execution remains an
  explicit acceptance command rather than a product-proof claim.
- An exact stable application or stable input device absent at open now returns
  typed `SourceUnavailable` with the selected stable key. Default-device,
  fuzzy-name, malformed-node, and generic backend failures retain their prior
  classifications. Selection remains fail-closed with no fallback.
- Focused Linux-source formatting, host package check/strict Clippy, and the
  Linux ARM64 crate test suite pass. The latest full protocol, hot-path
  allocation, workspace-format, and strict workspace-Clippy gates pass.
- No scaffold, mock, fallback, replacement-following, provider integration, or
  new loopback-only path was introduced.

## Native capture fault and identity closeout — 2026-07-23

- Status: `SAFE-TO-TEST`, with deterministic Linux live-source and Windows
  fail-closed cells `VM-PROVEN`.
- Windows WASAPI process-loopback now queries the selected PID before opening;
  an invalid PID can no longer initialize a silent stream and report capture
  success.
- `CaptureSource::identity_strength()` is now the canonical truth boundary.
  Windows process-only application sources report `ProcessId` rather than
  overstating executable names as stable application IDs. macOS and Linux keep
  their stronger identity only where platform evidence supplies it.
- The change is setup/control-path only. It adds no allocation, lock, blocking,
  logging, async work, or platform call to an audio callback.
- Linux live application and virtual-microphone disappearance cells retain
  exact lineage, return `failed-continuity`, finalize recording, and show zero
  reported capture-bridge, normalization, recorder-edge, or discontinuity
  drops. One initial-alignment gap per stem remains explicit. That artifact
  predates the Linux ring/pool telemetry above, so it does not claim zero loss
  for the newly visible boundary. Windows invalid PID/microphone cells return
  nonzero with diagnostics matching the expected patterns and leave no WAV
  after the CLI open-order correction.
- Output-device identities with a device UID now report
  `StableDeviceUid`, matching input-device identity semantics.
- Acceptance: 33 `pks-capture` tests, Windows ARM64 backend cross-check, native
  Windows ARM64 build, and final native negative cells pass. Full evidence:
  `../../docs/reports/2026-07-23-cross-platform-native-capture-proof.md`.
- No fallback, replacement-following, mock, scaffold, provider path, or new
  loopback-only product behavior was introduced.

## Full-workspace protocol and downstream contract gate — 2026-07-23

- Status: `SAFE-TO-TEST`. `scripts/check_protocol.sh` now scans all 85 Rust
  source files under `src`, `crates`, and `examples`; it no longer defaults to
  changed files or silently passes because the workspace has no root `src`.
- The gate enforces measurement suffixes, section-banner removal, GWT test
  names, canonical vocabulary, semantic choice types, and a new executable
  LAW-14 check requiring a nearby `SAFETY:` invariant for every unsafe block.
- Existing code was brought to the gate: measurement names retain explicit
  units/ratios, legacy tests use GWT names, PipeWire callback ownership is
  called a stream subscription, and normalized resampling writes into
  preallocated storage without `Vec::push` on `process()`.
- LAW-15 now executes zero-allocation gates for pipeline processing, runtime
  plan dispatch/execution, and the audio encode/decode path instead of emitting
  a non-binding grep warning.
- The full workspace protocol gate, strict all-target Clippy, and all central
  workspace tests pass, including the allocation gates.
- Downstream impact was checked in `pks`, `control-plane`, `relay`, and
  `web-receiver`. The current `pks` suite has 177 passing tests and strict
  Clippy passes. Control-plane short tests, relay short race
  tests, and web type-check/build pass. Their long soak packages were
  intentionally not rerun as part of this naming/protocol repair.
- No capture fallback, unbounded queue, mock, scaffold, provider integration,
  or loopback-only product path was introduced.

## Native ARM64 platform proof — 2026-07-23

- Status: Linux and Windows are `SAFE-TO-TEST` and `VM-PROVEN`; macOS remains
  the only `REAL-DEVICE-PROVEN` product slice.
- Windows natively builds the current backend and CLI and passes discovery,
  system mix, exact Edge process-tree capture, virtualized microphone capture,
  WAV, and three same-source capture-reopen cells per source. A concurrent independent
  distractor is rejected by 69.529 dB.
- Linux natively builds the current backend and CLI and passes discovery,
  system mix, exact PipeWire application capture, named virtual microphone
  capture, WAV, and three same-source capture-reopen cells per source. Stable
  application identity remains exact even when PipeWire exposes no PID; no
  system-mix fallback is permitted. A concurrent distractor is rejected by
  123.361 dB.
- Linux corrections cover one-time PipeWire initialization, joined object
  lifetimes, `InputDevice`, exact stable-node targeting, no-fallback session
  policy, SPA chunk/layout handling, and separation of the OS callback maximum
  from PocketStation's 10/20 ms transport frames.
- The final Linux matrix validates exit status, at least 80% requested WAV
  duration, and RMS above 0.001. All twelve cells pass; main WAVs are exactly
  5.00 seconds and expected tones survive.
- Evidence and remaining physical-device/drop-telemetry boundaries:
  `../../docs/reports/2026-07-23-cross-platform-native-capture-proof.md`.
- Linux also passes a bounded two-stem recording plus VM-to-host
  relay/browser cell: 750 source frames per stem, zero runtime/relay-edge
  drops or discontinuities, and 561 browser packets per bus with zero packet
  loss. Windows relay/browser and both platform connector cells remain open.
- No provider code, model graph, unbounded queue, mock capture fallback, or
  physical-device claim was introduced.

## Native ARM64 platform audit checkpoint — 2026-07-21

- Status: macOS remains `REAL-DEVICE-PROVEN`; Linux is `PARTIAL` and
  `VM-PROVEN`; Windows is `SAFE-TO-TEST` at the cross-compile boundary and
  native execution is `BLOCKED`.
- Mechanical Windows repairs restore
  `cargo check -p pks-capture-windows --target aarch64-pc-windows-msvc` and
  targeted format. They update stale exports/imports and current frame/source
  field names; they do not claim native capture.
- Strict Windows Clippy still reports the existing range-loop and
  eight-argument capture-loop findings. They were not suppressed.
- Native Linux evidence proves system-mix capture, while exact-process capture
  fails and then aborts during PipeWire teardown. `InputDevice` remains
  unsupported on Linux and Windows.
- Static inspection also found that Windows exact-process capture passes
  `include_tree = false`, which pinned `wasapi` maps to excluding the target
  process tree. This is a P0 correctness blocker for the next implementation
  step, not a product claim.
- Full evidence and ordered repair plan:
  `../../docs/reports/2026-07-21-cross-platform-native-capture-audit.md`.
- No capture redesign, provider code, scaffold, mock, fallback, or new
  loopback-only product path was introduced.

## W7.6 fast destination-fault matrix correction — 2026-07-20

- Status: `SAFE-TO-TEST`; deterministic destination isolation passes without a
  long soak or a simulated real-device claim.
- The local product-proof example hard-coded a 64-frame edge after branch-pool
  ownership gained one explicit receiver-in-flight slot. That requested 65
  slots from the fixed 64-slot atomic pool and panicked before every fault cell.
  The example now consumes the runtime plan's published maximum edge capacity
  instead of duplicating an invalid constant.
- A regression constructs every normal, slow-connector, slow-recorder,
  connector-failure, and recorder-failure topology and proves all bounded pools
  are valid before media starts.
- Five parallel two-second cells pass. Normal flush delivers 100/100 frames per
  destination. Slow connectors drop only their own 46–47 frames while both
  browser branches deliver 100/100. Connector failures emit one worker failure
  per connector while browsers deliver 100/100. Recorder failure finalizes
  incomplete while all connector/browser branches deliver 100/100. Slow
  recorder drops only recorder frames while connector/browser branches deliver
  102/102. Evidence is under
  `pocketstation-lab/artifacts/product-proof/w7-fast-*-2s-pass-2026-07-20`.
- This is deterministic bounded-runtime evidence, not permission, physical
  device, network reconnect, or production-scale evidence. No scaffold, mock,
  hidden fallback, provider code, unbounded queue, or hot-path work was added.

## W7.5 saturated-edge classification correction — 2026-07-20

- Status: `SAFE-TO-TEST`; the new long candidate remains open.
- The first corrected 60-minute candidate proved capture and recording
  continuity but exposed one synchronized destination recovery burst between
  2,470 and 2,480 seconds. Recorder edges peaked at 15/50 with zero loss;
  every eight-frame relay/connector edge saturated and dropped. Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-soak-60m-corrected-2026-07-20`.
- When a branch copy pool and its queue are simultaneously full, the router now
  observes queue state before classifying a failed copy acquisition. Saturated
  edges report `queue_full`; `branch_pool_exhausted` remains reserved for copy
  ownership exhaustion while queue capacity is still available.
- The receiver-in-flight regression now explicitly requests
  `CopyToBranchPool`. It proves both sides of the contract: a popped in-flight
  frame does not prevent the next enqueue, and queue-plus-in-flight saturation
  is classified as queue-full without pretending the copy pool is undersized.
- Acceptance passes: all 36 runtime tests, both debug and release allocation
  gates, targeted strict Clippy, and workspace format. No queue wait, blocking,
  allocation, lock, logging, panic, scaffold, mock, or loopback-only path was
  added to dispatch.

## W7.4 branch ownership stress correction — 2026-07-20

- Status: `SAFE-TO-TEST`; the corrected isolated long candidate remains open.
- The rejected 60-minute candidate stayed live for 3,600 seconds with zero
  capture-bridge, normalization, recorder-edge, worker, and observation drops,
  but correctly failed the continuity gate. At one common host scheduling
  stall, the default eight-frame remote/connector edges reported one or two
  branch-pool exhaustion drops and both normalized inputs exposed one source
  sequence boundary. Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-soak-60m-binding-2026-07-19`.
- The branch failures exposed an ownership-plan defect: a copy pool reserved
  only the queue capacity even though its sequential receiver can own one
  already-popped frame while the queue accepts the next frame. The memory plan
  now names that maximum in-flight ownership, reserves queue capacity plus the
  receiver allowance, and includes the extra slot in bounded memory accounting.
- A regression test holds the popped frame from a one-frame edge and proves the
  next frame is enqueued without pool exhaustion. A genuinely full slow edge
  remains isolated and is now classified as `queue_full`, not mislabeled as
  branch-pool exhaustion.
- The committed correction passed a 300-second real-device vertical slice:
  application delivered 15,000 frames and microphone delivered 15,001 frames
  independently to recording, relay/browser, and example connector branches.
  Every edge reported zero drops, zero branch-pool exhaustion, zero continuity
  events, and zero worker failures; recording finalized complete. Evidence:
  `pocketstation-lab/artifacts/product-proof/w7-branch-ownership-300s-2026-07-20`.
- The simultaneous source sequence boundary is not suppressed or reclassified.
  The corrected candidate must run without competing builds/fault injections;
  any repeated source boundary remains a W7 failure requiring native capture
  ownership/drop evidence.
- Acceptance passes: 71 `pks-graph` tests, 36 `pks-runtime` tests, both release
  allocation tests, targeted strict Clippy, and workspace format check. No
  scaffold, mock, loopback-only path, hot-path allocation, lock, blocking,
  async work, logging, or panic was introduced.

## W13 consolidated-candidate operational contract — 2026-08-02

- Status: `PARTIAL`, active for the exact W16 single-package candidate.
- The next focused gate binds already-owned capture authorization observations,
  source-generation recovery, route drop accounting, common-clock latency
  coverage, bounded queue/resource observations, and destination isolation into
  one deterministic integration contract.
- The contract must report attempted/delivered/dropped counts, explicit drop
  reasons, nanosecond units, latency sample coverage, and queue capacity/peak;
  zero samples or inferred permission state fail closed.
- This is component evidence only. It adds no automatic source substitution,
  permission guessing, virtual-driver dependency, provider implementation,
  product fallback, or physical-device claim.

## W7.3 exact-source and authorization truth — 2026-07-19

- Status: `SAFE-TO-TEST`; real permission-transition cells and the corrected
  long candidate remain open.
- Capture authorization snapshots now accept authoritative platform
  observations, carry a monotonic observation timestamp, and report an
  unavailable source as unavailable instead of synthesizing capability.
- The macOS control path reads microphone authorization without prompting.
  CoreAudio process-tap creation/start returns the exact operation stage and
  raw `OSStatus`; only the documented permission status maps to typed
  `PermissionDenied`, while every other status remains a typed backend failure.
- Recorder permission events now identify their scope as the explicit Session
  capture grant. They do not claim that a Session grant proves OS permission.
- The first exact-PID real smoke exposed an identity mismatch: process capture
  emitted a PID-derived frame source while the selected source and recorder
  retained the stable application identity. `CaptureMode::ExactApplication`
  now carries both the pinned PID and resolved stable ID. The macOS adapter
  opens only that PID and emits the stable frame identity; Linux/Windows
  targeted paths retain the same contract.
- Corrected real evidence:
  `pocketstation-lab/artifacts/product-proof/w7-exact-pid-auth-12s-pass2-2026-07-19`.
  Both stems reached recording, relay/browser, and connector branches with zero
  drops or continuity events; the microphone permission observation was
  authoritative `allowed` and the lifecycle event log was empty.
- macOS discovery now labels unbundled audio processes from their executable
  basename when AppKit has no application display name. The PID remains the
  exact process-lifetime identity; the fallback label is never promoted to a
  stable or security identity.
- No authorization query, allocation, lock, log, or error classification was
  added to an audio callback. No automatic restart, source substitution,
  system-mix fallback, scaffold, mock, or loopback-only path was introduced.
- Acceptance passes: `cargo test --workspace`, strict workspace Clippy, format,
  and the accepted `product_proof_local` example compile gate.

## W0 product-proof baseline — 2026-07-19

- Protected the existing graph/runtime, capture-adapter, relay, benchmark, and
  documentation worktrees before the Session façade implementation.
- Recorded repository HEADs, overlap classification, and exact acceptance
  results in the factory-root
  `docs/reports/PHASE2_W0_BASELINE_2026-07-19.md`.
- Baseline result: component tests, format, workspace Clippy, and
  `scripts/check_protocol.sh` pass; the accepted `quickstart` build fails
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
- Added the authoritative `quickstart`; it declares application and
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
  allocation tests, the `quickstart` build, workspace strict Clippy,
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
- Real macOS exact-process capture passed with 281 consumed frames, 287,744
  samples, RMS 0.141005, and zero drops at the then-visible captured-frame
  stream boundary. That artifact predates the native-ring counters documented
  above and makes no zero-loss claim for them.
- All 112 CLI tests pass against the updated capture API.
- The capture-stream example is target-gated so Linux and Windows all-targets
  checks compile without pretending the macOS system-loopback endpoint exists.
- Linux capture tests explicitly reject the stream-capacity setup error so the
  cross-platform `CaptureError` contract remains exhaustively checked.

## Capture timestamp epoch observability — 2026-07-24

- `pks-capture` now exposes `timestamp_epoch_clamps_total` with the other native
  capture observations.
- Capture backends can initialize the shared monotonic timestamp domain during
  setup, before a realtime callback reads it.
- The macOS input adapter maps capture instants before the process epoch to the
  earliest representable non-zero timestamp and records each mapping. Zero
  remains reserved for an unavailable timestamp.
- The change adds no allocation, blocking, logging, or async work to the
  callback. The callback performs an initialized clock read, bounded
  pool/ring work, and relaxed atomic observations.
- The physical built-in-microphone acceptance produced 150/150 frames with no
  pipeline loss. Evidence is owned by
  `pocketstation-io/pks/artifacts/macos-mic-timestamp-repair-2026-07-24`.
- `bash scripts/check_protocol.sh`, focused capture/timing tests, strict clippy,
  the product quickstart build, and the full workspace tests pass.

## Session control events and capture-owned lineage — 2026-07-26

- `pks-session` now owns a bounded, non-blocking control-event queue with
  public polling and queue observations. Event values preserve stable session,
  stem, route, and endpoint IDs and keep source, endpoint, rollback, and
  finalization failures separate in the terminal outcome. Construction and
  publication authority remain crate-private.
- Queue overflow drops the newest event and records depth, peak depth,
  enqueue, drop, and receiver-closure counts. Six focused tests pass for FIFO
  delivery, overflow, closure, and terminal failure preservation.
- `pks-capture` now establishes `CaptureOpenMetadata` only after native open
  succeeds. `CaptureOwner` wraps frames with the authoritative session/stem
  seed, the declared monotonic capture clock, named initial source and
  permission epochs, and discontinuity state advanced from typed source runtime
  events. Session code no longer needs to fabricate lineage epoch literals.
- `cargo test -p pks-capture` passes all 50 tests and
  `cargo clippy -p pks-capture --all-targets -- -D warnings` passes.
- `RunningSession` now publishes the typed lifecycle, source, rollback,
  endpoint, finalization, and terminal events and exposes the sole receiver.
  Failed startup returns `SessionStartFailure`, retaining the root typed error,
  the bounded receiver, and every exact rollback failure instead of reducing
  rollback truth to a count or dropping the only receiver.
- The focused `pks-session` tests pass 27/27, strict all-target Clippy passes,
  and the full 107-file CODE_PROTOCOL gate passes. This is component-level
  `SAFE-TO-TEST` evidence; concrete relay/browser and example connector
  endpoint adapters plus the installed real-path Session proof remain open.

## W11 transactional RunningSession — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-session` now owns one real startup and shutdown
  transaction over the accepted graph, runtime, capture, and endpoint owners.
- Startup validates the narrow one-application plus one-microphone topology,
  prepares exact endpoint batches, starts endpoint workers behind one closed
  gate, opens both capture owners, creates the real runner, transfers all
  resources through a bounded worker Start command, waits for readiness, and
  opens the gate exactly once before publishing `Running`.
- Thread-spawn failure retains the captures and runner for caller-thread
  rollback. Every later failure unwinds runner, captures, started endpoints,
  and prepared endpoints in reverse acquisition order. Failed startup returns
  the root error, exact typed rollback details, and the bounded event receiver;
  it publishes `Failed` and a failed terminal outcome before returning.
- Runtime ingress accepts only capture-owned `LineagedAudioFrame` values.
  Typed source disappearance terminates the Session path and is preserved in
  the terminal outcome. Full bounded source queues, lineage rejection, runner
  failure, capture finalization failure, endpoint stop failure, endpoint join
  failure, and worker panic all prevent a successful stop result.
- Stop is idempotent. It requests runtime termination, joins capture and runner
  ownership, requests endpoint stop, joins and finalizes every endpoint, then
  publishes `Stopped` only for a clean result or `Failed` otherwise.
- Four transactional tests cover five endpoint owners for six routes,
  two authoritative source lineages, zero pre-gate delivery, repeated stop,
  endpoint prepare/start rollback, second-capture-open rollback, exact failure
  events, and zero leaked test owners.
- Integrated acceptance passes: 50 `pks-capture`, 43 `pks-runtime` plus three
  allocation gates, six `pks-endpoint`, 47 `pks-nodes`, and 27 `pks-session`
  tests; focused strict Clippy; workspace format; and the full 107-file
  CODE_PROTOCOL gate.
- No concrete relay/browser or connector driver, public compatibility façade,
  C adapter, provider implementation, mock product path, fallback, helper
  process, or new loopback-only path was introduced. Those remain later W11
  gates.

## W11 exact application process-instance declaration — 2026-07-26

- Status: `SAFE-TO-TEST`; the Session declaration now has an explicit
  `ApplicationSelector::ProcessInstance` form that retains both the selected
  process ID and the discovered stable application identity.
- Compilation preserves the two values as separate typed source-node
  configuration fields. Session startup lowers only this strong form to
  `CaptureMode::ExactApplication`; it is never weakened to the legacy bare
  `Process` or name-based modes.
- The existing `ProcessId` selector remains explicitly process-lifetime scoped.
  The existing stable-application selector remains stable-identity scoped.
  Linux and macOS capture-mode behavior is unchanged; the Windows backend
  continues to parse the stable process-incarnation fingerprint and verify the
  PID plus creation time before and after WASAPI activation.
- Focused tests prove compiled-configuration and runtime-mode preservation.
  `cargo test -p pks-session -p pks-capture --locked` passes 36 and 50 tests,
  strict focused Clippy passes, and both `pks-session` and
  `pks-capture-windows` cross-check for `aarch64-pc-windows-msvc`. The full
  108-file CODE_PROTOCOL gate passes.
- No fallback, mock, scaffold, provider integration, CLI/SDK behavior, capture
  hot-path work, or loopback-only product claim was introduced.

## W11 live plan-edge observation handle — 2026-07-26

- Status: `SAFE-TO-TEST`; `pks-runtime` now exposes a cloneable, read-only
  `PlanEdgeObservationHandle` from `PlanEdgeReceiver` before receiver ownership
  moves into an endpoint worker.
- The handle is exported from `pks-runtime`; downstream endpoint adapters do
  not reach through a private module or duplicate snapshots.
- The handle shares the existing authoritative `EdgeTelemetry` atomics. It
  introduces no duplicate counters and exposes no mutation, lifecycle, sender,
  or receiver authority.
- Live snapshots retain queue capacity/depth/peak, enqueue/delivery/drop
  reasons, latency coverage, discontinuities, worker failures, and shutdown
  discards after the receiver and router have been dropped.
- Three Given/When/Then tests prove producer-side queue saturation and full
  drops, consumer-side sequence/timestamp discontinuities, and post-drop
  shutdown-discard snapshots through a cloned handle. The focused plan-router
  gate passes 16/16 tests, strict all-target `pks-runtime` Clippy passes, and
  workspace formatting passes.
- No provider implementation, mock, scaffold, fallback, new counter source, or
  loopback-only product path was introduced. Concrete endpoint adapters still
  own the next W11 integration step.

## W11 portable codec build flags — 2026-07-28

- Status: `SAFE-TO-TEST`; the repository no longer injects host-specific
  `target-cpu=native` and `-march=native` flags into every Rust and C build.
- The global flags made release artifacts depend on the build machine and
  broke the real `aarch64-linux-android` codec archive at the NDK compiler
  boundary. The NDK correctly rejected the inherited host-only C flag.
- Native CPU tuning is now an explicit benchmark or local-development choice,
  not an implicit workspace policy. Release and SDK artifacts use the selected
  target toolchain's portable defaults.
- `scripts/build-android-codec-c.sh` pins NDK `26.1.10909125`, API 29,
  `aarch64-linux-android`, and `arm64-v8a`, supplies the exact NDK linker and
  archiver, and writes the immutable archive layout consumed by the Android
  SDK and lab gate.
- This change introduces no scaffold, mock, fallback, or loopback-only path.
  The Android linked-component gate remains responsible for proving the exact
  archive, JNI shared library, definitions, unresolved symbols, and hashes.

## W11 active architecture truth — 2026-07-28

- Active repository instructions now name `pks-dsp`, `pks-session`,
  `pks-session-c`, and `pks-codec-c` according to their accepted ownership.
- Superseded ADR implementation notes now distinguish retained decision history
  from current package and FFI guidance.
- The broad v3 AudioGraph document remains available as historical product
  vision, but its header and footer both point to the binding product,
  repository, execution, and crate contracts.
- No implementation, API, wire behavior, product claim, scaffold, mock, or
  loopback path changed.

## W12 Rust registry-role policy — 2026-07-28

- Status: `SAFE-TO-TEST`; the workspace now identifies the supported public
  Rust registry surface with machine-readable package roles.
- `pocketstation` is the single `public-facade`. Only its exact transitive
  workspace normal/target dependency closure is marked `facade-dependency`
  and remains publishable.
- `pks-codec`, `pks-codec-c`, and `pks-session-c` are explicitly `deferred`
  and `publish = false`; the Whisper example is explicitly `example` and
  remains non-publishable. These packages are not dependencies of the
  supported façade.
- `scripts/publish.sh` now derives and validates the façade closure instead of
  selecting every publishable workspace package. It rejects missing/unknown
  roles, role/closure drift, accidental non-closure publication, incomplete
  dependency ordering, and any order that does not place `pocketstation`
  last.
- This policy does not publish crates and does not create a provider,
  compatibility path, mock, scaffold, fallback, or loopback-only product
  behavior.

## W12 Linux protocol-gate dependency — 2026-07-28

- Status: `SAFE-TO-TEST`; the CI and release-validation environments now
  install `ripgrep` before invoking `scripts/check_protocol.sh`.
- The first main-branch W12 run passed workspace tests, strict Clippy, the
  release quickstart, and architecture constraints, then failed closed because
  the Ubuntu runner did not provide the protocol scanner's required `rg`
  executable.
- The correction changes no Rust source, product API, runtime behavior,
  publication closure, or release trigger. The publish job remains gated by
  the complete validation job.
- No scaffold, mock, fallback, provider, or loopback-only path is introduced.

## W12 crates.io partial-publication recovery — 2026-07-28

- Status: `SAFE-TO-TEST`; the first protected `0.1.0` publication passed its
  complete validation job and published six dependency crates before
  crates.io rejected the seventh first-time crate with HTTP 429 and an
  explicit retry timestamp.
- Publication is now exact-version idempotent: registry-visible versions are
  skipped, missing versions resume in dependency order, first-time crate names
  use conservative pacing, and only a crates.io 429 with a parseable retry
  timestamp can trigger a bounded retry. Registry-query failures and every
  other publish error fail closed.
- Manual recovery requires the exact version-matched release tag, current
  `main`, the release commit as an ancestor, and a path allowlist proving that
  no package or product source changed after the release.
- A fake Cargo/registry/sleep contract proves six-version skip and nine-version
  resume, 429 retry timing, and non-429 fail-closed behavior without network or
  publication.
- The partial-resume assertion compares the exact crate set rather than
  platform-specific ordering among independent topological nodes; dependency
  order and the public façade-last invariant remain enforced by the publisher.
- No crate source, public API, runtime behavior, scaffold, mock, provider, or
  loopback-only path changed.

## W12 public façade docs.rs correction — 2026-07-28

- Status: `SAFE-TO-MERGE`; docs.rs received `pocketstation 0.1.0` but build
  `3978948` failed before rustdoc because its default Linux target selected the
  native PipeWire backend and the docs.rs sandbox does not provide
  `libpipewire-0.3`.
- The public façade API surface is target-independent; native runtime backends
  remain platform-specific. Documentation will use one explicitly pinned
  Windows cross-compilation target, which exercises the public Rust surface
  without requiring a host audio development package.
- The façade patch release is independent from the unchanged `0.1.0` internal
  dependency closure. CI and protected release validation cross-document the
  configured docs.rs target before publication.
- The exact cross-target `cargo doc`, normalized package inspection, full
  workspace tests, strict Clippy, quickstart release build, architecture gate,
  CODE_PROTOCOL, Actionlint, and the 15-package closure dry run pass locally.
- This correction changes no Session API, runtime behavior, capture path,
  provider integration, scaffold, mock, fallback, or loopback-only claim.

## W12 Session-owned observed-lineage multistem recorder — 2026-07-28

- Status: `SAFE-TO-TEST`; `pks-nodes` now exposes the additive
  `SessionMultistemEndpointCoordinator::new(output_root, group_id)` declaration
  without accepting caller-fabricated Session, endpoint, route, stem, source,
  clock, generation, or permission identities.
- Exact `prepare_batch` inputs supply the Session, endpoint, typed stem/route,
  sample specification, label, recording group, and one common Session
  timeline origin. Missing typed context, mixed origins, and duplicate
  endpoint, route, stem, or label identities fail during preparation.
- The first delivered frame for every stem must carry authoritative
  `FrameLineage`. Its source, clock, source generation, and permission epoch
  initialize the recorder manifest; raw frames and later identity drift fail
  closed. Successful Session delivery records only
  `SessionCaptureGrant/Allowed` and does not claim an operating-system
  permission decision.
- Both stems map the common `SessionTimelineOrigin` to Session time zero, so
  independently captured application and microphone frames retain one
  comparable recording timeline.
- The public 0.1 `MultistemRecording::start` and explicit-config coordinator
  remain source-compatible. Their raw-frame compatibility path is unchanged;
  required-lineage behavior is selected by a private typed mode used only by
  the canonical Session coordinator.
- Focused tests prove two-stem derivation and final completion, common origin,
  missing typed route rejection, raw first-frame rejection without an
  artifact, and later permission-lineage mismatch with an incomplete outcome.
  All 56 `pks-nodes` tests, strict all-target/all-feature Clippy, workspace
  diff checks, and the complete 125-file CODE_PROTOCOL gate pass.
- No provider, connector, mock, scaffold, fallback, operating-system
  permission assertion, or loopback-only product claim was introduced. The
  full Rust façade and Lab reference artifact remain separate W12 acceptance
  gates.

## W12 central package-boundary repair — 2026-07-29

- Status: `SAFE-TO-MERGE`; the package repair defined by AUDIO-032 is
  complete on candidate `pks-20260729-w12-central-boundary-repair`.
- `pks-session` now owns stable language-neutral Session result codes and the
  bounded polled-audio projection consumed by foreign-language adapters. Its
  supported implementation no longer depends on the transitional
  `pks-nodes` package.
- `pks-recording` now owns the concrete multistem WAV endpoint, recording
  coordinator, finalization, manifest, and recording outcomes.
  `pks-nodes` retains no duplicate recording implementation.
- `pks-nodes` and `pks-dsp` are non-publishable deferred packages. The
  validated façade publication closure is now 14 packages and contains only
  packages required by `pocketstation`.
- Full workspace tests, strict workspace/all-target Clippy, formatting,
  release-mode `quickstart`, architecture constraints,
  `CODE_PROTOCOL`, and the 14-package publish dry run all pass at commit
  `364ebac`.
- This is a component and package-ownership result. It introduces no provider,
  connector, scaffold, mock, fallback, loopback-only path, or product-proof
  claim. The W12 Rust reference and Lab evidence remain the next acceptance
  slice.

## W12 public recording reference integration — 2026-07-29

- Status: `SAFE-TO-TEST`; the focused implementation and component gates pass,
  while the exact registry-installed Lab artifact remains the W12 exit.
- The accepted `pks-recording` coordinator, `StemHandle::record` declaration,
  Session host registration, and safe recording receipt remain the canonical
  owners. The active slice only projects their setup and outcomes through the
  public `pocketstation` façade.
- `Session::builder().recording_root(...)` registers that existing owner.
  Recording routes without an explicit root fail with
  `session.missing_recording_configuration`; `RunningSession` retains the
  terminal recording outcome. Stable recording codes cover every concrete
  recorder failure plus incomplete and not-yet-finalized outcomes without
  exposing string-message matching to language adapters.
- The public quickstart observes both source-aware stems under a bounded
  deadline and requires complete two-stem finalization. The deterministic
  conformance fixture emits truthful frame timing and proves that a saturated
  bounded polled-audio branch drops independently while both recording edges
  deliver 16 continuous frames without drops.
- Focused acceptance passes: 17 recording tests, 59 Session tests, eight
  façade tests including conformance, and strict all-target Clippy. The CLI
  migration and Lab exact-registry artifact remain separate repository gates.
- Registry patch identities now match the changed publication boundary:
  `pks-endpoint 0.1.1`, new `pks-recording 0.1.0`, `pks-session 0.1.1`, and
  public `pocketstation 0.1.2`. The public façade advances past the already
  published documentation-only `pocketstation 0.1.1`; the other ten closure
  packages remain at their
  already-visible `0.1.0` versions. The 14-package dry run passes in exact
  dependency order; no version already visible on crates.io is overwritten.
- No provider, new runtime, new package, mock, scaffold, fallback, or
  loopback-only production path is introduced.
- The pre-publication registry audit found that the documentation-only
  `pocketstation 0.1.1` release already occupies that façade version. The W12
  façade therefore advances to unused version `0.1.2`; the new endpoint,
  recording, and Session versions remain `0.1.1`, `0.1.0`, and `0.1.1`.
- A repeated release-gate run exposed a scheduler-dependent conformance
  failure: the finite fixture emitted four-sample frames on a three-millisecond
  wall cadence, allowing its eight-frame recording edge to overflow. The
  fixture now emits eight real 20-millisecond, 960-sample frames per source and
  waits on source-completion and destination-saturation counters rather than a
  sleep. The formerly failing recording-isolation case passes 20 consecutive
  runs.
- After that correction, full workspace tests, strict all-target Clippy,
  release quickstart compilation, `CODE_PROTOCOL`, architecture constraints,
  formatting, and the exact 14-package publication dry run pass for
  `pocketstation 0.1.2`.
- The `pocketstation-v0.1.2` validation run correctly withheld publication
  when its recovery test still expected the superseded 15-package closure.
  The recovery fixture now expects six already-visible packages and the exact
  eight missing packages, including `pks-recording` and excluding deferred
  `pks-dsp` and `pks-nodes`; it expects seven inter-package pacing waits.

## W13 macOS microphone authorization boundary — 2026-08-06

- Status: `SAFE-TO-MERGE`; physical acceptance remains bound to a newly
  committed and packaged candidate.
- The macOS input backend now rejects `Denied`, `Restricted`, and `Revoked`
  observations before opening CPAL, with the stable typed operation
  `opening the macOS microphone input stream`. Allowed and non-authoritative
  observations retain the native open path.
- Focused permission tests and strict workspace all-target Clippy pass. No
  prompt automation, permission inference, capture fallback, mock, scaffold,
  or loopback-only product path was introduced.
- Audit of the earlier frozen `pocketstation-0.1.2.crate` proved that it
  predates this repair. Its physical permission artifacts are retained as
  diagnostic history but cannot close exact-candidate W13 acceptance.

## W13 Windows bounded-worker scheduling repair — 2026-08-08

- Status: `SAFE-TO-TEST`; Windows guest acceptance remains a separate Lab
  artifact and no physical-device claim is made.
- A fresh W20-based ARM64 guest matrix passed system mix, exact application,
  microphone, two restart rounds, exact-application isolation, and the
  15-second concurrent app-plus-microphone Session proof. All three third
  restart cells then failed closed with explicit bounded dispatch/capture
  queue loss.
- The WASAPI capture producer already joined the Windows `Audio` MMCSS task,
  but its owned dispatch consumer did not. The consumer now joins the same
  task for its lifetime so ordinary scheduler pressure cannot starve the
  bounded handoff while the producer continues delivering packets.
- MMCSS and the one-millisecond timer request are now paired with explicit
  teardown on the owning worker. Registration remains fail-open, while every
  existing drop counter remains authoritative and fail-closed.
- No queue was made unbounded, no capacity or loss threshold changed, and no
  fallback, mock, scaffold, loopback product path, or soak was introduced.

## W18 public Session engine source registration — 2026-08-08

- Status: `SAFE-TO-MERGE` for the registration slice only; Session source
  declaration, compilation, preparation, and runtime lifecycle remain separate
  W18 acceptance tasks and are not claimed here.
- `SessionEngineBuilder::register_source_factory` now owns the validated open
  `SourceFactory` registry. `build()` transfers that registry into the one
  `SessionEngine`; read-only manifest lookup proves the registration is retained
  rather than written into an unused temporary registry.
- Registration is keyed by open `SourceTypeId`. Complete manifest validation is
  enforced at the public boundary, zero revisions fail typed, and a duplicate
  stable identity cannot silently replace the first registered revision.
- Three focused `session::source_registration` tests, strict all-target and
  all-feature Clippy, the release `quickstart` build, and the full
  `CODE_PROTOCOL` gate pass.
- No closed source enum, industry/customer vocabulary, capture callback change,
  runtime source worker, scaffold, mock, fallback, loopback-only path, platform
  matrix, or soak was introduced.

## W19 signal-aware async Operator preparation — 2026-08-08

- Status: `SAFE-TO-MERGE` for `W19-ASYNC-PREPARE-CONTEXT` only. Public named
  Session composition, Session-owned multi-stage runtime, and generated-audio
  reentry remain separate gated W19 tasks.
- `AsyncNode::prepare` now receives an `AsyncOperatorPrepareContext` containing
  the exact execution partition and bounded input/output edge records. Each
  record carries its named port, negotiated `SignalSpec`, `MediaCaps`, complete
  `RouteSettings`, optional compiled `EdgeId`, and capacity in signals.
- Session runtime preparation no longer converts an Operator input through
  `prepare_context_for_media` or fabricates `SampleSpec`. It preserves the
  compiler's named input, negotiated media and contract, and the exact bounded
  capacity from the typed-edge or audio-edge plan. Audio endpoint workers keep
  their specialized audio-only `PrepareContext`.
- Direct external harnesses with wildcard manifests can supply an explicitly
  negotiated signal-shaped context through `spawn_with_context`; its ports,
  contracts, and capacities must agree with the actual bounded worker edges or
  preparation fails closed.
- Real prepare execution is covered for text, event, metrics, control, binary,
  and schema-backed custom signals. A rejection test proves that a capacity
  mismatch cannot pass as accepted preparation. The existing Session-owned
  audio-to-Operator route and the external Whisper connector both pass using
  negotiated audio media.
- Focused async-operator and graph gates, all 508 library tests plus all
  integration/ABI/hot-path targets, strict all-target/all-feature Clippy,
  Whisper's 15 tests and strict Clippy, release `quickstart`, and the
  complete `CODE_PROTOCOL` gate pass.
- No realtime callback, audio pool, hot-path executor, provider vocabulary,
  customer/domain type, scaffold, mock, fallback, loopback-only product path,
  platform matrix, release, or soak was introduced.

## W19 Session operator instance and named-connection declaration — 2026-08-08

- Status: `SAFE-TO-MERGE` for `W19-OPERATOR-INSTANCE-CONNECTIONS` only.
  Session-owned multi-input runtime execution remains the next separately gated
  task; this checkpoint does not claim that a multi-input Session can start.
- `Session::operator` now declares exactly one `OperatorInstanceSpec`.
  `StemHandle`, `SourceOutputHandle`, and `DerivedStreamHandle` connect to an
  exact `OperatorInputHandle`, while `OperatorInstanceHandle::output` exposes
  exact named outputs. `through()` writes the same instance and connection
  records and is no longer a separate one-input schema.
- Session schema 1.4 separates operator identity/configuration from
  `OperatorConnectionSpec`. One instance can own multiple named input
  connections and multiple named derived routes without hidden duplicate
  nodes.
- The Session compiler creates all declared operator nodes before wiring
  connections. This permits operator-to-operator references independent of
  declaration order and validates unknown ports, missing required inputs,
  duplicate single-multiplicity inputs, signal/media compatibility, direction,
  and graph cycles before runtime preparation.
- Focused `session::operator_connections` and `graph::named_ports` gates pass.
  Compiler tests prove that two independent audio stems enter one two-input
  operator node and that its two named outputs leave that same node. All 516
  library tests and every integration/ABI/hot-path target pass, as do strict
  all-target/all-feature Clippy, release `quickstart`, architecture
  constraints, formatting, and the complete `CODE_PROTOCOL` gate.
- No runtime owner, callback, audio pool, hot-path executor, provider/customer
  vocabulary, scaffold, mock, fallback, loopback-only path, platform matrix,
  release, or soak was introduced.

## W19 Session-owned composed operator runtime — 2026-08-08

- Status: `SAFE-TO-MERGE` for `W19-SESSION-COMPOSED-RUNTIME` only.
  Generated-audio reentry, the W19 fault matrix, and every W20 release/freeze
  task remain separately gated and are not claimed here.
- `PreparedSession` and `RunningSession` now own one worker per declared
  operator instance across compiled audio inputs, external typed-source inputs,
  and upstream operator outputs. Three-stage chains and named multi-input/output
  instances use one bounded typed-edge implementation and retain exact port,
  signal, media, edge-contract, capacity, lifecycle, cancellation, join, and
  finalization observations.
- Public `RunningSession` metrics expose each Session-owned external source,
  operator instance, exact input port, and derived route without exposing
  internal worker or fanout constructors. Shutdown cancels composed operators
  in reverse dependency order and preserves the existing graceful drain order.
- The external public-Session consumer found and fixed a compiler authority
  defect: a derived endpoint whose chain originated at an external source had
  inherited the source-input connection route ID. Endpoint configuration now
  always carries the actual derived route ID while source identity remains
  independent metadata.
- The focused public-Session composition test passes, all 517 library tests and
  every integration/ABI/hot-path target pass, strict all-target/all-feature
  Clippy passes, the release `quickstart` builds, and the complete
  `CODE_PROTOCOL` gate passes.
- This slice changes no capture callback, audio pool, or realtime executor. It
  adds no unbounded queue, manual fixture worker/fanout, generated-audio claim,
  provider/customer/domain vocabulary, scaffold, mock, loopback product path,
  platform matrix, release, freeze, or soak.

## W20 typed compiled Session bindings — 2026-08-09

- Status: `SAFE-TO-MERGE` for `W20-TYPED-SESSION-BINDINGS` only. Endpoint and
  recording ownership, public API reduction, module decomposition, executable
  extension ABI, sidecar lifecycle, packaging, release, and Core 1.0 freeze
  remain separately gated.
- `CompiledSession` now owns a typed `CompiledSessionBindings` table keyed by
  compiled `NodeId`. Each Session-created node carries its exact source, stem,
  operator, endpoint route, stream origin, or generated-audio ownership without
  encoding those identities into `NodeConfig` string keys.
- Runtime preparation consumes those typed bindings for built-in and external
  sources, operator inputs and outputs, raw and derived endpoints, and generated
  audio ingress. Missing or incompatible bindings fail with typed preparation
  errors; Session metadata is no longer reparsed from extension configuration.
- `NodeConfig` remains the open, opaque extension-owned configuration surface.
  An external endpoint may use a key that previously collided with an internal
  metadata name without overriding the compiled Session identity. No provider,
  customer, or domain enum was introduced.
- The polled-audio endpoint now receives its route through typed
  `EndpointRouteContext`; this removed its last dependency on a string route ID
  and fixed every affected Session host and C-ABI regression cell.
- Focused compiler tests pass 13/13, focused runtime-preparation tests pass 5/5,
  and all 410 all-target/all-feature tests pass. Strict Clippy, the release
  `quickstart`, formatting, hot-path checks, and the complete
  `CODE_PROTOCOL` gate pass.
- This slice changes no capture callback, audio buffer pool, realtime executor,
  queue capacity, platform implementation, product path, scaffold, mock,
  loopback claim, physical evidence, release, freeze, or soak.

## W20 endpoint and recording ownership — 2026-08-09

- Status: `SAFE-TO-MERGE` for `W20-ENDPOINT-RECORDING-OWNERSHIP` only. Public
  API reduction, module decomposition, executable extension ABI, sidecar
  lifecycle, packaging, release, and Core 1.0 freeze remain separately gated.
- Every endpoint input now uses one `EndpointPortInput` containing its exact
  port, `SignalSpec`, media, `RouteSettings`, receiver, and required typed
  Session route context. `EndpointReceiver` represents either the bounded
  realtime audio receiver with its prepare context or the bounded signal
  receiver; provenance does not select a different endpoint interface.
- `EndpointDriverFactory` has one `prepare` method and
  `EndpointDriverRegistry` has one `prepare_batch` dispatch. The former
  `prepare_derived`, `DerivedEndpointDriverInput`, separate signal-route
  context, and derived-contract option fields are removed.
- Raw audio and operator-derived routes are assembled into the same pending
  input representation and enter one endpoint batch-preparation function.
  Default factories own one lifecycle per route; factories may declare an open
  typed shared group without Session recognizing recorder or provider names.
- The multistem recording factory now owns group selection and validation.
  Recorder descriptor/configuration construction and the `record` convenience
  API live in the recording-specific Session extension; generic Session draft
  and runtime code no longer inspect recording group policy.
- Focused endpoint tests pass 31/31 and recording tests pass 12/12. All 410
  all-target/all-feature tests, strict Clippy, release `quickstart`,
  formatting, hot-path checks, and the complete `CODE_PROTOCOL` gate pass.
- This slice changes no capture callback, audio buffer pool, realtime executor,
  queue capacity, platform implementation, scaffold, mock, physical proof,
  release, freeze, or soak.

## W20 capability and performance recovery — 2026-08-09

- Status: `SAFE-TO-MERGE` for the named recovery task; candidate
  `pks-20260809-capability-performance-recovery-3` is bound by the hashed
  acceptance manifest. This is not public API, release, Core 1.0 freeze,
  physical-platform, or competitive acceptance.
- Real libopus encode/decode/PLC and deterministic timing drift/correction are
  again compiled in the single `pocketstation` engine. Focused codec/timing,
  100-frame allocation gates, and the previously compiled C++ codec consumer
  pass. Criterion encode/decode/PLC targets compile; measured baselines remain
  pending.
- The Rust capture handoff now has direct zero-allocation gates for normal
  delivery, full-ring drop-newest, closed-start discard, and pool exhaustion.
  Seventy-one focused capture tests pass. These gates do not substitute for
  physical CoreAudio, PipeWire, or WASAPI callback evidence.
- Typed asynchronous edges now enforce both count and byte bounds: 64 signals
  maximum per branch, 1 MiB default payload, and 16 MiB hard payload maximum.
  Oversized payloads are rejected before partial fan-out, and observations
  expose maximum payload and maximum buffered payload bytes. Focused
  typed-edge and async-operator tests pass; measured baselines remain pending.
- The stale empty `src/dsp/` and `src/runtime/nodes/` directories were removed.
  Simplified historical AEC/denoise/VAD/watermark code remains deliberately
  absent rather than being restored as a production claim.
- No long soak, provider/customer/domain vocabulary, unbounded queue, second
  engine, or LiveKit superiority claim was introduced.

### Measured recovery update

- Corrected every Criterion target to use `harness = false`; the benches now
  execute rather than merely compile under Rust's default benchmark harness.
- Added measured local baselines for pool acquire/drop, synthetic capture
  handoff, captured-frame SPSC, Opus encode/decode/PLC, typed one/three-branch
  fan-out, three-branch runtime routing, realtime executor, two-source runner,
  generated-audio reentry, and public two-source Session lifecycle. Exact
  intervals and limitations are recorded in
  `docs/execution/evidence/W20-CAPABILITY-PERFORMANCE-RECOVERY/LOCAL_SHORT_BASELINE_2026-08-09.md`.
- Replaced the generated-audio bridge's 1 ms idle polling dependency with an
  explicit non-realtime consumer-thread wakeup. The local typed-PCM to pooled
  audio round trip measured 3.2206–3.3499–3.5198 us. A 100 ms timeout remains
  only as lost-wakeup/abandonment fallback.
- Added a standalone `conformance-fixtures` feature compile correction; it no
  longer relies on `internal-testing` to name the crate-private capture delivery
  result. The Session lifecycle benchmark is explicitly `LOOPBACK-ONLY`.
- The queue inventory found variable-size control events that were count-bound
  but not byte-bound. Capture-runtime events now enforce a 64 KiB per-event and
  `capacity × 64 KiB` owned-memory ceiling; Session events enforce 1 MiB and
  `capacity × 1 MiB`. Both expose byte depth/peak and oversized-drop truth and
  have direct rejection tests. Fixed-size trace records and capacity-one async
  lifecycle rendezvous are now explicit in the capability ledger.
- Removed historical codec policy profiles from Core. Explicit `OpusConfig`
  remains the codec primitive; product/transport presets belong outside Core.
- `pocketstation::codec` and `pocketstation::timing` are now deliberate public
  capability namespaces. The Opus API uses caller-owned buffers, validates the
  configured frame duration, supports bounded 10/20/40/60 ms frames, and has no
  shipping mock encoder/decoder or allocating vector convenience path.
- Added direct-libopus calibration cases beside the wrapper Criterion cases.
  The later local wrapper slowdown matched direct libopus in the same process,
  so it is recorded as a host power/scheduling regime change rather than a Core
  regression; no unsupported threshold or LiveKit comparison is claimed.
- Audited the target capture boundaries in source. Rust CPAL, PipeWire, and
  WASAPI paths plus the native macOS tap/ASP ring paths now have a protocol
  regression gate forbidding allocation, locks, blocking, async, logging, and
  panic operations at their callback/write boundary. macOS RMS calculation was
  moved from the tap callback to the non-realtime reader. Physical target proof
  remains separately pending and is not inferred from this source audit.
- Added a measured timing baseline for drift observation/snapshot and PI
  correction tick. Exact intervals, command lines, host limitations, and all
  other local measurements remain in the named evidence document.
- All 440 unit tests plus integration/ABI/allocation targets pass. Strict
  all-target/all-feature Clippy, release quickstart, all benchmark compilation,
  and `CODE_PROTOCOL` pass. Physical platform and competitive claims remain
  pending and are not inferred from these local component measurements.

## W20 ownership remediation in progress — 2026-08-09

- Status: `PARTIAL`. These edits are implementation work on the dirty current
  candidate and have not been rerun through acceptance. Earlier green evidence
  does not approve them.
- Session lifecycle telemetry no longer lives inside `running.rs`: operator
  input aggregation and runtime/final metric bindings now have private,
  lifecycle-owned modules. This changes no queue, counter, pool, capture
  callback, worker scheduling, or endpoint behavior.
- The async signal runtime now separates bounded operator I/O, observation
  state, and worker failure vocabulary from worker execution. The existing
  typed-edge count/byte bounds and saturation outcomes remain the authority;
  no unbounded or realtime signal queue was introduced.
- Session compilation and preparation now separate immutable compiled output,
  typed runtime mappings, and compile/prepare errors from their coordinating
  algorithms. The compiler still emits `CompiledSessionBindings` and runtime
  preparation still consumes those bindings directly; `NodeConfig` remains
  extension configuration rather than an internal identity transport.
- The deferred gate remains unchanged: no tests, benchmarks, soak, platform
  claim, public API acceptance, ABI acceptance, release, or Core 1.0 freeze is
  inferred from this structural slice.

## W20 public API boundary — 2026-08-12

- Status: `SAFE-TO-MERGE` for `W20-PUBLIC-API-BOUNDARY`; this does not accept
  module decomposition, native ABI callbacks, sidecar hosting, release, or the
  Core 1.0 freeze.
- The normal crate root now exposes the Session façade, checked signal and
  audio ownership contracts, source/operator/endpoint authoring contracts,
  stable errors, and immutable observations. Runtime workers, routers, queue
  constructors, registries, compiler/preparation owners, and sidecar framing
  remain private; the hidden `internal-testing` feature is the only benchmark
  reach-through.
- `Stream<T>` remains Rust compile-time façade metadata. Runtime and ABI
  identity remain `SignalSpec` plus stable schema identifiers; no Rust `T`,
  provider type, customer type, or industry vocabulary entered the engine.
- A packaged external repository compiled and ran supported capture, source,
  operator, endpoint, typed-stream, and Session declarations without
  `internal-testing` or a source-tree reach-through.
- The exact all-target/all-feature tests, all-feature rustdoc, packaged
  consumer, strict Clippy, release `quickstart`, and `CODE_PROTOCOL`
  gates pass. The acceptance manifest is
  `docs/execution/evidence/W20-PUBLIC-API-BOUNDARY.acceptance.json` in the
  workspace execution authority.
- No capture callback, pool ownership, queue capacity, realtime executor,
  physical platform path, competitive claim, or endurance claim changed.

## W20 module decomposition — 2026-08-12

- Status: `SAFE-TO-MERGE` for `W20-MODULE-DECOMPOSITION`; native callbacks,
  sidecar hosting, release, and Core 1.0 remain unaccepted.
- Graph contracts now have the canonical public `pocketstation::graph`
  namespace while compiler, plan, registry, and runtime machinery stay private.
- Session composition for polled audio and recording now lives under Session
  extensions; lower frame, capture, graph, runtime, endpoint, recording,
  codec, and timing modules do not import Session.
- Legacy direct operator-worker construction is test/internal-only. The public
  shipping path is Session-owned composition, not a second runtime.
- No source directory is empty. Codec and timing are compiled owners; advanced
  DSP remains intentionally external rather than represented by a placeholder.
- All-target/all-feature tests, strict Clippy, release quickstart, and protocol
  gates pass. The acceptance manifest is
  `docs/execution/evidence/W20-MODULE-DECOMPOSITION.acceptance.json`.
- No callback, pool, bounded queue, hot-path `Drop`, realtime executor,
  physical path, or endurance claim changed, so no new long soak is required.

## W20 native executable ABI — 2026-08-12

- Status: `SAFE-TO-MERGE` for `W20-NATIVE-EXECUTABLE-ABI`; candidate
  `pks-20260812-w20-native-abi-3` is hash-accepted. This does not accept the
  sidecar, managed SDK parity, release, physical proof, or Core freeze.
- Extension ABI 1.1 copies descriptor data and owns validation, create,
  prepare, produce/process/consume, request-stop, finish, instance destroy,
  and registration destroy through the existing Session engine. C callbacks
  execute only on source/operator/endpoint workers and cannot declare the
  reserved realtime PCM signal namespace.
- Payload buffers and typed queues are bounded. A C metrics snapshot projects
  the native Session source/operator/route/endpoint counters, byte ceilings,
  saturation, and failures; a typed-only Session reports no synthetic audio
  queue.
- The packaged installed-header consumer passes success and operator-failure
  lifecycle cases with C11 atomic observations. A consumer compiled against a
  frozen Extension ABI 1.0 header still links and runs against the candidate.
- All 443 unit tests plus integration/ABI/allocation/benchmark targets, strict
  Clippy, release quickstart, and `CODE_PROTOCOL` pass. Acceptance is
  `docs/execution/evidence/W20-NATIVE-EXECUTABLE-ABI.acceptance.json`.
- No capture callback, audio pool, `rtrb` edge, realtime executor, or hot-path
  `Drop` changed; no new endurance run is required.
- Sidecar lifecycle, managed-language conformance, release, and Core freeze are
  explicitly outside this task.

## W20 cross-language conformance — 2026-08-12

- Status: `LOOPBACK-ONLY` and hash-accepted for
  `W20-CROSS-LANGUAGE-CONFORMANCE`; candidate
  `pks-20260808-w20-language-conformance-4` is not a product, remote,
  physical-device, release, or Core 1.0 claim.
- Rust `Stream<T>`, executable C source/operator/endpoint callbacks, Python,
  JavaScript, and a Python PKSS sidecar execute the canonical public Session
  implementation from the same installed `pocketstation-0.1.2.crate` source
  artifact.
- Every embedded language matches one neutral success and injected
  operator-failure vector: stable `SignalSpec`/schema/role identity, source,
  operator and endpoint lifecycle counts, bounded queue/route capacities,
  maximum buffered bytes, terminal observations, and stop outcome.
- The Session-owned sidecar additionally proves stable signal/schema identity,
  success echo, crash isolation, and deterministic child reaping through the
  bounded PKSS protocol.
- Rust `T` remains façade-local. `TypeId`, `PhantomData`, and `Stream<T>` do
  not cross the C header, language-neutral vector, or PKSS protocol. The C ABI
  remains C++-parsable, fixed-width, explicitly sized/versioned, and based on
  opaque Session handles, which is the frozen contract for later Swift/Kotlin
  projections without implementing another engine now.
- The exact three acceptance commands and independent verifier pass. The
  acceptance manifest is
  `docs/execution/evidence/W20-CROSS-LANGUAGE-CONFORMANCE.acceptance.json` in
  the workspace authority.
- No capture callback, audio pool, `rtrb` edge, realtime executor, codec, or
  timing owner changed; no long soak was run or required.

## W20 clean installed consumer — 2026-08-12

- Status: `LOOPBACK-ONLY` and hash-accepted for
  `W20-CLEAN-INSTALLED-CONSUMER`; no product, remote, physical-device,
  release, or Core 1.0 claim is inferred.
- The proof creates and commits a separate source repository, resolves Cargo
  only to the extracted `pocketstation-0.1.2.crate`, installs a built Python
  wheel and JavaScript native module, links the installed C header/library,
  and drives the installed bounded PKSS sidecar protocol.
- The clean repository supplies typed and PCM sources, three chained
  operators, a named multi-input/output operator, typed and audio endpoints,
  generated-audio reentry, executable C callbacks, Python, JavaScript, and a
  Python sidecar without a workspace reference, relative dependency, source
  patch, `internal-testing`, or a core edit.
- Generated audio delivers four PCM frames with queue peak 4/8, exact 61,440
  byte queue-plus-pool bound, zero unexplained loss, and joined Session-owned
  lifecycle. The committed external Git tree remains byte-clean after all
  consumers execute.
- The exact three acceptance commands and independent hash verifier pass. The
  acceptance manifest is
  `docs/execution/evidence/W20-CLEAN-INSTALLED-CONSUMER.acceptance.json` in the
  workspace authority.
- No capture callback, pool, realtime queue/executor, codec, timing, or hot
  path changed; no endurance run was performed.

## W20 final W14-W17 requalification — 2026-08-12

- Status: `LOOPBACK-ONLY` and hash-accepted for
  `W20-FINAL-REQUALIFICATION`; candidate
  `pks-20260812-w20-final-requalification-9` is the accepted installed-package
  regression candidate, not a physical, remote, release, or Core 1.0 claim.
- The extracted package passed 443 all-feature unit tests, public Session and
  SessionTrace tests, signal/core-extension tests, C and C++ ABI conformance,
  all-target/all-feature compilation, and the public quickstart build.
- The packaged native surface contains the real process-tap, authorization,
  ASP reader and explicitly opt-in direct ASP driver sources. The obsolete
  native stub and broken platform example are absent from the package.
- The external Whisper provider and its consumer compile and test against the
  extracted package without `internal-testing` or private runtime imports.
- The public Session real-Whisper cell delivered 60 raw frames on an
  independent audio branch and one typed transcript through two successful
  real provider invocations. Bounded saturation accounted for two input drops;
  source close, operator process, lineage, endpoint, runtime and finalization
  outcomes remained clean and Session stop succeeded.
- Immutable W15, W16 and W17 artifacts were independently reverified without
  rerunning W10 endurance or changing any historical evidence classification.
  Acceptance is
  `docs/execution/evidence/W20-FINAL-REQUALIFICATION.acceptance.json` in the
  workspace execution authority.

## W20 Core 1.0 release preparation — 2026-08-13

- Status: `SAFE-TO-TEST` for release candidate
  `pks-20260813-w20-release-1-0-0-17`; publication and independent registry
  consumption remain acceptance predicates and are not claimed here.
- The single package version is `1.0.0`. Release notes state the exact
  extension-complete architecture and preserve the accepted physical,
  loopback, platform, and competitive evidence boundaries.
- The package includes the native sources and public C header. The publish
  gate no longer permits `--allow-dirty` or `--no-verify`; publication must
  originate from one clean intentional release commit.
- No Session behavior, public Rust API, C ABI, PKSS protocol, callback, pool,
  queue, codec, timing, or runtime execution path changed in this slice.

## Core 1.0.1 documentation and packaging correction — 2026-08-13

- Status: `SAFE-TO-TEST`. Version `1.0.0` was published, but docs.rs selected
  the configured Windows target on its Linux builder and failed while building
  bundled Opus with an incompatible cross toolchain. The Core freeze remains
  gated until the corrective patch is published, documented, and independently
  consumed.
- Version `1.0.1` selects docs.rs' native Linux target and builds the public
  contracts with `default-features = false`. Native capture remains the default
  product feature on macOS, Windows, and Linux; disabling default features now
  provides an explicit contracts-only build for rustdoc and tooling.
- The public README is now the crate-level docs.rs landing page and leads with
  developer outcomes, a compiling Session example, the two-lane execution
  model, realtime guarantees, extension contracts, platform evidence
  boundaries, prerequisites, and direct documentation paths. The linked public
  guides are included in the crate archive.
- The single-package publish guard permits exactly those six curated public
  documentation files and rejects every other `docs/` path, preserving the
  original ban on execution evidence, ADRs, internal standards, and historical
  reports in the registry artifact.
- Contracts-only Clippy and rustdoc pass with warnings denied; all-feature
  Clippy also passes. No callback algorithm, pool capacity, queue policy,
  `AudioFrame`, codec, timing, C ABI, PKSS frame, or Session execution semantic
  changed.
- The compatibility tools now accept an explicitly hash-pinned registry
  archive and discover its versioned package root, allowing the patch release
  to be checked directly against immutable `1.0.0` bytes instead of only the
  pre-release `0.1.2` baseline.
- Public permission documentation now distinguishes an unavailable preflight
  observation from actual capture authorization: macOS provides the current
  authoritative query, while every backend reports the authoritative selected
  source result during Session prepare/open. `NotObservable` is explicitly
  neither success nor denial.

## Windows microphone permission preflight — 2026-08-13

- Status: `SAFE-TO-TEST` for patch `1.0.2`. Windows 10 version 1903 and newer
  now uses the non-prompting `AppCapability("Microphone").CheckAccess()`
  authority. Allowed, user-denied, system-restricted/not-declared, and
  prompt-required states map to the existing stable permission vocabulary;
  API or platform failure remains honestly `NotObservable`.
- The query initializes WinRT only on the calling control thread, accepts an
  already initialized apartment, balances initialization ownership, never
  requests access, and never runs on a capture callback.
- Linux intentionally remains `NotObservable` at preflight because XDG portal,
  PipeWire/WirePlumber, direct ALSA, ACL, sandbox, and container policy do not
  expose one stable process-wide microphone permission authority. Selected
  source prepare/open outcomes remain authoritative on every platform.

## Linux capture module ownership correction — 2026-08-13

- Status: `SAFE-TO-MERGE`. The Linux platform module now names its concrete
  PipeWire/ALSA implementation `pipewire` instead of nesting a second `linux`
  module inside `capture::platform::linux`.
- The callback source-contract test follows the renamed implementation and
  continues to audit the same realtime callback source.
- Strict workspace Clippy passes with all targets, all features, the locked
  dependency graph, and warnings denied. No capture behavior, callback code,
  pool capacity, queue policy, permission semantics, or public API changed.

## Registry documentation gate alignment — 2026-08-13

- Status: `SAFE-TO-MERGE`. CI and release validation now execute the exact
  documentation configuration declared for docs.rs: native Linux,
  `default-features = false`, and no dependency documentation.
- The stale Ubuntu-to-Windows MSVC documentation command was invalid because
  the non-Rust Opus build requires a native Windows toolchain. Windows runtime
  qualification remains an independent native-platform gate and is not
  reclassified by this documentation correction.
- No Rust source, public API, ABI, capture path, codec behavior, or realtime
  execution contract changed.

## W20 public documentation narrative correction — 2026-08-14

- Status: `SAFE-TO-MERGE` for documentation candidate
  `pks-20260814-w20-public-docs-20`; Core 1.0 remains frozen.
- The public README now leads with the implemented developer workflow: capture
  one desktop application and microphone once, preserve independent
  source-aware stems, and fan them out concurrently to Operators, application
  callbacks, remote delivery, and aligned multistem recording.
- Public documentation now explains the stronger architectural boundary as a
  provenance-preserving bounded execution contract across realtime audio,
  typed signals, Rust, C, and process sidecars. Established realtime
  primitives remain engineering guarantees rather than novelty claims.
- The compatibility guide no longer assigns PocketStation the label
  "innovative." It requires concrete workflow, contract, and evidence language
  and preserves the separate burden for novelty or superiority claims.
- Contracts-only rustdoc with warnings denied, the release public quickstart,
  the complete `CODE_PROTOCOL` gate, and the Core freeze-policy gate pass. No
  Rust source, API, ABI, PKSS protocol, callback, pool, queue, runtime behavior,
  scaffold, mock, fallback, provider, or evidence classification changed.

## PocketStation 1.0.3 documentation patch release — 2026-08-14

- Status: `BLOCKED` for release candidate
  `pks-20260814-w20-docs-release-21`. The immutable source tag and GitHub
  Release exist, but publication stopped before crates.io because the clean
  runner's `CODE_PROTOCOL` gate unconditionally required a sibling `pks`
  checkout. Workflow `31850004592` preserves the failed gate.
- The package version and public install snippets advance from `1.0.2` to
  `1.0.3`, and the accepted W20 public narrative is included in the registry
  archive and crate-level docs.
- Release notes now live in one canonical 1.x-focused `RELEASE_NOTES.md` with a
  concise compatible-line contract and release history. Immutable tags and
  GitHub Releases preserve exact earlier bytes. The numbered top-level
  patch-note files are removed from the current source and package archive.
- This patch changes only documentation and release metadata. It does not
  change dependencies, feature defaults, Rust API, C ABI, PKSS protocol,
  capture, callback, pool, queue, codec, timing, graph, Session runtime, or
  evidence classifications.

## PocketStation 1.0.4 documentation patch release — 2026-08-14

- Status: `BLOCKED` for replacement candidate
  `pks-20260814-w20-docs-release-22`. The clean workflow passed the corrected
  protocol and package dry-run, then stopped before crates.io because it still
  referenced a recovery-test script removed during single-package
  consolidation. Workflow `31851000841` preserves the failed gate.
- The deterministic failure is reduced to the standalone Core checkout. The
  cross-repository `pks` ownership assertion now reports `NOT OBSERVABLE` when
  its sibling checkout is absent and remains fully enforced when the sibling
  is present. This changes release qualification only, not product runtime.
- The package version and public install snippets advance to `1.0.4`. The
  canonical 1.x `RELEASE_NOTES.md` records both the failed 1.0.3 publication
  attempt and this correction without recreating numbered patch-note files.

## PocketStation 1.0.5 documentation patch release — 2026-08-14

- Status: `SAFE-TO-TEST` for replacement candidate
  `pks-20260814-w20-docs-release-23`; publication and external consumption are
  not claimed until the immutable registry, docs.rs, and clean-consumer gates
  pass.
- `scripts/publish.sh` now checks the exact package version on crates.io before
  an actual upload. A visible version completes idempotently, a missing version
  publishes once, and an unavailable or unexpected registry response fails
  closed without attempting publication.
- `scripts/test-publish-recovery.sh` deterministically covers all four cases
  for the consolidated single package. No product runtime or frozen contract
  changes.

## Canonical 1.0.0 registry reset — 2026-08-16

- Status: `SAFE-TO-MERGE`. The owner deleted the complete `pocketstation`
  package from crates.io; the registry subsequently returned `404` for the
  package name. The source package, lockfile, and public install snippets are
  reset to the canonical `1.0.0` version.
- `RELEASE_NOTES.md` now describes the compatible 1.x product and contract
  globally instead of presenting documentation-only publication attempts as
  product releases. Historical evidence above remains unchanged.
- `cargo package --allow-dirty --locked` verified the generated
  `pocketstation-1.0.0.crate`, and
  `cargo build --release --example quickstart --locked` passed.
- The immediate crates.io upload was rejected by the registry's deleted-name
  reuse hold. No replacement package was published by this step. No runtime,
  Rust API, C ABI, PKSS, capture, hot-path, scaffold, mock, or evidence
  classification changed.

## W21 SDK-neutral native extension library host — 2026-08-16

- Status: `SAFE-TO-MERGE` for candidate
  `pks-20260816-w21-core-native-extension-host-bridge-1`. The focused dynamic
  library tests, full all-feature/all-target suite, strict Clippy, formatting,
  no-default-features check, release quickstart, C ABI compatibility gate,
  CODE_PROTOCOL gate, and executor document/state validators pass.
- The prior Python blocker was narrowed after a complete engine audit: Core
  already executes C source/operator/endpoint callbacks through the canonical
  Session. The missing primitive was a versioned packaged-library entrypoint,
  absolute-path loader, atomic import into the public Rust `Session`, and a
  retained executable-code lifetime.
- Extension ABI 1.2 adds `pks_extension_library_v1` without introducing a new
  descriptor, callback model, registry, compiler, runtime, graph, or lifecycle
  owner. Extension ABI 1.1 layouts, offsets, and Core symbols remain the
  required compatibility baseline subset.
- A separately compiled dynamic-library fixture passes six focused tests:
  canonical source → operator → endpoint execution, exactly-once instance and
  registration destruction, relative-path rejection, missing entrypoint,
  unsupported ABI, malformed acquired-registration cleanup, and transactional
  duplicate import.
- Foreign callbacks remain on blocking/async/external partitions. No callback,
  allocation, dynamic loading, locking, or managed execution was added to the
  capture or realtime PCM path. The plugin is a conformance fixture only; this
  step does not claim remote, physical-device, language-SDK parity, or release
  readiness.

## W21 provider-neutral connector authoring contract — 2026-08-17

- Status: `PARTIAL` historical predecessor, superseded by the connector runtime
  authoring correction at the top of this file. It was `SAFE-TO-MERGE` for candidate
  `pks-20260817-w21-connector-authoring-contract-1`; every mandatory executor
  predicate is bound to the task acceptance manifest.
- `pocketstation::connector` now provides an inspectable manifest, typed and
  finite configuration, redacted secrets, bounded delivery/retry/readiness
  policies, explicit readiness transitions, stable classified errors,
  observations, Session-scoped registration, and conformance fixtures.
- The new registration path lowers into the existing `NodeDefinition` and
  `EndpointDriverFactory` authorities. It adds no registry, compiler, route
  table, scheduler, worker lifecycle, provider type, or execution engine.
- Ten connector-contract tests pass through the public Session path, including
  configuration rejection, secret redaction, duplicate identity, preparation
  rollback, start failure, saturation accounting, cancellation, join failure,
  and worker-panic containment. The existing public Session fan-out test also
  passes after migrating its observed connector to the new contract.
- Full acceptance passes: 444 library tests plus every workspace target and
  benchmark target, strict all-target/all-feature Clippy, rustdoc warnings as
  errors, release quickstart, architecture constraints, `CODE_PROTOCOL`, Core
  freeze policy, and both executor validators.
- Concrete protocols remain outside Core. Rust packages implement the factory;
  managed SDKs consume packaged native connectors or supported bounded
  extension/sidecar boundaries. The current native extension ABI remains
  typed-signal-only, so this step does not claim arbitrary pure-Python or
  pure-JavaScript PCM connector authoring.

## W21 connector grouping public-surface correction — 2026-08-17

- The first independent Relay connector implementation exposed a public API
  omission: `EndpointDriverFactory::preparation_group` returned grouping types
  that the crate did not re-export for external implementations.
- Core now re-exports the existing `EndpointPreparationGroup` and
  `EndpointGroupId` without changing grouping, preparation, or runtime
  semantics. An external-style connector test names and returns a shared group.
- This correction adds no provider dependency, protocol, worker, queue,
  registry, or execution path. Relay implementation remains outside Core.

## W21 portable connector semantics consumer — 2026-08-17

- Core independently consumes the canonical protocol connector vectors and
  verifies manifest identity, typed configuration, secret-default rejection,
  package composition, port identity, finite Endpoint admission/deadline
  policy, and orthogonal provider status.
- The corpus limits are checked against Core's public connector limits so the
  portable contract cannot silently drift from the Rust authority.
- The focused all-feature test and strict Clippy gate pass. This is a `MOCKED`
  conformance corpus, not product-path or provider evidence.
- No Core runtime, package version, provider dependency, tag, or release state
  changed.

## W21 bounded connector transport boundary — 2026-08-19

- Status: `SAFE-TO-MERGE`. Core now exposes a finite, versioned PCM connector
  record carrying endpoint, connector, route, Session, source, stream, stem,
  clock, sequence, timestamp, duration, discontinuity, permission, media, and
  port identity. The existing Connector worker creates it only off realtime.
- `Connector::sidecar` adapts that record and canonical typed configuration to
  the existing bounded PKSS process host. Endpoint remains lifecycle authority;
  Source remains inbound authority; Connector remains outbound-only. No queue,
  retry engine, provider protocol, or second runtime was added.
- The real two-stem Session conformance test encodes and decodes every delivered
  frame and verifies exact route and lineage identity. Configuration tests cover
  every value kind, secret redaction, bounds, unknown kinds, and trailing data.
- All 461 library tests and all targets/features pass, as do strict Clippy,
  rustdoc, release quickstart, architecture checks, CODE_PROTOCOL, C/PKSS ABI,
  package assembly, and the exact `HEAD` semver comparison. The historical
  registry-baseline semver command still reports pre-existing public enum
  changes unrelated to this task.
- No package version, tag, push, publication, provider implementation, remote
  metadata protocol, SDK API, scaffold, mock, or new product claim changed.

## W21 Core audio input and native trust correction — 2026-08-19

- Application-owned PCM now enters the canonical Source lifecycle through the
  intent-first `AudioInput` façade or the explicit `PcmSource` ownership path.
  Both use one preallocated bounded implementation with typed full, closed,
  invalid-buffer, discontinuity, cancellation, and recovery observations.
- `SourceTypeId` now enforces bounded ASCII reverse-domain source-contract
  syntax ending in a non-zero `vN`. Manifest revision, implementation
  generation, and runtime source-attachment generation are distinct.
- Native extension loading is an explicit unsafe raw-library trust boundary.
  Core authenticates neither publisher nor signature and admits only the
  supported non-realtime Extension ABI registrations.
- The owner explicitly accepted the cleaned connector/native Rust API reset on
  `1.1.x` without restoring `ManagedConnector` compatibility aliases. The API
  gate binds the exact approved break set to `pocketstation-v1.1.0`, reports
  `INTENTIONAL_BREAK_ACCEPTED`, and fails on any additional incompatibility.
- Pre-commit qualification passes: 464 Core unit tests plus all targets and
  benches, AudioInput fan-out/reentry/multistem recording, native dynamic
  extension execution, C/C++ ABI, hot-path allocation checks, strict Clippy,
  rustdoc, architecture, CODE_PROTOCOL, quickstart, package verification, and
  the Python native/extension projection checks. No scaffold, mock, provider,
  second runtime, version, tag, push, or publication was introduced.
- Exact-candidate packaging excludes the internal connector ADR while retaining
  the public connector and extension guides; the single-package allowlist
  enforces that boundary.

## W21 Core 1.1.1 legal release candidate — 2026-08-19

- The owner authorized the exact `1.1.1` version edit, main-branch push,
  immutable `pocketstation-v1.1.1` tag, and crates.io publication.
- Candidate source is the accepted Core freeze commit plus only the Cargo
  package and lockfile version change. Existing tags remain immutable.

## W21 physical microphone timeline correction — 2026-08-20

- macOS input timestamps now anchor once to CoreAudio's first capture instant
  and advance by represented sample frames, excluding callback scheduling
  jitter while preserving capture age and observable gaps.
- The physical application + microphone → Relay/browser + multistem recording
  path completed with 315 application frames and 169 microphone frames, zero
  drops, zero discontinuities, zero future timestamps, and two complete stems.
- All Core tests/features, strict Clippy, rustdoc, release quickstart,
  architecture checks, and CODE_PROTOCOL pass. No version, tag, push, or
  publication changed.

## W21 Python prerequisite contracts — 2026-08-20

- Core now exposes authoritative running lifecycle state, a finite blocking
  receipt over the existing bounded polled-audio queues, and the trace-record
  and component-identity types required by stable SDK observations.
- `Source::system_mix()` is a real built-in Session source. It lowers through
  the canonical graph and requests the platform `SystemMix` capture mode; an
  unsupported backend fails through the existing typed capture error path.
- Qualification passes: 471 Core unit tests plus every target and feature,
  normal-build public API checks, strict Clippy, rustdoc, product quickstart,
  built-in lowering boundary, and CODE_PROTOCOL. No queue, runtime, callback
  path, scaffold, version, tag, push, or publication was added.
- Cross-language qualification found and fixed a SystemMix media mismatch:
  the built-in ingress now declares the stereo contract produced by native
  desktop capture. A normal public conformance test proves bounded audio
  reaches the canonical receipt. Authorization policy is now nameable from
  the public SDK boundary without exposing capture implementation modules.

## W21 structured compiler diagnostics — 2026-08-20

- Session compilation failures now retain stable typed diagnostic codes and
  finite node, edge, component, port, direction, and expected/actual facts.
- The stable `SessionStartError` owns the diagnostic only on compile failure;
  bindings no longer need to parse diagnostic messages.
- Full Core tests/benches, strict Clippy, rustdoc, and CODE_PROTOCOL pass. No
  runtime, hot path, version, tag, push, publication, scaffold, or mock changed.

## W21 SDK observation boundaries — 2026-08-20

- Endpoint receipts retain exact route enqueue/receive timestamps; polled SDK
  delivery additionally retains endpoint enqueue and poll timestamps without
  adding a queue or touching capture callbacks.
- Recording outcomes now carry the canonical Session and endpoint-group
  identity plus the finalized manifest path and schema revision.
- Clock-domain descriptors distinguish unspecified, process-monotonic, and
  provider-defined authority without inventing provider epochs.
- All Core targets and benches, strict Clippy, rustdoc, hot-path tests, Python
  native tests, and the complete Python suite pass. No version, tag, push,
  publication, scaffold, mock, or new product claim was introduced.

## W21 multi-source Operator binding correction — 2026-08-21

- Session declaration now leaves repeated named-input acceptance to the
  compiled Operator manifest: `Multiplicity::Many` accepts finite fan-in while
  `Multiplicity::One` still fails before runtime.
- Runtime preparation resolves every compiled worker input by both target port
  and exact source origin, so two application-owned PCM Sources cannot alias
  the first connection's lineage when sharing one Operator input.
- A real Session test proves two independent `AudioInput` Sources reach one
  multi-input Operator with distinct source identities. The complete
  all-target/all-feature suite (474 Core unit tests plus integration, ABI,
  hot-path and benchmark targets), strict Clippy, warnings-as-errors rustdoc,
  release quickstart, architecture constraints, and CODE_PROTOCOL pass.
- This is a Core correctness prerequisite for the Python transcription proof;
  it adds no provider code, queue, runtime, version, tag, push, or publication.

## W21 Windows native-capture build correction — 2026-08-24

- The Windows system-loopback implementation now reads runtime-event state
  through the public observation handle. The previous call reached a
  test-only helper and caused the published `1.1.2` source to fail during a
  Windows release build.
- The exact one-line correction compiled in a native Windows 11 ARM64 VM with
  Rust 1.95 and produced an installable Python ARM64 wheel. A second clean
  Python 3.13 environment completed the installed consumer and 500-cycle
  sync/async resource gate with no route loss or thread growth. This evidence
  is `SAFE-TO-TEST` because the immutable corrected Core crate has not been
  published.
- Core CI and publication validation now compile the default release library
  on macOS and Windows before a release can publish. This closes the coverage
  gap that allowed a platform-gated release defect to pass Linux validation.
- Local acceptance passes: formatting, the complete all-feature workspace
  suite, strict all-target/all-feature Clippy, warnings-as-errors rustdoc,
  release library compilation, release `quickstart`, architecture
  constraints, and `CODE_PROTOCOL`.
- No runtime, queue, public API, scaffold, mock, version, tag, push, or
  publication was introduced. A new immutable patch release requires separate
  owner authorization; the existing `pocketstation-v1.1.2` tag remains
  unchanged.
## Public source-language cleanup — 2026-08-25

- Removed private milestone, ADR, wave, and engineering-law identifiers from
  public source comments, workflows, compatibility metadata, and test prose.
- Technical requirements and behavior remain explicit without links to local
  planning records.
- Formatting, shell syntax checks, and 471 library tests pass.

## Public quickstart and documentation cleanup — 2026-08-26

- Renamed the Rust example to `quickstart` and removed the fictional
  `PocketStation Demo` application requirement.
- The command now asks the user to choose a running application. Microphone
  capture and recording are disabled unless `--microphone` or `--record` is
  provided.
- Updated the public README, quickstart, architecture, compatibility,
  Connector, and extension pages to use the same behavior and direct language.
- The portable Connector test now reads the byte-identical packaged fixture,
  so a clean repository does not require an undeclared sibling checkout.
- Rust formatting, the portable Connector test, release quickstart build/help,
  and public Markdown target checks pass. No mock or loopback path was added.
- Updated four slice/collection operations for strict Rust 1.98 Clippy without
  changing their behavior. Strict all-target/all-feature Clippy passes on the
  local Rust 1.95 minimum toolchain; GitHub CI provides the Rust 1.98 check.

## Recording shutdown intent — 2026-08-27

- The multistem recording Endpoint now preserves the Session's explicit
  `Drain` or `Abort` request. A drain accepts frames already admitted to each
  recording route before finalizing, while an abort stops without weakening a
  previous abort request.
- A real Session regression test writes one frame and immediately drains the
  Session. The finalized WAV and recording outcome retain that accepted frame.
- Core formatting, the output-cancellation test, every target, strict
  all-target/all-feature Clippy, and warnings-as-errors rustdoc pass. This is a
  lifecycle correctness change; it adds no provider, queue, scaffold, mock,
  version, tag, push, or publication.

## macOS application display-name selection — 2026-08-28

- macOS application capture now accepts an exact, case-insensitive display
  name or bundle identifier, matching the public selector behavior already
  provided on Linux and Windows.
- Selection preserves the discovered stable application identity and captures
  every discovered process for that identity. A display name shared by
  different application identities fails before capture begins.
- Focused Core tests cover display names, bundle identifiers, multiple
  processes, ambiguous names, and missing process identity.
- An isolated Python wheel built against this Core completed the physical
  application + microphone → OpenAI Realtime + Relay/browser + three-stem
  recording path using `Brave Browser`, not its bundle identifier. The artifact
  is classified `REAL-DEVICE-PROVEN`; acoustic hearing, echo cancellation,
  provider-history truncation, and WAN/TURN remain unqualified.

## Core 1.1.3 public release candidate — 2026-08-28

- Rewrote the release notes for developers using PocketStation: the page now
  explains selective generated-audio cancellation, its external-playout
  boundary, macOS application selection, compatibility, and the upgrade
  command without exposing private release governance.
- Removed the public freeze-policy checker and its CODE_PROTOCOL/CI hooks. The
  workspace execution system remains the authority for private release
  authorization and evidence.
- The macOS and Windows release-library jobs now install a pinned CMake 3.31
  and use Ninja before compiling bundled libopus. This avoids both CMake 4's
  removed compatibility mode and generator-name coupling to the runner's
  Visual Studio release.
- `CODE_PROTOCOL`, hot-path allocation tests, formatting, and strict Clippy
  pass. The cross-platform GitHub jobs and package dry run must pass again
  before merge, tag, or publication.

## Core 1.1.3 installed quickstart correction — 2026-08-28

- Updated every current public install declaration from 1.1.2 to the published
  PocketStation 1.1.3 crate.
- The README and Rust quickstart now lead with a three-line application capture
  declaration using the same public API on macOS, Windows, and Linux. The text
  separates the shared API from platform-specific permissions and build
  prerequisites.
- The three-line path selects one application, routes it to the bounded polling
  Endpoint, and starts the Session. The complete repository example remains the
  authority for prompting, finite observation, optional microphone/recording,
  and joined shutdown.

## Canonical process-selector identity — 2026-08-28

- A real macOS seven-selector capture attempt found that a process-ID selection
  delivered Spotify audio under a PID-derived `SourceId`, while the other six
  selector forms retained Spotify's discovered application identity.
- macOS process selection now resolves the live process before opening the tap,
  retains its discovered stable identity, and verifies the process start time
  after the tap starts. Windows process selection now resolves the discovered
  process-instance identity before WASAPI activation.
- This is control-plane work performed before capture starts. It does not add
  allocation, locking, blocking, logging, or panic behavior to an audio
  callback or realtime partition.
- All 489 library tests, every target and feature, native ABI consumers,
  hot-path allocation gates, strict Clippy, warnings-as-errors rustdoc, and the
  release quickstart build pass. The complete macOS and Windows real capture
  matrices remain the acceptance evidence for this change.

## PipeWire capture-buffer timestamps — 2026-08-31

- The Linux selector matrix found that a PipeWire callback can contain more
  than one PocketStation output frame. Anchoring that callback at callback
  arrival made later normalized frames appear to have future source
  timestamps.
- Linux now anchors the first sample before callback arrival using the native
  buffer duration and PipeWire's non-negative capture-path delay. PipeWire
  graph ticks remain the cadence and discontinuity authority after that first
  anchor.
- The correction remains inside the allocation-free, lock-free, blocking-free,
  async-free, log-free, and panic-free capture callback. It does not clamp
  downstream latency measurements or hide invalid timestamps.
- Focused Linux tests pass. The exact seven-selector VM matrix remains the real
  acceptance gate before this item is classified as complete.

## PipeWire graph-time mapping — 2026-08-31

- The first 10 ms Ubuntu selector matrix exposed a scheduling-dependent clock
  error: the first buffer was anchored to callback arrival even when
  PipeWire's graph-time report was older.
- Linux now uses each PipeWire buffer's presentation timestamp when available.
  It maps that timestamp into PocketStation's process clock through the shared
  Linux monotonic domain. The graph-time report age remains the bounded
  fallback when a buffer does not contain header metadata.
- The correction reads one realtime-safe monotonic clock and performs bounded
  integer arithmetic. It does not allocate, lock, block, log, or clamp a
  downstream timestamp.
- Focused Linux unit tests pass. Repeated 10 ms Ubuntu VM runs captured the
  exact application through every selector with zero active delivery loss.
  PipeWire application buffers can carry presentation timestamps later than
  callback arrival, so source-presentation latency is not used as a selector
  correctness gate. Queue latency and future-presentation counts remain
  explicit; end-to-end latency qualification is a separate task.

## Independent multistem recording startup — 2026-08-30

- Each declared recording stem now starts when its own first authoritative
  frame arrives. A silent or late generated-audio stem no longer prevents
  active application and microphone stems from draining their bounded routes.
- The recording manifest begins with every declared stem and fills dynamic
  lineage fields after that stem starts. Its schema revision is now 2.
- A three-stem regression sends 20 application frames and 20 microphone frames
  before the generated-audio stem begins, then verifies 41 written frames and
  zero recording-edge drops.
- The installed physical application + microphone → OpenAI Realtime +
  Relay/browser + three-stem recording proof completed with zero active route
  drops, zero recording drops, and zero recording discontinuities. Receiver
  playout position and acoustic hearing remain explicitly unavailable.
- Core formatting, every target and feature, strict all-target/all-feature
  Clippy, warnings-as-errors rustdoc, and the release quickstart build pass.
## Explicit 10 ms voice capture profile — 2026-08-30

- Added a typed `AudioFrameDuration` Session setting with `Ms20` as the
  unchanged default and `Ms10` as an explicit voice profile.
- CoreAudio, PipeWire, ALSA, and WASAPI setup now size their bounded frame
  normalizers and pools from the selected duration. Native callback buffers
  remain variable-sized inputs; callbacks are normalized without allocating
  on the capture path.
- Session graph audio port contracts use the same selected frame size, so the
  profile is not limited to codec configuration.
- The owning macOS build and focused normalization tests pass. Linux and
  Windows compilation and real guest execution remain required before this is
  classified as cross-platform `REAL`.

## Cross-platform release lint — 2026-08-31

- GitHub's Linux strict-Clippy job found that the frame-count accessor used by
  the Core Audio pool remained compiled on platforms that do not consume it.
- The accessor is now compiled only for macOS. Frame normalization behavior,
  callback work, and the public Session contract are unchanged.
- The exact 1.1.4 candidate must pass Linux strict Clippy and the macOS and
  Windows release-library jobs before merge.

## Endpoint frame-size negotiation — 2026-08-31

- Endpoint ports that declare no fixed frame size now remain frame-size
  agnostic during Session compilation.
- The compiler no longer turns an unspecified endpoint frame size into a
  hidden 20 ms requirement. This allows an explicitly selected 10 ms capture
  profile to connect to Relay, recording, and other endpoints that accept
  either supported frame duration.
- A focused regression verifies that an endpoint accepting any frame size is
  compatible with exact 480-sample stereo frames at 48 kHz.

## Reproducible release automation — 2026-08-31

- Every third-party action used by Core CI and publication is pinned to an
  immutable commit. Package behavior and the 1.1.4 public API are unchanged.

## Public developer documentation — 2026-08-31

- The repository README now leads to an application-capture quickstart, task
  guides, concepts, operations, troubleshooting, compatibility, and API
  reference.
- New guides cover application and microphone routing, application-owned PCM,
  output cancellation boundaries, multistem recording, observations, and
  platform setup without exposing maintainer records.
- The curated Cargo package includes every public guide. Local Markdown links,
  Rust doctests, the release quickstart, and the package-content list pass.

## Repeated Windows permission observation — 2026-08-31

- Windows microphone permission observation now keeps library-owned WinRT
  initialization alive until the calling thread exits.
- Repeated permission queries no longer tear down an apartment while the
  Windows projection retains cached activation state.
- Host-owned COM apartments remain host-owned. PocketStation balances only
  initialization that it performed, on the same thread, outside audio paths.
- A Windows regression queries the current microphone capability twice in one
  process. The exact Windows CI job remains the release acceptance gate.

## Connector operations guidance — 2026-09-01

- The Connector guide now starts from the outbound-delivery task and separates
  the concise function and type APIs from the advanced manifest and driver SPI.
- Platform guidance now covers non-prompting permission checks, selector
  persistence scopes, explicit rediscovery, ambiguity rejection, and
  application-owned fallback and retry policy.
- The documentation examples use the public Session, Source, discovery,
  permission, Connector, and Endpoint APIs. Rust doctests and the Connector
  example are the acceptance gates for this update.

## Extension guide entry path — 2026-09-02

- The README now explains integration work by direction: media entering the
  Session, computation inside it, or media leaving for another system.
- The normal Connector function appears before driver, manifest, ABI, and
  sidecar vocabulary. The extension guide explains when each lower-level
  deployment boundary is needed before naming its contract.
- Documentation review now rejects comparison tables that introduce undefined
  public concepts without a task, normal choice, or concrete example.

## Route settings API — 2026-09-02

- `RouteSettings` describes the media accepted by a route, while
  `DeliveryPolicy` describes buffering, loss, copying, timing, and observation.
- The same cleanup uses `ExecutionSafety`, `RouteObservability`, and
  `RouteLatencyMeasurement` throughout the public API.
- Session polling, Endpoint declarations, Connector declarations, Operator
  ports, and route observations use the same route-settings vocabulary without
  introducing another graph or runtime.
- The full Rust test suite, strict Clippy, warning-free Rustdoc, protocol check,
  and local Markdown-link check pass.

## Route settings cleanup — 2026-09-03

- Removed unused compatibility names and methods for route settings, execution
  safety, route observations, latency measurements, and Connector declaration.
- Session operator metrics now expose `input_delivery`; the former aggregate
  field is not retained as an alias.
- Core, Connector, Endpoint, Operator, examples, and tests now use one public
  vocabulary. Internal graph edges retain their graph meaning.
- This is an intentional pre-adoption API cleanup. It is not source compatible
  with the unreleased candidate that exposed the removed names.

## Capture-only native builds — 2026-09-03

- `native-capture` no longer compiles or links Opus. Capture-only applications
  can use CoreAudio, WASAPI, PipeWire, or ALSA without carrying Relay's codec
  library.
- The default feature set continues to include native capture and the existing
  Rust and C Opus APIs. Applications that disable defaults can request
  `opus-codec` explicitly.
- Codec tests and benchmarks now state their feature requirement. CI verifies
  the capture-only dependency set and links a debug native library on macOS
  and Windows.
- The local capture-only library, all 466 library tests, the complete default
  test suite, strict Clippy, and no-default documentation build pass.
- A Windows 11 ARM64 VM built the exact Core commit with only
  `native-capture`, verified that both the Core and Minutes dependency trees
  exclude `opus` and `audiopus_sys`, and linked the real Minutes debug
  executable with its `pocketstation-capture` feature. The result is
  correctness evidence, not Windows latency evidence. Lab artifact:
  `artifacts/capture-only-build/windows-arm64/20260904T011357Z`.
- The release candidate is the next permitted patch, `1.1.8`. Publication
  remains gated on the exact release checks and protected-main CI.
- The curated crate now includes both Connector examples declared by Cargo, so
  installed-source users can run the concise and advanced examples without a
  packaging warning or a missing file.
- The Rust API compatibility gate now compares this patch with the latest
  published Core release, `pocketstation-v1.1.7`, instead of the superseded
  1.1.3 baseline. All 196 compatibility checks pass. The C ABI and PKSS
  compatibility gate also passes against the accepted ABI baseline.
- Protected-main and release validation passed on Linux, macOS, and Windows at
  commit `965a0763d884e08b76fe99070543680006ed68a8`. The immutable
  `pocketstation-v1.1.8` tag and GitHub release point to that commit.
- crates.io published `pocketstation 1.1.8` with checksum
  `419bd4c5dac0735627be925273e2c92d4ab4c8d75982c248707797ff01eaafa5`.
  A clean registry consumer compiled `native-capture` with no Opus dependency.
- No scaffold, mock, or loopback-only implementation was introduced.

## Public system-audio Source — 2026-09-04

- `Source::system_audio()` now declares the complete desktop output mix through
  the same Session API used for selected applications and microphones.
- The declaration lowers to the existing Core Audio, WASAPI, and PipeWire
  system-mix implementations. It does not add another capture engine or run new
  work on an audio callback.
- System audio has its own graph ingress, source identity, stem identity,
  routes, recording outcome, and Session observations.
- The conformance Session test exercises the complete declaration, compile,
  capture, route, frame, and stop sequence with a distinct system-audio source.
- A physical macOS run recorded 467 stereo frames over 4.98 seconds with no
  stale frames or discontinuity ranges. The WAV contains audible media with a
  measured maximum level of -14.1 dB. Artifact:
  `/tmp/pks-system-audio-macos.qZF5oU/session-1`.
- Windows and Linux guest execution remain required before the feature can be
  released as cross-platform `REAL`.
- The first Linux Session run returned silent frames because the PipeWire
  capture stream requested sink-monitor ports without naming the default output
  target. The candidate now uses WirePlumber's stable default-output alias;
  the repeated audible 10 ms and 20 ms guest runs are the acceptance gate.
- No scaffold, provider mock, or second media runtime was introduced.

## Windows debug linking and ALSA coexistence — 2026-09-04

- A registry consumer exposed a Windows ARM64 debug-link failure in the old
  Opus binding. Core now uses `opus 0.4`, backed by the current bundled
  libopus binding, while keeping the public codec API unchanged.
- The normal `native-capture` feature still includes Core Audio, WASAPI,
  PipeWire, and the ALSA fallback. Applications that already own part of the
  audio stack can select `coreaudio-capture`, `wasapi-capture`, or
  `pipewire-capture` without adding unrelated native libraries.
- The complete all-feature test suite passes with 502 library tests. The
  PipeWire-only suite passes with 468 library tests, and its dependency tree
  contains neither ALSA nor Opus.
- Strict Clippy, formatting, CODE_PROTOCOL, Rust API compatibility against
  `pocketstation-v1.1.9`, C ABI compatibility, and PKSS compatibility pass.
- The exact `57b3d2237b40debe385b4ed0c029e078e9811955` candidate passes both
  the default and WASAPI-only debug link builds plus strict WASAPI-only Clippy
  in a Windows 11 ARM64 UTM guest. This is component-correctness evidence; the
  VM did not exercise a physical Windows audio device.
- The same candidate passes the default and PipeWire-only debug builds plus
  strict PipeWire-only Clippy in an Ubuntu 24.04 ARM64 guest. A real PipeWire
  Session captured 4.94 seconds from a controlled virtual output with no
  recorded gaps and RMS amplitude `0.140811`. This is `LOOPBACK-ONLY`
  correctness evidence, not physical-device or latency evidence. The WAV
  SHA-256 is `4c81e6c9454735775f49995329773eb299dfc2e7baee58452590b3b9cd51ca4d`.
- A physical macOS Session on the same candidate captured 5.024 seconds of
  audible 48 kHz stereo system audio, recorded no gaps, and finalized cleanly.
  The WAV SHA-256 is
  `8a036cb9da57152e35027750f8458c4e90f2944028446f7d7ac403d72cc516f2`.
- No capture implementation, callback behavior, scaffold, mock, or loopback-
  only product claim was added.

## macOS selected-application exit reporting — 2026-09-10

- A Core Audio process tap can remain open after its selected process exits.
  The Session previously received no source event in that case.
- Application capture now retains each selected process creation time and
  checks those process instances every 100 ms on the tap reader thread. PID
  reuse cannot redirect an existing Session to another process.
- When every process selected by the tap has exited, the Session reports
  `source-instance-exited` and requires explicit discovery followed by a new
  Session. System-audio capture is unchanged.
- A physical macOS JavaScript run captured a controlled application and the
  built-in microphone, terminated the application, observed the source event,
  finalized both recordings, and captured a replacement application through a
  new Session. Diagnostic artifact:
  `pocketstation-lab/artifacts/w21-javascript-product-proof/run-20260910T-diagnostic-2`.
- All 504 library tests, integration and documentation tests, strict Clippy,
  the quickstart compile gate, and the complete CODE_PROTOCOL check pass.
- The Core Audio callback remains unchanged. No scaffold, provider mock,
  automatic source switch, or loopback-only product claim was introduced.

## macOS voice capture profile — 2026-09-10

- Status: `REAL-DEVICE-PROVEN` for complete 10 ms and 20 ms application and
  microphone frames on the recorded Mac. This is not a sub-20-ms
  selected-application latency result.
- The published `1.1.10` crate predates the process-tap frame normalizer. A
  local physical run at commit `720c050` confirms that the correction emits
  only complete 480-sample frames at 10 ms and 960-sample frames at 20 ms for
  both a selected application and the built-in microphone.
- CoreAudio reported a supported I/O range of 15–4,096 frames. The 10 ms
  profile reduced the aggregate-device setting from 512 to the requested 480
  frames. The 20 ms profile retained the smaller 512-frame setting instead of
  increasing it to 960.
- The accepted five-second runs delivered 500 application and 499 microphone
  frames at 10 ms, then 250 application and 249 microphone frames at 20 ms.
  Both profiles completed with zero delivery loss, partial frames, sequence
  gaps, future timestamps, or shutdown failures.
- Core route-enqueue-to-receive time was 2.097 ms at p95. At 10 ms, the
  selected application's first-sample-to-route maximum was 51.062 ms and the
  microphone maximum was 16.415 ms. At 20 ms, those maxima were 56.483 ms and
  36.955 ms. The I/O adjustment therefore fixes callback cadence but does not
  support a sub-20-ms selected-application acquisition claim on this host.
- Physical evidence is stored at
  `docs/execution/evidence/W21-CORE-MACOS-VOICE-PROFILE/macos-physical` and is
  bound to Core commit `ce96e087004718696bc1c4f5daf46b1afcf480c3`.
- Source timestamps remain the time of the first captured sample. This work
  does not replace them with callback or consumer time and does not introduce
  another capture implementation, scaffold, mock, or release action.

## macOS process-tap acquisition localization — 2026-09-18

- The selected-application source-age delay is below JavaScript and Core route
  delivery. On the measured 48 kHz aggregate device, Core Audio reports zero
  device latency, zero stream latency, and a read-only 1,024-frame input safety
  offset. That offset alone is 21.333 ms.
- Controlled A/B runs found that disabling sub-tap drift compensation,
  requesting a 5 ms native callback, and declaring zero extra sub-tap input
  latency do not change the 1,024-frame safety offset. Those experimental
  changes were not retained.
- Both supported product profiles now request at most a 10 ms native callback.
  The 10 ms profile is unchanged; a 20 ms product frame is assembled from two
  exact 480-frame native batches instead of straddling the 512-frame default.
  The empty-ring reader poll is bounded at 500 microseconds instead of one
  millisecond; this remains off the realtime callback.
- `PKS_TAP_DIAG=1` now reports device, safety-offset, writability, and stream
  latency properties from the control thread. Source timestamps still use
  Core Audio's first-sample `inInputTime`.
- All 509 library tests, strict all-target/all-feature Clippy, formatting, and
  the release quickstart compile gate pass. The unchanged physical 30 ms and
  45 ms source-age thresholds remain the acceptance authority. The first
  aligned-cadence run improved the 20 ms application p95 from 52.637 ms to
  45.096 ms with zero route loss or discontinuity, while 10 ms remained above
  its platform floor and one microphone stream error was retained.
- No timestamp substitution, second capture engine, scaffold, mock,
  benchmark-only queue, or release action was introduced.

## Explicit microphone replacement and reacquisition — 2026-09-24

- A running Session can now keep its application or system-audio source and
  compiled destinations alive while the host explicitly replaces one
  microphone with an exact device selector.
- Safe replacement opens the new physical source before detaching the current
  capture. Failed prepare/open leaves the current microphone attached.
- Exact reacquisition closes the current capture before reopening the supplied
  selector, covering native routes that require teardown to reset. Failed
  reopen leaves the microphone explicitly detached without corrupting Session
  finalization or unrelated stems.
- Same-device reopen retains physical source identity while advancing source
  generation and discontinuity. Different-device replacement exposes the new
  source identity; Core never pretends the devices are continuous.
- Session metrics expose attempt, completion, pre-attach failure and response-
  timeout totals, the attached source if any, continuity, and latest completion
  time. Native-format and capture observations always describe the currently
  attached capture rather than stale predecessor state.
- Core does not choose fallback devices, infer a recovery policy, retry
  automatically, or treat open/attach as first-frame or useful-signal proof.
- The recovery selector may deliberately name the current default; that is
  host-owned policy, and Core still reports the resulting physical source
  identity plus generation and discontinuity instead of implying continuity.
- Focused internal and public-facade conformance tests pass. Full Core and Lab
  acceptance gates remain required before this candidate is accepted.
- No scaffold, mock, automatic fallback, provider integration, or physical-HFP
  product claim was introduced.

## SDK native-format conformance projection — 2026-09-24

- The existing deterministic `conformance-fixtures` capture backend now
  reports the exact PCM format it opens: 48 kHz interleaved `f32`, with two
  channels for application/system audio and one channel for the microphone.
- The public conformance test verifies that native-format observations remain
  aligned with the two Session sources and preserve rate, channel count, and
  sample representation. Python and Node installed-package tests can now
  exercise format projection through real Core observations instead of only
  comparing declaration inventories.
- This changes only the finite deterministic fixture. It is `LOOPBACK-ONLY`
  component evidence and does not claim a physical device format, Bluetooth
  HFP behavior, Teams routing, Minutes integration, or release publication.
- No production capture callback, runtime policy, scaffold, mock product path,
  automatic fallback, VM, or retained evidence was changed.

## HFP-shaped managed-SDK conformance vector — 2026-09-24

- The deterministic conformance backend now gives exact microphone selectors
  distinct physical source identities and exposes one named HFP-shaped vector
  as 16 kHz mono signed 16-bit native PCM.
- Replacement of the default fixture microphone with that selector is proven
  to advance source generation and discontinuity, report the new physical
  source identity and opened native format, and deliver replacement frames on
  the existing microphone Stem.
- Frames remain canonical Session `f32` after the simulated native conversion;
  the native-format observation records what the capture endpoint opened.
- This is a finite `LOOPBACK-ONLY` cross-language fixture. It does not claim
  that a physical Bluetooth headset, Teams route, or Minutes integration has
  passed.

## Connector cancellation test determinism — 2026-09-25

- The connector cancellation contract now waits for the worker to enter its
  run callback before asserting that abort intent reached that callback.
- This removes a scheduler race where immediate cancellation could correctly
  cancel prepared work before the worker ran, while the test incorrectly
  required one completed run.
- The wait is test-only and bounded at two seconds. Production cancellation,
  connector behavior, realtime paths, and public APIs are unchanged.
- The release matrix also compiles the frame-normalizer gap reset only where
  it is used (macOS and tests), keeping strict Windows Clippy warning-free.
- No scaffold, mock, loopback product path, VM, or retained evidence changed.

## Microphone continuity patch release — 2026-09-26

- Candidate 1.1.12 carries the accepted source-timeline correction at 9ce2af1.
  Four clean physical cases delivered 2,700 observed frames at 10/20 ms with
  zero discontinuities or losses. This is built-in macOS correctness only.
- Version and current install documentation now identify the compatible patch.
  Publication follows protected CI, exact package and independent registry
  consumption; old tags, artifacts and historical failures remain unchanged.

- Release CI attempt 36266530731 passed native builds and Linux tests, then
  rejected the release-note phrase “bounded queue sizes”. Replaced it with
  “queue capacities”; no code or check was changed. Full CI must pass again.

## 2026-09-29 — AEC ownership correction and deep source review

Status: **PARTIAL**. W21-AEC-CORE-INTEGRATION / candidate 135 is active.
The user rejected an external AEC example as the product implementation.
AUDIO-042 records built-in Session ownership and an AEC-specific AUDIO-034
exception. The factory architecture review compares WebRTC, Apple, Windows,
PipeWire, LiveKit and evaluation protocols with the actual prototype.

Concrete gaps: package exclusion, manual factory/port wiring, caller-owned clock
alignment, stereo averaging, unknown output-delay semantics, fixed derivation
generation, non-yielding native work under cooperative async deadlines, and
non-durable terminal diagnostics. A source review is not a physical experiment.

The native prototype compiled. Its test command completed with one pass and two
failures at Session host setup: default features were disabled and the host
returned UnsupportedPlatform. No Session AEC result is accepted. The prototype
is temporarily retained for migration; no new product stub was introduced.

Product/demo outcome remains pending: app-only capture, and processed mic with
an explicitly selected reference through the shipping Session API. Reference
engine synthetic proof is separately accepted in Bench a7e2bdb and unchanged.
No physical, platform-wide, speech or competitive result is claimed.

Staff review: documentation and admission correction only; purpose and boundary
recorded in AUDIO-042. Scaffold inventory updated: yes. Runtime CODE_PROTOCOL,
quickstart, installed-consumer and complete Core checks remain required when
implementation changes land. Decision: NEEDS MANUAL REVIEW for runtime acceptance.


## 2026-09-29 — Built-in AEC implementation checkpoint, candidate 135

Status: **PARTIAL**. This step is implemented and verified for prepared same-clock
PCM; the complete Core task and physical qualification are not accepted.

- Purpose/files: replace the unshipped example package with private
  `src/echo_cancellation`, a default engine dependency, typed Session composition,
  retained observations and the normal Core integration test.
- Product/demo: Session executes real WebRTC processing for 10/20 ms mono and
  anti-phase stereo input while preserving the raw application. Application-only
  use remains independent. Missing reference fails visibly without stopping a
  healthy application branch. Foreign and identical inputs are rejected.
- Architecture: capture still owns devices, timing still owns clocks, runtime
  still owns generic execution. Fixed compiled-input port dispatch in runtime;
  no AEC-specific routing workaround. One blocking worker owns each processor.
- Manual CODE_PROTOCOL review removed allocating/blocking custom destructor
  cleanup. The private umbrella `dsp/aec` was replaced by the admitted feature's
  specific name. No empty framework or provider catalog was introduced.
- Tests: 617 Core tests across 25 suites pass with all features. Four AEC tests
  also pass from the packaged crate with default features. Protocol (including
  strict all-target/all-feature Clippy and callback allocation checks), minimal
  build, release quickstart compilation and Cargo package verification pass.
- Scaffold inventory updated: yes. No production mock, bypass or muted-mic
  substitute. PCM test sources are deterministic; no physical or speech claim.
- Remaining: device-clock adaptation, actual playback-reference acquisition,
  measured algorithmic delay, further reset/recovery cases, SDK/CLI projection
  and physical double-talk qualification. Native work cannot be force-preempted.
- Evidence: factory `docs/execution/evidence/W21-AEC-CORE-INTEGRATION/integration/`.
  Earlier failed attempts remain preserved. Existing 34 engine artifacts remain
  separately sealed. This is an unreleased candidate; no version or main changed.
- Staff decision: PASS for this bounded implementation checkpoint;
  NEEDS MANUAL REVIEW before full AEC acceptance. Eight JSON predicates stay pending.


## 2026-09-29 — Short private AEC names

User-requested naming-only follow-up: `src/aec` and
`src/session/extensions/aec.rs` replace the longer private module names.
All five processor module files remain byte-identical; only imports and module
names change. Public API and Cargo feature remain unchanged. Current owner docs,
ADR, execution envelope and JSON task registry reference the new names.
Execution validation, CODE_PROTOCOL and all four focused Session tests pass.
Required quickstart compile is recorded in the naming evidence directory.
No scaffold introduced. Overall AEC remains PARTIAL with the previous open gates.

## 2026-09-30 — AEC admission and startup, candidate 138

Status: **PARTIAL**; release and all eight Core acceptance predicates remain pending.

- Admit actual render frames independently of microphone frames in source-time
  order, retaining separate stereo channels. Avoid artificial reference splicing
  and requiring a future reference frame at normal EOF. The existing 80 ms audio
  queues limit retained input; missing references fail explicitly.
- Own timestamp-cadence residuals in `timing`. Small residuals are observed;
  sequence loss, source changes and larger jumps reset processing. Synthetic
  common-clock tests exercise ±100 ppm sample clocks, ±100 us timestamp jitter,
  independent starts, loss and recovery. This is not arbitrary hardware-clock
  adaptation or a physical-device qualification.
- Initialize both native PCM formats and clear initialization audio before real
  input. The native engine otherwise discarded the first reference when its
  first capture changed format, producing a different first-block signal delay.
  The real Session first-transient regression covers mono/stereo and 10/20 ms.
- Observe cancelled native awaits separately from failures, retain the first
  terminal error after late replies, and preserve observations after stop.
- Added real-engine near-end/double-talk and startup/tail probes. They are
  deterministic PCM tests. Engine-only probes show that trimming nominal432
  samples destroys a first transient; no such compensation was added. Complete
  processor tail output and per-frame microphone provenance through generated
  audio reentry remain unresolved. The delay observation stays unknown.
- Product/demo: prepared PCM through the standard Session operation, normal raw
  stem fan-out and independent app-only use. No provider, hardware driver,
  parallel Session engine or production scaffold added. Inventory updated: yes.
- Validation receipts, including failed attempts, are in factory
  `docs/execution/evidence/W21-AEC-CORE-INTEGRATION/release-preparation-138/`.
  Final CODE_PROTOCOL (including hot-path and strict Clippy checks) and635 tests
  across28 suites pass after the startup correction. Required release quickstart
  compilation is recorded separately. These are candidate-source diagnostics.
- Staff decision: NEEDS MANUAL REVIEW for complete AEC acceptance and publication.
  No version, tag, release, deployment or main branch changed.

## 2026-09-30 — AEC final audio and processed-frame origin, candidate 138

Status: **PARTIAL**, software checkpoint ready for SDK requalification.

- Preserve causal native output and first transients. Graceful finish emits at
  most 40 ms of native history, identified as zero-input padding, without fake
  reference frames or inflated microphone counts. Cancellation/reset discard old
  history explicitly; failed partial drains cannot be retried. The cap is a
  termination policy, not mathematical convergence of adaptive comfort noise.
- Carry fixed-size input provenance and processing generation through audio
  fan-out, generated reentry, polled reads and recording event ledgers. Nominal
  432-sample filter delay is distinct from CPU time and qualified device delay.
- Keep generic routing alive until actual producer EOF and operator flush, then
  finalize recording. Retain the existing polled ring after stop so consumers
  can drain accepted final frames; cancellation discards them. Retired empty
  rings are released and unread retired rings cannot accumulate across restarts.
- Fix empty-receive/producer-close races and record actual drain-deadline expiry.
  Inline provenance avoids allocation on realtime ownership transfer; six
  reviewed 64-bit layout ceilings constrain memory growth.
- Validation: 649 tests across 29 suites, all-feature strict Clippy, hot-path
  checks, CODE_PROTOCOL, release quickstart and minimal build pass. Native
  first/last impulse, 10/20 ms mono/stereo, real Session WAV and post-stop read
  tests preserve the final audio; actual pool exhaustion fails visibly.
  Package guard now requires the AEC source and curated guide; its initial
  precommit run refused dirty source and is rerun after the clean commit.
- Evidence: factory release-preparation-138/core-tail-*. Failed attempts remain
  recorded. All runtime checks bind unchanged source; only the package guard
  script was added afterward, with no runtime source edits.
- Remaining real defect: native application/system sources declare stereo and
  microphones mono, but AEC currently uses the global Session layout for both.
  Prepared same-layout PCM tests do not qualify normal desktop composition.
  Device-clock adaptation, physical quality and platform packaging remain open.
- No new product mock, provider, capture permission or callback work. Test PCM
  and lifecycle doubles are software evidence only. Inventory updated: yes.
  Staff decision: PASS for this bounded tail/provenance correction; NEEDS MANUAL
  REVIEW for complete AEC acceptance/release. No version/tag/publish/deploy.

### Native AEC CI prerequisites — 2026-09-30

Workflow preparation installs pinned Meson 1.7.2, Ninja 1.11.1.4 and CMake
3.31.6 with Rust 1.95.0 llvm-tools on existing Linux/macOS build jobs. Existing
triggers, permissions, release guards, targets and acceptance commands are
preserved. The default native AEC dependency needs these tools.

Staff review: workflow semantic/parser comparison and git diff whitespace
checks PASS; evidence core-ci-prerequisites-check.json in factory Candidate138
release-preparation. No workflow dispatched, version edited or release made.
This prepares dependencies; it does not establish hosted/platform qualification.
Scaffold inventory n/a; no runtime/hot-path behavior changed.

### Accepted source input on graceful stop — 2026-09-30

Status: **PARTIAL**, source lifecycle correction in candidate 138.

- Purpose: Session stop previously cancelled external sources before their
  accepted input drained. An installed Python immediate-stop run returned a
  successful stop after losing an accepted PCM frame. Core now separates
  graceful finish from cancellation and drains source input before closing.
- Ownership: Session source runtime owns optional `SourceDriver::drain` and
  `cancel` lifecycle hooks. Existing drivers keep their default behavior;
  drained emissions use the existing format, identity and continuity checks.
  Providers remain outside Core. PCM admission uses one atomic state and one
  writer; closing admission cannot lose a write already publishing to its
  existing queue. The source worker waits at most 100 ms for that writer.
- Finish rejects later writes as `Closed`, preserves accepted frames and does
  not increment cancellation counters. Cancel invokes producer cleanup even
  before the first read, rejects writes as `Cancelled` and discards pending
  receipt audio. Source drain has a one-second cooperative deadline. A provider
  callback that does not return cannot be forcibly interrupted.
- Processing and cleanup failures retain the first error, still call close,
  and count the failed source once. No second queue, capture callback wait,
  product mock, provider implementation or permission change was introduced.
  Scaffold inventory: n/a; deterministic lifecycle test drivers are test-only.
- Focused validation: 16 source tests, three atomic admission/cancel tests and
  eight Session tests pass with native macOS capture enabled and AEC excluded.
  The Session tests stop immediately after start with one/eight queued frames,
  both open/closed producers, exact PCM/identity and held-buffer ownership.
  Failure receipts remain preserved alongside successful tests in factory
  `release-preparation-138/core-source-drain-*` evidence.
- Staff review: parent review passed the admission/lifecycle design. Full
  CODE_PROTOCOL, all-feature tests, quickstart and final SDK artifact gates
  remain required before merge/release. This software checkpoint does not
  establish physical capture quality, platform qualification or AEC leadership.
  No version, tag, registry publication or deployment changed.

Source follow-up review: final producer-abandonment polling now performs the
acquire fence required by the pinned rtrb contract before checking the final
frame. The deterministic public Session regression already covers closed and
open producers with one/eight accepted frames and exact delivery after immediate
stop. Four external-source runtime tests pass, including drain and close failure
in both the stop result and terminal event. All 247-file CODE_PROTOCOL checks,
14 hot-path regressions, formatting and all-feature Clippy passed in
core-source-protocol-02 before the final three-line fence correction; root will
run final protocol and broad Core gates on the complete frozen source. The first
protocol attempt lacked the existing Meson cache environment and failed while
trying to download Abseil; its failure is retained, not counted as a code failure
or passing gate. No tests or quality thresholds were weakened.

Final source lifecycle gate: root independently completed 671 tests across 31
suites with zero ignored tests, strict all-feature CODE_PROTOCOL, the release
quickstart, and the minimal consumer on unchanged final source. Receipts are
core-final-full-01, core-final-protocol-01, core-final-quickstart-01 and
core-final-minimal-01. Staff review PASS covers the atomic admission boundary,
graceful drain, exact cancel/close handling, terminal external-source failures,
runtime drain failures, and the final producer acquire fence. The source-only
checkpoint includes the shared lifecycle terminal corrections while leaving the
separate mixed-channel AEC changes to their own commit. No new product scaffold.
Decision: SAFE-TO-MERGE for this source lifecycle correction; final SDK artifacts,
physical AEC and platform/release qualification remain separate requirements.

## 2026-09-30 — independently negotiated AEC input channels

PARTIAL product qualification; complete software checkpoint. Session resolves
mic and playback layouts independently, validates concrete 48kHz f32/cadence
before declaration, and preserves input channel counts through ingress. Mono
mic/stereo app and the inverse run at 10/20ms. Standard AEC3 tuning with stereo
content detection disabled preserves the negotiated reference from startup;
no custom transform or parameter tuning. Original and held-out stereo/near-end
thresholds pass; historical failures are retained. Core full suite671/31 with
zero ignored, CODE_PROTOCOL/strict Clippy, release quickstart and minimal build
pass against unchanged source (core-final-{full,protocol,quickstart,minimal}-01).
Independent package consumption follows this commit. Physical acoustics,
independent clocks, SDK final packages and platform qualification remain open.
Root scaffold inventory updated; no product mock or provider dependency added.

## 2026-09-30 — interrupted AEC replies and required signal delivery

PARTIAL product qualification; failure-path corrections ready for final gates.
AEC retains Interrupted after late native completion and counts produced frames
whose response is abandoned, including a successful oneshot send followed by a
receiver drop before polling. The response owns its accounting until consumed;
late native observation publication cannot erase the discarded-output counter.
Existing duration observations now measure complete native command wall time,
including queue wait, reset and reference analysis. The 100ms process deadline,
DSP configuration, acoustic gates and ordinary cancellation semantics are unchanged.
Ten focused real-engine/lifecycle tests pass (core-aec-interruption-01), including
abandoned normal and flush responses, post-send abandonment and consumed output.

Required typed routes now reject nonterminal as well as terminal values on full
or closed destinations. This applies equally to Source and Operator output;
required delivery failure is visible in source/operator counters, Session stop
and terminal events. Intentionally lossy routes still continue other branches;
the existing saturation fixture now explicitly declares DropAllowed instead of
relying on the broken required-route behavior. Eleven focused required-route
tests pass (core-required-signal-01). The strengthened Source loss counter and
all 578 Core unit tests pass on unchanged final source in
core-pressure-full-unit-01. No native device, provider or timing mock was added;
the real Session regression uses the existing deterministic custom Source and
Endpoint fixture. Full integration, CODE_PROTOCOL, quickstart, package and rebuilt
SDK consumers remain required before accepting this changed release candidate.
No tag, publication or deployment changed in this checkpoint.

Final software validation passes:578 unit tests,96 integration tests and5 doctests
(679 distinct tests), CODE_PROTOCOL including strict Clippy/hot-path checks, and
the release quickstart build. The integration invocation filtered one unit test
already passed in the complete578-unit gate; no test is untested by that filter.
Receipts:core-pressure-integration-01, core-pressure-protocol-01,
core-pressure-quickstart-01 and core-pressure-doc-01. All retained run sources
were unchanged. Package/SDK rebuilds and native portability remain ahead; the
earlier physical proof remains bound to5559610, not automatically to this commit.

## 2026-09-30 — explicit subscription revocation and cancellation delivery

PARTIAL product qualification; software correction passes. Closing a typed
subscription revokes only that branch, physically discards queued payloads and
reports accepted unread/future rejected values as discarded. Unexpected receiver
loss and full required routes remain failures. Session cancellation broadcasts to
all workers before finalization; DiscardQueued rejects/counts late computed
outputs, while DrainQueued preserves accepted input despite a pending wake.
DeliveryPolicy and RouteSettings expose the existing loss policy explicitly.

Six deterministic regressions cover branch isolation, payload release, receiver
drop, cancellation immediately after processing, idempotent cancellation, and
pre-first-poll DrainQueued behavior. Final gates on the exact frozen source pass:
685 tests across 31 suites with zero ignored, CODE_PROTOCOL including strict
Clippy/hot-path checks, release quickstart, and no-default-features check. Receipts
core-delivery-{full,protocol,quickstart,minimal}-01 and
core-delivery-focused-02 bind the source map core-delivery-source-01.json.
The prepared timing tests do not qualify independently clocked hardware.

No new scaffold, mock, or provider path; existing deterministic lifecycle fixtures
exercise real Core execution. Decision: SAFE-TO-MERGE for this bounded correction.
SDK native rebuilds and the four previously failing installed Python regressions
remain required, followed by physical and platform/release qualification.
No version, tag, publication or deployment changed.

## 2026-09-30 — Session AEC deadline and late completion regression

Test-only checkpoint: normal Session echo composition now has a deterministic
regression for a delayed native response. The fixture holds one microphone
command before the actual APM worker, preserving the production manifest and its
100 ms deadline. Session reports the timeout and interruption while independent
application PCM and metrics remain available. Releasing the command executes real
APM; the abandoned response counts one discarded output and reaches no consumer.
The test joins both its command relay and the native worker, delivers another
application frame after AEC failure, and retains the failed Session stop outcome.

Only `src/aec/worker_tests.rs` and this progress record change. Production sources,
public APIs, defaults, native engine settings and build inputs are unchanged from
71e203c. This verifies a transport-induced late response, not forced preemption of
a hung native function, physical cancellation quality, or arbitrary device clocks.
The test-only barrier introduces no live scaffold or alternative audio engine;
scaffold inventory update: n/a.

Validation on unchanged test source: all 11 AEC worker tests pass;
CODE_PROTOCOL passes for 247 Rust files, including 14 hot-path regressions,
formatting and strict all-target/all-feature Clippy; release quickstart compiles.
Receipts in factory `release-preparation-138/` are core-deadline-worker-tests-01,
core-deadline-protocol-01 and core-deadline-quickstart-01. The initial exact-name
invocation selected zero tests and is retained as non-qualification evidence;
the corrected invocation and final worker suite execute the new regression.
Parent staff review: PASS for this scoped test checkpoint. Eight Core predicates
and physical/platform/release acceptance remain controlled by the root executor.
No version, tag, publication or deployment changed.

## 2026-10-01 — optional Session AEC, candidate 142

SAFE-TO-TEST for the bounded software slice. Default Core enables native capture
and Opus only; `aec` explicitly enables the established engine. The retained
`echo-cancellation` feature delegates to `aec`. Session composition and typed
observations remain available in lean builds; requesting an absent processor
returns an explicit error without changing the graph. `aec_available()` reports
build contents, never acoustic quality or native-device support. Engine-owned
factory code is gated independently from the public lightweight API.

The existing quickstart supports explicit `--aec`, reference coverage, independent
application/raw microphone/processed microphone stems, recording and terminal
observations. Application-only capture remains independent. No live scaffold,
provider, alternate algorithm or automatic capture-scope expansion was added.
Inventory updated in the factory. CODE_PROTOCOL, 14 hot-path regressions, strict
all-feature Clippy and formatting pass; 682 tests across 34 full-feature suites,
default absence/raw delivery, no-default compilation, six Session AEC tests,
five doctests and both lean/enabled release quickstarts pass. API compatibility
passes 196 checks against 1.1.9. Receipts and source hashes are in factory
`docs/execution/evidence/W21-AEC-OPTIONAL-INTEGRATION/`.

Staff gate PASS for lean/opt-in source behavior. Physical acoustic quality,
independent device clocks, platform artifact qualification and native AEC
selection are unfinished. Known/unknown microphone processing provenance and
protection against implicit duplicate cancellation are registered separately;
they are not implemented here. No version, tag, push, publication or deployment.

## 2026-10-02 — native AEC selection contract, candidate 144

The Session API now records an explicit native AEC request for one microphone
and one exact playback device. The lean build retains this declaration API.
Unsupported capture backends reject preparation before opening the input; the
request never silently falls back to raw or portable processing. Native and
portable AEC cannot be declared on the same microphone ancestry, including
late generic Operator connections. This request is intent, not an active
processing fact.

At open and replacement, Session admits the native route only when the backend
reports matching actual and requested microphone identities, matching playback
device, enabled and non-bypassed effect, confirmed reference, and a live route
handle. The handle can be invalidated irreversibly. Session then discards queued
microphone media, detaches that source and reports failure; unrelated application
media continues. Replacement/reopen retains the declared exact reference and
must pass the same checks. The native route uses the microphone stem and adds no
portable Operator or second processing pass.

The route tests use MOCKED backend facts with actual Session PCM delivery and
teardown. Built-in macOS, Windows and Linux capture backends do not implement
the native request yet. No physical acoustic quality, device-route coverage,
clock recovery or SDK native selection is claimed. App-only capture remains
independent. No new package, repository, fork, version, tag, publication or
deployment was made. Scaffold inventory: the mocked native backend is a test
double only; there is no live scaffold. Final acceptance receipts and staff
gate belong to the root executor task W21-AEC-NATIVE-SELECTION-CONTRACT.

## 2026-10-02 — Windows native AEC VM probe rejection guard, candidate 145

The Windows physical-endpoint probe now treats only the observed unsupported
controllable-effect rejection as an expected no-route result. A different
Session startup failure fails the test, so broken initialization or routing
cannot pass as a successful native AEC admission guard. This is a test-only
correction. The virtual Windows endpoint still does not establish active AEC,
audible PCM or acoustic quality. No scaffold or product path was added;
scaffold inventory update: n/a. Final acceptance receipts belong to the
factory W21-AEC-NATIVE-DEVICE-BACKENDS executor.

## 2026-10-05 — Core-owned recording trigger clips

PARTIAL workflow qualification. Added source-aware finalized recording clips and
bounded generic Session-time context windows; Capturo delegates extraction to
Core. The reader reuses the recording writer schema/checksum and preserves
independent PCM channels, provenance, boundary truncation and discontinuities.
Existing buffer counts are not sample counts. No callback code, AI engine,
provider, dependency, version, fork or publication changed. External Session
conformance is deterministic PCM, not physical microphone evidence. Core gate
results and final source hashes belong to W21-RECORDING-TRIGGER-CLIPS in the
factory JSON registry. Live pre/post-roll and external wake-word integration
remain separately qualified; no live scaffold is introduced.

Verification of the finalized-recording slice completed: 727 tests across 38
all-target/all-feature results (zero failures/ignored); five public default
clip conformance tests; compiling public documentation; CODE_PROTOCOL including
strict Clippy and hot-path checks; release quickstart and rustfmt check. Capturo
consumes the same API through its real Session recorder/storage integration.
Source-qualified component evidence is in the factory task's final/ directory.
No physical/wake/live-buffer/SDK distribution claim follows from these checks.
The five pre-existing release-preparation edits remain preserved and outside
this source commit. Formal clean-owner acceptance is still pending.


## 2026-10-05 — finalized recording clip SDK parity

Phase 2 W21-RECORDING-TRIGGER-CLIPS, candidate147. Core exposes stable
recording.clip_* error codes for the existing finalized-recording reader.
Python and Node delegate extraction, format validation, sample rounding, source
identity and checksum checks to this reader; no detector, provider, callback or
default AEC dependency changes. Normal Session PCM conformance passes, preserving
independent application/microphone stereo stems and exact source/timestamp data.

Validation: 728 all-target/all-feature tests, six public default clip tests, six
doc tests, CODE_PROTOCOL/hot-path checks, strict Clippy and release quickstart
passed. SDK installed consumers compare identical WAV SHA-256 and u64 metadata;
full receipts are in factory evidence/W21-RECORDING-TRIGGER-CLIPS/sdk-parity-147.
No live rolling history, wake recognition, physical platform, publication or
registry availability claim. The five prior release edits remain byte-preserved.
Staff review PASS for the finalized-recording code; formal clean-owner
acceptance remains pending because unrelated release edits are preserved.

### W21 live audio history — candidate 148, 2026-10-06

REAL component implementation; LOOPBACK-ONLY installed integration.
`Session::audio_history` and explicit source/stem retention use the ordinary
grouped Endpoint runtime. Default shared limits are 30s / 16 MiB PCM / 4096
buffers; all queue, buffer-count and PCM bounds are explicit. Independent stems,
channel interleaving, Session time, clock/generation/permission identity and exact
sample indexes survive extraction. Delayed post-context, expired/missing context,
source resets, clear, graceful completion, cancellation purge and immediate
worker failure are distinct. No detector, inference, dependency, feature,
version or implicit device capture was added.

Validation: 738 all-target/all-feature tests pass; the later oversized-input
failure case also passes in the five-test history suite (739 distinct Core
tests across both runs). Seven doc examples, CODE_PROTOCOL including hot-path
checks/strict Clippy, and release quickstart compilation pass. Lab installs real
local Python/Node artifacts: two independently retained/recorded PCM stems,
288 history buffers and 128 concurrent reads per SDK, exact live/finalized and
cross-SDK WAV bytes, bounded retention, explicit expiry, zero recorder drops,
new-Session capture and cancellation purge.

Evidence: factory `docs/execution/evidence/W21-RECORDING-TRIGGER-CLIPS/live-history-148`.
Scaffold inventory: no new runtime stub/mock; controlled Lab PCM is explicitly
LOOPBACK-ONLY. Staff review: PASS for this component boundary. Release/clean-owner
acceptance remains pending against preserved unrelated manifests/AEC work; no
physical capture, wake recognition, native-AEC or platform claim. Capturo's
consumer integration is next and is not declared done here.
# 2026-10-06 — clip/history release preparation

User resumes publication of existing Core 1.1.13 and corresponding SDKs where
qualification permits. The prepared version and release notes now cover the
committed bounded live-history and finalized-recording clip APIs. Same-source
local component/Lab evidence remains in candidate148; release package and CI
qualification belong to candidate149. No new dependency or native fork change.
Native macOS/Linux AEC remains unsupported; physical/Windows qualification is
in progress. Release preparation does not claim registry publication.
