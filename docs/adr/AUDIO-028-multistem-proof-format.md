# AUDIO-028 — Multistem proof format and finalization

- Status: Accepted
- Date: 2026-07-19
- Owners: `pks-nodes`, `pks-runtime`, `pks-timing`
- Serves: Phase 2 W4–W8 product proof

## Context

The narrow proof must preserve application and microphone audio as independent,
aligned, source-aware stems. A playable WAV alone cannot prove source identity,
clock alignment, permission state, gaps, destination isolation, or whether file
headers were finalized after a failure.

The proof format must stay inspectable with standard tools and must not create a
custom media container before the product slice is proven.

## Decision

Each recording is a directory with this fixed minimum layout:

```text
session-<session-id>/
  manifest.json
  stems/
    <stem-label>.wav
  events/
    discontinuities.jsonl
    permissions.jsonl
  metrics/
    destinations.jsonl
    summary.json
```

W4 uses the labels `application` and `microphone`; the format accepts additional
validated stem labels without changing core enums.

Stem audio is 32-bit IEEE-float WAV at its declared sample rate and channel
count. Recorder workers own file allocation, writes, checksums, and header
finalization. Realtime code only performs a non-blocking write to the bounded
compiled edge.

Every stem declaration records:

- session, source, stem, and clock IDs;
- source generation and permission epoch;
- source and session timeline origins;
- sample rate, channel count, and sample format;
- first and final normalized timestamps;
- written, silence-filled, stale, and dropped frame counts;
- explicit gap ranges and finalization state;
- finalized file size and `fnv1a64` integrity checksum.

`fnv1a64` is an evidence-corruption checksum, not a security primitive. A future
artifact signing layer may add SHA-256 without changing the WAV or lineage
semantics.

`pks-timing` owns source-to-session timestamp normalization. The recorder
receives an immutable timeline mapping and never estimates or corrects a clock
inside the file writer.

Missing time is never compressed. The worker writes silence for a positive
normalized timestamp gap and emits a matching discontinuity event. Duplicate,
overlapping, or backward frames are rejected and reported rather than silently
changing the timeline.

`manifest.json` is written with `recording` state before workers start. Clean
stop drains each bounded queue, finalizes every WAV header, writes event and
metric sidecars, then atomically replaces the manifest with `complete` state.
Any worker or finalization failure produces `incomplete` state with the exact
stem error. A process crash therefore leaves a visible non-complete manifest.

Each stem has an independent compiled edge and worker. Recorder queue pressure,
disk failure, or worker exit may drop only that recorder branch; it cannot block
capture, a connector, or a browser/remote route.

## Rejected alternatives

- MKV, custom containers, and compressed archives: unnecessary for the narrow
  proof and harder to inspect independently.
- One mixed WAV: destroys the independent-stem product claim.
- One shared recorder queue: lets a slow stem block another stem.
- Timestamp compression on loss: creates playable but false evidence.
- File I/O on capture or realtime processing threads: violates the hot-path
  contract.
- Counter-only recording sinks: do not produce product evidence.

## Acceptance evidence

W4 must demonstrate two independently playable WAV files, aligned normalized
durations, exact identity fields, explicit silence-backed gaps, deterministic
checksums and finalization, readable incomplete state on failure, and a slow
recorder branch that does not stop another compiled edge.
