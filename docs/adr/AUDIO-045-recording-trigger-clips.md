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

The live slice adds `Session::audio_history(AudioHistoryConfig)` and explicit
`SourceOutputHandle::retain_audio` / `StemHandle::retain_audio` routes. This is
production code in `recording`, using existing grouped Endpoint execution,
source-aware queues and Session startup/shutdown. Ordinary capture has no history
worker or retention allocation. No duplicate capture, processing engine, package,
feature or dependency is introduced.

One endpoint control worker retains up to 64 independently identified stems.
Default shared limits are 30 seconds, 16 MiB PCM and 4096 buffers; maximums are
120 seconds, 64 MiB and 65536 buffers. Oldest PCM is evicted before incoming PCM
is copied. Buffer metadata capacity shrinks after eviction and clear. Reads copy
bounded PCM under a control mutex and encode WAV after release; callbacks and
realtime partitions retain the existing bounded copy-to-branch-pool boundary.
Callers bound concurrent reads and delayed-trigger retries. There is no hidden
request backlog, detector catalog or native inference.

Live windows deliberately differ from finalized file windows: missing pre-roll
returns Expired; future post-roll returns NotReady; gaps return MissingContext
instead of invented silence; completed short captures return Ended. Cancellation
purges history; graceful stop leaves bounded retained history readable. Source
generation, clock or permission changes reset that stem. Failure immediately
purges PCM and reports Failed. Continuous samples use exact sample counters,
including fractional-nanosecond durations, rather than repeated rounded offsets.

Python sync/async and Node bind these Core APIs. Python native reads release the
GIL; async reads run on workers. Node native reads use blocking workers and exact
bigint identities/timestamps. Cancelling an await does not interrupt a started
copy; applications own concurrency and returned clips.

Qualification uses ordinary Session execution with controlled PCM and installed
local native artifacts in Lab. It proves routing/retention/slicing, not physical
acquisition, recognition, wake-word quality, native AEC, platform breadth or
release readiness. Capturo integration remains a separate consumer step.
