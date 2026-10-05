# AUDIO-045: Source-aware context clips from recording

Status: Accepted ownership; PARTIAL workflow qualification, 2026-10-05.

The user assigns generic trigger/clip support to Core and explicitly excludes
native AI claims. Existing async Operators accept PCM and emit typed signals
with timing, lineage, bounded routes, cancellation and required-delivery failures.
Recognition and application actions remain external. This is the narrow recording
primitive missing from that composition; the external consumer failed to import
RecordedAudio/RecordingClipWindow before implementation (factory evidence).

Core adds RecordedAudio, RecordingClipWindow, RecordingClip and exact RecordedStem
metadata in the shipping package, with no dependencies/features/version change.
SessionRecordingOutcome/Receipt delegate to the same recording implementation.
The reader reuses the writer manifest and checksum; buffer count is never treated
as sample count. Preserve channel interleaving, source/Session/stem/clock,
generation, permission epoch, normalized sample-zero timestamp and gaps.
Do file I/O/checksumming only on a blocking control worker. Never recapture,
remix, infer a source from text, or silently broaden permission scope.

A finalized-file clip is not live pre-roll. Reader limits: 2 MiB manifest, 64 stems,
1024 gaps/stem, 1 GiB selected WAV, 120-second requested interval, 32 MiB output.
Context shortens at actual media boundaries; sample rounding is explicit.
Recorded silence for missing audio and discontinuities survive replay.
FNV is the recorder corruption check, not authenticity. Applications authorize
storage access and may bind a cryptographic hash to the original recording.
The caller owns the recording directory: concurrent adversarial writes are not
a supported trust model. Each extraction rechecks manifest and WAV integrity.

Capturo configures literal rules, accepts source-scoped ASR match intervals,
stores immutable session-owned clips and displays History. Future generic
signal/telemetry detectors use the same ns interval without introducing provider
catalogs into Core. Wake phrases use external KWS Operators and the existing
bounded signal path; Capturo opens its composer, never implicitly captures/sends.

Next distinct qualification: live bounded history Endpoint on the existing
runtime, delayed triggers, lost context, cancellation, output quotas and recovery.
No live, acoustic, AI-superiority, SDK binary or release claim is made here.
