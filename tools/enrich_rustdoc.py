#!/usr/bin/env python3
"""Add declaration-specific rustdoc to every undocumented public Rust item.

The documentation compiler's private rustdoc artifact supplies exact source
spans.  The public/private join in ``symbol-manifest.jsonl`` determines the
intentional API set.  This script only edits records that are both public and
missing source documentation, and it refuses to run when their source files no
longer match the frozen manifest.
"""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import re
from collections import defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / ".doc-intel"
IMPL_OWNERS: dict[int, str] = {}
SOURCE_DOC_IMPROVEMENTS = {
    "pocketstation::codec::encoder::OpusApplication": "Selects the Opus encoder mode used to tune speech or general audio.",
    "OpusConfig::channels": "Selects the mono or stereo channel layout accepted by the encoder.",
    "OpusConfig::application": "Selects the Opus application mode used when the encoder is created.",
    "pocketstation::graph::signal::spec::SignalClass::Event": "Carries discrete event payloads described by an `EventFormat`.",
    "pocketstation::graph::signal::spec::SignalClass::Binary": "Carries an opaque binary payload described by a `BinaryFormat`.",
    "anchored": "Creates a sample timeline whose first buffer starts at the supplied nonzero monotonic timestamp.",
    "expected": "Returns the expected value when a compilation diagnostic compares two values.",
    "actual": "Returns the observed value when a compilation diagnostic compares two values.",
    "diagnostic": "Converts a Session compiler failure into stable language-neutral location and comparison fields.",
}

STRUCT_DOCS = {
    "Operator": "Declares one operator instance, including its stable operator identity and validated node configuration.",
    "TypedOperator": "Binds an operator declaration to its typed input and output ports so graph connections preserve signal specifications.",
    "SessionCompiler": "Compiles an immutable Session declaration into a validated graph and runtime plan.",
    "CompiledSession": "Owns the validated Session specification and declarations produced by compilation.",
    "AudioInputBuffer": "Leases bounded PCM storage from an external-audio input until the caller submits or releases it.",
    "SourceOutputIdentity": "Identifies one declared source output by source type, output port, and stream identity.",
    "SourceCancellation": "Exposes the cancellation state observed by a running external source driver.",
    "SourceEmission": "Carries one external-source emission with its output-port identity and signal envelope.",
    "SessionTraceRecorder": "Collects ordered lifecycle records and writes the trace artifact during Session finalization.",
    "SessionTrace": "Contains the ordered lifecycle records read from a Session trace artifact.",
    "SessionTraceTerminal": "Records the terminal Session disposition and component failures stored in a trace.",
    "SessionTraceValidation": "Reports the validated identity and record count of a parsed Session trace.",
    "ExtensionSignal": "Owns one signal payload used by the native-extension conformance fixtures.",
    "PksSessionUtf8": "Borrows a UTF-8 byte range across the C Session ABI as a pointer and length.",
    "SignalTiming": "Carries a signal timestamp, clock domain, and timing semantics without rewriting source lineage.",
    "PermissionEpoch": "Identifies the permission-observation generation attached to captured lineage.",
    "SignalLineage": "Preserves source, stream, generation, discontinuity, and policy identity across signal processing.",
    "Connector": "Declares a connector endpoint and the manifest-backed configuration used to instantiate it.",
    "RegisteredConnector": "Retains a connector declaration after its factory has been registered with the node registry.",
    "PksExtensionAbiVersion": "Carries the major and minor native-extension ABI versions checked during loading.",
    "PksExtensionPort": "Describes one native-extension port across the C ABI, including direction and signal metadata.",
    "SignalContinuityObservation": "Reports sequence or timestamp continuity observed for one signal stream.",
    "SignalContinuityTracker": "Tracks sequence and timing progress so discontinuities remain observable.",
    "ConnectorSecret": "Owns a connector secret with redacted diagnostics and byte clearing on explicit reset or drop.",
    "ConnectorConfigurationField": "Declares one typed connector configuration field and its validation constraints.",
    "ConnectorConfigurationSchema": "Validates connector configuration values against the manifest-declared field set.",
    "SignalEnvelope": "Carries a typed signal payload together with timing, lineage, continuity, and terminal metadata.",
    "PksExtensionCallbacks": "Defines the optional function table through which a native extension prepares, runs, stops, and releases instances.",
    "PksExtensionLibrary": "Owns a loaded native-extension library and the registrations imported from its validated descriptor.",
    "PksExtensionSignalView": "Borrows one signal payload and metadata for delivery into a native-extension callback.",
    "PksExtensionSignalBuffer": "Provides bounded extension-owned storage for a signal returned through the native ABI.",
    "LocalSourceProvider": "Discovers and resolves capture sources through the target platform backend.",
    "ConnectorCapability": "Declares a capability advertised by a connector manifest.",
    "ConnectorRequirement": "Declares a host or configuration requirement that must be satisfied before connector use.",
    "ConnectorAudioMetadata": "Carries source, stream, timing, and format metadata beside a connector audio record.",
    "Pipeline": "Builds typed operator connections on a Session while preserving port and signal contracts.",
    "FrameLineage": "Preserves source, stream, sequence, clock, generation, and discontinuity identity for an audio frame.",
    "ConnectorErrorCode": "Carries the stable external error code exported for a connector failure.",
    "AudioBufferPool": "Owns fixed-capacity reusable audio slots and reports acquisition pressure without allocating per frame.",
    "RuntimePlanner": "Validates the graph and produces the bounded runtime execution and memory plan.",
    "AudioCaps": "Declares the sample formats, channel layouts, and rates accepted by an audio port.",
    "ResolvedEdge": "Binds one compiled graph edge to its resolved source, destination, and contract.",
    "GraphIr": "Stores the resolved nodes, edges, and topological order used by runtime planning.",
    "AsyncOperatorWorker": "Owns the asynchronous operator task, typed I/O, cancellation, and terminal join result.",
    "StemLabel": "Stores the validated human-readable label used for one recording stem.",
    "EndpointDriverFinalization": "Reports an endpoint driver's terminal observations and any finalization failure.",
    "PreparedEndpoint": "Owns endpoint resources after preparation and before its runtime driver starts.",
    "RunningEndpoint": "Owns a started endpoint driver until shutdown and finalization complete.",
    "SidecarDeadlines": "Sets finite startup, I/O, shutdown, and reap deadlines for a sidecar process.",
    "SourceGeneration": "Identifies one appearance generation of a capture source across loss and reappearance.",
    "ClockDriftEstimator": "Estimates source-clock drift from accumulated source and Session timing observations.",
    "OutputPortRef": "Names an operator output port used as the origin of a graph connection.",
    "InputPortRef": "Names an operator or endpoint input port used as the target of a graph connection.",
    "Session": "Owns a mutable Session declaration and the host configuration used to compile, prepare, and start it.",
    "RunningSession": "Owns a started Session together with event, polling, recording, trace, and stop resources.",
    "SidecarProtocolLimits": "Sets the maximum sidecar message and buffered-byte sizes enforced by protocol I/O.",
    "SidecarMessage": "Carries one typed control or signal message across the sidecar protocol.",
    "NativeExtensionRegistration": "Identifies one node registration imported transactionally from a native extension.",
    "ClockCorrectionController": "Applies bounded proportional corrections from measured clock offsets without changing lineage.",
    "PlanRunnerCancellation": "Shares a lock-free cancellation flag between the Session owner and the realtime plan runner.",
    "TypedEdgeFanout": "Publishes one immutable signal envelope to the bounded branches of a compiled fan-out edge.",
    "SourceRuntime": "Owns an external source driver's cancellation handle, observations, and terminal worker join.",
    "CaptureBackendSet": "Supplies the application and microphone capture backends used while preparing a Session.",
    "Compiler": "Runs the ordered graph-validation passes that resolve a graph specification into executable IR.",
    "FanOutGroup": "Groups the compiled edges that share one output port as their origin.",
    "FanInGroup": "Groups the compiled edges mixed into one input port.",
    "SessionSpecVersion": "Identifies the major and minor version of the immutable Session declaration schema.",
    "MultistemRecording": "Owns the per-stem recording workers and coordinates their terminal finalization outcome.",
    "TypedEdgePublishReport": "Reports how many fan-out branches accepted or dropped one published signal.",
}


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def words(name: str) -> str:
    """Render a Rust identifier as a restrained English noun phrase."""
    name = re.sub(r"^(?:Pks|PKS)_?", "", name)
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", name)
    name = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1 \2", name)
    value = name.replace("::", " ").replace("_", " ").strip().lower()
    replacements = {
        " id": " identifier", " ids": " identifiers", " ns": " nanoseconds",
        " ms": " milliseconds", " hz": " hertz", " utf8": " UTF-8",
    }
    for old, new in replacements.items():
        if value.endswith(old):
            value = value[: -len(old)] + new
    value = re.sub(r"\bapi\b", "API", value)
    value = re.sub(r"\babi\b", "ABI", value)
    value = re.sub(r"\bpcm\b", "PCM", value)
    value = re.sub(r"\bio\b", "I/O", value)
    return value or "value"


