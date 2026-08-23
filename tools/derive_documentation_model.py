#!/usr/bin/env python3
"""Derive PocketStation capabilities, journeys, terminology, and page work queue.

The semantic names and boundaries in this file are reviewed synthesis. Source
hashes and line ranges are always loaded from the frozen repository manifest.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / ".doc-intel"


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def write_jsonl(path: Path, records: list[dict[str, Any]]) -> None:
    with path.open("w") as handle:
        for record in records:
            handle.write(json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n")


FILES = {record["path"]: record for record in read_jsonl(DB / "repository-manifest.jsonl")}
SYMBOLS = read_jsonl(DB / "symbol-manifest.jsonl")
SYMBOLS_BY_NAME: dict[str, list[dict[str, Any]]] = {}
for symbol in SYMBOLS:
    SYMBOLS_BY_NAME.setdefault(symbol["name"], []).append(symbol)


def evidence(*paths: str, classification: str = "DIRECT") -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    for path in paths:
        record = FILES[path]
        dossier = json.loads((ROOT / record["dossier"]).read_text())
        result.append({
            "path": path,
            "content_hash": record["sha256"],
            "lines": [1, dossier["line_count"]],
            "symbol": None,
            "classification": classification,
        })
    return result


CAPABILITY_SPECS = [
    ("CAP-001", "Install and feature-select the crate", "Add PocketStation to a Cargo package and choose native capture, contracts-only, conformance, or internal test features.", ["repository", "release"], ["Cargo.toml", ".github/workflows/ci.yml"], True, True),
    ("CAP-002", "Declare a Session", "Describe sources, operators, endpoints, streams, and recording routes before runtime preparation.", ["session"], ["src/session/declaration/draft.rs", "src/session/declaration/spec.rs"], True, True),
    ("CAP-003", "Select and resolve capture sources", "Discover capture candidates and resolve application, process, device, and system queries to stable source identities.", ["capture"], ["src/capture/query.rs", "src/capture/selection.rs", "src/session/declaration/selector.rs"], True, True),
    ("CAP-004", "Capture application audio", "Prepare application-scoped capture through the platform backend selected for the current target.", ["capture", "platform"], ["src/capture/capture_owner.rs", "src/capture/platform/macos/session_backend.rs", "src/capture/platform/windows/session_backend.rs", "src/capture/platform/linux/session_backend.rs"], True, True),
    ("CAP-005", "Capture microphone audio", "Select the default or identified input device and open native microphone capture.", ["capture", "platform"], ["src/capture/query.rs", "src/capture/platform/macos/input.rs", "src/capture/platform/windows/session_backend.rs", "src/capture/platform/linux/session_backend.rs"], True, True),
    ("CAP-006", "Capture system audio", "Represent and open platform system-loopback capture where the selected backend implements it.", ["capture", "platform"], ["src/capture/query.rs", "src/capture/platform/macos/loopback.rs", "src/capture/platform/windows/session_backend.rs", "src/capture/platform/linux/session_backend.rs"], True, True),
    ("CAP-007", "Observe permission and source lifecycle", "Query non-prompting authorization where observable and receive source-generation, loss, and permission-epoch changes.", ["capture", "platform"], ["src/capture/authorization.rs", "src/capture/events.rs", "src/capture/lifecycle_registry.rs"], True, True),
    ("CAP-008", "Preserve frame identity and lineage", "Carry source, stream, stem, route, clock, sequence, generation, and derivation identity with audio frames.", ["frame"], ["src/frame/identity.rs", "src/frame/lineage.rs", "src/frame/audio.rs"], True, True),
    ("CAP-009", "Map clocks and correct drift", "Map source timestamps into a Session timeline and estimate or correct clock drift.", ["timing"], ["src/timing/timeline_mapping.rs", "src/timing/clock_drift.rs", "src/timing/clock_correction.rs"], True, True),
    ("CAP-010", "Compile Session declarations", "Validate declarations, resolve bindings, and lower a Session specification into an executable plan.", ["session", "graph"], ["src/session/compile/mod.rs", "src/session/compile/bindings.rs", "src/graph/compile/resolve.rs"], False, True),
    ("CAP-011", "Prepare runtime resources", "Prepare source and endpoint runtimes while preserving the mapping back to declaration identities.", ["session", "runtime"], ["src/session/prepare/mod.rs", "src/session/prepare/prepared.rs", "src/session/prepare/mappings.rs"], True, True),
    ("CAP-012", "Start, cancel, stop, and finalize a Session", "Coordinate Session startup, rollback, cancellation, steady-state ownership, stop, and terminal outcomes.", ["session", "runtime"], ["src/session/lifecycle/engine.rs", "src/session/lifecycle/running.rs", "src/session/lifecycle/rollback.rs"], True, True),
    ("CAP-013", "Route realtime audio", "Deliver pooled audio frames through independent fixed-capacity routes governed by explicit edge policy.", ["runtime", "graph"], ["src/runtime/audio/router.rs", "src/runtime/audio/runner.rs", "src/graph/ports.rs"], True, True),
    ("CAP-014", "Poll bounded audio batches", "Consume routed audio from the built-in polled-audio endpoint through bounded batch leases and receipts.", ["endpoint", "session"], ["src/endpoint/polled_audio.rs", "src/endpoint/polled_audio_driver.rs"], True, True),
    ("CAP-015", "Record aligned multistem output", "Configure stems and finalize per-stem outcomes through the recording endpoint lifecycle.", ["recording", "session"], ["src/recording/config.rs", "src/recording/endpoint.rs", "src/recording/writer.rs"], True, True),
    ("CAP-016", "Describe graph contracts", "Declare typed ports, media capabilities, partition safety, copy, loss, delivery, and observability policy.", ["graph"], ["src/graph/ports.rs", "src/graph/partition.rs", "src/graph/spec.rs"], True, True),
    ("CAP-017", "Implement asynchronous operators", "Register operator factories that consume and emit named typed signals on the asynchronous execution lane.", ["graph", "runtime"], ["src/graph/signal/operator.rs", "src/runtime/signal/operator.rs"], True, True),
    ("CAP-018", "Carry typed signals", "Represent audio-adjacent text, event, binary, metric, and custom-schema payloads with timing and lineage.", ["graph", "runtime"], ["src/graph/signal/spec.rs", "src/graph/signal/envelope.rs", "src/graph/signal/payload.rs"], True, True),
    ("CAP-019", "Bridge asynchronous output into audio", "Return generated PCM from asynchronous processing through an explicit bounded audio reentry bridge.", ["runtime", "graph"], ["src/runtime/bridge/audio.rs", "src/runtime/audio/executor.rs"], True, True),
    ("CAP-020", "Implement endpoint drivers", "Prepare, start, receive, cancel, and finalize destinations behind the endpoint driver contract.", ["endpoint", "session"], ["src/endpoint/contract.rs", "src/endpoint/runtime.rs", "src/endpoint/registry.rs"], True, True),
    ("CAP-021", "Declare connector manifests and configuration", "Describe connector identity, ports, configuration schema, secrets, and delivery policy without embedding a provider protocol in Core.", ["connector"], ["src/connector/manifest.rs", "src/connector/configuration.rs", "src/secret.rs"], True, True),
    ("CAP-022", "Run connector workers", "Supervise connector delivery, acknowledgement, readiness, cancellation, drain, and abort while reporting retry attempts and typed retryability.", ["connector", "endpoint"], ["src/connector/worker/supervisor.rs", "src/connector/worker/driver.rs", "src/connector/readiness.rs", "src/connector/error.rs", "src/connector/observations.rs"], True, True),
    ("CAP-023", "Load native extension libraries", "Validate and load a versioned native library, acquire registrations, and retain executable ownership for their lifetime.", ["native_extension", "abi"], ["src/native_extension/library.rs", "src/native_extension/mod.rs", "src/abi/executable_extension.rs"], True, True),
    ("CAP-024", "Use the versioned C ABI", "Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.", ["abi"], ["include/pocketstation.h", "src/abi/session/abi.rs", "src/abi/codec.rs"], True, True),
    ("CAP-025", "Host managed-process sidecars", "Exchange bounded protocol messages with a child process under explicit deadlines and lifecycle states.", ["runtime", "connector"], ["src/runtime/lifecycle/sidecar_host.rs", "src/runtime/lifecycle/sidecar_protocol.rs", "src/connector/sidecar.rs"], True, True),
    ("CAP-026", "Observe Session metrics and events", "Read route, source, operator, sidecar, endpoint, drop, latency, queue, and terminal observations.", ["session", "runtime", "capture", "connector", "endpoint"], ["src/session/lifecycle/observations.rs", "src/session/lifecycle/events.rs", "src/runtime/signal/observations.rs"], True, True),
    ("CAP-027", "Record and validate Session traces", "Persist lifecycle trace records and validate their structural and terminal consistency.", ["session"], ["src/session/lifecycle/trace.rs"], True, True),
    ("CAP-028", "Inject external PCM", "Acquire bounded input buffers and write externally produced PCM through the source extension lifecycle.", ["session", "capture"], ["src/session/extensions/audio_input/mod.rs", "src/session/extensions/audio_input/buffer.rs", "src/session/extensions/audio_input/source.rs"], True, True),
    ("CAP-029", "Encode and decode Opus", "Configure stateful Opus encoders and decoders and convert between PocketStation audio frames and packets.", ["codec"], ["src/codec/encoder.rs", "src/codec/decoder.rs", "src/codec/profile.rs"], True, True),
    ("CAP-030", "Classify public failures", "Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.", ["error_code", "session", "capture", "graph", "endpoint", "recording", "connector", "abi", "codec", "frame", "native_extension", "runtime"], ["src/error_code.rs", "src/session/error_code.rs", "src/recording/error_code.rs"], True, True),
    ("CAP-031", "Validate protocol and conformance boundaries", "Check ABI layout, cross-language behavior, connector vectors, and protocol compatibility against versioned fixtures.", ["abi", "connector", "repository", "conformance"], ["src/conformance.rs", "tests/protocol_compatibility.rs", "tests/abi_session_c_conformance.rs"], True, True),
    ("CAP-032", "Build and publish repository artifacts", "Run architecture, protocol, package, platform, and release checks used by the repository publication workflow.", ["release", "repository"], [".github/workflows/ci.yml", ".github/workflows/publish.yml", "scripts/publish.sh"], False, True),
    ("CAP-033", "Integrate transcription processing", "Use the repository-owned Whisper example to send captured stems to an external transcription process with evidence output.", ["integration"], ["examples/whisper-transcribe/src/lib.rs", "examples/whisper-transcribe/src/process_evidence.rs"], True, True),
]


CAPABILITIES = [
    {
        "capability_id": identifier,
        "name": name,
        "description": description,
        "domains": domains,
        "guide_required": guide_required,
        "troubleshooting_applicable": troubleshooting,
        "status": "analyzed",
        "evidence": evidence(*paths),
    }
    for identifier, name, description, domains, paths, guide_required, troubleshooting in CAPABILITY_SPECS
]


def journey(identifier: str, name: str, audience: str, outcome: str, capabilities: list[str], steps: list[str], paths: list[str]) -> dict[str, Any]:
    return {
        "journey_id": identifier, "name": name, "audience": audience, "outcome": outcome,
        "capability_ids": capabilities, "steps": steps, "status": "analyzed", "evidence": evidence(*paths),
    }


JOURNEYS = [
    journey("JOURNEY-001", "Reach first captured frames", "Rust application developer", "Build a Session that captures an application and microphone and polls their independent frames.", ["CAP-001", "CAP-002", "CAP-004", "CAP-005", "CAP-014", "CAP-012"], ["install", "declare", "route", "start", "poll", "stop"], ["examples/product_quickstart.rs"]),
    journey("JOURNEY-002", "Select a durable source", "Capture application developer", "Discover and resolve a source selector while preserving identity and source-generation changes.", ["CAP-003", "CAP-007", "CAP-008"], ["discover", "select", "resolve", "observe"], ["src/capture/query.rs", "src/capture/selection.rs"]),
    journey("JOURNEY-003", "Record separate stems", "Media application developer", "Record independent source stems and inspect finalization outcomes after Session stop.", ["CAP-002", "CAP-015", "CAP-012"], ["configure root", "label stems", "start", "stop", "inspect outcome"], ["examples/product_quickstart.rs", "src/recording/writer.rs"]),
    journey("JOURNEY-004", "Add an asynchronous operator", "Signal-processing developer", "Declare typed ports, implement an operator factory, and route its output.", ["CAP-016", "CAP-017", "CAP-018", "CAP-010"], ["define manifest", "register", "connect", "compile", "run"], ["examples/operator-consumer/src/lib.rs", "src/graph/signal/operator.rs"]),
    journey("JOURNEY-005", "Return generated audio", "Speech-to-speech developer", "Bridge asynchronous PCM output back into the bounded audio lane.", ["CAP-017", "CAP-019", "CAP-013"], ["declare bridge", "prepare", "write", "observe saturation"], ["src/runtime/bridge/audio.rs"]),
    journey("JOURNEY-006", "Author a connector", "Connector developer", "Declare a connector manifest and run its endpoint worker under finite delivery and shutdown policy.", ["CAP-020", "CAP-021", "CAP-022", "CAP-026", "CAP-031"], ["declare", "configure", "prepare", "deliver", "acknowledge", "drain", "conform"], ["examples/connector_authoring.rs", "src/connector/worker/driver.rs"]),
    journey("JOURNEY-007", "Load a compiled extension", "Native integration developer", "Load a trusted absolute library path and import its registrations transactionally.", ["CAP-023", "CAP-024", "CAP-030"], ["build library", "canonicalize path", "load entrypoint", "acquire registrations", "retain receipt"], ["src/native_extension/library.rs"]),
    journey("JOURNEY-008", "Bind through C", "C or managed-SDK implementer", "Create and operate a Session through ABI handles, status codes, and versioned callbacks.", ["CAP-024", "CAP-030", "CAP-031"], ["declare", "compile", "prepare", "start", "observe", "stop", "release"], ["include/pocketstation.h", "tests/abi_session_c_success_conformance.c"]),
    journey("JOURNEY-009", "Host an out-of-process worker", "Managed-runtime integrator", "Spawn a sidecar and enforce bounded messages, deadlines, cancellation, and terminal state.", ["CAP-025", "CAP-022", "CAP-026"], ["configure process", "start", "exchange", "cancel or drain", "inspect state"], ["src/runtime/lifecycle/sidecar_host.rs"]),
    journey("JOURNEY-010", "Diagnose a running Session", "Application operator", "Correlate events, metrics, trace records, stable error codes, and terminal outcomes.", ["CAP-026", "CAP-027", "CAP-030", "CAP-012"], ["subscribe", "snapshot", "trace", "stop", "classify"], ["src/session/lifecycle/observations.rs", "src/session/lifecycle/trace.rs"]),
    journey("JOURNEY-011", "Handle platform permission", "Desktop application developer", "Perform non-prompting observation, own the prompt UX, and treat source opening as authoritative.", ["CAP-004", "CAP-005", "CAP-006", "CAP-007"], ["observe", "prompt in host", "prepare source", "handle typed result"], ["src/capture/authorization.rs", "src/capture/platform/mod.rs"]),
    journey("JOURNEY-012", "Inject external PCM", "Audio integration developer", "Acquire bounded buffers, write PCM, and observe source runtime outcomes.", ["CAP-028", "CAP-008", "CAP-013"], ["configure", "acquire", "write", "route", "cancel"], ["src/session/extensions/audio_input/mod.rs"]),
    journey("JOURNEY-013", "Encode and decode a stream", "Codec user", "Configure Opus state and convert audio frames to packets and back.", ["CAP-029", "CAP-008", "CAP-030"], ["choose profile", "create encoder", "encode", "create decoder", "decode"], ["src/codec/encoder.rs", "src/codec/decoder.rs"]),
    journey("JOURNEY-014", "Validate an integration", "Repository maintainer", "Run protocol, ABI, connector, package, and example checks at the frozen source revision.", ["CAP-031", "CAP-032"], ["select suite", "provide external fixtures", "execute", "record scope"], [".github/workflows/ci.yml", "scripts/check_protocol.sh"]),
    journey("JOURNEY-015", "Transcribe captured stems", "Applied AI developer", "Run the repository transcription example and preserve process evidence for its external boundary.", ["CAP-004", "CAP-005", "CAP-014", "CAP-033"], ["configure", "capture", "poll", "submit", "persist evidence"], ["examples/whisper-transcribe/src/main.rs", "examples/whisper-transcribe/src/process_evidence.rs"]),
]


def defining_symbol(name: str) -> str:
    public = [symbol for symbol in SYMBOLS_BY_NAME.get(name, []) if symbol["public_api"]]
    choices = public or SYMBOLS_BY_NAME.get(name, [])
    return choices[0]["symbol_id"] if choices else "not_found"


TERM_SPECS = [
    ("Session", "Session", "Session", "A declaration-time owner for sources, routes, operators, endpoints, and recording intent.", [], ["session runtime", "context"], ["RunningSession", "SessionSpec"]),
    ("source", "Source", "source", "A declaration that identifies where a Session obtains a signal.", ["capture source"], ["input thing"], ["stem", "stream"]),
    ("stem", "StemId", "stem", "A source-aware branch identity retained through routing and recording.", [], ["channel"], ["source", "lineage"]),
    ("stream", "Stream", "stream", "A declaration-time typed connection origin used to compose Session routes.", [], ["track"], ["signal", "stem"]),
    ("lineage", "FrameLineage", "lineage", "Immutable identity and sequence metadata carried with a frame or signal.", ["provenance"], [], ["source", "stem", "clock domain"]),
    ("clock domain", "ClockDomainId", "clock domain", "An identity that states which clock produced a timestamp.", [], [], ["Session timeline", "lineage"]),
    ("Session timeline", "SessionTimelineOrigin", "Session timeline", "The common monotonic time origin used to correlate Session work.", [], ["wall clock"], ["clock domain"]),
    ("route", "RouteId", "route", "A compiled delivery path from an output to an independent consumer.", ["edge"], [], ["backpressure", "loss policy"]),
    ("backpressure", "BackpressurePolicy", "backpressure", "The declared response when a bounded route cannot accept work immediately.", [], [], ["loss policy", "capacity"]),
    ("loss policy", "LossPolicy", "loss policy", "The declared rule for data loss at a constrained edge.", ["drop policy"], [], ["backpressure"]),
    ("copy policy", "CopyPolicy", "copy policy", "The declared ownership or copying rule for an edge crossing.", [], [], ["route"]),
    ("operator", "AsyncOperatorFactory", "operator", "An extension that transforms or produces typed signals on the asynchronous lane.", ["processing stage"], ["filter"], ["signal", "named port"]),
    ("endpoint", "EndpointDriverFactory", "endpoint", "A destination driver prepared and finalized through the Session lifecycle.", ["destination"], [], ["connector", "route"]),
    ("connector", "ConnectorManifest", "connector", "A typed endpoint integration contract with configuration, delivery, readiness, and failure policy.", [], ["provider plugin"], ["endpoint", "sidecar"]),
    ("native extension", "NativeExtensionLibrary", "native extension", "A compiled dynamic library that contributes versioned source, operator, or endpoint registrations.", [], ["plugin"], ["C ABI"]),
    ("sidecar", "SidecarProcessSpec", "sidecar", "A managed child process participating through a bounded lifecycle protocol.", [], ["service"], ["connector"]),
    ("signal", "SignalEnvelope", "signal", "A typed asynchronous payload with specification, timing, and lineage.", [], ["message"], ["audio frame", "operator"]),
    ("audio frame", "AudioFrame", "audio frame", "A pooled PCM buffer paired with sample specification and frame lineage.", ["frame"], [], ["signal", "buffer pool"]),
    ("buffer pool", "AudioBufferPool", "buffer pool", "A fixed-capacity owner that supplies reusable audio storage.", [], [], ["audio frame"]),
    ("audio reentry", "SessionAudioReentryMetrics", "audio reentry", "The bounded crossing that returns asynchronously generated PCM to the audio lane.", ["Bridge"], [], ["operator"]),
    ("Session declaration", "SessionSpec", "Session declaration", "The immutable specification produced before compilation and runtime preparation.", ["Session spec"], [], ["Session", "compiled Session"]),
    ("compiled Session", "CompiledSession", "compiled Session", "A validated and resolved Session plan ready for resource preparation.", [], [], ["Session declaration", "prepared Session"]),
    ("prepared Session", "PreparedSession", "prepared Session", "A Session whose source and endpoint resources have been prepared but not started.", [], [], ["compiled Session", "running Session"]),
    ("running Session", "RunningSession", "running Session", "The runtime owner used to observe and stop active Session execution.", [], [], ["prepared Session", "terminal outcome"]),
    ("terminal outcome", "SessionTerminalOutcome", "terminal outcome", "The structured result of Session stop and component finalization.", [], ["exit status"], ["running Session"]),
    ("observation", "SessionMetricsSnapshot", "observation", "A metrics, event, or trace value exposed without redefining runtime behavior.", ["metric"], ["telemetry guarantee"], ["Session trace"]),
    ("Session trace", "SessionTrace", "Session trace", "A persisted sequence of lifecycle trace records with validation support.", [], ["log file"], ["observation"]),
    ("source generation", "SourceGeneration", "source generation", "A monotonic identity revision used when an observed source changes incarnation.", [], [], ["source", "lineage"]),
    ("permission epoch", "PermissionEpoch", "permission epoch", "A revision associated with authorization state observation.", [], [], ["source generation"]),
    ("readiness", "ConnectorReadinessPolicy", "readiness", "The connector policy governing readiness observations before delivery.", [], ["health"], ["connector"]),
    ("retryability", "ConnectorRetryability", "retryability", "The connector failure classification that tells application policy whether the same operation may be attempted again, must wait for reconfiguration, or must not be retried.", [], ["retry budget", "infinite retry"], ["connector"]),
    ("C ABI", "PksSessionStatus", "C ABI", "The versioned C representation of Session and extension operations.", [], ["Rust API"], ["native extension"]),
    ("conformance", "conformance", "conformance", "Executable evidence that a boundary matches a versioned contract under stated conditions.", [], ["qualification"], ["C ABI"]),
]


TERMINOLOGY = {
    "status": "reviewed",
    "terms": [
        {
            "canonical_name": canonical, "code_spelling": code, "human_spelling": human,
            "definition": definition, "aliases": aliases, "forbidden_aliases": forbidden,
            "first_defining_symbol": defining_symbol(code), "related_concepts": related,
        }
        for canonical, code, human, definition, aliases, forbidden, related in TERM_SPECS
    ],
}


PAGES: list[dict[str, Any]] = []


def add_page(page_id: str, title: str, path: str, doc_class: str, gate: int, domains: list[str], capabilities: list[str], evidence_paths: list[str]) -> None:
    PAGES.append({
        "page_id": page_id, "title": title, "path": path, "doc_class": doc_class, "gate": gate,
        "domains": domains, "capability_ids": capabilities, "evidence": evidence(*evidence_paths), "status": "pending",
    })


# Gate 7: orientation, concepts, lifecycle, platforms, and internals.
GATE7 = [
    ("DOC-000", "PocketStation", "README.md", "overview", ["all"], [item[0] for item in CAPABILITY_SPECS], ["Cargo.toml", "src/lib.rs", "examples/product_quickstart.rs"]),
    ("DOC-001", "PocketStation documentation", "docs/README.md", "overview", ["all"], ["CAP-001", "CAP-002"], ["Cargo.toml", "src/lib.rs"]),
    ("DOC-002", "Install PocketStation", "docs/getting-started/installation.md", "getting-started", ["repository", "release"], ["CAP-001"], ["Cargo.toml"]),
    ("DOC-003", "Rust quickstart", "docs/getting-started/rust-quickstart.md", "getting-started", ["session", "capture", "recording"], ["CAP-002", "CAP-004", "CAP-005", "CAP-014", "CAP-015"], ["examples/product_quickstart.rs"]),
    ("DOC-004", "Run the examples", "docs/getting-started/examples.md", "getting-started", ["repository", "integration"], ["CAP-001", "CAP-031", "CAP-033"], ["Cargo.toml", "examples/product_quickstart.rs"]),
    ("DOC-005", "Platform prerequisites", "docs/getting-started/platform-prerequisites.md", "getting-started", ["capture", "platform"], ["CAP-004", "CAP-005", "CAP-006", "CAP-007"], ["Cargo.toml", "src/capture/platform/mod.rs"]),
    ("DOC-006", "Session mental model", "docs/concepts/session.md", "concept", ["session"], ["CAP-002"], ["src/session/declaration/draft.rs"]),
    ("DOC-007", "Source selection", "docs/concepts/source-selection.md", "concept", ["capture"], ["CAP-003"], ["src/capture/query.rs", "src/capture/selection.rs"]),
    ("DOC-008", "Application capture", "docs/concepts/application-capture.md", "concept", ["capture", "platform"], ["CAP-004"], ["src/capture/capture_owner.rs"]),
    ("DOC-009", "Microphone capture", "docs/concepts/microphone-capture.md", "concept", ["capture", "platform"], ["CAP-005"], ["src/capture/authorization.rs"]),
    ("DOC-010", "System capture", "docs/concepts/system-capture.md", "concept", ["capture", "platform"], ["CAP-006"], ["src/capture/query.rs"]),
    ("DOC-011", "Permissions and source lifecycle", "docs/concepts/permissions-and-source-lifecycle.md", "concept", ["capture", "platform"], ["CAP-007"], ["src/capture/authorization.rs", "src/capture/events.rs"]),
    ("DOC-012", "Frame identity and lineage", "docs/concepts/frame-lineage.md", "concept", ["frame"], ["CAP-008"], ["src/frame/lineage.rs"]),
    ("DOC-013", "Timing and clocks", "docs/concepts/timing-and-clocks.md", "concept", ["timing"], ["CAP-009"], ["src/timing/timeline_mapping.rs", "src/timing/clock_drift.rs"]),
    ("DOC-014", "Session compilation", "docs/concepts/session-compilation.md", "concept", ["session", "graph"], ["CAP-010"], ["src/session/compile/mod.rs"]),
    ("DOC-015", "Runtime preparation", "docs/concepts/runtime-preparation.md", "concept", ["session", "runtime"], ["CAP-011"], ["src/session/prepare/mod.rs"]),
    ("DOC-016", "Session lifecycle", "docs/concepts/session-lifecycle.md", "concept", ["session", "runtime"], ["CAP-012"], ["src/session/lifecycle/engine.rs", "src/session/lifecycle/running.rs"]),
    ("DOC-017", "Realtime routing", "docs/concepts/realtime-routing.md", "concept", ["runtime", "graph"], ["CAP-013"], ["src/runtime/audio/router.rs"]),
    ("DOC-018", "Polled audio", "docs/concepts/polled-audio.md", "concept", ["endpoint", "session"], ["CAP-014"], ["src/endpoint/polled_audio_driver.rs"]),
    ("DOC-019", "Multistem recording", "docs/concepts/multistem-recording.md", "concept", ["recording", "session"], ["CAP-015"], ["src/recording/endpoint.rs", "src/recording/writer.rs"]),
    ("DOC-020", "Graph contracts", "docs/concepts/graph-contracts.md", "concept", ["graph"], ["CAP-016"], ["src/graph/ports.rs"]),
    ("DOC-021", "Asynchronous operators", "docs/concepts/async-operators.md", "concept", ["graph", "runtime"], ["CAP-017"], ["src/graph/signal/operator.rs"]),
    ("DOC-022", "Signals and streams", "docs/concepts/signals-and-streams.md", "concept", ["graph", "runtime"], ["CAP-018"], ["src/graph/signal/envelope.rs", "src/session/declaration/typed_stream.rs"]),
    ("DOC-023", "Audio reentry", "docs/concepts/audio-reentry.md", "concept", ["runtime", "graph"], ["CAP-019"], ["src/runtime/bridge/audio.rs"]),
    ("DOC-024", "Endpoint lifecycle", "docs/concepts/endpoints.md", "concept", ["endpoint", "session"], ["CAP-020"], ["src/endpoint/contract.rs", "src/endpoint/runtime.rs"]),
    ("DOC-025", "Connector model", "docs/concepts/connectors.md", "concept", ["connector"], ["CAP-021"], ["src/connector/manifest.rs", "src/connector/configuration.rs"]),
    ("DOC-026", "Connector worker lifecycle", "docs/concepts/connector-workers.md", "concept", ["connector", "endpoint"], ["CAP-022"], ["src/connector/worker/supervisor.rs"]),
    ("DOC-027", "Native extension libraries", "docs/concepts/native-extensions.md", "concept", ["native_extension", "abi"], ["CAP-023"], ["src/native_extension/library.rs"]),
    ("DOC-028", "C ABI ownership", "docs/concepts/c-abi-ownership.md", "concept", ["abi"], ["CAP-024"], ["include/pocketstation.h", "src/abi/session/handle.rs"]),
    ("DOC-029", "Sidecar lifecycle", "docs/concepts/sidecars.md", "concept", ["runtime", "connector"], ["CAP-025"], ["src/runtime/lifecycle/sidecar_host.rs"]),
    ("DOC-030", "Observations and metrics", "docs/concepts/observability.md", "concept", ["session", "runtime", "capture", "connector", "endpoint"], ["CAP-026"], ["src/session/lifecycle/observations.rs"]),
    ("DOC-031", "Session traces", "docs/concepts/session-traces.md", "concept", ["session"], ["CAP-027"], ["src/session/lifecycle/trace.rs"]),
    ("DOC-032", "External PCM input", "docs/concepts/external-pcm.md", "concept", ["session", "capture"], ["CAP-028"], ["src/session/extensions/audio_input/mod.rs"]),
    ("DOC-033", "Opus codec state", "docs/concepts/opus-codec.md", "concept", ["codec"], ["CAP-029"], ["src/codec/encoder.rs", "src/codec/decoder.rs"]),
    ("DOC-034", "Error and status model", "docs/concepts/error-model.md", "concept", ["error_code", "session", "capture", "graph", "endpoint", "recording", "connector", "abi", "codec", "frame", "native_extension", "runtime"], ["CAP-030"], ["src/error_code.rs", "src/session/error_code.rs"]),
    ("DOC-035", "Conformance and qualification", "docs/concepts/conformance.md", "concept", ["abi", "connector", "repository", "conformance"], ["CAP-031"], ["src/conformance.rs", "tests/protocol_compatibility.rs"]),
    ("DOC-036", "Release evidence boundary", "docs/concepts/release-evidence.md", "concept", ["release", "repository"], ["CAP-032"], [".github/workflows/publish.yml", "scripts/publish.sh"]),
    ("DOC-037", "Transcription integration boundary", "docs/concepts/transcription-integration.md", "concept", ["integration"], ["CAP-033"], ["examples/whisper-transcribe/src/lib.rs"]),
    ("DOC-038", "Build, prepare, and start", "docs/lifecycle/build-prepare-start.md", "lifecycle", ["session", "runtime"], ["CAP-002", "CAP-010", "CAP-011", "CAP-012"], ["src/session/lifecycle/start_contract.rs"]),
    ("DOC-039", "Running ownership", "docs/lifecycle/running.md", "lifecycle", ["session", "runtime"], ["CAP-012", "CAP-026"], ["src/session/lifecycle/running.rs"]),
    ("DOC-040", "Cancellation and rollback", "docs/lifecycle/cancellation-and-rollback.md", "lifecycle", ["session", "runtime"], ["CAP-012"], ["src/session/lifecycle/rollback.rs"]),
    ("DOC-041", "Stop, drain, and finalization", "docs/lifecycle/stop-drain-finalize.md", "lifecycle", ["session", "runtime", "recording", "connector", "endpoint"], ["CAP-012", "CAP-015", "CAP-020", "CAP-022"], ["src/session/lifecycle/running.rs", "src/endpoint/runtime.rs"]),
    ("DOC-042", "Terminal outcomes", "docs/lifecycle/terminal-outcomes.md", "lifecycle", ["session"], ["CAP-012", "CAP-030"], ["src/session/lifecycle/events.rs"]),
    ("DOC-043", "Platform support and evidence", "docs/platform/compatibility.md", "compatibility", ["platform", "capture"], ["CAP-004", "CAP-005", "CAP-006", "CAP-007", "CAP-031"], ["src/capture/platform/mod.rs", ".github/workflows/ci.yml"]),
    ("DOC-044", "macOS capture", "docs/platform/macos.md", "platform", ["platform", "capture"], ["CAP-004", "CAP-005", "CAP-006", "CAP-007"], ["src/capture/platform/macos/mod.rs"]),
    ("DOC-045", "Windows capture", "docs/platform/windows.md", "platform", ["platform", "capture"], ["CAP-004", "CAP-005", "CAP-006", "CAP-007"], ["src/capture/platform/windows/mod.rs"]),
    ("DOC-046", "Linux capture", "docs/platform/linux.md", "platform", ["platform", "capture"], ["CAP-004", "CAP-005", "CAP-006", "CAP-007"], ["src/capture/platform/linux/mod.rs"]),
    ("DOC-047", "Permission ownership", "docs/platform/permissions.md", "platform", ["platform", "capture"], ["CAP-007"], ["src/capture/authorization.rs"]),
    ("DOC-048", "Architecture overview", "docs/architecture/overview.md", "internals", ["all"], ["CAP-010", "CAP-012", "CAP-013", "CAP-018"], ["src/lib.rs", "src/runtime/mod.rs"]),
    ("DOC-049", "Runtime planner", "docs/internals/runtime-planner.md", "internals", ["session", "graph", "runtime"], ["CAP-010", "CAP-011"], ["src/graph/compile/plan.rs", "src/session/compile/compiled.rs"]),
    ("DOC-050", "Realtime audio lane", "docs/internals/realtime-audio-lane.md", "internals", ["runtime", "frame"], ["CAP-008", "CAP-013"], ["src/runtime/audio/runner.rs", "src/frame/pool.rs"]),
    ("DOC-051", "Asynchronous signal lane", "docs/internals/async-signal-lane.md", "internals", ["runtime", "graph"], ["CAP-017", "CAP-018"], ["src/runtime/signal/edge.rs", "src/runtime/signal/operator.rs"]),
    ("DOC-052", "Memory ownership and buffer pools", "docs/internals/memory-ownership.md", "internals", ["frame", "runtime"], ["CAP-008", "CAP-013"], ["src/frame/audio.rs", "src/frame/pool.rs"]),
    ("DOC-053", "Platform backend boundary", "docs/internals/platform-backends.md", "internals", ["capture", "platform"], ["CAP-004", "CAP-005", "CAP-006"], ["src/capture/platform/mod.rs"]),
    ("DOC-054", "ABI and conformance model", "docs/internals/abi-conformance.md", "internals", ["abi", "repository"], ["CAP-024", "CAP-031"], ["src/abi/mod.rs", "src/conformance.rs"]),
    ("DOC-055", "Glossary", "docs/glossary.md", "glossary", ["all"], [item[0] for item in CAPABILITY_SPECS], ["src/lib.rs"]),
    ("DOC-056", "Cargo features and build surfaces", "docs/concepts/cargo-features.md", "concept", ["repository", "release"], ["CAP-001"], ["Cargo.toml", "build.rs"]),
    ("DOC-057", "Release and version information", "RELEASE_NOTES.md", "release", ["release", "repository"], ["CAP-001", "CAP-032"], ["Cargo.toml", ".github/workflows/publish.yml"]),
]
for spec in GATE7:
    add_page(*spec[:4], 7, *spec[4:])

# Gate 8: concrete developer tasks.
GATE8 = [
    ("GUIDE-001", "Capture a desktop application", "docs/how-to/capture-application.md", ["capture", "platform"], ["CAP-003", "CAP-004", "CAP-007"], ["examples/product_quickstart.rs"]),
    ("GUIDE-002", "Capture the default microphone", "docs/how-to/capture-microphone.md", ["capture", "platform"], ["CAP-005", "CAP-007"], ["examples/product_quickstart.rs"]),
    ("GUIDE-003", "Capture application and microphone stems", "docs/how-to/capture-app-and-mic.md", ["capture", "session", "frame"], ["CAP-002", "CAP-004", "CAP-005", "CAP-008"], ["examples/product_quickstart.rs"]),
    ("GUIDE-004", "Select a process-scoped source", "docs/how-to/select-process-source.md", ["capture"], ["CAP-003", "CAP-007"], ["src/session/declaration/selector.rs"]),
    ("GUIDE-005", "Observe permission without prompting", "docs/how-to/observe-permission.md", ["capture", "platform"], ["CAP-007"], ["src/capture/authorization.rs", "src/lib.rs"]),
    ("GUIDE-006", "Poll audio without unbounded buffering", "docs/how-to/poll-audio.md", ["endpoint", "session"], ["CAP-014", "CAP-013"], ["examples/product_quickstart.rs", "src/endpoint/polled_audio_driver.rs"]),
    ("GUIDE-007", "Fan out one source", "docs/how-to/fan-out-source.md", ["session", "runtime", "graph"], ["CAP-002", "CAP-013", "CAP-016"], ["src/session/declaration/draft.rs", "src/runtime/audio/router.rs"]),
    ("GUIDE-008", "Choose route capacity and loss policy", "docs/how-to/configure-route-policy.md", ["graph", "runtime"], ["CAP-013", "CAP-016"], ["src/graph/ports.rs"]),
    ("GUIDE-009", "Record independent stems", "docs/how-to/record-stems.md", ["recording", "session"], ["CAP-015"], ["examples/product_quickstart.rs"]),
    ("GUIDE-010", "Inspect recording outcomes", "docs/how-to/inspect-recording-outcome.md", ["recording", "session"], ["CAP-015", "CAP-030"], ["src/recording/writer.rs", "src/session/extensions/recording.rs"]),
    ("GUIDE-011", "Implement an asynchronous operator", "docs/how-to/implement-operator.md", ["graph", "runtime"], ["CAP-016", "CAP-017", "CAP-018"], ["examples/operator-consumer/src/lib.rs"]),
    ("GUIDE-012", "Connect named operator ports", "docs/how-to/connect-operator-ports.md", ["graph", "session"], ["CAP-002", "CAP-016", "CAP-017"], ["src/session/declaration/tests/operator_connections.rs"]),
    ("GUIDE-013", "Return generated PCM through a bridge", "docs/how-to/return-generated-audio.md", ["runtime", "graph"], ["CAP-019", "CAP-017"], ["src/runtime/bridge/audio.rs"]),
    ("GUIDE-014", "Implement an endpoint driver", "docs/how-to/implement-endpoint.md", ["endpoint", "session"], ["CAP-020"], ["src/endpoint/contract.rs"]),
    ("GUIDE-015", "Author a connector", "docs/guides/connectors.md", ["connector", "endpoint"], ["CAP-021", "CAP-022", "CAP-031"], ["examples/connector_authoring.rs"]),
    ("GUIDE-016", "Configure connector secrets", "docs/how-to/configure-connector-secrets.md", ["connector"], ["CAP-021", "CAP-030"], ["src/connector/configuration.rs", "src/secret.rs"]),
    ("GUIDE-017", "Test connector conformance", "docs/how-to/test-connector-conformance.md", ["connector", "repository"], ["CAP-022", "CAP-031"], ["tests/connector_portable_semantics.rs"]),
    ("GUIDE-018", "Build and load a native extension", "docs/guides/extensions.md", ["native_extension", "abi"], ["CAP-023", "CAP-024", "CAP-031"], ["tests/fixtures/native_extension_plugin.rs", "src/native_extension/library.rs"]),
    ("GUIDE-019", "Operate a Session through C", "docs/how-to/use-c-session-api.md", ["abi"], ["CAP-024", "CAP-030"], ["tests/abi_session_c_success_conformance.c"]),
    ("GUIDE-020", "Host a managed-process sidecar", "docs/how-to/host-sidecar.md", ["runtime", "connector"], ["CAP-025", "CAP-022"], ["src/runtime/lifecycle/sidecar_host.rs"]),
    ("GUIDE-021", "Instrument a Session", "docs/how-to/instrument-session.md", ["session", "runtime"], ["CAP-026", "CAP-027"], ["src/session/lifecycle/observations.rs", "src/session/lifecycle/trace.rs"]),
    ("GUIDE-022", "Stop a Session and inspect failures", "docs/how-to/stop-session.md", ["session", "runtime", "endpoint", "recording"], ["CAP-012", "CAP-015", "CAP-020", "CAP-030"], ["src/session/lifecycle/running.rs"]),
    ("GUIDE-023", "Inject external PCM", "docs/how-to/inject-external-pcm.md", ["session", "capture"], ["CAP-028"], ["tests/audio_input.rs"]),
    ("GUIDE-024", "Encode and decode Opus", "docs/how-to/opus-roundtrip.md", ["codec", "frame"], ["CAP-029", "CAP-008"], ["tests/codec_opus_roundtrip.rs"]),
    ("GUIDE-025", "Run protocol checks", "docs/how-to/run-protocol-checks.md", ["repository", "release", "abi", "connector", "conformance"], ["CAP-031", "CAP-032"], ["scripts/check_protocol.sh"]),
    ("GUIDE-026", "Run the transcription example", "docs/how-to/run-transcription.md", ["integration", "capture"], ["CAP-033", "CAP-004", "CAP-005"], ["examples/whisper-transcribe/README.md", "examples/whisper-transcribe/src/main.rs"]),
    ("GUIDE-027", "Choose crate features", "docs/how-to/choose-features.md", ["repository", "release"], ["CAP-001"], ["Cargo.toml"]),
    ("GUIDE-028", "Capture system audio", "docs/how-to/capture-system-audio.md", ["capture", "platform"], ["CAP-003", "CAP-006", "CAP-007"], ["src/capture/query.rs", "src/capture/platform/mod.rs"]),
    ("GUIDE-029", "Map source time into the Session timeline", "docs/how-to/map-source-time.md", ["timing", "frame"], ["CAP-008", "CAP-009"], ["src/timing/timeline_mapping.rs"]),
    ("GUIDE-030", "Prepare resources before start", "docs/how-to/prepare-session.md", ["session", "runtime"], ["CAP-010", "CAP-011", "CAP-012"], ["src/session/prepare/mod.rs", "src/session/lifecycle/start_contract.rs"]),
]
for page_id, title, path, domains, caps, paths in GATE8:
    add_page(page_id, title, path, "how-to", 8, domains, caps, paths)

# Gate 9: predictable references; rustdoc remains the symbol-level authority.
GATE9 = [
    ("REF-001", "Rust API reference", "docs/reference/rust-api.md", "reference", ["all"], [item[0] for item in CAPABILITY_SPECS], ["src/lib.rs"]),
    ("REF-002", "Session API", "docs/reference/session.md", "reference", ["session"], ["CAP-002", "CAP-010", "CAP-011", "CAP-012"], ["src/session/mod.rs"]),
    ("REF-003", "Capture API", "docs/reference/capture.md", "reference", ["capture", "platform"], ["CAP-003", "CAP-004", "CAP-005", "CAP-006", "CAP-007"], ["src/capture/mod.rs"]),
    ("REF-004", "Frame and lineage API", "docs/reference/frames.md", "reference", ["frame"], ["CAP-008"], ["src/frame/mod.rs"]),
    ("REF-005", "Timing API", "docs/reference/timing.md", "reference", ["timing"], ["CAP-009"], ["src/timing/mod.rs"]),
    ("REF-006", "Graph and route contracts", "docs/reference/graph.md", "reference", ["graph"], ["CAP-016", "CAP-017", "CAP-018", "CAP-019"], ["src/graph/mod.rs"]),
    ("REF-007", "Endpoint API", "docs/reference/endpoints.md", "reference", ["endpoint"], ["CAP-014", "CAP-020"], ["src/endpoint/mod.rs"]),
    ("REF-008", "Recording API", "docs/reference/recording.md", "reference", ["recording"], ["CAP-015"], ["src/recording/mod.rs"]),
    ("REF-009", "Connector API", "docs/reference/connectors.md", "reference", ["connector"], ["CAP-021", "CAP-022"], ["src/connector/mod.rs"]),
    ("REF-010", "Native extension API", "docs/reference/native-extensions.md", "reference", ["native_extension", "abi"], ["CAP-023"], ["src/native_extension/mod.rs"]),
    ("REF-011", "Sidecar protocol", "docs/reference/sidecar-protocol.md", "protocol-reference", ["runtime", "connector"], ["CAP-025"], ["src/runtime/lifecycle/sidecar_protocol.rs"]),
    ("REF-012", "C ABI reference", "docs/reference/c-abi.md", "protocol-reference", ["abi"], ["CAP-024", "CAP-031"], ["include/pocketstation.h"]),
    ("REF-013", "Configuration reference", "docs/reference/configuration.md", "config-reference", ["all"], ["CAP-001", "CAP-002", "CAP-013", "CAP-015", "CAP-021", "CAP-022", "CAP-025", "CAP-028", "CAP-029"], ["Cargo.toml", "src/connector/configuration.rs"]),
    ("REF-014", "Feature flags", "docs/reference/features.md", "config-reference", ["repository", "release"], ["CAP-001", "CAP-031"], ["Cargo.toml"]),
    ("REF-015", "Compatibility and evidence", "docs/compatibility/README.md", "compatibility", ["all"], ["CAP-031", "CAP-032"], ["docs/compatibility/c-abi-v1.baseline", ".github/workflows/ci.yml"]),
    ("REF-016", "Opus codec API", "docs/reference/codec.md", "reference", ["codec"], ["CAP-029"], ["src/codec/mod.rs"]),
    ("REF-017", "Observation API", "docs/reference/observations.md", "reference", ["session", "runtime", "capture", "connector", "endpoint"], ["CAP-026", "CAP-027"], ["src/session/lifecycle/observations.rs"]),
    ("REF-018", "Error code index", "docs/reference/error-codes.md", "error-reference", ["all"], ["CAP-030"], ["src/error_code.rs", "src/session/error_code.rs", "src/recording/error_code.rs"]),
    ("REF-019", "Behavior evidence index", "docs/reference/behavior-evidence.md", "reference", ["all"], [item[0] for item in CAPABILITY_SPECS], ["src/lib.rs"]),
    ("REF-020", "Protocol surface index", "docs/reference/protocol-surface.md", "protocol-reference", ["all"], ["CAP-021", "CAP-022", "CAP-023", "CAP-024", "CAP-025", "CAP-031"], ["include/pocketstation.h", "src/connector/mod.rs"]),
    ("REF-021", "Test evidence index", "docs/reference/test-evidence.md", "reference", ["all"], ["CAP-031", "CAP-032"], [".github/workflows/ci.yml", "tests/public_api_boundary.rs"]),
    ("REF-022", "Lifecycle evidence index", "docs/reference/lifecycle-evidence.md", "reference", ["all"], ["CAP-012", "CAP-020", "CAP-022", "CAP-025"], ["src/session/lifecycle/mod.rs", "src/endpoint/runtime.rs"]),
]
for spec in GATE9:
    add_page(*spec[:4], 9, *spec[4:])

# Gate 10: failure references, symptom-driven troubleshooting, and supported recommendations.
GATE10 = [
    ("ERR-001", "Session failures", "docs/errors/session.md", "error-reference", ["session"], ["CAP-010", "CAP-011", "CAP-012", "CAP-030"], ["src/session/error.rs", "src/session/lifecycle/events.rs"]),
    ("ERR-002", "Capture failures", "docs/errors/capture.md", "error-reference", ["capture", "platform"], ["CAP-003", "CAP-004", "CAP-005", "CAP-006", "CAP-007", "CAP-030"], ["src/capture/mod.rs"]),
    ("ERR-003", "Graph and signal failures", "docs/errors/graph-and-signals.md", "error-reference", ["graph", "runtime"], ["CAP-016", "CAP-017", "CAP-018", "CAP-019", "CAP-030"], ["src/graph/node.rs", "src/runtime/signal/error.rs"]),
    ("ERR-004", "Endpoint failures", "docs/errors/endpoints.md", "error-reference", ["endpoint"], ["CAP-014", "CAP-020", "CAP-030"], ["src/endpoint/runtime.rs"]),
    ("ERR-005", "Recording failures", "docs/errors/recording.md", "error-reference", ["recording"], ["CAP-015", "CAP-030"], ["src/recording/writer.rs"]),
    ("ERR-006", "Connector failures", "docs/errors/connectors.md", "error-reference", ["connector"], ["CAP-021", "CAP-022", "CAP-030"], ["src/connector/error.rs"]),
    ("ERR-007", "Extension and ABI failures", "docs/errors/extensions-and-abi.md", "error-reference", ["native_extension", "abi"], ["CAP-023", "CAP-024", "CAP-030"], ["src/native_extension/library.rs", "src/abi/session/error.rs"]),
    ("ERR-008", "Frame and codec failures", "docs/errors/frames-and-codec.md", "error-reference", ["frame", "codec"], ["CAP-008", "CAP-029", "CAP-030"], ["src/frame/audio.rs", "src/codec/mod.rs"]),
    ("ERR-009", "Runtime and sidecar failures", "docs/errors/runtime-and-sidecars.md", "error-reference", ["runtime"], ["CAP-013", "CAP-019", "CAP-025", "CAP-030"], ["src/runtime/lifecycle/sidecar_host.rs", "src/runtime/signal/error.rs"]),
    ("TRBL-001", "Session fails before start", "docs/troubleshooting/session-start.md", "troubleshooting", ["session", "runtime", "graph"], ["CAP-002", "CAP-010", "CAP-011", "CAP-012", "CAP-030"], ["src/session/compile/error.rs", "src/session/prepare/error.rs"]),
    ("TRBL-002", "No application audio arrives", "docs/troubleshooting/no-application-audio.md", "troubleshooting", ["capture", "platform", "session"], ["CAP-003", "CAP-004", "CAP-007", "CAP-026", "CAP-030"], ["src/capture/events.rs", "src/capture/observations.rs"]),
    ("TRBL-003", "No microphone audio arrives", "docs/troubleshooting/no-microphone-audio.md", "troubleshooting", ["capture", "platform", "session"], ["CAP-005", "CAP-007", "CAP-026", "CAP-030"], ["src/capture/authorization.rs", "src/capture/observations.rs"]),
    ("TRBL-004", "Permission state is denied or unobservable", "docs/troubleshooting/permission-state.md", "troubleshooting", ["capture", "platform"], ["CAP-004", "CAP-005", "CAP-006", "CAP-007", "CAP-030"], ["src/capture/authorization.rs"]),
    ("TRBL-005", "A capture source disappears", "docs/troubleshooting/source-loss.md", "troubleshooting", ["capture", "session"], ["CAP-003", "CAP-007", "CAP-008", "CAP-012", "CAP-030"], ["src/capture/events.rs", "src/capture/lifecycle_registry.rs"]),
    ("TRBL-006", "Frames or signals are dropped", "docs/troubleshooting/drops-and-saturation.md", "troubleshooting", ["runtime", "graph", "endpoint"], ["CAP-013", "CAP-014", "CAP-016", "CAP-017", "CAP-018", "CAP-019", "CAP-026"], ["src/runtime/audio/router.rs", "src/runtime/signal/edge.rs"]),
    ("TRBL-007", "A recording is incomplete", "docs/troubleshooting/recording-incomplete.md", "troubleshooting", ["recording", "session"], ["CAP-015", "CAP-012", "CAP-030"], ["src/recording/writer.rs"]),
    ("TRBL-008", "A connector is not ready", "docs/troubleshooting/connector-readiness.md", "troubleshooting", ["connector", "endpoint"], ["CAP-020", "CAP-021", "CAP-022", "CAP-026", "CAP-030"], ["src/connector/readiness.rs", "src/connector/status.rs"]),
    ("TRBL-009", "A native extension does not load", "docs/troubleshooting/native-extension-load.md", "troubleshooting", ["native_extension", "abi"], ["CAP-023", "CAP-024", "CAP-030"], ["src/native_extension/library.rs"]),
    ("TRBL-010", "A sidecar misses a deadline", "docs/troubleshooting/sidecar-deadline.md", "troubleshooting", ["runtime", "connector"], ["CAP-025", "CAP-022", "CAP-026", "CAP-030"], ["src/runtime/lifecycle/sidecar_host.rs"]),
    ("TRBL-011", "Opus conversion fails", "docs/troubleshooting/opus.md", "troubleshooting", ["codec", "frame"], ["CAP-029", "CAP-008", "CAP-030"], ["src/codec/encoder.rs", "src/codec/decoder.rs"]),
    ("TRBL-012", "Session stop reports component failures", "docs/troubleshooting/session-stop.md", "troubleshooting", ["session", "endpoint", "recording", "connector", "runtime"], ["CAP-012", "CAP-015", "CAP-020", "CAP-022", "CAP-025", "CAP-030"], ["src/session/lifecycle/running.rs"]),
    ("TRBL-013", "A conformance check cannot find external fixtures", "docs/troubleshooting/conformance-fixtures.md", "troubleshooting", ["abi", "connector", "repository", "conformance"], ["CAP-031", "CAP-032"], ["tests/connector_portable_semantics.rs", "scripts/check_protocol.sh"]),
    ("TRBL-014", "Transcription process evidence is missing", "docs/troubleshooting/transcription-evidence.md", "troubleshooting", ["integration"], ["CAP-033", "CAP-030"], ["examples/whisper-transcribe/src/process_evidence.rs"]),
    ("TRBL-015", "A native-capture build fails", "docs/troubleshooting/native-build.md", "troubleshooting", ["repository", "release", "platform", "capture"], ["CAP-001", "CAP-004", "CAP-005", "CAP-006"], ["Cargo.toml", "build.rs"]),
    ("TRBL-016", "Timestamps diverge or discontinuities appear", "docs/troubleshooting/timing.md", "troubleshooting", ["timing", "frame", "capture"], ["CAP-007", "CAP-008", "CAP-009", "CAP-026"], ["src/timing/clock_drift.rs", "src/capture/timeline.rs"]),
    ("TRBL-017", "Session trace validation fails", "docs/troubleshooting/session-trace.md", "troubleshooting", ["session"], ["CAP-027", "CAP-026", "CAP-030"], ["src/session/lifecycle/trace.rs"]),
    ("TRBL-018", "External PCM input is saturated", "docs/troubleshooting/external-pcm.md", "troubleshooting", ["session", "capture", "runtime"], ["CAP-028", "CAP-013", "CAP-026", "CAP-030"], ["src/session/extensions/audio_input/buffer.rs"]),
    ("BEST-001", "Size bounded routes from observations", "docs/best-practices/route-sizing.md", "best-practice", ["runtime", "graph"], ["CAP-013", "CAP-016", "CAP-026"], ["src/runtime/audio/router.rs", "src/session/lifecycle/observations.rs"]),
    ("BEST-002", "Keep realtime callbacks bounded", "docs/best-practices/realtime-boundaries.md", "best-practice", ["runtime", "frame"], ["CAP-013", "CAP-008"], ["src/runtime/audio/runner.rs", "scripts/lint/check-architecture-constraints.sh"]),
    ("BEST-003", "Preserve source identity", "docs/best-practices/source-identity.md", "best-practice", ["capture", "frame"], ["CAP-003", "CAP-007", "CAP-008"], ["src/capture/identity.rs", "src/frame/lineage.rs"]),
    ("BEST-004", "Treat stop outcomes as data", "docs/best-practices/terminal-outcomes.md", "best-practice", ["session", "recording", "endpoint", "connector"], ["CAP-012", "CAP-015", "CAP-020", "CAP-022", "CAP-030"], ["src/session/lifecycle/events.rs"]),
    ("BEST-005", "Honor connector retryability", "docs/best-practices/connector-retries.md", "best-practice", ["connector"], ["CAP-021", "CAP-022", "CAP-026", "CAP-030"], ["src/connector/error.rs", "src/connector/observations.rs"]),
    ("BEST-006", "Load extensions from trusted absolute paths", "docs/best-practices/native-extension-trust.md", "best-practice", ["native_extension", "abi"], ["CAP-023", "CAP-024"], ["src/native_extension/library.rs"]),
    ("BEST-007", "Keep qualification claims scoped", "docs/best-practices/evidence-boundaries.md", "best-practice", ["platform", "repository", "release"], ["CAP-004", "CAP-005", "CAP-006", "CAP-031", "CAP-032"], [".github/workflows/ci.yml", "src/capture/platform/mod.rs"]),
    ("SEC-001", "Security boundaries", "docs/security/boundaries.md", "security", ["connector", "native_extension", "abi", "runtime"], ["CAP-021", "CAP-023", "CAP-024", "CAP-025"], ["src/secret.rs", "src/native_extension/library.rs"]),
]
for page_id, title, path, doc_class, domains, caps, paths in GATE10:
    add_page(page_id, title, path, doc_class, 10, domains, caps, paths)


write_jsonl(DB / "capabilities.jsonl", CAPABILITIES)
write_jsonl(DB / "user-journeys.jsonl", JOURNEYS)
write_jsonl(DB / "page-manifest.jsonl", PAGES)
(DB / "terminology.json").write_text(json.dumps(TERMINOLOGY, indent=2, sort_keys=True) + "\n")
print(f"capabilities={len(CAPABILITIES)} journeys={len(JOURNEYS)} pages={len(PAGES)} terms={len(TERMINOLOGY['terms'])}")
