#!/usr/bin/env python3
"""Strict anti-gaming checks for the repository documentation compiler.

These checks deliberately test semantic shape, provenance precision, and
cross-ledger coherence.  Counts alone are not evidence that analysis happened.
The caller persists the returned report and maps every finding to hard gates.
"""

from __future__ import annotations

import json
import re
import subprocess
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


RAW_UNKNOWN = {"", "unknown", "tbd", "todo"}
MEANINGFUL_CLAIM_SECTIONS = {
    "Invariants and guarantees",
    "Procedure",
    "Verify the outcome",
    "Failure signals",
    "Evidenced causes",
    "Corrective action",
    "Recommendation",
    "Reason",
    "Tradeoff",
    "Supported environment",
    "Success",
}
PUBLIC_SEMANTIC_FIELDS = (
    "summary",
    "responsibility",
    "when_to_use",
    "when_not_to_use",
    "panic_behavior",
    "blocking_behavior",
    "cancellation",
    "thread_safety",
    "ordering",
    "backpressure",
    "limits",
    "units",
)
NATIVE_FILLER = (
    re.compile(r"^Stores the .+ used by ", re.I),
    re.compile(r"^Selects .+ behavior for ", re.I),
    re.compile(r"^Reported when the owning operation encounters ", re.I),
    re.compile(r"^Owns bounded access to ", re.I),
    re.compile(r"^Returns whether .+ applies to ", re.I),
    re.compile(r"^Executes the graph-node behavior defined for ", re.I),
    re.compile(r"^Carries the .+ value required by ", re.I),
    re.compile(r"^Names the .+ represented by ", re.I),
    re.compile(r"^`[^`]+` chooses the .+ option for this contract\.$", re.I),
    re.compile(r"^Enumerates the supported .+ cases\.$", re.I),
    re.compile(r"^Defines the public .+ value\.$", re.I),
    re.compile(r"^Names the .+ type used by the public API\.$", re.I),
    re.compile(r"^Types and operations for .+\.$", re.I),
    re.compile(r"^Describes the .+ contract\.$", re.I),
    re.compile(r"^Returns `[^`]+` when the owning operation detects ", re.I),
    re.compile(r"^Classifies failures reported as ", re.I),
    re.compile(r"^Reports the .+ failure case defined by ", re.I),
    re.compile(r"^Reports a [aeiou]", re.I),
    re.compile(
        r"^(?:Adds|Cancels|Compiles|Executes|Inserts|Joins|Plans|Polls for|Prepares|Publishes|Reads|Receives|Records|Registers|Removes|Resolves|Sends|Spawns|Starts|Stops|Validates|Writes) [a-z]+ for `",
        re.I,
    ),
)


def _read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def _read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def _identifier(record: dict[str, Any]) -> str:
    for key in (
        "file_id",
        "symbol_id",
        "edge_id",
        "behavior_id",
        "error_id",
        "config_id",
        "lifecycle_id",
        "protocol_id",
        "claim_id",
        "page_id",
        "unknown_id",
    ):
        if record.get(key):
            return str(record[key])
    return "unidentified-record"