def owner_for(record: dict[str, Any], by_id: dict[str, dict[str, Any]]) -> str:
    if record.get("compiler_id") in IMPL_OWNERS:
        return IMPL_OWNERS[record["compiler_id"]]
    parent = by_id.get(record.get("parent", ""))
    if parent and parent.get("name"):
        return parent["name"]
    parts = record.get("qualified_name", "").split("::")
    if len(parts) >= 2:
        candidate = parts[-2]
        if candidate and candidate not in {
            "pocketstation", "abi", "capture", "codec", "connector", "endpoint",
            "frame", "graph", "native_extension", "recording", "runtime", "session", "timing",
        }:
            return candidate
    return "PocketStation"


def field_doc(name: str, owner: str) -> str:
    exact = {
        "struct_size_bytes": f"Stores the byte size of the `{owner}` ABI structure.",
        "abi_major": f"Stores the major ABI version expected by `{owner}`.",
        "abi_minor": f"Stores the minor ABI version expected by `{owner}`.",
        "registration_context": f"Carries the opaque registration context used by `{owner}` callbacks.",
        "library_context": f"Carries the opaque library context used by `{owner}` callbacks.",
        "instance_context": f"Carries the opaque instance context used by `{owner}` callbacks.",
        "max_payload_bytes": f"Limits payload storage for `{owner}`, in bytes.",
        "reserved": f"Reserves storage for forward-compatible evolution of `{owner}`.",
        "data": f"Carries the data owned or referenced by `{owner}`.",
        "flags": f"Carries the bit flags defined by `{owner}`.",
        "message": f"Carries the diagnostic message reported by `{owner}`.",
        "reason": f"Carries the reason reported by `{owner}`.",
        "expected": f"Records the value expected by `{owner}`.",
        "actual": f"Records the value observed by `{owner}`.",
        "minimum": f"Sets the inclusive minimum accepted by `{owner}`.",
        "maximum": f"Sets the inclusive maximum accepted by `{owner}`.",
        "from": f"Identifies the origin represented by `{owner}`.",
        "to": f"Identifies the destination represented by `{owner}`.",
        "source": f"Carries the source selected for `{owner}`.",
        "output": f"Carries the output produced by `{owner}`.",
        "failure": f"Carries the failure reported by `{owner}`.",
        "outcome": f"Carries the terminal outcome reported by `{owner}`.",
        "observations": f"Carries the observations collected for `{owner}`.",
    }
    if name in exact:
        return exact[name]
    phrase = words(name)
    if name.endswith(("_id", "_ids")) or name == "id":
        return f"Identifies the {phrase} recorded by `{owner}`."
    if name.endswith("_ns"):
        return f"Stores the {phrase.removesuffix(' nanoseconds')} value for `{owner}`, in nanoseconds."
    if name.endswith("_ms"):
        return f"Stores the {phrase.removesuffix(' milliseconds')} value for `{owner}`, in milliseconds."
    if name.endswith("_hz"):
        return f"Stores the {phrase.removesuffix(' hertz')} value for `{owner}`, in hertz."
    if name.endswith(("_bytes", "_byte_count")):
        return f"Stores the {phrase.removesuffix(' bytes')} size for `{owner}`, in bytes."
    if name.endswith("_total"):
        return f"Counts the total number of {words(name[:-6])} observed by `{owner}`."
    if "capacity" in name:
        return f"Sets the {phrase} available to `{owner}`."
    if "depth" in name or "peak" in name:
        return f"Reports the {phrase} observed by `{owner}`."
    if name.startswith(("is_", "has_")) or name.endswith("_enabled") or name in {"enabled", "ready", "required", "terminal"}:
        return f"Indicates whether {phrase} applies to `{owner}`."
    if name.endswith("_count") or name.startswith("number_of_"):
        return f"Stores the number of {words(name.removesuffix('_count'))} represented by `{owner}`."
    if name.endswith("_index") or name == "index":
        return f"Stores the {phrase} used by `{owner}`."
    if name.endswith("_error"):
        return f"Carries the {phrase} reported by `{owner}`."
    if name.endswith("_callback") or name in {
        "prepare", "create", "source_next", "operator_process", "endpoint_consume",
        "request_stop", "finish", "destroy_instance", "destroy_registration",
        "validate_configuration", "acquire_registration",
    }:
        return f"Provides the {phrase} callback used by `{owner}`."
    return f"Stores the {phrase} used by `{owner}`."


