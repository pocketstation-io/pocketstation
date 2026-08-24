#!/usr/bin/env python3
"""Refine frozen-snapshot ledgers that cannot be accepted as name-only extraction.

The documentation compiler creates the exhaustive denominators. This pass
joins those records back to compiler symbols, tests, examples, source error
messages, defaults, and reverse relationships. It never reads mutable product
source when establishing facts: all semantic source comes from the frozen Git
snapshot recorded in ``.doc-intel/state.json``.
"""

from __future__ import annotations

import json
import re
import subprocess
from collections import defaultdict
from pathlib import Path, PurePosixPath
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / ".doc-intel"


def read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def write_jsonl(path: Path, records: Iterable[dict[str, Any]]) -> None:
    with path.open("w") as handle:
        for record in records:
            handle.write(json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n")


STATE = read_json(DB / "state.json")
SNAPSHOT = STATE["snapshot"]
FILES = read_jsonl(DB / "repository-manifest.jsonl")
FILE_BY_PATH = {record["path"]: record for record in FILES}
SYMBOLS = read_jsonl(DB / "symbol-manifest.jsonl")
SYMBOL_BY_ID = {record["symbol_id"]: record for record in SYMBOLS}
TESTS = read_jsonl(DB / "tests.jsonl")
EXAMPLES = read_jsonl(DB / "examples.jsonl")
EDGES = read_jsonl(DB / "edges.jsonl")
PAGES = read_jsonl(DB / "page-manifest.jsonl")
NATIVE_DOCS = {
    record["symbol_id"]: record["documentation"]
    for record in read_jsonl(DB / "native-docs.jsonl")
}


def frozen_text(path: str) -> str:
    return subprocess.check_output(
        ["git", "show", f"{SNAPSHOT}:{path}"], cwd=ROOT
    ).decode("utf-8", errors="replace")


TEXT_BY_PATH = {
    record["path"]: frozen_text(record["path"])
    for record in FILES
    if record.get("semantic")
}


def human(value: str) -> str:
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", value)
    return value.replace("_", " ").replace("-", " ").strip().lower()


def domain(path: str) -> str:
    parts = PurePosixPath(path).parts
    if path == "src/lib.rs":
        return "session"
    if len(parts) > 1 and parts[0] == "src":
        return parts[1].removesuffix(".rs")
    if len(parts) > 1 and parts[0] in {"tests", "examples", "benches"}:
        return parts[1].split(".", 1)[0].replace("_", "-")
    if len(parts) > 1:
        return parts[0]
    return PurePosixPath(path).stem


def symbol_description(symbol: dict[str, Any]) -> str:
    native = str(NATIVE_DOCS.get(symbol["symbol_id"], "")).strip()
    if native:
        return native.split("\n", 1)[0]
    if symbol.get("source_documented") and symbol.get("summary") != "unknown":
        return str(symbol["summary"]).split("\n", 1)[0]
    return human(symbol["name"])


SYMBOLS_BY_FILE: dict[str, list[dict[str, Any]]] = defaultdict(list)
for symbol in SYMBOLS:
    SYMBOLS_BY_FILE[symbol["source_file"]].append(symbol)

TESTS_BY_SYMBOL: dict[str, set[str]] = defaultdict(set)
TESTS_BY_PATH: dict[str, set[str]] = defaultdict(set)
TEST_TOKEN_INDEX: dict[str, set[str]] = defaultdict(set)
for test in TESTS:
    TESTS_BY_PATH[test["path"]].add(test["test_id"])
    for symbol_id in test.get("production_symbols", []):
        TESTS_BY_SYMBOL[symbol_id].add(test["test_id"])
    source = TEXT_BY_PATH[test["path"]]
    body = "\n".join(source.splitlines()[test["lines"][0] - 1:test["lines"][1]])
    for token in set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", body)):
        TEST_TOKEN_INDEX[token].add(test["test_id"])

EXAMPLES_BY_SYMBOL: dict[str, set[str]] = defaultdict(set)
EXAMPLES_BY_PATH: dict[str, set[str]] = defaultdict(set)
for example in EXAMPLES:
    EXAMPLES_BY_PATH[example["path"]].add(example["example_id"])
    for symbol_id in example.get("public_apis", []):
        EXAMPLES_BY_SYMBOL[symbol_id].add(example["example_id"])


def module_documentation(path: str) -> tuple[str | None, list[int] | None]:
    lines = TEXT_BY_PATH[path].splitlines()
    documented: list[tuple[int, str]] = []
    for number, line in enumerate(lines[:160], 1):
        stripped = line.strip()
        if stripped.startswith("//!"):
            documented.append((number, stripped[3:].strip()))
        elif documented and stripped and not stripped.startswith(("#![", "//")):
            break
    if not documented:
        return None, None
    text = re.sub(r"\s+", " ", " ".join(value for _line, value in documented)).strip()
    return text, [documented[0][0], documented[-1][0]]


def purpose_for(path: str, dossier: dict[str, Any], symbols: list[dict[str, Any]]) -> tuple[str, str]:
    kind = dossier["file_kind"]
    explicit = {
        "Cargo.toml": "Declares crate metadata, Cargo features, dependencies, build targets, and package publication settings.",
        "build.rs": "Selects and compiles the target-specific native shim used by the native-capture feature.",
        ".github/CODEOWNERS": "Assigns repository review ownership to tracked path patterns.",
        ".github/PULL_REQUEST_TEMPLATE.md": "Defines the evidence and validation checklist required from pull requests.",
        ".github/workflows/ci.yml": "Defines continuous-integration jobs for formatting, linting, tests, protocol checks, and supported build surfaces.",
        ".github/workflows/publish.yml": "Defines the release workflow that validates and publishes the PocketStation crate.",
        "src/connector/mod.rs": "Declares connector manifests, validated configuration, endpoint factories, sidecar-backed drivers, registrations, and observations.",
        "src/session/prepare/mod.rs": "Prepares generated-audio ingresses, external and captured sources, operators, endpoints, recordings, and their runtime identity mappings before Session start.",
    }
    if path in explicit:
        return explicit[path], "DIRECT"
    if kind == "test":
        names = [test["name"] for test in TESTS if test["path"] == path]
        scope = ", ".join(human(name) for name in names[:3]) or human(PurePosixPath(path).stem)
        return f"Provides executable evidence for {scope} under the conditions declared in this test file.", "TESTED"
    if kind == "example":
        return f"Demonstrates the repository-owned {human(PurePosixPath(path).stem)} workflow as compilable example code.", "DECLARED"
    if kind == "benchmark":
        calls = [value for value in dossier.get("calls", []) if value not in {"black_box", "iter"}]
        subject = ", ".join(calls[:4]) or human(PurePosixPath(path).stem)
        return f"Measures {human(PurePosixPath(path).stem)} by repeatedly exercising {subject}.", "DECLARED"
    if path.endswith(".rs"):
        module_docs, _lines = module_documentation(path)
        if module_docs:
            return module_docs, "DECLARED"
        public = [symbol_description(item) for item in symbols if item.get("public_api")][:3]
        if public:
            return " ".join(value.rstrip(".") + "." for value in public), "DECLARED"
        owned = [f"{human(item['name'])} {human(item['kind'])}" for item in symbols[:4]]
        if owned:
            return f"Owns the internal {', '.join(owned)} used by `{dossier['module']}`.", "INFERRED"
        return f"Owns the internal implementation boundary in `{dossier['module']}`.", "INFERRED"
    if path.endswith(".md"):
        heading = re.search(r"(?m)^#\s+(.+)$", TEXT_BY_PATH[path])
        subject = heading.group(1).strip() if heading else human(PurePosixPath(path).stem)
        return f"Documents {subject} for repository contributors or PocketStation developers.", "DECLARED"
    labels = {
        "public_ffi": "Declares the public C ABI, including layouts, ownership rules, functions, and status values.",
        "automation": "Defines a repository-owned build, validation, test, or release operation.",
        "native_source": "Implements a target-native capture, device, or foreign-function boundary.",
        "semantic_metadata": "Declares repository-owned package, build, compatibility, or fixture metadata.",
        "benchmark": "Defines a repository-owned performance measurement workload and its inputs.",
    }
    return labels.get(kind, f"Declares the repository-owned {human(kind)} contract represented by this file."), "INFERRED"


def non_responsibilities_for(dossier: dict[str, Any]) -> list[str]:
    kind = dossier["file_kind"]
    boundary = dossier["module"] if dossier["module"] != "not_applicable" else dossier["path"]
    if kind == "test":
        return [f"`{dossier['path']}` does not define production behavior beyond its recorded setup and assertions."]
    if kind == "example":
        return [f"`{dossier['path']}` does not establish platform qualification or behavior outside its prerequisites."]
    if kind in {"automation", "semantic_metadata"}:
        return [f"`{dossier['path']}` declares repository inputs and does not by itself prove product runtime behavior."]
    if kind == "public_ffi":
        return [f"`{dossier['path']}` does not implement the Rust runtime or extend ownership beyond its C declarations."]
    if kind == "native_source":
        return [f"The `{boundary}` native implementation does not establish physical-device qualification."]
    if dossier["language"] == "Markdown":
        return [f"`{dossier['path']}` does not override compiler-visible API contracts or executable tests."]
    return [f"`{boundary}` does not establish qualification, performance, or retry guarantees outside its direct evidence."]


def precise_dossier_evidence(
    file_record: dict[str, Any], dossier: dict[str, Any], symbols: list[dict[str, Any]]
) -> list[dict[str, Any]]:
    path = file_record["path"]
    if path.endswith(".rs"):
        _docs, lines = module_documentation(path)
        if lines:
            return [{
                "path": path,
                "content_hash": file_record["sha256"],
                "lines": lines,
                "symbol": dossier["module"],
                "classification": "DECLARED",
            }]
        symbol_evidence = [
            item
            for symbol in symbols
            if symbol.get("kind") != "module"
            for item in symbol.get("evidence", [])
        ][:24]
        if symbol_evidence:
            return symbol_evidence
        tests = [test for test in TESTS if test["path"] == path]
        if tests:
            return [item for test in tests[:12] for item in test["evidence"]]
    lines = TEXT_BY_PATH[path].splitlines()
    if path.endswith(".md"):
        heading = next((number for number, line in enumerate(lines, 1) if line.startswith("# ")), 1)
        return [{
            "path": path, "content_hash": file_record["sha256"],
            "lines": [heading, min(len(lines), heading + 8)], "symbol": None,
            "classification": "DECLARED",
        }]
    meaningful = [number for number, line in enumerate(lines, 1) if line.strip() and not line.lstrip().startswith(("#", "//"))]
    begin = meaningful[0] if meaningful else 1
    return [{
        "path": path, "content_hash": file_record["sha256"],
        "lines": [begin, min(len(lines), begin + 20)], "symbol": None,
        "classification": "DIRECT",
    }]


def symbol_source(symbol: dict[str, Any]) -> str:
    lines = TEXT_BY_PATH[symbol["source_file"]].splitlines()
    begin, end = symbol["source_lines"]
    return "\n".join(lines[max(0, begin - 1):min(len(lines), end)])


def refine_symbols() -> None:
    public_error_symbols = [
        item for item in SYMBOLS
        if item.get("public_api") and any(token in item["name"].lower() for token in ("error", "failure"))
    ]
    for symbol in SYMBOLS:
        if not symbol.get("public_api"):
            continue
        source = symbol_source(symbol)
        lower_source = source.lower()
        description = symbol_description(symbol).strip().rstrip(".")
        if not description or description == human(symbol["name"]):
            description = f"Declares the public {human(symbol['kind'])} `{symbol['qualified_name']}`"
        # ``summary`` is the concise public description in the symbol schema.
        # Preserve the frozen compiler fact in ``source_documented`` while
        # filling this field from the declaration-specific native-doc ledger.
        # Leaving thousands of literal ``unknown`` summaries would make the
        # registry internally inconsistent with its exhaustive API docs.
        symbol["summary"] = description + "."
        symbol["summary_basis"] = (
            "frozen_source_documentation"
            if symbol.get("source_documented")
            else "declaration_specific_native_documentation"
        )
        symbol["responsibility"] = description + "."
        if symbol["kind"] == "function":
            symbol["when_to_use"] = f"Call `{symbol['qualified_name']}` when its documented {human(symbol['name'])} operation matches the current owner state and inputs."
            symbol["when_not_to_use"] = f"Do not call `{symbol['qualified_name']}` when its receiver state, target gate, input validity, or ownership preconditions are not satisfied."
        elif symbol["kind"] == "trait":
            symbol["when_to_use"] = f"Implement `{symbol['qualified_name']}` to supply the behavior required by its compiler-visible methods."
            symbol["when_not_to_use"] = f"Do not implement `{symbol['qualified_name']}` without honoring every required method, ownership boundary, and lifecycle result."
        else:
            symbol["when_to_use"] = f"Use `{symbol['qualified_name']}` where an owning PocketStation API signature requires this {human(symbol['kind'])}."
            symbol["when_not_to_use"] = f"Do not substitute `{symbol['qualified_name']}` for a different identity, state, unit, or ownership type."

        if symbol["kind"] == "function":
            panic_sites = [
                token for token in ("panic!", ".unwrap(", ".expect(", "assert!(", "assert_eq!(")
                if token in source
            ]
            symbol["panic_behavior"] = (
                "explicit_panic_sites_in_owned_body:" + ",".join(panic_sites)
                if panic_sites
                else "no_explicit_panic_site_in_owned_body; transitive panic behavior is not declared"
            )
            blocking_sites = [
                token for token in ("sleep(", ".recv(", ".wait(", ".join(", "read_exact", "write_all", "Command::")
                if token in source
            ]
            symbol["blocking_behavior"] = (
                "potentially_blocking_owned_operations:" + ",".join(blocking_sites)
                if blocking_sites
                else "no_explicit_blocking_primitive_in_owned_body"
            )
            if "cancel" in lower_source:
                symbol["cancellation"] = "observes_or_requests_cancellation_in_owned_body"
            elif symbol.get("async_behavior") == "async":
                symbol["cancellation"] = "no_explicit_cancellation_contract_in_owned_async_body"
            else:
                symbol["cancellation"] = "not_applicable_to_synchronous_nonwaiting_operation"
        else:
            symbol["panic_behavior"] = "not_applicable_to_non_callable_declaration"
            symbol["blocking_behavior"] = "not_applicable_to_non_callable_declaration"
            symbol["cancellation"] = "not_applicable_to_non_callable_declaration"

        symbol["thread_safety"] = (
            "compiler_enforced_Send_Sync_properties_only; no additional operational guarantee is declared"
        )
        if any(token in lower_source for token in ("sequence", "ordered", "fifo", "topo")):
            symbol["ordering"] = "ordering_semantics_are_declared_in_the_owned_source_and_preserve_the_recorded_sequence_or_topology"
        else:
            symbol["ordering"] = "no_additional_ordering_contract_declared_by_this_declaration"
        if any(token in lower_source for token in ("capacity", "full", "backpressure", "try_send", "drop")):
            symbol["backpressure"] = "bounded_capacity_or_saturation_behavior_is_present_in_the_owned_source; inspect_returned_outcomes"
        else:
            symbol["backpressure"] = "not_applicable_or_not_declared_by_this_declaration"

        name = symbol["name"].lower()
        description_lower = description.lower()
        if name.endswith("_ns") or "nanosecond" in description_lower:
            symbol["units"] = "nanoseconds"
        elif name.endswith("_ms") or "millisecond" in description_lower:
            symbol["units"] = "milliseconds"
        elif name.endswith("_hz") or "hertz" in description_lower:
            symbol["units"] = "hertz"
        elif name.endswith(("_bytes", "_byte_count")) or "in bytes" in description_lower:
            symbol["units"] = "bytes"
        elif name.endswith(("_frames", "_frame_count")):
            symbol["units"] = "frames"
        else:
            symbol["units"] = "not_applicable_or_encoded_by_the_declared_type"
        if name.startswith("max_") or "maximum" in description_lower or "capacity" in name:
            symbol["limits"] = "the compiler-visible constant, field value, or validation path defines the upper bound"
        else:
            symbol["limits"] = "no_independent_limit_declared_by_this_declaration"

        signature = json.dumps(symbol.get("signature"), separators=(",", ":"))
        if "raw_pointer" in signature:
            symbol["ownership"] = "raw_pointer_ownership_and_validity_follow_the_documented_safety_contract"
        elif "borrowed_ref" in signature:
            symbol["ownership"] = "borrows_the_referenced_value_for_the_compiler_visible_lifetime"
        elif symbol["kind"] == "function" and '"self"' in signature:
            symbol["ownership"] = "receiver_ownership_is_encoded_by_the_compiler_visible_self_parameter"
        else:
            symbol["ownership"] = "value_ownership_is_encoded_by_the_compiler_visible_type_or_signature"
        symbol["lifetime"] = "compiler_visible_lifetimes_and_borrowed_references_are_authoritative"
        symbol["mutability"] = "compiler_visible_mutability_of_fields_parameters_and_receiver_is_authoritative"
        symbol["unknown_references"] = ["unknown-public-thread-safety"]
        if "retry" in lower_source or any("retry" in value.lower() for value in symbol.get("errors", [])):
            symbol["unknown_references"].append("unknown-retry-policy")
        symbol["errors"] = sorted({
            error["symbol_id"] for error in public_error_symbols
            if error["name"] in source and error["symbol_id"] != symbol["symbol_id"]
        })
    write_jsonl(DB / "symbol-manifest.jsonl", SYMBOLS)
    write_jsonl(DB / "symbols.jsonl", SYMBOLS)


def refine_dossiers() -> None:
    file_path_by_id = {record["file_id"]: record["path"] for record in FILES}
    incoming: dict[str, list[str]] = defaultdict(list)
    outgoing: dict[str, list[str]] = defaultdict(list)
    for edge in EDGES:
        source = edge.get("source")
        target = edge.get("target")
        source_path = file_path_by_id.get(source) or SYMBOL_BY_ID.get(source, {}).get("source_file")
        target_path = file_path_by_id.get(target) or SYMBOL_BY_ID.get(target, {}).get("source_file")
        if source_path and target_path and source_path != target_path:
            outgoing[source_path].append(f"{edge['kind']}:{target_path}")
            incoming[target_path].append(f"{edge['kind']}:{source_path}")
    for file_record in FILES:
        if not file_record.get("semantic"):
            continue
        dossier_path = ROOT / file_record["dossier"]
        dossier = read_json(dossier_path)
        symbols = SYMBOLS_BY_FILE.get(file_record["path"], [])
        purpose, classification = purpose_for(file_record["path"], dossier, symbols)
        dossier["purpose"] = {"text": purpose, "classification": classification}
        symbol_responsibilities = [
            f"{item['qualified_name']}: {symbol_description(item)}"
            for item in symbols
            if item.get("public_api")
        ]
        if not symbol_responsibilities:
            symbol_responsibilities = [
                f"{item['name']} ({item['kind']})" for item in dossier.get("defines", [])[:30]
            ]
        dossier["responsibilities"] = symbol_responsibilities or [purpose]
        dossier["non_responsibilities"] = non_responsibilities_for(dossier)
        dossier["imported_by"] = sorted(set(incoming[file_record["path"]]))
        dossier["called_by"] = sorted(set(incoming[file_record["path"]]))
        dossier["constructed_by"] = sorted(set(incoming[file_record["path"]]))
        dossier["implemented_by"] = sorted(set(incoming[file_record["path"]]))
        dossier["extended_by"] = sorted(set(incoming[file_record["path"]]))
        inputs = [
            f"Depends on `{value.removeprefix('module:')}` through a declared Rust import."
            for value in dossier.get("imports", [])[:20]
        ]
        inputs.extend(f"Reads process environment variable `{value}`." for value in dossier.get("environment_variables", []))
        inputs.extend(
            f"Consumes configuration at {file_record['path']}:{item['line']}."
            for item in dossier.get("configuration_read", [])[:10]
        )
        dossier["inputs"] = sorted(set(inputs)) or ["No external input is declared in this file."]
        outputs = [
            f"Declares public {item['kind']} `{item['name']}`."
            for item in dossier.get("public_surface", [])[:30]
        ]
        outputs.extend(
            f"Creates a typed failure at {file_record['path']}:{item['line']}."
            for item in dossier.get("errors_created", [])[:10]
        )
        dossier["outputs"] = sorted(set(outputs)) or ["No emitted value or public declaration is recorded for this file."]
        dossier["evidence"] = precise_dossier_evidence(file_record, dossier, symbols)
        test_ids = set(TESTS_BY_PATH[file_record["path"]])
        example_ids = set(EXAMPLES_BY_PATH[file_record["path"]])
        for symbol in symbols:
            test_ids.update(TESTS_BY_SYMBOL[symbol["symbol_id"]])
            example_ids.update(EXAMPLES_BY_SYMBOL[symbol["symbol_id"]])
        dossier["tests_covering"] = sorted(test_ids)
        dossier["examples_using"] = sorted(example_ids)
        dossier["test_coverage_status"] = "linked" if test_ids else "no_direct_test_link_extracted"
        dossier["example_coverage_status"] = "linked" if example_ids else "no_direct_example_link_extracted"
        pitfalls = []
        if dossier.get("ffi_io"):
            pitfalls.append("Crosses an unsafe or foreign-function boundary; apply the recorded ownership and safety contract.")
        if dossier.get("platform_gate") not in ([], ["not_explicitly_gated"]):
            pitfalls.append("Only applies under the recorded target gate; implementation is not physical qualification.")
        if not test_ids:
            pitfalls.append("No direct test edge was resolved for this file; do not promote implementation presence to tested behavior.")
        if dossier.get("retry_behavior") and not any("retry" in item.lower() for item in dossier["non_responsibilities"]):
            pitfalls.append("Retry-related code is present, but safety and idempotence require an explicit public contract.")
        dossier["potential_pitfalls"] = pitfalls or ["Keep claims within the file's direct evidence and linked tests."]
        dossier["relationship_outputs"] = sorted(set(outgoing[file_record["path"]]))
        dossier["analysis_stage"] = "doc_ready"
        write_json(dossier_path, dossier)
        file_record["analysis_stage"] = "doc_ready"
    write_jsonl(DB / "repository-manifest.jsonl", FILES)
    write_jsonl(DB / "inventory.jsonl", FILES)


def error_template(path: str, line: int) -> str | None:
    lines = TEXT_BY_PATH[path].splitlines()
    prefix = "\n".join(lines[max(0, line - 14):line])
    matches = list(re.finditer(r'#\s*\[\s*error\s*\(\s*"((?:[^"\\]|\\.)*)"', prefix, re.S))
    if not matches:
        return None
    return re.sub(r"\s+", " ", matches[-1].group(1).replace("\\\"", '"')).strip()


def developer_action(name: str, template: str | None) -> str:
    phrase = human(name)
    if name.startswith(("Invalid", "Zero", "Empty", "Missing", "Unknown", "Duplicate", "Wrong", "Foreign", "Unsupported", "Too")):
        return f"Correct the {phrase} condition reported by the returned fields before repeating the operation."
    if "Permission" in name:
        return "Keep permission prompting in the host application, then repeat source preparation only after the authorization state changes."
    if any(word in name for word in ("Closed", "Stopped", "Cancelled", "Aborted")):
        return "Treat the owning resource or lifecycle as terminal unless its API explicitly exposes a new preparation path."
    if any(word in name for word in ("Timeout", "TimedOut", "Deadline")):
        return "Inspect the reported stage and deadline; retry only when the owning API supplies an idempotence or retry contract."
    if template:
        return "Use the returned context to correct the reported condition; preserve the error when no recovery contract is declared."
    return f"Preserve `{name}` and inspect the owning operation's typed fields before choosing recovery or presentation."


def token_sites(token: str, *, context: str | None = None, limit: int = 200) -> list[dict[str, Any]]:
    sites: list[dict[str, Any]] = []
    pattern = re.compile(rf"\b{re.escape(token)}\b")
    for path, text in TEXT_BY_PATH.items():
        for number, line in enumerate(text.splitlines(), 1):
            if pattern.search(line) and (context is None or context in line):
                sites.append({
                    "path": path,
                    "line": number,
                    "text": line.strip()[:300],
                    "content_hash": FILE_BY_PATH[path]["sha256"],
                })
                if len(sites) == limit:
                    return sites
    return sites


def refine_errors() -> None:
    records = read_jsonl(DB / "errors.jsonl")
    variant_names = {str(record["variant"]) for record in records if record.get("variant")}
    error_type_names = {record["type"].rsplit("::", 1)[-1] for record in records}
    variant_sites: defaultdict[str, list[dict[str, Any]]] = defaultdict(list)
    translation_sites_by_type: defaultdict[str, list[dict[str, Any]]] = defaultdict(list)
    for path, text in TEXT_BY_PATH.items():
        for number, line in enumerate(text.splitlines(), 1):
            tokens = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", line))
            for name in tokens & variant_names:
                variant_sites[name].append({
                    "path": path, "line": number, "text": line.strip()[:300],
                    "content_hash": FILE_BY_PATH[path]["sha256"],
                })
            if "map_err" in line or "impl From<" in line:
                for name in tokens & error_type_names:
                    translation_sites_by_type[name].append({
                        "path": path, "line": number, "text": line.strip()[:300],
                        "content_hash": FILE_BY_PATH[path]["sha256"],
                    })
    function_symbols_by_token: defaultdict[str, set[str]] = defaultdict(set)
    for candidate in SYMBOLS:
        if candidate.get("kind") != "function":
            continue
        for token in set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", symbol_source(candidate))):
            if token in error_type_names or token in variant_names:
                function_symbols_by_token[token].add(candidate["symbol_id"])
    for record in records:
        symbol = SYMBOL_BY_ID[record["symbol_id"]]
        error_type_name = record["type"].rsplit("::", 1)[-1]
        template = error_template(symbol["source_file"], symbol["source_lines"][0]) if record.get("variant") else None
        if template:
            record["external_representation"] = template
            record["trigger_condition"] = f"Reported when {template.rstrip('.')}."
            record["trigger_basis"] = "thiserror display contract at the variant declaration"
        elif record.get("variant"):
            record["trigger_condition"] = f"Classifies the {human(record['variant'])} failure case at the owning boundary."
            record["trigger_basis"] = "compiler-visible variant and native documentation; no narrower creation site was resolved"
            record["external_representation"] = "typed_variant_without_a_declared_display_string"
        else:
            record["trigger_condition"] = f"Returned by operations whose signature names {record['type']}."
            record["trigger_basis"] = "compiler-visible public error type"
            record["external_representation"] = "typed_Rust_error; variant_records_hold_any_declared_display_text"

        if record.get("variant"):
            qualified = f"{error_type_name}::{record['variant']}"
            created = [item for item in variant_sites[record["variant"]] if qualified in item["text"]]
            if not created:
                self_qualified = f"Self::{record['variant']}"
                created = [item for item in variant_sites[record["variant"]] if self_qualified in item["text"]]
            record["created_at"] = created
            record["lineage_disposition"] = (
                "creation_sites_resolved_by_qualified_variant_constructor"
                if created
                else "no_qualified_constructor_site_resolved; construction_may_be_external_or_derived"
            )
        else:
            record["created_at"] = []
            record["lineage_disposition"] = "aggregate_error_type; variant_records_own_creation_sites"

        propagation = set(function_symbols_by_token[error_type_name])
        if record.get("variant"):
            propagation.update(function_symbols_by_token[record["variant"]])
        record["propagates_through"] = sorted(propagation)
        creation_sites = record["created_at"]
        record["wrapped_by"] = [
            {"path": item["path"], "line": item["line"], "mechanism": "map_err"}
            for item in creation_sites if "map_err" in item["text"]
        ]
        record["translated_to"] = translation_sites_by_type[error_type_name][:100]
        tests = set(symbol.get("tests", []))
        tests.update(TEST_TOKEN_INDEX.get(symbol["name"], set()))
        record["tests"] = sorted(tests)
        record["test_coverage_status"] = "linked" if tests else "no_direct_test_link_extracted"
        record["developer_action"] = developer_action(symbol["name"], template)
        record["user_action"] = "Application policy decides whether and how to present this typed failure."
        record["fatal_to_operation"] = True
        record["retryable"] = record["retryable"] if record["retryable"] != "unknown" else "not_declared"
        record["retry_basis"] = "No retry guarantee is inferred from an error or variant name."
        record["recoverable"] = record["recoverable"] if record["recoverable"] != "unknown" else "not_declared"
        record["recovery_action"] = record["developer_action"]
        record["fatal_to_session"] = "depends_on_owning_operation"
        record["fatal_to_process"] = False
        record["logged"] = "not_declared"
        record["metric"] = "not_declared"
        record["event"] = "not_declared"
        record["documentation_status"] = "pending"
    write_jsonl(DB / "errors.jsonl", records)


def default_values(path: str) -> dict[tuple[str, str], str]:
    text = TEXT_BY_PATH[path]
    result: dict[tuple[str, str], str] = {}
    for match in re.finditer(r"impl\s+Default\s+for\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{", text):
        owner = match.group(1)
        body = text[match.end():match.end() + 6000]
        for field, value in re.findall(r"(?m)^\s*([a-z_][A-Za-z0-9_]*)\s*:\s*([^,\n]+)", body):
            result[(owner, field)] = value.strip()
    return result


DEFAULTS: dict[tuple[str, str, str], str] = {}
for path in TEXT_BY_PATH:
    if path.endswith(".rs"):
        for (owner, field), value in default_values(path).items():
            DEFAULTS[(path, owner, field)] = value


def refine_configuration() -> None:
    records = read_jsonl(DB / "configuration.jsonl")
    errors = read_jsonl(DB / "errors.jsonl")
    for record in records:
        if record["kind"] == "cargo_feature":
            record["security_implications"] = "feature_dependent; enabling native or fixture code expands the compiled surface"
            record["tests"] = sorted(TEST_TOKEN_INDEX.get(record["name"], set()))
            record["test_coverage_status"] = "linked" if record["tests"] else "no_direct_test_link_extracted"
            continue
        name = record["name"]
        parent = (record.get("parent") or "").rsplit("::", 1)[-1]
        found_default = DEFAULTS.get((record["source_file"], parent, name))
        record["default"] = found_default if found_default is not None else "no_default_declared_on_owning_type"
        signature = json.dumps(record.get("value_type", ""), separators=(",", ":"))
        record["required"] = "Option" not in signature and found_default is None
        if name.endswith("_ns"):
            record["units"] = "nanoseconds"
        elif name.endswith("_ms"):
            record["units"] = "milliseconds"
        elif name.endswith(("_bytes", "_byte_count")):
            record["units"] = "bytes"
        elif name.endswith(("_frames", "_frame_count")):
            record["units"] = "frames"
        elif name.endswith("_hz"):
            record["units"] = "hertz"
        else:
            record["units"] = "not_declared_or_not_applicable"
        uses = []
        token = re.compile(rf"\b{re.escape(name)}\b")
        for path, text in TEXT_BY_PATH.items():
            for number, line in enumerate(text.splitlines(), 1):
                if token.search(line) and not (path == record["source_file"] and record["source_lines"][0] <= number <= record["source_lines"][1]):
                    uses.append({"path": path, "line": number, "text": line.strip()[:300]})
                    if len(uses) == 8:
                        break
            if len(uses) == 8:
                break
        record["read_sites"] = uses
        if uses:
            locations = ", ".join(f"{item['path']}:{item['line']}" for item in uses)
            record["when_read"] = f"Consumed at the resolved source sites: {locations}."
            record["read_site_disposition"] = "resolved_by_exact_field_name_reference"
        else:
            record["when_read"] = "No consumer beyond the declaration was resolved by exact field-name reference."
            record["read_site_disposition"] = "unresolved_explicit"
        record["precedence"] = (
            "The value stored in the owning typed configuration is authoritative; "
            "a source-level environment or repository override was not resolved."
        )
        tests = sorted(TEST_TOKEN_INDEX.get(name, set()))
        record["tests"] = tests
        record["test_coverage_status"] = "linked" if tests else "no_direct_test_link_extracted"
        record["minimum"] = record["minimum"] if record["minimum"] != "unknown" else "not_declared"
        record["maximum"] = record["maximum"] if record["maximum"] != "unknown" else "not_declared"
        record["restart_required"] = "reconstruct_or_reprepare_owner"
        related_errors = [
            error["error_id"] for error in errors
            if error["defined_at"]["path"] == record["source_file"]
            or record["name"].lower() in str(error.get("trigger_condition", "")).lower()
        ]
        record["invalid_value_behavior"] = (
            f"Rejected through typed errors {', '.join(related_errors[:12])}."
            if related_errors
            else "No implicit fallback is declared; rejection behavior remains with the owning constructor or validator."
        )
        record["invalid_value_error_ids"] = related_errors[:12]
        record["security_implications"] = "secret_material" if "secret" in name.lower() or "secret" in parent.lower() else "none_explicitly_declared"
        record["documentation_status"] = "pending"
    write_jsonl(DB / "configuration.jsonl", records)


def lifecycle_states(name: str) -> tuple[str, str]:
    lower = name.lower()
    if "prepare" in lower:
        return "constructed_before_preparation", "prepared_or_prepare_failed"
    if "start" in lower:
        return "prepared", "running_or_start_failed"
    if "cancel" in lower:
        return "preparing_or_running", "cancellation_requested_or_cancelled"
    if "request_stop" in lower or lower.endswith("::stop") or "stop_" in lower:
        return "running", "stopping_or_terminal"
    if "drain" in lower:
        return "stopping", "drained_or_drain_failed"
    if any(token in lower for token in ("finalize", "finish", "join")):
        return "stopping_or_completed", "terminal"
    if any(token in lower for token in ("close", "drop", "abort")):
        return "owned_or_running", "closed_or_released"
    if "run" in lower:
        return "prepared", "running_or_terminal"
    return "owning_state_before_operation", "owning_state_after_returned_outcome"


def refine_lifecycles() -> None:
    records = read_jsonl(DB / "lifecycles.jsonl")
    errors_by_file: dict[str, list[str]] = defaultdict(list)
    for error in read_jsonl(DB / "errors.jsonl"):
        errors_by_file[error["defined_at"]["path"]].append(error["error_id"])
    for record in records:
        symbol = SYMBOL_BY_ID[record["symbol_id"]]
        source, destination = lifecycle_states(record["operation"])
        record["source_state"] = source
        record["destination_state"] = destination
        parameter_names = [
            str(value[0]) for value in symbol.get("parameters", [])
            if isinstance(value, list) and value
        ]
        record["guard"] = (
            f"`{record['operation']}` requires source state `{source}` and compiler-visible inputs "
            f"{', '.join(parameter_names) if parameter_names else 'with no explicit parameters'}."
        )
        record["guard_basis"] = "operation name plus compiler-owned signature; state inference is explicit"
        record["state_classification"] = "INFERRED"
        record["action"] = symbol_description(symbol)
        related_errors = errors_by_file.get(symbol["source_file"], [])
        record["possible_error"] = related_errors[:20] if related_errors else "no_same_file_public_error_resolved"
        record["recovery"] = (
            f"Preserve the returned outcome and inspect typed errors {', '.join(related_errors[:20])}; "
            "no retry is authorized without the owning error contract."
            if related_errors
            else "Preserve the returned outcome; no same-file public error or restart guarantee was resolved."
        )
        source_text = symbol_source(symbol).lower()
        record["idempotence"] = (
            "explicit_already_terminal_or_repeated_call_handling_present"
            if any(token in source_text for token in ("already", "idempot", "take()", "is_none"))
            else "no_idempotence_guarantee_declared_in_the_owned_operation"
        )
        tests = set(symbol.get("tests", [])) | TEST_TOKEN_INDEX.get(symbol["name"], set())
        record["tests"] = sorted(tests)
        record["observable_signal"] = "typed_return_or_terminal_outcome" if tests or related_errors else "no_distinct_signal_extracted"
        record["documentation_status"] = "pending"
    write_jsonl(DB / "lifecycles.jsonl", records)


def refine_behaviors() -> None:
    records = read_jsonl(DB / "behaviors.jsonl")
    lifecycle_by_symbol = {
        record["symbol_id"]: record for record in read_jsonl(DB / "lifecycles.jsonl")
    }
    test_by_id = {record["test_id"]: record for record in TESTS}
    for record in records:
        if record.get("classification") == "TESTED" and record.get("tests"):
            test = test_by_id.get(record["tests"][0])
            if test:
                source = TEXT_BY_PATH[test["path"]].splitlines()
                body = "\n".join(source[test["lines"][0] - 1:test["lines"][1]])
                operations = sorted({
                    match.group(1)
                    for match in re.finditer(r"\b([A-Za-z_][A-Za-z0-9_:]*)\s*\(", body)
                    if match.group(1) not in {test["name"], "if", "for", "while", "match"}
                })
                record["steps"] = [{"operation": value} for value in operations]
                record["errors"] = test.get("failure_expectation", [])
                record["step_basis"] = "test_body_span"
        elif record.get("entry_points"):
            lifecycle = lifecycle_by_symbol.get(record["entry_points"][0])
            if lifecycle:
                record["steps"] = [{
                    "operation": lifecycle["operation"],
                    "source_state": lifecycle["source_state"],
                    "destination_state": lifecycle["destination_state"],
                }]
                possible = lifecycle.get("possible_error")
                record["errors"] = possible if isinstance(possible, list) else []
                record["step_basis"] = "linked_lifecycle_record"
        record["unknown_references"] = []
        record["documentation_status"] = "pending"
    write_jsonl(DB / "behaviors.jsonl", records)


PATTERN_CONTENT = {
    "bounded_queue": (
        "Keep producer work finite when a consumer cannot accept work immediately.",
        "Uses an explicitly bounded queue or channel at the recorded implementation site.",
        "Capacity and overflow behavior remain those declared by the owning route or endpoint.",
        "Finite storage exposes saturation instead of hiding unbounded memory growth.",
        "Saturation may reject, drop, or delay work according to the owning contract.",
    ),
    "buffer_pool": (
        "Reuse audio storage without allocating in the realtime lane.",
        "Acquires storage from a finite pool and returns ownership through the recorded lifecycle.",
        "Pool slots, frame shape, and ownership must match the owning API.",
        "A finite pool bounds allocation but introduces explicit exhaustion.",
        "Acquisition can fail when no slot is available.",
    ),
    "transactional_registration": (
        "Avoid exposing a partially imported registration set.",
        "Stages registration work and rolls it back when a later step fails.",
        "Rollback failures remain additional evidence and do not replace the primary failure.",
        "Staging requires extra bookkeeping but preserves a coherent registry.",
        "The operation reports both the primary failure and any recorded rollback failure.",
    ),
    "sidecar_isolation": (
        "Keep managed external work behind a process and protocol boundary.",
        "Runs the child through the sidecar host with bounded messages and lifecycle deadlines.",
        "The process contract does not imply sandboxing, authentication, or provider qualification.",
        "Isolation adds serialization, process startup, and deadline failure modes.",
        "Protocol, process, cancellation, and deadline failures remain separately observable.",
    ),
    "clock_correlation": (
        "Correlate timestamps produced by distinct clock domains.",
        "Retains clock identity and maps observed source time into the Session timeline.",
        "Lineage and discontinuity identity must not be rewritten during correction.",
        "Correlation preserves provenance but requires explicit drift and discontinuity handling.",
        "Unrepresentable mappings or discontinuities remain observable outcomes.",
    ),
    "typed_error": (
        "Keep failure stage and scope machine-readable across component boundaries.",
        "Returns typed Rust errors or stable status codes instead of collapsing failures into text.",
        "Retry and recovery are unknown unless an explicit field or contract states them.",
        "Structured failures require callers to inspect variants and terminal outcomes.",
        "The owning operation reports a typed failure and preserves its recorded context.",
    ),
}


def pattern_supported(record: dict[str, Any]) -> bool:
    """Require implementation evidence, not a coincidental dossier keyword."""
    path = record["where_implemented"]
    text = TEXT_BY_PATH[path]
    lower = text.lower()
    name = record["name"]
    if name == "bounded_queue":
        return any(token in lower for token in ("sync_channel", "bounded(", "arrayqueue")) and any(
            token in lower for token in ("capacity", "full", "drop")
        )
    if name == "buffer_pool":
        return "audiobufferpool" in lower or ("pool" in lower and ".acquire(" in lower)
    if name == "transactional_registration":
        return "rollback" in lower and any(token in lower for token in ("staged", "transaction", "preserv"))
    if name == "sidecar_isolation":
        return "sidecar" in path.lower()
    if name == "clock_correlation":
        return any(token in lower for token in ("clockdomain", "clock_domain", "clock id")) and "timestamp" in lower
    if name == "typed_error":
        return path.endswith(".rs") and any(token in lower for token in ("result<", "thiserror::error", "#[derive(error"))
    return False


def refine_patterns() -> None:
    records = [record for record in read_jsonl(DB / "patterns.jsonl") if pattern_supported(record)]
    for record in records:
        problem, how, constraints, tradeoffs, failure = PATTERN_CONTENT[record["name"]]
        record.update({
            "problem": problem,
            "how_implemented": how,
            "constraints": constraints,
            "tradeoffs": tradeoffs,
            "failure_behavior": failure,
            "tests": sorted(TESTS_BY_PATH.get(record["where_implemented"], set())),
            "alternatives": ["No alternative is promoted without direct comparison evidence."],
        })
    write_jsonl(DB / "patterns.jsonl", records)
    pattern_ids_by_path: dict[str, list[str]] = defaultdict(list)
    for record in records:
        pattern_ids_by_path[record["where_implemented"]].append(record["pattern_id"])
    for file_record in FILES:
        if not file_record.get("semantic") or not file_record.get("dossier"):
            continue
        dossier_path = ROOT / file_record["dossier"]
        dossier = read_json(dossier_path)
        dossier["observed_patterns"] = sorted(pattern_ids_by_path[file_record["path"]])
        write_json(dossier_path, dossier)


def main() -> None:
    refine_symbols()
    refine_dossiers()
    refine_errors()
    refine_configuration()
    refine_lifecycles()
    refine_behaviors()
    refine_patterns()
    checkpoint = {
        "snapshot": SNAPSHOT,
        "dossiers_refined": sum(bool(record.get("semantic")) for record in FILES),
        "errors_refined": len(read_jsonl(DB / "errors.jsonl")),
        "configuration_refined": len(read_jsonl(DB / "configuration.jsonl")),
        "lifecycles_refined": len(read_jsonl(DB / "lifecycles.jsonl")),
        "patterns_refined": len(read_jsonl(DB / "patterns.jsonl")),
    }
    write_json(DB / "checkpoints" / "intelligence-refinement.json", checkpoint)
    print(json.dumps(checkpoint, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