def run_strict_protocol_audit(
    root: Path,
    state: dict[str, Any],
    *,
    files: list[dict[str, Any]],
    symbols: list[dict[str, Any]],
    edges: list[dict[str, Any]],
    behaviors: list[dict[str, Any]],
    errors: list[dict[str, Any]],
    configuration: list[dict[str, Any]],
    lifecycles: list[dict[str, Any]],
    protocols: list[dict[str, Any]],
    pages: list[dict[str, Any]],
    claims: list[dict[str, Any]],
    unknowns: list[dict[str, Any]],
) -> dict[str, Any]:
    issues: list[dict[str, Any]] = []
    metrics: dict[str, Any] = {}

    def add(
        code: str,
        category: str,
        gates: tuple[int, ...],
        message: str,
        affected: list[str] | tuple[str, ...],
        **detail: Any,
    ) -> None:
        identifiers = list(dict.fromkeys(str(item) for item in affected))
        issues.append(
            {
                "code": code,
                "category": category,
                "gates": list(gates),
                "message": message,
                "affected_count": len(identifiers),
                "affected_examples": identifiers[:50],
                **detail,
            }
        )

    file_by_path = {record["path"]: record for record in files}
    symbol_by_id = {record["symbol_id"]: record for record in symbols}
    record_by_id: dict[str, dict[str, Any]] = {}
    for records, field in (
        (symbols, "symbol_id"), (behaviors, "behavior_id"),
        (errors, "error_id"), (configuration, "config_id"),
        (lifecycles, "lifecycle_id"), (protocols, "protocol_id"),
        (_read_jsonl(root / ".doc-intel" / "tests.jsonl"), "test_id"),
        (_read_jsonl(root / ".doc-intel" / "examples.jsonl"), "example_id"),
        (_read_jsonl(root / ".doc-intel" / "patterns.jsonl"), "pattern_id"),
    ):
        for record in records:
            if record.get(field):
                record_by_id[str(record[field])] = record

    # A forced rebuild currently captures its own deletions as if they were the
    # repository's initial state.  That is not a trustworthy freeze record.
    snapshot_record = _read_json(root / ".doc-intel" / "snapshot.json")
    initialization_status = snapshot_record.get("working_tree_status_at_initialization", [])
    generated_deletions = [
        item for item in initialization_status
        if re.match(r"\s*D\s+\.doc-intel/(?:files|checkpoints)/", str(item))
    ]
    metrics["snapshot_initialization_status_entries"] = len(initialization_status)
    if generated_deletions:
        add(
            "SNAPSHOT-SELF-DELETION-STATUS",
            "snapshot",
            (0, 12),
            "The frozen snapshot records documentation-compiler cleanup as initial working-tree state.",
            generated_deletions,
        )

    dossiers: list[tuple[dict[str, Any], dict[str, Any]]] = []
    file_line_counts: dict[str, int] = {}
    for record in files:
        if record.get("semantic") and record.get("dossier"):
            dossier = _read_json(root / record["dossier"])
            dossiers.append((record, dossier))
            file_line_counts[record["path"]] = int(dossier.get("line_count", 0))

    generic_purpose: list[str] = []
    raw_io: list[str] = []
    whole_file_evidence: list[str] = []
    non_responsibility_usage: defaultdict[str, list[str]] = defaultdict(list)
    dossier_calls = 0
    for file_record, dossier in dossiers:
        path = file_record["path"]
        purpose = dossier.get("purpose", {})
        purpose_text = purpose.get("text", "") if isinstance(purpose, dict) else str(purpose)
        if re.match(r"^Implements .+ responsibilities, including ", purpose_text):
            generic_purpose.append(path)
        for value in dossier.get("non_responsibilities", []):
            non_responsibility_usage[str(value)].append(path)
        raw_values = [
            str(value)
            for field in ("inputs", "outputs")
            for value in dossier.get(field, [])
        ]
        if any(
            re.match(r"^(?:#\[cfg|(?:pub\s+)?fn\s|use\s|super::|return\s+Err|if\s+let\s+Err)", value)
            for value in raw_values
        ):
            raw_io.append(path)
        evidence = dossier.get("evidence", [])
        if evidence and all(
            item.get("lines") == [1, dossier.get("line_count")] for item in evidence
        ):
            whole_file_evidence.append(path)
        dossier_calls += len(dossier.get("calls", []))

    repeated_non_responsibilities = {
        text: paths for text, paths in non_responsibility_usage.items() if len(paths) > 10
    }
    metrics.update(
        {
            "dossiers": len(dossiers),
            "generic_dossier_purposes": len(generic_purpose),
            "dossiers_with_raw_io_syntax": len(raw_io),
            "dossiers_with_only_whole_file_evidence": len(whole_file_evidence),
            "dossier_call_references": dossier_calls,
        }
    )
    if generic_purpose:
        add(
            "DOSSIER-TEMPLATED-PURPOSE",
            "dossiers",
            (1, 12),
            "Dossier purposes use a mechanically substituted 'Implements ... responsibilities' template.",
            generic_purpose,
        )
    if repeated_non_responsibilities:
        affected = [path for paths in repeated_non_responsibilities.values() for path in paths]
        add(
            "DOSSIER-REPEATED-NONRESPONSIBILITY",
            "dossiers",
            (1, 12),
            "Identical generic non-responsibility text is reused across more than ten semantic files.",
            affected,
            repeated_text_counts={text: len(paths) for text, paths in repeated_non_responsibilities.items()},
        )
    if raw_io:
        add(
            "DOSSIER-RAW-SYNTAX-AS-SEMANTICS",
            "dossiers",
            (1, 4, 12),
            "Dossier inputs or outputs contain raw declarations/control-flow rather than analyzed semantics.",
            raw_io,
        )
    if whole_file_evidence:
        add(
            "DOSSIER-IMPRECISE-EVIDENCE",
            "dossiers",
            (1, 12),
            "Dossiers rely only on whole-file evidence spans instead of precise responsibility evidence.",
            whole_file_evidence,
        )

    public_symbols = [record for record in symbols if record.get("public_api")]
    unknown_by_field: dict[str, list[str]] = {}
    for field in PUBLIC_SEMANTIC_FIELDS:
        affected = [
            record["symbol_id"]
            for record in public_symbols
            if str(record.get(field, "")).strip().lower() in RAW_UNKNOWN
        ]
        if affected:
            unknown_by_field[field] = affected
    unresolved_symbol_ids = sorted({item for values in unknown_by_field.values() for item in values})
    metrics["public_symbols"] = len(public_symbols)
    metrics["public_symbol_unknowns_by_field"] = {
        field: len(values) for field, values in unknown_by_field.items()
    }
    if unresolved_symbol_ids:
        add(
            "SYMBOL-RAW-UNKNOWN-SEMANTICS",
            "symbols",
            (2, 9, 12),
            "Public symbols retain raw unknown semantic fields without per-record disposition.",
            unresolved_symbol_ids,
            counts_by_field={field: len(values) for field, values in unknown_by_field.items()},
        )
    placeholder_signatures = [
        record["symbol_id"]
        for record in public_symbols
        if isinstance(record.get("signature"), str)
        and str(record["signature"]).startswith("See source excerpt")
    ]
    if placeholder_signatures:
        add(
            "SYMBOL-PLACEHOLDER-SIGNATURE",
            "symbols",
            (2, 9, 12),
            "Public compiler records contain a placeholder instead of a compiler signature.",
            placeholder_signatures,
        )

    edge_kinds = Counter(str(record.get("kind")) for record in edges)
    edge_statuses = Counter(str(record.get("status")) for record in edges)
    metrics["edge_kinds"] = dict(sorted(edge_kinds.items()))
    metrics["edge_statuses"] = dict(sorted(edge_statuses.items()))
    if dossier_calls and not edge_kinds.get("CALLS"):
        add(
            "GRAPH-MISSING-CALL-EDGES",
            "relationships",
            (3, 12),
            "Dossiers report call references but the relationship graph contains no CALLS edges.",
            [f"{dossier_calls} dossier call references"],
        )
    grep = subprocess.run(
        ["git", "grep", "-n", "-E", r"impl.* for ", state["snapshot"], "--", "src", "native"],
        cwd=root,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    implementation_sites = [line for line in grep.stdout.splitlines() if line]
    metrics["implementation_sites"] = len(implementation_sites)
    if implementation_sites and not edge_kinds.get("IMPLEMENTS"):
        add(
            "GRAPH-MISSING-IMPLEMENTATION-EDGES",
            "relationships",
            (3, 12),
            "The frozen source contains trait implementation sites but the graph contains no IMPLEMENTS edges.",
            implementation_sites,
        )
    ffi_files = [
        record["path"]
        for record in files
        if record.get("file_kind") in {"public_ffi", "native_source"}
    ]
    if ffi_files and not (edge_statuses.get("dynamic") or edge_statuses.get("external")):
        add(
            "GRAPH-MISSING-BOUNDARY-EDGES",
            "relationships",
            (3, 4, 12),
            "Native/FFI files exist but no dynamic or external boundary edge is represented.",
            ffi_files,
        )

    contaminated_behaviors: list[str] = []
    behavior_unknowns: list[str] = []
    for behavior in behaviors:
        test_like_steps = {
            str(step.get("operation"))
            for step in behavior.get("steps", [])
            if str(step.get("operation", "")).startswith("given_")
        }
        if behavior.get("classification") == "TESTED" and len(test_like_steps) > 1:
            contaminated_behaviors.append(behavior["behavior_id"])
        if any(str(value).strip().lower() in RAW_UNKNOWN for value in behavior.get("errors", [])):
            behavior_unknowns.append(behavior["behavior_id"])
    metrics["contaminated_test_behaviors"] = len(contaminated_behaviors)
    if contaminated_behaviors:
        add(
            "BEHAVIOR-WHOLE-FILE-STEP-CONTAMINATION",
            "behaviors",
            (4, 10, 12),
            "Test behavior steps include names of other tests from the same source file.",
            contaminated_behaviors,
        )
    if behavior_unknowns:
        add(
            "BEHAVIOR-RAW-UNKNOWN-ERROR",
            "behaviors",
            (4, 10, 12),
            "Behavior records contain raw unknown errors without an explicit unknown-ledger reference.",
            behavior_unknowns,
        )

    empty_error_lineage = [
        record["error_id"]
        for record in errors
        if not any(record.get(field) for field in (
            "created_at", "propagates_through", "wrapped_by", "translated_to"
        ))
        and not record.get("lineage_disposition")
    ]
    external_unknown = [
        record["error_id"]
        for record in errors
        if str(record.get("external_representation", "")).strip().lower() in RAW_UNKNOWN
    ]
    generic_error_actions = [
        record["error_id"]
        for record in errors
        if str(record.get("developer_action", "")).startswith("Preserve the typed failure and follow")
    ]
    metrics.update(
        {
            "errors_without_lineage_or_disposition": len(empty_error_lineage),
            "errors_with_unknown_external_representation": len(external_unknown),
            "errors_with_generic_developer_action": len(generic_error_actions),
        }
    )
    if empty_error_lineage:
        add(
            "ERROR-LINEAGE-ABSENT",
            "errors",
            (4, 10, 12),
            "Error records have no creation/propagation/wrapping/translation lineage and no explicit non-applicability disposition.",
            empty_error_lineage,
        )
    if external_unknown:
        add(
            "ERROR-EXTERNAL-REPRESENTATION-UNKNOWN",
            "errors",
            (4, 10, 12),
            "Error external representations remain raw unknown values.",
            external_unknown,
        )
    if generic_error_actions:
        add(
            "ERROR-GENERIC-DEVELOPER-ACTION",
            "errors",
            (4, 10, 12),
            "Error developer actions repeat a generic preservation instruction instead of error-specific recovery.",
            generic_error_actions,
        )

    invalid_configuration = [
        record["config_id"]
        for record in configuration
        if record.get("kind") == "variant"
        or "Error" in str(record.get("parent", ""))
    ]
    generic_configuration = [
        record["config_id"]
        for record in configuration
        if str(record.get("when_read", "")).startswith("Read at the recorded construction")
    ]
    metrics["configuration_error_or_variant_records"] = len(invalid_configuration)
    if invalid_configuration:
        add(
            "CONFIGURATION-DENOMINATOR-POLLUTED",
            "configuration",
            (4, 10, 12),
            "The configuration denominator includes enum/error variants that are not configuration inputs.",
            invalid_configuration,
        )
    if generic_configuration:
        add(
            "CONFIGURATION-GENERIC-READ-SITES",
            "configuration",
            (4, 10, 12),
            "Configuration records substitute a shared generic read-time statement for exact consumption semantics.",
            generic_configuration,
        )

    invalid_lifecycles: list[str] = []
    generic_lifecycles: list[str] = []
    for record in lifecycles:
        symbol = symbol_by_id.get(record.get("symbol_id"), {})
        if symbol.get("kind") in {"variant", "enum", "struct_field", "constant"}:
            invalid_lifecycles.append(record["lifecycle_id"])
        if (
            record.get("source_state") in {"declared_or_compiled", "state_declared_by_owning_type"}
            or str(record.get("guard", "")).startswith("The receiver, arguments, and current lifecycle")
        ):
            generic_lifecycles.append(record["lifecycle_id"])
    metrics["non_operation_lifecycle_records"] = len(invalid_lifecycles)
    if invalid_lifecycles:
        add(
            "LIFECYCLE-NON-OPERATION-DENOMINATOR",
            "lifecycles",
            (4, 10, 12),
            "Lifecycle transitions are assigned to enum variants or other non-operation symbols.",
            invalid_lifecycles,
        )
    if generic_lifecycles:
        add(
            "LIFECYCLE-GENERIC-TRANSITIONS",
            "lifecycles",
            (4, 10, 12),
            "Lifecycle source states/guards are keyword templates rather than validated transitions.",
            generic_lifecycles,
        )

    protocol_fields = {
        "message_schema", "encoding", "decoding", "versioning", "limits",
        "endpoints", "lifecycle",
    }
    incomplete_protocols = [
        record["protocol_id"]
        for record in protocols
        if protocol_fields - record.keys()
    ]
    trivial_protocols = [
        record["protocol_id"]
        for record in protocols
        if record.get("name") in {"new", "default", "clone", "drop"}
    ]
    metrics["protocol_records_missing_contract_fields"] = len(incomplete_protocols)
    if incomplete_protocols:
        add(
            "PROTOCOL-CONTRACT-FIELDS-ABSENT",
            "protocols",
            (4, 10, 12),
            "Protocol records lack schema, encoding/decoding, versioning, limits, endpoint, and lifecycle dispositions.",
            incomplete_protocols,
        )
    if trivial_protocols:
        add(
            "PROTOCOL-TRIVIAL-SYMBOLS",
            "protocols",
            (4, 10, 12),
            "Constructors or utility methods are counted as protocol elements without protocol-specific evidence.",
            trivial_protocols,
        )

    unknown_ids = {record.get("unknown_id") for record in unknowns}
    metrics["explicit_unknown_ledger_records"] = len(unknown_ids)
    if unresolved_symbol_ids or behavior_unknowns or external_unknown:
        add(
            "UNKNOWN-LEDGER-NOT-RECONCILED",
            "unknowns",
            (4, 9, 10, 12),
            "Raw record-level unknowns are not linked to explicit unknown-ledger entries.",
            unresolved_symbol_ids + behavior_unknowns + external_unknown,
            explicit_unknown_ledger_records=len(unknown_ids),
        )

    claim_sections_by_page: defaultdict[str, set[str]] = defaultdict(set)
    summary_usage: defaultdict[str, list[str]] = defaultdict(list)
    broad_claims: list[str] = []
    claim_record_mismatches: list[str] = []
    code_evidence_mismatches: list[str] = []
    page_sections: dict[str, dict[str, str]] = {}
    for page in pages:
        path = root / page["path"]
        if not path.exists():
            continue
        text = path.read_text()
        matches = list(re.finditer(r"(?m)^##\s+(.+?)\s*$", text))
        page_sections[page["page_id"]] = {
            match.group(1).strip(): text[
                match.end():matches[index + 1].start() if index + 1 < len(matches) else len(text)
            ]
            for index, match in enumerate(matches)
        }

    def overlaps(left: dict[str, Any], right: dict[str, Any]) -> bool:
        if left.get("path") != right.get("path"):
            return False
        left_lines = left.get("lines", [])
        right_lines = right.get("lines", [])
        return (
            len(left_lines) == 2 and len(right_lines) == 2
            and int(left_lines[0]) <= int(right_lines[1])
            and int(right_lines[0]) <= int(left_lines[1])
        )

    for claim in claims:
        page_id = str(claim.get("documentation_page"))
        section = str(claim.get("section"))
        claim_id = str(claim.get("claim_id"))
        claim_sections_by_page[page_id].add(section)
        summary_usage[str(claim.get("claim_summary", ""))].append(str(claim.get("claim_id")))
        for evidence in claim.get("evidence", []):
            source_path = str(evidence.get("path", ""))
            if source_path.startswith("examples/"):
                continue
            # A C harness or repository test script can legitimately be one
            # executable test record covering its complete file.  The
            # anti-gaming rule targets broad implementation/prose claims that
            # cite an entire source file instead of a precise declaration.
            if evidence.get("classification") == "TESTED":
                continue
            if source_path in file_by_path and evidence.get("lines") == [
                1, file_line_counts.get(source_path)
            ]:
                broad_claims.append(claim_id)
                break
        section_text = page_sections.get(page_id, {}).get(section, "")
        identifiers = set(re.findall(
            r"\b(?:sym|test|example|behavior|error|config|life|protocol|pattern)-[0-9a-f]{8,}\b",
            section_text,
        ))
        for identifier in sorted(identifiers):
            record = record_by_id.get(identifier)
            if not record:
                claim_record_mismatches.append(f"{claim_id}:unknown-record:{identifier}")
                continue
            if not any(
                overlaps(claim_evidence, record_evidence)
                for claim_evidence in claim.get("evidence", [])
                for record_evidence in record.get("evidence", [])
            ):
                claim_record_mismatches.append(f"{claim_id}:missing-evidence:{identifier}")

        if section == "Concrete repository example":
            for identifier in sorted(item for item in identifiers if item.startswith("example-")):
                record = record_by_id.get(identifier, {})
                source_path = str(record.get("path", ""))
                if not source_path:
                    continue
                frozen = subprocess.run(
                    ["git", "show", f"{state['snapshot']}:{source_path}"],
                    cwd=root, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                    check=False,
                )
                code_blocks = re.findall(r"```(?:rust|c|cpp)\n(.*?)\n```", section_text, re.S)
                record_evidence = record.get("evidence", [])
                evidence_covers_source = any(
                    claim_evidence.get("path") == source_path
                    and len(claim_evidence.get("lines", [])) == 2
                    and len(source_evidence.get("lines", [])) == 2
                    and int(claim_evidence["lines"][0]) <= int(source_evidence["lines"][0])
                    and int(claim_evidence["lines"][1]) >= int(source_evidence["lines"][1])
                    for claim_evidence in claim.get("evidence", [])
                    for source_evidence in record_evidence
                )
                if (
                    frozen.returncode != 0
                    or frozen.stdout.strip() not in {block.strip() for block in code_blocks}
                    or not evidence_covers_source
                ):
                    code_evidence_mismatches.append(f"{claim_id}:source-copy:{identifier}")
    repeated_claims = {
        summary: ids for summary, ids in summary_usage.items()
        if summary and len(ids) > 10
    }
    uncovered_sections: list[str] = []
    for page in pages:
        path = root / page["path"]
        if not path.exists():
            continue
        headings = {
            match.group(1).strip()
            for match in re.finditer(r"(?m)^##\s+(.+?)\s*$", path.read_text())
        }
        for heading in sorted(headings & MEANINGFUL_CLAIM_SECTIONS):
            if heading not in claim_sections_by_page.get(page["page_id"], set()):
                uncovered_sections.append(f"{page['page_id']}:{heading}")
    evidence_index_claims = [
        claim["claim_id"] for claim in claims if claim.get("section") == "Evidence index"
    ]
    metrics.update(
        {
            "claims": len(claims),
            "claims_using_whole_file_evidence": len(set(broad_claims)),
            "evidence_index_claims": len(evidence_index_claims),
            "meaningful_sections_without_claim_mapping": len(uncovered_sections),
            "claims_missing_explicit_record_evidence": len(claim_record_mismatches),
            "repository_examples_not_exactly_synchronized": len(code_evidence_mismatches),
        }
    )
    if broad_claims:
        add(
            "CLAIM-IMPRECISE-WHOLE-FILE-EVIDENCE",
            "claims",
            (7, 8, 9, 10, 12),
            "Documentation claims use whole files as evidence instead of precise spans/symbols.",
            broad_claims,
        )
    if uncovered_sections:
        add(
            "CLAIM-MEANINGFUL-SECTIONS-UNMAPPED",
            "claims",
            (7, 8, 10, 12),
            "Substantive documentation sections make claims that are absent from the claim ledger.",
            uncovered_sections,
        )
    if claim_record_mismatches:
        add(
            "CLAIM-EXPLICIT-RECORD-EVIDENCE-MISMATCH",
            "claims",
            (7, 8, 9, 10, 12),
            "A documentation section names a stable evidence record but its claim does not include overlapping evidence from that record.",
            claim_record_mismatches,
        )
    if code_evidence_mismatches:
        add(
            "EXAMPLE-SOURCE-COPY-DRIFT",
            "examples",
            (8, 11, 12),
            "A repository example published as executable code is not an exact copy of its frozen source record.",
            code_evidence_mismatches,
        )
    if repeated_claims:
        add(
            "CLAIM-REPEATED-GENERIC-SUMMARIES",
            "claims",
            (7, 8, 10, 12),
            "Identical claim summaries are reused across more than ten records.",
            [identifier for ids in repeated_claims.values() for identifier in ids],
            repeated_summary_counts={summary: len(ids) for summary, ids in repeated_claims.items()},
        )
    if claims and len(evidence_index_claims) / len(claims) > 0.5:
        add(
            "CLAIM-LEDGER-DOMINATED-BY-EVIDENCE-INDEX",
            "claims",
            (7, 8, 9, 10, 12),
            "More than half of claim records describe generated evidence-index rows rather than prose assertions.",
            evidence_index_claims,
            ratio=round(len(evidence_index_claims) / len(claims), 4),
        )

    how_to_without_code: list[str] = []
    for page in pages:
        if page.get("doc_class") != "how-to":
            continue
        text = (root / page["path"]).read_text()
        if not re.search(r"(?m)^```", text):
            how_to_without_code.append(page["page_id"])
    metrics["how_to_pages_without_code_fences"] = len(how_to_without_code)
    if how_to_without_code:
        add(
            "HOWTO-NO-EXECUTABLE-SYNTAX",
            "people_documentation",
            (8, 12),
            "How-to pages contain no code/command blocks showing the concrete API procedure.",
            how_to_without_code,
        )

    native_docs = _read_jsonl(root / ".doc-intel" / "native-docs.jsonl")
    native_by_symbol = {
        record.get("symbol_id"): str(record.get("documentation", "")).strip()
        for record in native_docs
    }
    missing_native_docs = [
        record["symbol_id"] for record in public_symbols
        if not native_by_symbol.get(record["symbol_id"])
    ]
    filler_native_docs = [
        record["symbol_id"]
        for record in native_docs
        if any(pattern.search(str(record.get("documentation", ""))) for pattern in NATIVE_FILLER)
    ]
    metrics["native_docs_matching_generic_filler"] = len(filler_native_docs)
    metrics["public_symbols_without_native_documentation"] = len(missing_native_docs)
    if filler_native_docs:
        add(
            "NATIVE-DOC-GENERIC-FILLER",
            "native_documentation",
            (9, 12),
            "Public source documentation matches mechanical field/variant/method filler patterns.",
            filler_native_docs,
        )
    if missing_native_docs:
        add(
            "NATIVE-DOC-PUBLIC-SYMBOL-MISSING",
            "native_documentation",
            (9, 12),
            "An intentionally public compiler symbol has no non-empty native documentation record.",
            missing_native_docs,
        )

    issues_by_gate: defaultdict[str, list[str]] = defaultdict(list)
    for issue in issues:
        summary = f"{issue['code']}: {issue['message']} ({issue['affected_count']} affected)"
        for gate in issue["gates"]:
            issues_by_gate[str(gate)].append(summary)

    return {
        "schema_version": 1,
        "snapshot": state.get("snapshot"),
        "protocol_sha256": state.get("protocol_sha256"),
        "contract_sha256": state.get("contract_sha256"),
        "status": "pass" if not issues else "fail",
        "metrics": metrics,
        "issues": issues,
        "failures_by_gate": dict(sorted(issues_by_gate.items(), key=lambda item: int(item[0]))),
    }
