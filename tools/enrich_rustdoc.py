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
        "source": f"Carries the source associated with `{owner}`.",
        "output": f"Carries the output produced by `{owner}`.",
        "failure": f"Carries the failure reported by `{owner}`.",
        "outcome": f"Carries the terminal outcome reported by `{owner}`.",
        "observations": f"Carries the observations collected for `{owner}`.",
    }
    if name in exact:
        return exact[name]
    phrase = words(name)
    if name.endswith(("_id", "_ids")) or name == "id":
        return f"Identifies the {phrase.removesuffix(' identifier').removesuffix(' identifiers')} associated with `{owner}`."
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
    return f"Stores the {phrase} associated with `{owner}`."


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
    if any(token in lower_owner for token in ("error", "failure")):
        return f"Reports {phrase}."
    if any(token in lower_owner for token in ("status", "state", "phase", "outcome", "disposition", "event", "delivery")):
        return f"Indicates the {phrase} state for `{owner}`."
    if any(token in lower_owner for token in ("policy", "mode", "kind", "scope", "direction", "format", "semantics", "level", "requirement", "selector")):
        return f"Selects {phrase} behavior for `{owner}`."
    return f"Represents the {phrase} case of `{owner}`."


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
        "output": f"Returns the output associated with {subject}.",
        "input": f"Returns the input associated with {subject}.",
        "configuration": f"Returns the configuration associated with {subject}.",
        "manifest": f"Returns the manifest associated with {subject}.",
        "kind": f"Returns the kind represented by {subject}.",
        "code": f"Returns the stable error or status code represented by {subject}.",
        "message": f"Returns the diagnostic message associated with {subject}.",
        "samples": f"Returns the audio samples held by {subject}.",
        "channels": f"Returns the channel count represented by {subject}.",
        "lineage": f"Returns the frame lineage associated with {subject}.",
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
        return f"Returns the {phrase} associated with {subject}."
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
        return f"Returns the {phrase} associated with {subject}."
    return f"Performs the {phrase} operation defined by {subject}."


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
        if lower.endswith(("error", "failure")):
            return f"Reports a {phrase}."
        if lower.endswith(("config", "configuration", "options", "policy", "spec")):
            return f"Configures {phrase.removesuffix(' configuration').removesuffix(' config').removesuffix(' options').removesuffix(' policy').removesuffix(' spec')}."
        if lower.endswith(("id", "identifier")):
            return f"Uniquely identifies {phrase.removesuffix(' identifier')}."
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
        return f"Represents {phrase} in the PocketStation API."
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
        return f"Defines the implementation contract for {role}."
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

    by_file: dict[str, list[tuple[int, dict[str, Any], str]]] = defaultdict(list)
    for record in missing:
        item = index[str(record["compiler_id"])]
        span = item.get("span")
        if not span or span.get("filename") != record["source_file"]:
            raise SystemExit(f"span mismatch for {record['symbol_id']}")
        line, column = span["begin"]
        by_file[record["source_file"]].append(((line << 32) | column, record, item_doc(record, by_id)))

    if args.export_ledger:
        ledger = []
        for path in sorted(by_file):
            for _sort_key, record, doc in by_file[path]:
                ledger.append({
                    "symbol_id": record["symbol_id"],
                    "qualified_name": record["qualified_name"],
                    "source_file": path,
                    "documentation": doc,
                })
        with (DB / "native-docs.jsonl").open("w") as handle:
            for row in sorted(ledger, key=lambda value: value["symbol_id"]):
                handle.write(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n")
        print(f"native_docs_exported={len(ledger)}")
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