def variant_doc(name: str, owner: str) -> str:
    phrase = words(name)
    lower_owner = owner.lower()
    explicit = {
        "Ok": "Indicates that the operation completed successfully.",
        "Success": "Indicates that the operation completed successfully.",
        "Cancelled": "Indicates that the operation was cancelled.",
        "Stopped": "Indicates that the operation stopped normally.",
        "AlreadyStopped": "Indicates that the operation had already stopped.",
        "TimedOut": "Indicates that the operation exceeded its deadline.",
        "PermissionDenied": "Reports that the required permission was denied.",
        "Unsupported": "Reports that the requested operation is unsupported.",
        "Unavailable": "Reports that the requested resource is unavailable.",
        "Empty": "Represents an empty value or collection.",
        "Full": "Reports that bounded capacity is full.",
        "Closed": "Reports that the underlying channel or resource is closed.",
    }
    if name in explicit:
        return explicit[name]
    if any(token in lower_owner for token in ("status", "state", "stage", "phase", "outcome", "disposition", "event", "delivery")):
        return f"Identifies the {phrase} state or stage represented by `{owner}`."
    if any(token in lower_owner for token in ("policy", "mode", "kind", "scope", "direction", "format", "semantics", "level", "requirement", "selector", "retryability")):
        return f"Selects {phrase} behavior for `{owner}`."
    if any(token in lower_owner for token in ("error", "failure")):
        return f"Reported when the owning operation encounters {phrase}."
    return f"Represents the {phrase} alternative defined by `{owner}`."


