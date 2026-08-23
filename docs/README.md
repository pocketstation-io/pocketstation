# PocketStation documentation

<!-- claims: CLM-DOC-001-CAP-001,CLM-DOC-001-CAP-002,CLM-DOC-001-SOURCE-001 -->

Use this index to move from a first Session to the exact contract, task, failure, or implementation detail you need. Public navigation follows developer responsibilities; the separate intelligence workspace preserves file, symbol, relationship, behavior, and claim provenance.

## Start here

- [Install PocketStation](/docs/getting-started/installation.md)
- [Run the Rust quickstart](/docs/getting-started/rust-quickstart.md)
- [Check platform prerequisites](/docs/getting-started/platform-prerequisites.md)
- [Run repository examples](/docs/getting-started/examples.md)

## Core concepts

- [Application capture](/docs/concepts/application-capture.md)
- [Asynchronous operators](/docs/concepts/async-operators.md)
- [Audio reentry](/docs/concepts/audio-reentry.md)
- [C ABI ownership](/docs/concepts/c-abi-ownership.md)
- [Cargo features and build surfaces](/docs/concepts/cargo-features.md)
- [Conformance and qualification](/docs/concepts/conformance.md)
- [Connector model](/docs/concepts/connectors.md)
- [Connector worker lifecycle](/docs/concepts/connector-workers.md)
- [Endpoint lifecycle](/docs/concepts/endpoints.md)
- [Error and status model](/docs/concepts/error-model.md)
- [External PCM input](/docs/concepts/external-pcm.md)
- [Frame identity and lineage](/docs/concepts/frame-lineage.md)
- [Graph contracts](/docs/concepts/graph-contracts.md)
- [Microphone capture](/docs/concepts/microphone-capture.md)
- [Multistem recording](/docs/concepts/multistem-recording.md)
- [Native extension libraries](/docs/concepts/native-extensions.md)
- [Observations and metrics](/docs/concepts/observability.md)
- [Opus codec state](/docs/concepts/opus-codec.md)
- [Permissions and source lifecycle](/docs/concepts/permissions-and-source-lifecycle.md)
- [Polled audio](/docs/concepts/polled-audio.md)
- [Realtime routing](/docs/concepts/realtime-routing.md)
- [Release evidence boundary](/docs/concepts/release-evidence.md)
- [Runtime preparation](/docs/concepts/runtime-preparation.md)
- [Session compilation](/docs/concepts/session-compilation.md)
- [Session lifecycle](/docs/concepts/session-lifecycle.md)
- [Session mental model](/docs/concepts/session.md)
- [Session traces](/docs/concepts/session-traces.md)
- [Sidecar lifecycle](/docs/concepts/sidecars.md)
- [Signals and streams](/docs/concepts/signals-and-streams.md)
- [Source selection](/docs/concepts/source-selection.md)
- [System capture](/docs/concepts/system-capture.md)
- [Timing and clocks](/docs/concepts/timing-and-clocks.md)
- [Transcription integration boundary](/docs/concepts/transcription-integration.md)

## Lifecycle

- [Build, prepare, and start](/docs/lifecycle/build-prepare-start.md)
- [Cancellation and rollback](/docs/lifecycle/cancellation-and-rollback.md)
- [Running ownership](/docs/lifecycle/running.md)
- [Stop, drain, and finalization](/docs/lifecycle/stop-drain-finalize.md)
- [Terminal outcomes](/docs/lifecycle/terminal-outcomes.md)

## Task guides

- [Author a connector](/docs/guides/connectors.md)
- [Build and load a native extension](/docs/guides/extensions.md)
- [Capture a desktop application](/docs/how-to/capture-application.md)
- [Capture application and microphone stems](/docs/how-to/capture-app-and-mic.md)
- [Capture system audio](/docs/how-to/capture-system-audio.md)
- [Capture the default microphone](/docs/how-to/capture-microphone.md)
- [Choose crate features](/docs/how-to/choose-features.md)
- [Choose route capacity and loss policy](/docs/how-to/configure-route-policy.md)
- [Configure connector secrets](/docs/how-to/configure-connector-secrets.md)
- [Connect named operator ports](/docs/how-to/connect-operator-ports.md)
- [Encode and decode Opus](/docs/how-to/opus-roundtrip.md)
- [Fan out one source](/docs/how-to/fan-out-source.md)
- [Host a managed-process sidecar](/docs/how-to/host-sidecar.md)
- [Implement an asynchronous operator](/docs/how-to/implement-operator.md)
- [Implement an endpoint driver](/docs/how-to/implement-endpoint.md)
- [Inject external PCM](/docs/how-to/inject-external-pcm.md)
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md)
- [Instrument a Session](/docs/how-to/instrument-session.md)
- [Map source time into the Session timeline](/docs/how-to/map-source-time.md)
- [Observe permission without prompting](/docs/how-to/observe-permission.md)
- [Operate a Session through C](/docs/how-to/use-c-session-api.md)
- [Poll audio without unbounded buffering](/docs/how-to/poll-audio.md)
- [Prepare resources before start](/docs/how-to/prepare-session.md)
- [Record independent stems](/docs/how-to/record-stems.md)
- [Return generated PCM through a bridge](/docs/how-to/return-generated-audio.md)
- [Run protocol checks](/docs/how-to/run-protocol-checks.md)
- [Run the transcription example](/docs/how-to/run-transcription.md)
- [Select a process-scoped source](/docs/how-to/select-process-source.md)
- [Stop a Session and inspect failures](/docs/how-to/stop-session.md)
- [Test connector conformance](/docs/how-to/test-connector-conformance.md)