def function_doc(name: str, owner: str) -> str:
    subject = f"`{owner}`"
    phrase = words(name)
    exact = {
        "new": f"Creates a new {subject}.",
        "try_new": f"Creates a new {subject} after validating its inputs.",
        "ok": f"Creates a successful status value for {subject}.",
        "default": f"Returns the default {subject} value.",
        "fmt": f"Formats {subject} with the requested formatter.",
        "as_str": f"Returns the stable string representation of {subject}.",
        "from": f"Converts the supplied value into {subject}.",
        "get": f"Returns the value held by {subject}.",
        "iter": f"Iterates over the values held by {subject}.",
        "len": f"Returns the number of values held by {subject}.",
        "is_empty": f"Returns whether {subject} contains no values.",
        "drop": f"Releases resources owned by {subject}.",
        "start": f"Starts the lifecycle represented by {subject}.",
        "stop": f"Stops {subject} and returns its terminal result.",
        "cancel": f"Requests cancellation of {subject}.",
        "prepare": f"Prepares resources required by {subject}.",
        "finish": f"Finishes work owned by {subject}.",
        "finalize": f"Finalizes {subject} and reports its outcome.",
        "close": f"Closes {subject} to further work.",
        "open": f"Opens the resource represented by {subject}.",
        "current": f"Returns the current value observed by {subject}.",
        "discover": f"Discovers the resources visible to {subject}.",
        "validate": f"Validates {subject} against its declared contract.",
        "encode": f"Encodes input through {subject}.",
        "decode": f"Decodes input through {subject}.",
        "send": f"Sends a value through {subject}.",
        "try_send": f"Attempts to send a value through {subject} without waiting for capacity.",
        "try_recv": f"Attempts to receive the next value from {subject} without waiting.",
        "recv": f"Receives the next value from {subject}.",
        "observe": f"Returns the current observation exposed by {subject}.",
        "observations": f"Returns the observations exposed by {subject}.",
        "observation": f"Returns the current observation exposed by {subject}.",
        "observation_handle": f"Returns a handle for reading observations from {subject}.",
        "stats": f"Returns the current statistics for {subject}.",
        "snapshot": f"Returns a point-in-time snapshot of {subject}.",
        "connect": f"Connects the requested ports through {subject}.",
        "process": f"Processes an input value through {subject}.",
        "output": f"Returns the output held by {subject}.",
        "input": f"Returns the input held by {subject}.",
        "configuration": f"Returns the configuration held by {subject}.",
        "manifest": f"Returns the manifest held by {subject}.",
        "kind": f"Returns the kind represented by {subject}.",
        "code": f"Returns the stable error or status code represented by {subject}.",
        "message": f"Returns the diagnostic message reported by {subject}.",
        "samples": f"Returns the audio samples held by {subject}.",
        "channels": f"Returns the channel count represented by {subject}.",
        "lineage": f"Returns the frame lineage carried by {subject}.",
        "resolve": f"Resolves {subject} into its validated representation.",
        "matches": f"Returns whether an input satisfies {subject}.",
        "declare": f"Adds the declaration represented by {subject} to its Session.",
        "deliver": f"Delivers the next input through {subject}.",
        "idle": f"Advances {subject} while no input is available.",
        "run": f"Runs {subject} until completion or cancellation.",
        "shutdown": f"Shuts down {subject} according to its lifecycle contract.",
        "request": f"Requests the state transition represented by {subject}.",
        "wait_for_stop": f"Waits until a stop request is visible to {subject}.",
        "read": f"Reads the persisted representation of {subject}.",
        "record": f"Attaches recording output to {subject}.",
        "result": f"Returns the result represented by {subject}.",
        "success": f"Returns whether {subject} completed successfully.",
        "into_parts": f"Consumes {subject} and returns its component values.",
        "to": f"Returns the destination owned by {subject}.",
        "stop_and_join": f"Stops {subject}, joins its worker, and returns the terminal result.",
        "finish_and_join": f"Finishes input to {subject}, joins its worker, and returns the terminal result.",
        "close_and_reap": f"Closes {subject} and reaps its child process.",
        "new_with_output_channels": f"Creates {subject} with the supplied output channels.",
        "process_ready": f"Processes the ready inputs for {subject}.",
        "declares_multistem_recording": f"Returns whether {subject} declares multistem recording.",
    }
    if name in exact:
        return exact[name]
    accessors = (
        "id", "session_id", "source_id", "stream_id", "stem_id", "endpoint_id",
        "connector_id", "operator_id", "route_id", "instance_id", "node_id",
        "sequence_number", "revision", "index", "port_name", "sample_rate_hz",
        "timestamp_ns", "permission_epoch", "signal_spec", "media", "source",
        "failure", "outcome", "stage", "documentation", "required",
    )
    if name in accessors:
        return f"Returns the {phrase} held by {subject}."
    predicates = {
        "runtime_worker_panicked", "cancellation_requested", "joined", "supports",
        "accepts", "contains", "declares_multistem_recording",
    }
    if name in predicates:
        return f"Returns whether {phrase} is true for {subject}."
    if name.startswith(("is_", "has_", "accepts_", "supports_")):
        return f"Returns whether {words(name.removeprefix('is_').removeprefix('has_'))} applies to {subject}."
    if name.startswith("with_"):
        return f"Sets the {words(name[5:])} on {subject} and returns the updated value."
    if name.startswith("set_"):
        return f"Sets the {words(name[4:])} used by {subject}."
    if name.startswith("from_"):
        return f"Creates {subject} from {words(name[5:])}."
    if name.startswith("as_"):
        return f"Borrows {subject} as {words(name[3:])}."
    if name.startswith("into_"):
        return f"Converts {subject} into {words(name[5:])}."
    if name.startswith("to_"):
        return f"Converts {subject} to {words(name[3:])}."
    for prefix, verb in (
        ("add_", "Adds"), ("insert_", "Inserts"), ("register_", "Registers"),
        ("remove_", "Removes"), ("unregister_", "Unregisters"), ("record_", "Records"),
        ("write_", "Writes"), ("read_", "Reads"), ("push_", "Pushes"),
        ("poll_", "Polls for"), ("take_", "Takes"), ("resolve_", "Resolves"),
        ("compile_", "Compiles"), ("validate_", "Validates"), ("cancel_", "Cancels"),
        ("prepare_", "Prepares"), ("start_", "Starts"), ("stop_", "Stops"),
        ("spawn_", "Spawns"), ("join_", "Joins"), ("execute_", "Executes"),
        ("send_", "Sends"), ("receive_", "Receives"), ("publish_", "Publishes"),
        ("observe_", "Records an observation for"), ("plan_", "Plans"),
    ):
        if name.startswith(prefix):
            return f"{verb} {words(name[len(prefix):])} for {subject}."
    if name.startswith("try_"):
        return f"Attempts to {words(name[4:])} through {subject}."
    semantic_actions = {
        "builder": f"Creates a builder for declaring {subject} sources, routes, and endpoints.",
        "capture": f"Declares a capture source on {subject} and returns its Session-scoped handle.",
        "endpoint": f"Declares an endpoint on {subject} and returns its Session-scoped handle.",
        "polled_audio": f"Declares a bounded polled-audio endpoint on {subject}.",
        "receive_sidecar_signal": f"Receives the next sidecar signal owned by {subject}.",
        "discover_sources": "Discovers capture sources available from the local provider.",
        "instantiate": f"Instantiates the runtime node described by {subject}.",
        "register": f"Registers a node definition with {subject} while preserving unique identities.",
        "negotiate": f"Negotiates the compatible media capabilities shared by {subject} and its peer.",
        "connect_with": f"Connects pipeline ports using the supplied edge contract on {subject}.",
        "copy_to_pool": f"Copies the shared frame into storage acquired from the supplied pool for {subject}.",
        "acquire": f"Attempts to acquire an available buffer slot from {subject}.",
        "tick": f"Applies one measured clock offset to {subject} and returns the bounded correction.",
        "request_stop": f"Requests a graceful stop from {subject}.",
        "request_shutdown": f"Requests the selected shutdown mode from {subject}.",
        "join_and_finalize": f"Joins {subject} and returns its finalization outcome.",
        "sidecar_connector_factory": "Creates a connector driver factory backed by the supplied sidecar process.",
        "insert": f"Inserts a typed configuration value into {subject}.",
        "with": f"Returns {subject} with the supplied entry applied.",
        "through": f"Routes the current stream through a declared operator using {subject}.",
        "send_to": f"Routes the current source output to the requested destination through {subject}.",
        "freeze": f"Freezes mutable storage owned by {subject} into its shared immutable form.",
        "mark_discontinuity": f"Marks the next value from {subject} as discontinuous.",
        "flush": f"Flushes pending output from {subject} at the end of a run.",
        "map_payload": f"Transforms the payload held by {subject} while preserving envelope metadata.",
        "report_readiness_success": f"Records a successful readiness probe for {subject}.",
        "create": f"Creates the runtime implementation described by {subject}.",
        "next": f"Produces the next source emission from {subject}.",
        "process_instance": f"Creates {subject} for one exact process instance.",
        "microphone": f"Creates {subject} for the selected microphone device.",
        "microphone_default": f"Creates {subject} for the host default microphone.",
        "observed": f"Creates observed signal timing for {subject}.",
        "out": f"Selects a named output port from {subject}.",
        "in_": f"Selects a named input port from {subject}.",
        "captured_frame_stream": "Wraps the supplied capture receiver as a stream of captured frames.",
        "source_runtime_event_channel": "Creates the bounded sender and receiver used for source runtime events.",
        "session_stop_failure_codes": "Returns every stable failure code carried by a Session stop result.",
        "discover_input_sources_native": "Discovers microphone input sources through the native macOS backend.",
        "session": "Runs the conformance assertions for the Session contract.",
        "connector": f"Declares a connector endpoint on {subject} with the supplied operator identity and configuration.",
        "engine_builder": f"Borrows the mutable engine builder owned by {subject}.",
        "dispatch_from": f"Routes one lineaged audio frame from the named plan output through {subject}.",
    }
    if name in semantic_actions:
        return semantic_actions[name]
    semantic_properties = {
        "failure_codes", "canonical_path", "registrations", "path", "samples_at_48k", "count",
        "hz", "frame_duration", "bitrate_kbps", "preparation_group", "definition", "async_factory",
        "async_factory_by_operator", "contains", "multiplicity", "rank", "clock", "backpressure",
        "delivery", "loss", "observability", "get_f32", "get_u32", "capacity_signals", "slot_size",
        "acquire_failures", "available_slots", "drift_ppm", "route_context", "session_timeline_origin",
        "sample_format", "delivery_readiness", "class", "role", "accepts", "input_edge", "output_edge",
        "permission", "deadline", "cancellation", "output_roles", "payload", "timing", "derivation",
        "execution_partition", "source_generation", "discontinuity_epoch", "policy_epoch", "upstream_timing",
        "operator_generation", "supports", "component", "operation", "error_class", "event_queue", "route",
        "operator", "derived_route", "source_to_receive_latency", "queue_capacity_signals",
        "queue_depth_signals", "queue_peak_signals", "pool_slots", "joined", "handle", "records",
        "runtime_worker_panicked", "target", "name", "outputs", "inputs", "disposition", "generation",
        "descriptor", "direction", "format", "retryability", "frame_samples_per_channel",
        "metrics_snapshot",
        "version", "major", "minor", "partition", "state", "frames_mixed", "receipt",
        "stems", "typed_edge", "output_root", "open_metadata", "output_pool_exhaustions",
        "source_declarations", "plan_edge_observation_handle", "generated_audio_ingresses",
        "source_failures", "endpoints", "native", "frame_stream_closed", "edge_buffer",
        "connections", "operator_mappings", "spec", "type_str", "source_mappings",
        "recording_receipt", "polled_audio_receipt", "rollback_failures", "error", "browser",
        "frames_pushed", "source_instances", "endpoint_declarations", "lane_underruns",
        "topo_order", "engine", "observation_receipt", "frames_captured",
        "cancellation_requested", "endpoint_failures", "external_source_declarations",
        "frames_emitted", "input_mut", "operators", "event", "finalization_failures",
        "worker_mappings",
    }
    if name in semantic_properties:
        return f"Returns the {phrase} associated with {subject}."
    noun_suffixes = (
        "_id", "_ids", "_name", "_kind", "_state", "_status", "_stage", "_code",
        "_reason", "_policy", "_mode", "_scope", "_spec", "_config", "_configuration",
        "_manifest", "_revision", "_version", "_index", "_count", "_total", "_capacity",
        "_capacity_frames", "_depth", "_depth_frames", "_peak", "_peak_frames", "_timeout",
        "_interval", "_threshold", "_rate", "_pct", "_ns", "_ms", "_hz", "_bytes",
        "_samples", "_frames", "_port", "_ports", "_input", "_inputs", "_output", "_outputs",
        "_source", "_lineage", "_metrics", "_observations", "_outcome", "_failure", "_error",
    )
    noun_names = {
        "application", "capabilities", "constraints", "context", "deprecation", "deprecated",
        "documentation", "edge_contract", "execution", "field", "fields", "frame", "health",
        "identity_strength", "manifest", "media", "metadata", "node", "origin", "readiness",
        "receiver", "recovery", "requirement", "requirements", "safety", "schema", "signal",
        "startup_timeout", "probe_interval", "success_threshold", "failure_threshold", "value",
        "value_kind", "visited", "worker", "writer", "writer_mut", "samples_mut",
    }
    if name in noun_names or name.endswith(noun_suffixes):
        return f"Returns the {phrase} held by {subject}."
    action_verbs = {
        "build": "Builds", "compile": "Compiles", "execute": "Executes",
        "spawn": "Spawns", "join": "Joins", "publish": "Publishes",
        "receive": "Receives", "send": "Sends", "wait": "Waits for",
        "open": "Opens", "close": "Closes", "process": "Processes",
    }
    first, _, remainder = name.partition("_")
    if first in action_verbs:
        target = words(remainder) if remainder else "its owned operation"
        return f"{action_verbs[first]} {target} for {subject}."
    return f"Executes the `{name}` contract for {subject}."


def item_doc(record: dict[str, Any], by_id: dict[str, dict[str, Any]]) -> str:
    name = record["name"]
    kind = record["kind"]
    owner = owner_for(record, by_id)
    phrase = words(name)
    lower = name.lower()
    if kind == "struct_field":
        return field_doc(name, owner)
    if kind == "variant":
        return variant_doc(name, owner)
    if kind == "function":
        return function_doc(name, owner)
    if kind == "module":
        return f"Types and operations for {phrase}."
    if kind == "struct":
        if name in STRUCT_DOCS:
            return STRUCT_DOCS[name]
        if lower.endswith(("error", "failure")):
            return f"Reports a {phrase}."
        if lower.endswith(("config", "configuration", "options", "policy", "spec")):
            subject = phrase.removesuffix(' configuration').removesuffix(' config').removesuffix(' options').removesuffix(' policy').removesuffix(' spec')
            return f"Configures {subject} behavior at its owning API boundary."
        if lower.endswith(("id", "identifier")):
            return f"Uniquely identifies {phrase.removesuffix(' identifier')} within its PocketStation ownership scope."
        if lower.endswith(("observations", "metrics", "snapshot", "stats")):
            return f"Reports the {phrase} collected at an observation boundary."
        if lower.endswith(("outcome", "result", "status")):
            return f"Reports the structured {phrase}."
        if lower.endswith(("builder", "draft")):
            return f"Builds a validated {phrase.removesuffix(' builder').removesuffix(' draft')} declaration."
        if lower.endswith(("manifest", "descriptor", "declaration")):
            return f"Describes the {phrase} contract."
        if lower.endswith(("handle", "lease", "guard")):
            return f"Owns bounded access to {phrase.removesuffix(' handle').removesuffix(' lease').removesuffix(' guard')}."
        roles = (
            (("context",), "Carries the inputs and runtime context required to {subject}."),
            (("factory",), "Constructs {subject} implementations from validated declarations."),
            (("registry",), "Indexes registered {subject} implementations by their stable identities."),
            (("sender", "writer"), "Sends {subject} values across its declared ownership boundary."),
            (("receiver",), "Receives {subject} values across its declared ownership boundary."),
            (("plan",), "Records the compiled execution and resource plan for {subject}."),
            (("mapping",), "Correlates the prepared identities and runtime resources for {subject}."),
            (("record",), "Records one immutable {subject} observation."),
            (("frame",), "Carries one {subject} payload together with its declared metadata."),
            (("contract",), "Declares the validated constraints applied to {subject}."),
            (("bridge",), "Transfers {subject} across the bounded runtime boundary it owns."),
            (("router",), "Routes {subject} according to the compiled edge contracts."),
            (("host",), "Owns the resources and lifecycle for {subject}."),
            (("executor", "runner"), "Executes {subject} according to its compiled plan and cancellation contract."),
            (("telemetry", "summary"), "Reports the counters and terminal facts collected for {subject}."),
            (("receipt",), "Retains the identity and observation access returned for {subject}."),
            (("input",), "Carries typed input for {subject}."),
            (("output",), "Carries typed output from {subject}."),
            (("node",), "Executes the graph-node behavior defined for {subject}."),
            (("source",), "Owns production of {subject} values and its lifecycle state."),
        )
        for suffixes, template in roles:
            if lower.endswith(suffixes):
                subject_name = phrase
                for suffix in suffixes:
                    subject_name = subject_name.removesuffix(f" {suffix}")
                return template.format(subject=subject_name or phrase)
        return f"Carries the typed state and values defined by the `{name}` public contract."
    if kind == "enum":
        if lower.endswith(("error", "failure")):
            return f"Classifies failures reported as {phrase}."
        if lower.endswith(("policy", "mode", "kind", "scope", "direction", "format", "state", "status", "stage", "requirement", "semantics", "level")):
            return f"Selects the {phrase} used by PocketStation."
        if lower.endswith(("event", "outcome", "result", "disposition", "observation")):
            return f"Classifies the observable {phrase}."
        return f"Enumerates the supported {phrase} cases."
    if kind == "trait":
        role = phrase.removesuffix(" factory").removesuffix(" provider").removesuffix(" driver")
        return f"Implement this trait to provide {role} behavior to PocketStation; its methods define the preparation and runtime contract."
    if kind == "type_alias":
        if lower.endswith("callback"):
            action_name = re.sub(r"Callback$", "", re.sub(r"^PksExtension", "", name))
            action = {
                "ValidateConfiguration": "validate extension configuration",
                "Create": "create an extension instance",
                "Prepare": "prepare an extension instance",
                "SourceNext": "produce the next source signal",
                "OperatorProcess": "process an operator input",
                "EndpointConsume": "consume an endpoint input",
                "Stop": "request an extension instance to stop",
                "Finish": "finish extension work",
                "Destroy": "destroy extension-owned context",
                "AcquireRegistration": "acquire an extension registration",
            }.get(action_name, words(action_name))
            return (
                f"Defines the optional C callback used to {action}; pointer validity and "
                "ownership follow the extension ABI contract."
            )
        if lower.endswith("future"):
            return f"Names the future returned by {phrase.removesuffix(' future')} operations."
        return f"Names the {phrase} type used by the public API."
    if kind in {"constant", "assoc_const"}:
        if name == "INITIAL":
            return f"Provides the initial value for `{owner}`."
        if name.startswith("MAX_"):
            return f"Sets the maximum supported {words(name[4:])}."
        if name.endswith("_MAJOR"):
            return f"Defines the major version of {words(name[:-6])}."
        if name.endswith("_MINOR"):
            return f"Defines the minor version of {words(name[:-6])}."
        return f"Defines the public {phrase} value."
    if kind == "assoc_type":
        return f"Specifies the error type returned by `{owner}` operations."
    return f"Documents the public {phrase} API item."