## Reference

- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Capture API](/docs/reference/capture.md)
- [Connector API](/docs/reference/connectors.md)
- [Endpoint API](/docs/reference/endpoints.md)
- [Frame and lineage API](/docs/reference/frames.md)
- [Graph and route contracts](/docs/reference/graph.md)
- [Lifecycle evidence index](/docs/reference/lifecycle-evidence.md)
- [Native extension API](/docs/reference/native-extensions.md)
- [Observation API](/docs/reference/observations.md)
- [Opus codec API](/docs/reference/codec.md)
- [Recording API](/docs/reference/recording.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [Session API](/docs/reference/session.md)
- [Test evidence index](/docs/reference/test-evidence.md)
- [Timing API](/docs/reference/timing.md)

## Configuration reference

- [Configuration reference](/docs/reference/configuration.md)
- [Feature flags](/docs/reference/features.md)

## Protocol reference

- [C ABI reference](/docs/reference/c-abi.md)
- [Protocol surface index](/docs/reference/protocol-surface.md)
- [Sidecar protocol](/docs/reference/sidecar-protocol.md)

## Error reference

- [Error code index](/docs/reference/error-codes.md)
- [Capture failures](/docs/errors/capture.md)
- [Connector failures](/docs/errors/connectors.md)
- [Endpoint failures](/docs/errors/endpoints.md)
- [Extension and ABI failures](/docs/errors/extensions-and-abi.md)
- [Frame and codec failures](/docs/errors/frames-and-codec.md)
- [Graph and signal failures](/docs/errors/graph-and-signals.md)
- [Recording failures](/docs/errors/recording.md)
- [Runtime and sidecar failures](/docs/errors/runtime-and-sidecars.md)
- [Session failures](/docs/errors/session.md)

## Troubleshooting

- [A capture source disappears](/docs/troubleshooting/source-loss.md)
- [A conformance check cannot find external fixtures](/docs/troubleshooting/conformance-fixtures.md)
- [A connector is not ready](/docs/troubleshooting/connector-readiness.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)
- [A native-capture build fails](/docs/troubleshooting/native-build.md)
- [A recording is incomplete](/docs/troubleshooting/recording-incomplete.md)
- [A sidecar misses a deadline](/docs/troubleshooting/sidecar-deadline.md)
- [External PCM input is saturated](/docs/troubleshooting/external-pcm.md)
- [Frames or signals are dropped](/docs/troubleshooting/drops-and-saturation.md)
- [No application audio arrives](/docs/troubleshooting/no-application-audio.md)
- [No microphone audio arrives](/docs/troubleshooting/no-microphone-audio.md)
- [Opus conversion fails](/docs/troubleshooting/opus.md)
- [Permission state is denied or unobservable](/docs/troubleshooting/permission-state.md)
- [Session fails before start](/docs/troubleshooting/session-start.md)
- [Session stop reports component failures](/docs/troubleshooting/session-stop.md)
- [Session trace validation fails](/docs/troubleshooting/session-trace.md)
- [Timestamps diverge or discontinuities appear](/docs/troubleshooting/timing.md)
- [Transcription process evidence is missing](/docs/troubleshooting/transcription-evidence.md)

## Best practices

- [Keep qualification claims scoped](/docs/best-practices/evidence-boundaries.md)
- [Keep realtime callbacks bounded](/docs/best-practices/realtime-boundaries.md)
- [Load extensions from trusted absolute paths](/docs/best-practices/native-extension-trust.md)
- [Preserve source identity](/docs/best-practices/source-identity.md)
- [Size bounded routes from observations](/docs/best-practices/route-sizing.md)
- [Treat stop outcomes as data](/docs/best-practices/terminal-outcomes.md)
- [Use finite connector retry budgets](/docs/best-practices/connector-retries.md)

## Platforms

- [Linux capture](/docs/platform/linux.md)
- [Permission ownership](/docs/platform/permissions.md)
- [Windows capture](/docs/platform/windows.md)
- [macOS capture](/docs/platform/macos.md)

## Compatibility

- [Platform support and evidence](/docs/platform/compatibility.md)
- [Compatibility and evidence](/docs/compatibility/README.md)

## Internals

- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [Architecture overview](/docs/architecture/overview.md)
- [Asynchronous signal lane](/docs/internals/async-signal-lane.md)
- [Memory ownership and buffer pools](/docs/internals/memory-ownership.md)
- [Platform backend boundary](/docs/internals/platform-backends.md)
- [Realtime audio lane](/docs/internals/realtime-audio-lane.md)
- [Runtime planner](/docs/internals/runtime-planner.md)

## Security

- [Security boundaries](/docs/security/boundaries.md)

## Terminology

- [Glossary](/docs/glossary.md)

## Releases

- [Release and version information](/RELEASE_NOTES.md)

## Evidence boundary

This page was verified against Git snapshot `3b7b970f6598239e5d435b60c8d132a955a1886c` and these primary files:

- `Cargo.toml:1-180` (`DIRECT`)
- `src/lib.rs:1-1129` (`DIRECT`)

A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.