def rustdoc_type_name(value: Any) -> str | None:
    if not isinstance(value, dict):
        return None
    resolved = value.get("resolved_path")
    if isinstance(resolved, dict) and resolved.get("path"):
        return resolved["path"].rsplit("::", 1)[-1]
    for key in ("borrowed_ref", "raw_pointer", "slice", "array"):
        nested = value.get(key)
        if isinstance(nested, dict):
            found = rustdoc_type_name(nested.get("type"))
            if found:
                return found
    for nested in value.values():
        if isinstance(nested, dict):
            found = rustdoc_type_name(nested)
            if found:
                return found
    return None


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="Write source changes; otherwise report the planned edit")
    parser.add_argument(
        "--refresh-field-owners",
        action="store_true",
        help="Replace first-pass payload-field docs with their recovered variant owner",
    )
    parser.add_argument(
        "--export-ledger",
        action="store_true",
        help="Export the generated native descriptions without editing source",
    )
    parser.add_argument(
        "--refresh-generic-functions",
        action="store_true",
        help="Replace first-pass generic function descriptions with semantic descriptions",
    )
    parser.add_argument(
        "--sync-current-generated",
        action="store_true",
        help="Replace or insert generated docs at frozen compiler spans in the current worktree",
    )
    parser.add_argument(
        "--sync-current-json",
        type=Path,
        help=(
            "Insert docs for frozen-public items that remain undocumented in a fresh "
            "private rustdoc JSON build of the current worktree"
        ),
    )
    parser.add_argument(
        "--refresh-current-generated",
        action="store_true",
        help="Replace previously generated source attributes with the current semantic descriptions",
    )
    parser.add_argument(
        "--polish-current-attributes",
        action="store_true",
        help="Repair known first-pass wording in current source documentation attributes",
    )
    args = parser.parse_args()

    records = read_jsonl(DB / "symbol-manifest.jsonl")
    by_id = {record["symbol_id"]: record for record in records}
    missing = [record for record in records if record.get("public_api") and not record.get("source_documented")]
    with gzip.open(DB / "checkpoints" / "rustdoc-private.json.gz", "rt") as handle:
        private = json.load(handle)
    index = private["index"]
    for item in index.values():
        impl = item.get("inner", {}).get("impl")
        if not isinstance(impl, dict):
            continue
        owner = rustdoc_type_name(impl.get("for"))
        if not owner:
            continue
        for child in impl.get("items", []):
            IMPL_OWNERS[int(child)] = owner

    if args.polish_current_attributes:
        changed = 0
        substitutions = (
            (
                re.compile(r'Returns the ([^"\n]+?) associated with (`[^`]+`)\.'),
                r'Returns the \1 held by \2.',
            ),
            (
                re.compile(r'Carries the source associated with (`[^`]+`)\.'),
                r'Carries the source selected for \1.',
            ),
            (
                re.compile(r'Represents the ([^"\n]+?) alternative defined by (`[^`]+`)\.'),
                r'Selects the \1 case of \2.',
            ),
            (
                re.compile(r'Returns the (runtime worker panicked|cancellation requested|joined|supports|accepts|contains) held by (`[^`]+`)\.'),
                r'Returns whether \1 is true for \2.',
            ),
        )
        for source_path in sorted((ROOT / "src").rglob("*.rs")):
            text = source_path.read_text()
            updated = text
            for pattern, replacement in substitutions:
                updated, count = pattern.subn(replacement, updated)
                changed += count
            if updated != text:
                source_path.write_text(updated)
        print(f"source_doc_attributes_polished={changed}")
        return

    by_file: dict[str, list[tuple[int, dict[str, Any], str]]] = defaultdict(list)
    for record in missing:
        item = index[str(record["compiler_id"])]
        span = item.get("span")
        if not span or span.get("filename") != record["source_file"]:
            raise SystemExit(f"span mismatch for {record['symbol_id']}")
        line, column = span["begin"]
        by_file[record["source_file"]].append(((line << 32) | column, record, item_doc(record, by_id)))

    if args.sync_current_json:
        current_private = json.loads(args.sync_current_json.read_text())
        if not current_private.get("includes_private"):
            raise SystemExit("current rustdoc JSON must include private items")
        if current_private.get("format_version") != private.get("format_version"):
            raise SystemExit("current and frozen rustdoc JSON format versions differ")
        current_index = current_private.get("index", {})
        current_by_file: dict[str, list[tuple[int, dict[str, Any], str]]] = defaultdict(list)
        already_documented = 0
        for record in missing:
            current_item = current_index.get(str(record["compiler_id"]))
            if not current_item:
                raise SystemExit(f"current rustdoc item absent for {record['symbol_id']}")
            if current_item.get("name") != record["name"]:
                raise SystemExit(f"current rustdoc name drift for {record['symbol_id']}")
            if next(iter(current_item.get("inner", {})), "unknown") != record["kind"]:
                raise SystemExit(f"current rustdoc kind drift for {record['symbol_id']}")
            if (current_item.get("docs") or "").strip():
                already_documented += 1
                continue
            span = current_item.get("span")
            if not span or span.get("filename") != record["source_file"]:
                raise SystemExit(f"current rustdoc span drift for {record['symbol_id']}")
            line, column = span["begin"]
            current_by_file[record["source_file"]].append(
                ((line << 32) | column, record, item_doc(record, by_id))
            )

        inserted = 0
        for path, entries in sorted(current_by_file.items()):
            source_path = ROOT / path
            text = source_path.read_text()
            lines = text.splitlines(keepends=True)
            starts: list[int] = []
            cursor = 0
            for line in lines:
                starts.append(cursor)
                cursor += len(line)
            seen_offsets: set[int] = set()
            for _sort_key, record, doc in sorted(entries, key=lambda value: value[0], reverse=True):
                span = current_index[str(record["compiler_id"])]["span"]
                line, column = span["begin"]
                offset = starts[line - 1] + column - 1
                if offset in seen_offsets:
                    raise SystemExit(
                        f"duplicate current documentation span at {path}:{line}:{column}"
                    )
                seen_offsets.add(offset)
                excerpt = text[offset : offset + 500]
                if record["name"] not in excerpt:
                    raise SystemExit(
                        f"current source guard failed for {record['symbol_id']} at {path}:{line}:{column}"
                    )
                attribute = f"#[doc = {json.dumps(doc, ensure_ascii=False)}] "
                text = text[:offset] + attribute + text[offset:]
                inserted += 1
            source_path.write_text(text)

        checkpoint = {
            "snapshot": json.loads((DB / "state.json").read_text())["snapshot"],
            "public_records_missing_in_snapshot": len(missing),
            "already_documented_in_current_source": already_documented,
            "public_records_enriched": inserted,
            "files_edited": len(current_by_file),
            "current_rustdoc_json_sha256": sha256(args.sync_current_json.read_bytes()),
            "symbol_ids": sorted(
                record["symbol_id"]
                for entries in current_by_file.values()
                for _sort_key, record, _doc in entries
            ),
        }
        (DB / "checkpoints").mkdir(parents=True, exist_ok=True)
        (DB / "checkpoints" / "rustdoc-enrichment.json").write_text(
            json.dumps(checkpoint, indent=2, sort_keys=True) + "\n"
        )
        print(
            f"current_docs_inserted={inserted} "
            f"already_documented={already_documented} files={len(current_by_file)}"
        )
        return

    if args.sync_current_generated:
        inserted = 0
        skipped_changed = 0
        manifest = {record["path"]: record for record in read_jsonl(DB / "repository-manifest.jsonl")}
        for path in sorted(by_file):
            source_path = ROOT / path
            if sha256(source_path.read_bytes()) != manifest[path]["sha256"]:
                skipped_changed += len(by_file[path])
                continue
            lines = source_path.read_text().splitlines(keepends=True)
            seen: set[tuple[int, int]] = set()
            for _sort_key, record, doc in sorted(by_file[path], key=lambda value: value[0], reverse=True):
                span = index[str(record["compiler_id"])]["span"]
                line, column = span["begin"]
                location = (line, column)
                if location in seen:
                    raise SystemExit(f"duplicate generated documentation span at {path}:{line}:{column}")
                seen.add(location)
                current = lines[line - 1]
                offset = column - 1
                prefix, rest = current[:offset], current[offset:]
                attribute = f"#[doc = {json.dumps(doc, ensure_ascii=False)}] "
                lines[line - 1] = prefix + attribute + rest
                inserted += 1
            source_path.write_text("".join(lines))
        print(f"generated_docs_inserted={inserted} skipped_changed_records={skipped_changed}")
        return

    if args.export_ledger:
        ledger = []
        generated = {
            record["symbol_id"]: doc
            for entries in by_file.values()
            for _sort_key, record, doc in entries
        }
        for record in records:
            if not record.get("public_api"):
                continue
            compiler_docs = index[str(record["compiler_id"])].get("docs")
            documentation = SOURCE_DOC_IMPROVEMENTS.get(
                record["qualified_name"],
                compiler_docs.strip() if compiler_docs else generated.get(record["symbol_id"], ""),
            )
            ledger.append({
                "symbol_id": record["symbol_id"],
                "qualified_name": record["qualified_name"],
                "source_file": record["source_file"],
                "documentation": documentation,
                "origin": (
                    "improved_in_current_source" if record["qualified_name"] in SOURCE_DOC_IMPROVEMENTS
                    else "snapshot_source" if compiler_docs
                    else "generated_for_current_source"
                ),
            })
        with (DB / "native-docs.jsonl").open("w") as handle:
            for row in sorted(ledger, key=lambda value: value["symbol_id"]):
                handle.write(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n")
        print(f"native_docs_exported={len(ledger)}")
        return

    if args.refresh_current_generated:
        native = {
            row["symbol_id"]: row
            for row in read_jsonl(DB / "native-docs.jsonl")
            if row.get("origin") == "generated_for_current_source"
        }
        changed = 0
        unchanged = 0
        already_different = []
        for path in sorted(by_file):
            source_path = ROOT / path
            text = source_path.read_text()
            for _sort_key, record, doc in by_file[path]:
                previous = native.get(record["symbol_id"])
                if not previous:
                    continue
                old_doc = previous["documentation"]
                old = f"#[doc = {json.dumps(old_doc, ensure_ascii=False)}]"
                new = f"#[doc = {json.dumps(doc, ensure_ascii=False)}]"
                if old_doc == doc and new in text:
                    unchanged += 1
                    continue
                if old not in text:
                    legacy_docs = []
                    if record["kind"] == "struct":
                        legacy_docs.extend([
                            f"Represents {words(record['name'])} in the PocketStation API.",
                            f"Represents the {words(record['name'])} value exposed by the PocketStation API.",
                        ])
                    legacy = next((
                        f"#[doc = {json.dumps(candidate, ensure_ascii=False)}]"
                        for candidate in legacy_docs
                        if f"#[doc = {json.dumps(candidate, ensure_ascii=False)}]" in text
                    ), None)
                    if not legacy:
                        already_different.append(record["symbol_id"])
                        continue
                    old = legacy
                text = text.replace(old, new, 1)
                changed += 1
            source_path.write_text(text)
        print(
            f"generated_docs_refreshed={changed} unchanged={unchanged} "
            f"already_different={len(already_different)}"
        )
        for symbol_id in already_different[:20]:
            print(f"already_different {symbol_id}")
        return

    if args.refresh_generic_functions:
        changed = 0
        for path in sorted(by_file):
            source_path = ROOT / path
            text = source_path.read_text()
            for _sort_key, record, doc in by_file[path]:
                if record["kind"] != "function":
                    continue
                owner = owner_for(record, by_id)
                old_doc = f"Performs the {words(record['name'])} operation defined by `{owner}`."
                old = f"#[doc = {json.dumps(old_doc, ensure_ascii=False)}]"
                new = f"#[doc = {json.dumps(doc, ensure_ascii=False)}]"
                if old == new or old not in text:
                    continue
                text = text.replace(old, new, 1)
                changed += 1
            source_path.write_text(text)
        print(f"generic_function_docs_refreshed={changed}")
        return

    if args.refresh_field_owners:
        changed = 0
        for path in sorted(by_file):
            source_path = ROOT / path
            lines = source_path.read_text().splitlines(keepends=True)
            for _sort_key, record, _doc in by_file[path]:
                parent = by_id.get(record.get("parent", ""))
                if record["kind"] != "struct_field" or not parent or parent.get("kind") != "variant":
                    continue
                line = index[str(record["compiler_id"])]["span"]["begin"][0]
                old = f"#[doc = {json.dumps(field_doc(record['name'], 'PocketStation'), ensure_ascii=False)}]"
                new = f"#[doc = {json.dumps(field_doc(record['name'], parent['name']), ensure_ascii=False)}]"
                if old not in lines[line - 1]:
                    raise SystemExit(f"generated field doc not found at {path}:{line} for {record['symbol_id']}")
                lines[line - 1] = lines[line - 1].replace(old, new, 1)
                changed += 1
            source_path.write_text("".join(lines))
        print(f"payload_field_owners_refreshed={changed}")
        return

    manifest = {record["path"]: record for record in read_jsonl(DB / "repository-manifest.jsonl")}
    for path in sorted(by_file):
        current = (ROOT / path).read_bytes()
        if sha256(current) != manifest[path]["sha256"]:
            raise SystemExit(f"refusing to edit changed source file: {path}")

    print(f"public_missing={len(missing)} files={len(by_file)}")
    for path in sorted(by_file):
        print(f"{len(by_file[path]):4d} {path}")
    if not args.apply:
        return

    for path, entries in by_file.items():
        source_path = ROOT / path
        text = source_path.read_text()
        lines = text.splitlines(keepends=True)
        starts: list[int] = []
        cursor = 0
        for line in lines:
            starts.append(cursor)
            cursor += len(line)
        edits: list[tuple[int, str, str]] = []
        for _sort_key, record, doc in entries:
            span = index[str(record["compiler_id"])]["span"]
            line, column = span["begin"]
            offset = starts[line - 1] + column - 1
            excerpt = text[offset : starts[min(len(starts) - 1, span["end"][0] - 1)] + 500]
            if record["name"] not in excerpt:
                raise SystemExit(f"source guard failed for {record['symbol_id']} at {path}:{line}:{column}")
            attribute = f"#[doc = {json.dumps(doc, ensure_ascii=False)}] "
            edits.append((offset, attribute, record["symbol_id"]))
        seen_offsets: set[int] = set()
        for offset, attribute, symbol_id in sorted(edits, reverse=True):
            if offset in seen_offsets:
                raise SystemExit(f"duplicate insertion offset for {symbol_id} in {path}")
            seen_offsets.add(offset)
            text = text[:offset] + attribute + text[offset:]
        source_path.write_text(text)

    checkpoint = {
        "snapshot": private.get("root"),
        "public_records_enriched": len(missing),
        "files_edited": len(by_file),
        "symbol_ids": sorted(record["symbol_id"] for record in missing),
    }
    (DB / "checkpoints" / "rustdoc-enrichment.json").write_text(
        json.dumps(checkpoint, indent=2, sort_keys=True) + "\n"
    )


if __name__ == "__main__":
    main()
