#!/usr/bin/env python3
"""Author evidence-backed pages from the reviewed documentation work queue."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import tomllib
from collections import defaultdict
from pathlib import Path, PurePosixPath
from typing import Any

from documentation_content import BEST_PRACTICES, CONCEPTS, GUIDES, TROUBLESHOOTING


ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / ".doc-intel"
BT = chr(96)
FENCE = BT * 3
PACKAGE_METADATA = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]
PACKAGE_VERSION = PACKAGE_METADATA["version"]
RUST_VERSION = PACKAGE_METADATA["rust-version"].removesuffix(".0")


def read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def write_jsonl(path: Path, records: list[dict[str, Any]]) -> None:
    with path.open("w") as handle:
        for record in records:
            handle.write(json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n")


STATE = read_json(DB / "state.json")
WORKSPACE_QUALIFICATION = read_json(DB / "checkpoints" / "workspace-qualification.json") \
    if (DB / "checkpoints" / "workspace-qualification.json").exists() else None
FILES = read_jsonl(DB / "repository-manifest.jsonl")
FILE_BY_PATH = {record["path"]: record for record in FILES}
PAGES = read_jsonl(DB / "page-manifest.jsonl")
CAPABILITIES = read_jsonl(DB / "capabilities.jsonl")
CAP_BY_ID = {record["capability_id"]: record for record in CAPABILITIES}
JOURNEYS = read_jsonl(DB / "user-journeys.jsonl")
SYMBOLS = read_jsonl(DB / "symbol-manifest.jsonl")
NATIVE_DOCS = {
    record["symbol_id"]: record["documentation"]
    for record in read_jsonl(DB / "native-docs.jsonl")
} if (DB / "native-docs.jsonl").exists() else {}
TESTS = read_jsonl(DB / "tests.jsonl")
EXAMPLES = read_jsonl(DB / "examples.jsonl")
ERRORS = read_jsonl(DB / "errors.jsonl")
CONFIGURATION = read_jsonl(DB / "configuration.jsonl")
BEHAVIORS = read_jsonl(DB / "behaviors.jsonl")
LIFECYCLES = read_jsonl(DB / "lifecycles.jsonl")
PROTOCOLS = read_jsonl(DB / "protocols.jsonl")
PATTERNS = read_jsonl(DB / "patterns.jsonl")
CONFLICTS = read_jsonl(DB / "conflicts.jsonl")
TERMINOLOGY = read_json(DB / "terminology.json")["terms"]
SNAPSHOT = STATE["snapshot"]

EVIDENCE_RECORDS: dict[str, dict[str, Any]] = {}
for records, identifier in (
    (SYMBOLS, "symbol_id"),
    (TESTS, "test_id"),
    (EXAMPLES, "example_id"),
    (ERRORS, "error_id"),
    (CONFIGURATION, "config_id"),
    (BEHAVIORS, "behavior_id"),
    (LIFECYCLES, "lifecycle_id"),
    (PROTOCOLS, "protocol_id"),
    (PATTERNS, "pattern_id"),
):
    for evidence_record in records:
        if evidence_record.get(identifier):
            EVIDENCE_RECORDS[str(evidence_record[identifier])] = evidence_record


def domain_for_path(path: str) -> str:
    parts = PurePosixPath(path).parts
    if len(parts) > 1 and parts[0] == "src":
        return parts[1].removesuffix(".rs")
    if len(parts) > 1 and parts[0] == "native":
        return "platform"
    if len(parts) > 1 and parts[0] in {"tests", "examples", "benches"}:
        name = parts[1].lower()
        rules = (
            ("abi", ("abi", "conformance_fixture", "macos_native_ring")),
            ("capture", ("capture", "audio_input", "external_source")),
            ("codec", ("codec", "opus")),
            ("connector", ("connector",)),
            ("endpoint", ("endpoint",)),
            ("frame", ("frame", "buffer_pool")),
            ("graph", ("graph", "operator", "typed_edge", "operator-consumer")),
            ("native_extension", ("extension",)),
            ("recording", ("record",)),
            ("runtime", ("runtime",)),
            ("session", ("session", "product_quickstart")),
            ("timing", ("timing", "clock")),
            ("integration", ("whisper",)),
        )
        for domain, tokens in rules:
            if any(token in name for token in tokens):
                return domain
        return "repository"
    if len(parts) > 1 and parts[0] == "scripts":
        return "release"
    return parts[0].removesuffix(".rs") if parts else "repository"


def record_domain(record: dict[str, Any]) -> str:
    if record.get("domain"):
        return record["domain"]
    evidence = record.get("evidence") or []
    path = evidence[0].get("path", "repository") if evidence else record.get("source_file", "repository")
    return domain_for_path(path)


def relevant(page: dict[str, Any], record: dict[str, Any]) -> bool:
    return "all" in page["domains"] or record_domain(record) in page["domains"]


def md(value: Any) -> str:
    return re.sub(r"\s+", " ", str(value).replace("|", "\\|").replace("\n", " ")).strip()


def code(value: Any) -> str:
    return f"{BT}{md(value)}{BT}"


def frozen_text(path: str) -> str:
    return subprocess.check_output(
        ["git", "show", f"{SNAPSHOT}:{path}"], cwd=ROOT, text=True
    )


def source_ref(record: dict[str, Any]) -> str:
    evidence = record.get("evidence") or []
    if evidence:
        item = evidence[0]
        return code(f"{item.get('path')}:{item.get('lines', ['?'])[0]}")
    return code(f"{record.get('source_file', 'unknown')}:{record.get('source_lines', ['?'])[0]}")


def page_symbols(page: dict[str, Any], limit: int = 18) -> list[dict[str, Any]]:
    candidates = [symbol for symbol in SYMBOLS if symbol["public_api"] and relevant(page, symbol)]
    words = {word.lower() for word in re.findall(r"[A-Za-z]+", page["title"]) if len(word) > 3}
    evidence_paths = {item["path"] for item in page["evidence"]}
    for capability_id in page["capability_ids"]:
        evidence_paths.update(item["path"] for item in CAP_BY_ID[capability_id]["evidence"])
    rank = {"module": 0, "trait": 1, "struct": 2, "enum": 3, "function": 4, "type_alias": 5, "constant": 6, "variant": 7, "struct_field": 8}
    candidates.sort(key=lambda symbol: (
        0 if symbol["source_file"] in evidence_paths else 1,
        -sum(word in symbol["name"].lower() or word in symbol["qualified_name"].lower() for word in words),
        rank.get(symbol["kind"], 20),
        symbol["qualified_name"],
    ))
    return candidates[:limit]


def page_tests(page: dict[str, Any], limit: int = 12) -> list[dict[str, Any]]:
    records = [record for record in TESTS if relevant(page, record)]
    words = {word.lower() for word in re.findall(r"[A-Za-z]+", page["title"]) if len(word) > 3}
    evidence_paths = {item["path"] for item in page["evidence"]}
    records.sort(key=lambda record: (
        0 if record["path"] in evidence_paths else 1,
        -sum(word in record["name"].lower() for word in words),
        record["path"], record["name"],
    ))
    return records[:limit]


def page_errors(page: dict[str, Any], limit: int = 12) -> list[dict[str, Any]]:
    evidence_paths = {item["path"] for item in page["evidence"]}
    terms = {
        word.lower() for word in re.findall(r"[A-Za-z]+", page["title"])
        if len(word) > 3 and word.lower() not in {"fails", "failure", "error", "reports"}
    }
    scored = []
    for record in ERRORS:
        if not relevant(page, record):
            continue
        searchable = f"{record['type']} {record.get('variant') or ''} {record.get('trigger_condition') or ''}".lower()
        score = (20 if record["defined_at"]["path"] in evidence_paths else 0) + sum(term in searchable for term in terms)
        if score:
            scored.append((score, record))
    scored.sort(key=lambda item: (-item[0], item[1]["type"], item[1].get("variant") or ""))
    return [record for _score, record in scored[:limit]]


def related_pages(page: dict[str, Any], limit: int = 8) -> list[dict[str, Any]]:
    own = set(page["capability_ids"])
    records = [candidate for candidate in PAGES if candidate["page_id"] != page["page_id"] and own.intersection(candidate["capability_ids"])]
    records.sort(key=lambda candidate: (-len(own.intersection(candidate["capability_ids"])), candidate["gate"], candidate["title"]))
    return records[:limit]


def make_claim(identifier: str, page: dict[str, Any], section: str, summary: str, evidence: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "claim_id": identifier,
        "documentation_page": page["page_id"],
        "documentation_path": page["path"],
        "section": section,
        "claim_summary": summary,
        "evidence": evidence,
        "last_verified_commit": SNAPSHOT,
        "status": "verified",
    }


def unique_evidence(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Return stable, de-duplicated evidence without widening source spans."""
    selected: dict[tuple[Any, ...], dict[str, Any]] = {}
    for item in items:
        key = (
            item.get("path"), tuple(item.get("lines", [])), item.get("symbol"),
            item.get("classification"), item.get("content_hash"),
        )
        selected[key] = item
    return [selected[key] for key in sorted(selected, key=lambda value: tuple(str(part) for part in value))]


def capability_evidence(page: dict[str, Any]) -> list[dict[str, Any]]:
    selected: list[dict[str, Any]] = []
    for capability_id in page["capability_ids"]:
        # A capability can draw on several representative files.  Preserve one
        # precise anchor per file rather than copying every dossier symbol into
        # every prose claim that mentions the capability.
        seen_paths: set[str] = set()
        for item in CAP_BY_ID[capability_id]["evidence"]:
            if item["path"] in seen_paths:
                continue
            seen_paths.add(item["path"])
            selected.append(item)
    return unique_evidence(selected)


def page_examples(page: dict[str, Any], section_text: str = "") -> list[dict[str, Any]]:
    page_paths = {item["path"] for item in page["evidence"]}
    return [
        record for record in EXAMPLES
        if record["path"] in page_paths or record["path"] in section_text
    ]


def section_claim_evidence(
    page: dict[str, Any], section: str, section_text: str
) -> list[dict[str, Any]]:
    """Select evidence actually used by one authored section.

    Stable record IDs printed in prose or tables take precedence.  Task and
    outcome sections additionally inherit the exact example/test/error records
    used to render them.  This prevents a broad page seed from masquerading as
    proof for unrelated test results or failure variants.
    """
    selected: list[dict[str, Any]] = []
    for identifier, record in EVIDENCE_RECORDS.items():
        if identifier in section_text:
            selected.extend(record.get("evidence", []))

    examples = page_examples(page, section_text)
    lower_section = section.lower()
    if lower_section in {
        "prerequisites", "procedure", "concrete repository example",
        "verify the outcome", "run the first session", "success",
    }:
        selected.extend(
            item for record in examples for item in record.get("evidence", [])
        )
    if lower_section in {
        "verify the outcome", "executable evidence", "invariants and guarantees",
        "success", "diagnosis",
    }:
        selected.extend(
            item for record in page_tests(page) for item in record.get("evidence", [])
        )
    if lower_section == "concrete repository example" and not examples:
        selected.extend(
            item for record in page_tests(page, 1) for item in record.get("evidence", [])
        )
    if any(token in lower_section for token in ("failure", "cause", "corrective", "retry")) \
            or lower_section == "important consequence":
        selected.extend(
            item for record in page_errors(page) for item in record.get("evidence", [])
        )
    if any(token in lower_section for token in ("configuration", "secret", "precedence")):
        selected.extend(
            item for record in [item for item in CONFIGURATION if relevant(page, item)][:12]
            for item in record.get("evidence", [])
        )
    if any(token in lower_section for token in ("lifecycle", "transition", "ownership")):
        selected.extend(
            item for record in [item for item in LIFECYCLES if relevant(page, item)][:12]
            for item in record.get("evidence", [])
        )
    if "protocol" in lower_section or "abi" in lower_section:
        selected.extend(
            item for record in PROTOCOLS if relevant(page, record)
            for item in record.get("evidence", [])
        )
    if lower_section == "current workspace qualification":
        selected.extend(
            item for record in CONFLICTS
            if record.get("kind") == "cross_repository_qualification"
            for item in record.get("evidence", [])
        )
    if lower_section in {"recommendation", "reason", "tradeoff", "when it does not apply"}:
        selected.extend(
            item for record in [item for item in PATTERNS if relevant(page, item)][:12]
            for item in record.get("evidence", [])
        )

    # Conceptual prose is grounded in the reviewed capability model.  Add it
    # after section-specific records so a missing exact record cannot be hidden
    # by a generic page seed.
    return unique_evidence(selected or capability_evidence(page) or page["evidence"])


def add_surface_claims(page: dict[str, Any], records: list[dict[str, Any]], prefix: str, id_field: str, summary_field: str) -> list[dict[str, Any]]:
    result = []
    for number, record in enumerate(records, 1):
        summary = record.get(summary_field) or record.get("name") or record.get("type") or record[id_field]
        result.append(make_claim(
            f"CLM-{page['page_id']}-{prefix}-{number:04d}",
            page,
            "Evidence index",
            f"The frozen inventory records {summary} as {prefix.lower()} evidence {record[id_field]}.",
            record["evidence"],
        ))
    return result


def claims_for(page: dict[str, Any], body: str) -> list[dict[str, Any]]:
    """Map factual/prescriptive prose sections to precise frozen evidence.

    Exhaustive inventory tables already carry their own stable evidence IDs;
    they are not duplicated into thousands of synthetic "evidence index"
    claims. Prose, procedures, guarantees, recovery advice, and task outcomes
    receive page-local claim records.
    """
    claims: list[dict[str, Any]] = []
    scope_evidence = capability_evidence(page)
    claims.append(make_claim(
        f"CLM-{page['page_id']}-SCOPE-001",
        page,
        "Scope",
        f"{page['title']} covers " + "; ".join(
            CAP_BY_ID[capability_id]["description"] for capability_id in page["capability_ids"]
        ),
        scope_evidence or page["evidence"],
    ))

    excluded = {
        "Scope", "Related documentation", "Evidence boundary", "Key API",
        "API reference", "Executable evidence", "Extracted lifecycle operations",
    }
    sections = list(re.finditer(r"(?m)^##\s+(.+?)\s*$", body))
    claim_number = 1
    for index, match in enumerate(sections):
        section = match.group(1).strip()
        if section in excluded:
            continue
        end = sections[index + 1].start() if index + 1 < len(sections) else len(body)
        section_text = body[match.end():end]
        evidence = section_claim_evidence(page, section, section_text)
        if len(re.findall(r"(?m)^\|", section_text)) > 3:
            continue
        section_text = re.sub(r"```.*?```", "", section_text, flags=re.S)
        units: list[str] = []
        for block in re.split(r"\n\s*\n", section_text):
            cleaned = re.sub(r"<!--.*?-->", "", block, flags=re.S)
            cleaned = re.sub(r"\[([^]]+)\]\([^)]+\)", r"\1", cleaned)
            cleaned = re.sub(r"[`*_>#|]", "", cleaned)
            cleaned = re.sub(r"(?m)^\s*(?:[-+] |\d+\.\s*)", "", cleaned)
            cleaned = re.sub(r"\s+", " ", cleaned).strip()
            if len(cleaned.split()) >= 6 and not set(cleaned) <= {"-", ":"}:
                units.append(cleaned)
        if not units:
            continue
        # One section record keeps related sentences together and maps the
        # exact published section, avoiding both untracked prose and one-record
        # per generated table row inflation.
        summary = " ".join(units)
        claim = make_claim(
            f"CLM-{page['page_id']}-TEXT-{claim_number:03d}",
            page,
            section,
            summary,
            evidence,
        )
        if section == "Current workspace qualification" and WORKSPACE_QUALIFICATION:
            claim["status"] = (
                "verified" if WORKSPACE_QUALIFICATION.get("passed") is True
                else "conflicted" if WORKSPACE_QUALIFICATION.get("status") == "conflicted"
                else "unknown_explicit"
            )
            claim["external_checkpoint"] = ".doc-intel/checkpoints/workspace-qualification.json"
            claim["external_commit"] = WORKSPACE_QUALIFICATION.get("external_commit")
        claims.append(claim)
        claim_number += 1
    claims.append(make_claim(
        f"CLM-{page['page_id']}-SOURCE-001",
        page,
        "Evidence boundary",
        f"{page['title']} is limited to its listed frozen-snapshot evidence and classifications.",
        unique_evidence(page["evidence"]),
    ))
    return claims


def claim_marker(claims: list[dict[str, Any]]) -> str:
    return "<!-- claims: " + ",".join(record["claim_id"] for record in claims) + " -->"


def scope_section(page: dict[str, Any]) -> str:
    lines = ["## Scope", ""]
    for capability_id in page["capability_ids"]:
        capability = CAP_BY_ID[capability_id]
        lines.append(f"- **{capability['name']}.** {capability['description']}")
    lines.extend([
        "",
        f"The scope of **{page['title']}** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.",
    ])
    return "\n".join(lines)


def symbols_table(page: dict[str, Any], limit: int = 18) -> str:
    symbols = page_symbols(page, limit)
    if not symbols:
        return f"No intentionally public Rust declaration is owned directly by **{page['title']}**. Its contract is expressed by the linked repository, protocol, or qualification evidence instead."
    lines = ["| Public declaration | Kind | Declared purpose | Source |", "|---|---|---|---|"]
    for symbol in symbols:
        purpose = NATIVE_DOCS.get(symbol["symbol_id"], symbol["summary"])
        purpose = purpose.split("\n\n", 1)[0]
        location = f"{symbol['source_file']}:{symbol['source_lines'][0]}"
        lines.append(f"| {code(symbol['qualified_name'])} | {md(symbol['kind'])} | {md(purpose)[:240]} | {code(location)} |")
    return "\n".join(lines)


def tests_text(page: dict[str, Any]) -> str:
    tests = page_tests(page)
    if not tests:
        return "No directly matching executable test is assigned to this page. That absence does not prove unsupported behavior; consult the unknowns ledger before making a guarantee."
    lines = [f"Executable evidence selected for **{page['title']}** is limited to each test's recorded setup and assertions:", ""]
    for record in tests:
        lines.append(f"- {code(record['name'])} — {record['behavior_under_test']} ({source_ref(record)}; {code(record['test_id'])}).")
    return "\n".join(lines)


def related_text(page: dict[str, Any]) -> str:
    records = related_pages(page)
    if not records:
        return ""
    return "## Related documentation\n\n" + "\n".join(f"- [{record['title']}](/%s)" % record["path"] for record in records)


def evidence_boundary(page: dict[str, Any]) -> str:
    lines = [
        "## Evidence boundary",
        "",
        f"The claims on **{page['title']}** are anchored to Git snapshot {code(SNAPSHOT)} and these primary files:",
        "",
    ]
    for item in page["evidence"]:
        location = f"{item['path']}:{item['lines'][0]}-{item['lines'][1]}"
        lines.append(f"- {code(location)} ({code(item['classification'])})")
    lines.extend([
        "",
        f"For **{page['title']}**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.",
    ])
    return "\n".join(lines)


def join_sections(*sections: str) -> str:
    return "\n\n".join(section.strip() for section in sections if section and section.strip())


def overview_body(page: dict[str, Any]) -> str:
    if page["path"] == "README.md":
        example = (ROOT / "examples/product_quickstart.rs").read_text().rstrip()
        install = f"{FENCE}toml\n[dependencies]\npocketstation = \"{PACKAGE_VERSION}\"\n{FENCE}"
        contracts = f"{FENCE}toml\npocketstation = {{ version = \"{PACKAGE_VERSION}\", default-features = false }}\n{FENCE}"
        # The crate root includes README.md as rustdoc. Keep the hardware-
        # dependent quickstart compiled as a doctest without executing capture.
        sample = f"{FENCE}rust,no_run\n{example}\n{FENCE}"
        commands = f"{FENCE}bash\ncargo test --examples --all-features\ncargo run --example product_quickstart\n{FENCE}"
        return join_sections(
            "PocketStation is a Rust library for declaring and running source-aware desktop audio Sessions. A Session can keep application and microphone sources separate, route them through bounded paths, expose polled audio, and finalize multistem recording. Extensions, connectors, the C ABI, and sidecars participate through the same declaration and lifecycle model.",
            f"## Install\n\nPocketStation {PACKAGE_VERSION} requires Rust {RUST_VERSION} or newer. Native capture is the default Cargo feature.\n\n" + install + "\n\nUse the contracts-only form when you need public declarations without a native capture backend:\n\n" + contracts,
            "## Run the first Session\n\nThe repository keeps its quickstart as a compiled Cargo example. It declares application and microphone capture, gives each source an independent polled-audio route and recording stem, observes two stems, and inspects both Session and recording outcomes.\n\n" + sample + "\n\nCompile before running, then run only on a host where the named application, microphone, permissions, and native dependencies are available:\n\n" + commands,
            "## Verify the outcome\n\nSuccess means the example observes at least two frames from each of two distinct stems, receives a successful Session stop outcome, and receives a recording outcome with two completed stems and no failed stems. Source, permission, route, stop, or recording failures are returned instead of being counted as success.",
            scope_section(page),
            "## Documentation map\n\n- [Start with the Rust quickstart](/docs/getting-started/rust-quickstart.md).\n- [Learn the Session model](/docs/concepts/session.md).\n- [Choose capture and permission behavior](/docs/concepts/source-selection.md).\n- [Configure routes and backpressure](/docs/concepts/realtime-routing.md).\n- [Author connectors](/docs/guides/connectors.md) or [native extensions](/docs/guides/extensions.md).\n- [Use the native Rust API reference](/docs/reference/rust-api.md).\n- [Diagnose observable symptoms](/docs/troubleshooting/session-start.md).",
            evidence_boundary(page),
        )
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for candidate in PAGES:
        if candidate["path"] not in {"README.md", "docs/README.md"}:
            grouped[candidate["doc_class"]].append(candidate)
    lines = [
        "Use this index to move from a first Session to the exact contract, task, failure, or implementation detail you need. Public navigation follows developer responsibilities; the separate intelligence workspace preserves file, symbol, relationship, behavior, and claim provenance.",
        "",
        "## Start here",
        "",
        "- [Install PocketStation](/docs/getting-started/installation.md)",
        "- [Run the Rust quickstart](/docs/getting-started/rust-quickstart.md)",
        "- [Check platform prerequisites](/docs/getting-started/platform-prerequisites.md)",
        "- [Run repository examples](/docs/getting-started/examples.md)",
    ]
    labels = {
        "concept": "Core concepts", "lifecycle": "Lifecycle", "how-to": "Task guides",
        "reference": "Reference", "config-reference": "Configuration reference",
        "protocol-reference": "Protocol reference", "error-reference": "Error reference",
        "troubleshooting": "Troubleshooting", "best-practice": "Best practices",
        "platform": "Platforms", "compatibility": "Compatibility", "internals": "Internals",
        "security": "Security", "glossary": "Terminology", "release": "Releases",
    }
    for doc_class in labels:
        records = grouped.get(doc_class, [])
        if not records:
            continue
        lines.extend(["", f"## {labels[doc_class]}", ""])
        for record in sorted(records, key=lambda value: (value["gate"], value["title"])):
            lines.append(f"- [{record['title']}](/%s)" % record["path"])
    return join_sections("\n".join(lines), evidence_boundary(page))


def getting_started_body(page: dict[str, Any]) -> str:
    title = page["title"].lower()
    if "install" in title:
        body = f"""## Prerequisites

- Rust {RUST_VERSION} or newer, as declared by package metadata.
- Cargo for dependency and feature resolution.
- Native platform development dependencies when the default native-capture feature is enabled.

## Add the dependency

{FENCE}toml
[dependencies]
pocketstation = "{PACKAGE_VERSION}"
{FENCE}

For a contracts-only build:

{FENCE}toml
pocketstation = {{ version = "{PACKAGE_VERSION}", default-features = false }}
{FENCE}

Run {code("cargo check")} to verify dependency resolution. Feature selection is compile-time configuration."""
    elif "quickstart" in title:
        example = (ROOT / "examples/product_quickstart.rs").read_text().rstrip()
        body = f"""## Audience

Use this quickstart if you are adding PocketStation to a Rust desktop application and want the first verified application-plus-microphone capture flow.

## Prerequisites

Choose a host with an application named {code("PocketStation Demo")}, an available default microphone, permission to open both sources, and a writable recording path.

## Supported environment

The crate requires Rust {RUST_VERSION} or newer. The program requires a target whose native-capture backend implements the selected sources; repository compilation is not physical-device qualification.

## Install

{FENCE}toml
[dependencies]
pocketstation = "{PACKAGE_VERSION}"
{FENCE}

## Program

This source is synchronized with {code("examples/product_quickstart.rs")}:

{FENCE}rust
{example}
{FENCE}

## Run it

Run {code("cargo run --example product_quickstart")} from the repository root on the prepared host.

## Success

The example accepts the run only after it observes two frames on each of two distinct stems, receives a successful Session stop outcome, and sees two completed recording stems with no failed stems.

## Common first-run failures

- The application selector cannot resolve {code("PocketStation Demo")}.
- Permission or source opening fails for application or microphone capture.
- The recording root is not writable.
- A polled route returns no frames before the example's bounded observation condition.

Preserve the returned typed error or terminal outcome; do not translate a missing prerequisite into capture success.

## Next steps

- [Learn the Session mental model](/docs/concepts/session.md).
- [Select capture sources](/docs/concepts/source-selection.md).
- [Inspect recording outcomes](/docs/how-to/inspect-recording-outcome.md).
- [Diagnose a Session that fails before start](/docs/troubleshooting/session-start.md)."""
    elif "example" in title:
        body = f"""## Compile the examples

Run {code("cargo test --examples --all-features")} for top-level examples. Run {code("cargo check --manifest-path examples/operator-consumer/Cargo.toml")} and {code("cargo check --manifest-path examples/whisper-transcribe/Cargo.toml")} for nested packages.

Compilation establishes API compatibility. It does not establish that capture devices, a named application, or an external transcription process is available.

## Choose an example

- {code("product_quickstart.rs")} exercises capture, polling, stop, and recording outcomes.
- {code("connector_authoring.rs")} declares and registers a connector without contacting its example destination.
- {code("operator-consumer")} consumes the operator contract as a separate package.
- {code("whisper-transcribe")} owns an external-process evidence boundary."""
    else:
        body = """## Before you build

Cargo selects target dependencies and native implementation through target-specific tables and the native-capture feature. A successful compile proves that the selected source builds. It does not prove device presence, permission, or physical qualification.

## Before you run

The host application owns permission prompts and source-selection user experience. Use non-prompting observation where implemented, then use Session preparation or source opening as the authoritative typed outcome.

## Verify the environment

Start with a contracts-only Cargo check. Enable the feature set you intend to ship and run available target tests. Keep build, virtual-machine, conformance, and physical-device evidence separately labeled."""
    return join_sections(
        body, scope_section(page),
        "## Public entry points\n\n" + symbols_table(page, 16),
        "## Executable evidence\n\n" + tests_text(page),
        related_text(page), evidence_boundary(page),
    )


def concept_body(page: dict[str, Any]) -> str:
    content = CONCEPTS.get(page["page_id"])
    if not content:
        raise RuntimeError(f"reviewed concept content is absent for {page['page_id']}")
    journeys = [journey for journey in JOURNEYS if set(page["capability_ids"]).intersection(journey["capability_ids"])]
    encountered = "\n".join(f"- **{journey['name']}** — {journey['outcome']}" for journey in journeys[:8])
    relationships = "\n".join(f"- {item}" for item in content["relationships"])
    invariants = "\n".join(f"- {item}" for item in content["invariants"])
    use = "\n".join(f"- {item}" for item in content["use"])
    return join_sections(
        f"## What it is\n\n{content['what']}",
        f"## Why it exists\n\n{content['why']}",
        "## Relationships\n\n" + relationships,
        "## Invariants and guarantees\n\n" + invariants,
        "## When you encounter it\n\n" + (encountered or f"You encounter {page['title'].lower()} through its declaration and runtime APIs."),
        "## Use it\n\n" + use,
        scope_section(page),
        "## Key API\n\n" + symbols_table(page, 10),
        "## Executable evidence\n\n" + tests_text(page),
        related_text(page), evidence_boundary(page),
    )


def lifecycle_body(page: dict[str, Any]) -> str:
    records = [record for record in LIFECYCLES if relevant(page, record)][:100]
    lines = ["| Operation | Trigger | Source state | Destination state | Evidence record |", "|---|---|---|---|---|"]
    for record in records:
        lines.append(f"| {code(record['operation'])} | {code(record['trigger'])} | {md(record['source_state'])} | {md(record['destination_state'])} | {code(record['lifecycle_id'])} |")
    return join_sections(
        scope_section(page),
        f"## Ownership transition\n\nFor **{page['title']}**, PocketStation keeps the declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types exposed by the source distinct. Do not collapse its stop outcome into a Boolean assumption: component and finalization failures remain structured data.",
        "## Extracted lifecycle operations\n\n" + ("\n".join(lines) if records else "No lifecycle record matches this evidence domain."),
        f"## Failure handling\n\nWithin **{page['title']}**, a transition whose guard, idempotence, or recovery is recorded as not declared has no published guarantee. Preserve its returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.",
        "## Executable evidence\n\n" + tests_text(page),
        related_text(page), evidence_boundary(page),
    )


PROCEDURES = {
    "GUIDE-001": ["Create a Session declaration.", "Build an ApplicationSelector whose evidence matches your selection need.", "Declare the application Source and attach a consumer route.", "Start the Session and retain RunningSession.", "Observe frames or typed capture failures, then stop and inspect the outcome."],
    "GUIDE-002": ["Observe permission without prompting when the target exposes that operation.", "Let the host application own any permission prompt.", "Declare the default or identified microphone Source.", "Attach a consumer before start.", "Treat preparation or source opening as the authoritative result."],
    "GUIDE-003": ["Declare application and microphone sources in one Session.", "Give each source an independent endpoint or route.", "Retain stem and source identity from frame lineage.", "Start once and consume both bounded routes.", "Stop once and inspect Session plus recording outcomes."],
    "GUIDE-004": ["Discover candidates through the source provider.", "Build a process or application query with the required scope.", "Resolve the query and retain stable source identity.", "Observe generation changes instead of assuming process identity is permanent.", "Handle empty or ambiguous resolution as a typed result."],
    "GUIDE-005": ["Call microphone_permission_observation before opening a source when preflight information helps the UI.", "Interpret NotObservable as neither allowed nor denied.", "Request permission only through the host application's platform UI.", "Prepare or start the selected source.", "Use the open result as the authoritative decision."],
    "GUIDE-006": ["Declare a separate polled_audio endpoint for each independent route.", "Send the source or stream to that endpoint.", "Call try_poll_audio for an immediate check or wait_poll with a finite timeout from non-realtime application code.", "Iterate only indices below the returned batch length and retain the route, endpoint, and poll observation timestamps needed for diagnosis.", "Release the lease promptly and inspect polling observations."],
    "GUIDE-007": ["Declare the source once.", "Create each consumer endpoint independently.", "Connect the same source output to each endpoint.", "Set explicit edge policy where the default is unsuitable.", "Observe each route separately so saturation remains attributable."],
    "GUIDE-008": ["Identify producer and consumer partitions.", "Choose finite capacity.", "Select backpressure, loss, copy, delivery, and observation policies.", "Compile and handle rejected contracts.", "Measure queue depth, saturation, and drops before changing capacity."],
    "GUIDE-009": ["Set the recording root on SessionBuilder.", "Call record with a label for each stem.", "Start and run the Session.", "Stop to trigger endpoint finalization.", "Inspect overall and per-stem recording outcomes."],
    "GUIDE-010": ["Retain RunningSession until stop returns.", "Preserve SessionStopOutcome.", "Read recording_outcome after stop and locate the schema-versioned recording manifest using the exported file-name constant.", "Check overall state plus completed and failed stem counts.", "Use error codes and per-stem results to diagnose partial finalization."],
    "GUIDE-011": ["Define named ports in AsyncOperatorManifest.", "Implement factory preparation.", "Return an async node that observes cancellation and declared policies.", "Register before Session compilation.", "Connect named ports and run the separate consumer example."],
    "GUIDE-012": ["Retain typed output and input declaration handles.", "Connect handles with compatible signal specifications.", "Use exact port names from the manifest and preserve each source-aware binding when several stems feed one operator.", "Compile and inspect SessionCompileDiagnostic for the stage, stable code, and affected component identities.", "Confirm every compiled binding targets the intended instance."],
    "GUIDE-013": ["Declare generated-audio output.", "Prepare the bounded audio-reentry bridge.", "Produce PCM matching the target sample specification.", "Write from the asynchronous lane.", "Observe accepted, saturated, closed, or cancelled outcomes."],
    "GUIDE-014": ["Implement EndpointDriverFactory preparation.", "Return a prepared driver with its start gate.", "Consume matching audio or signal inputs.", "Honor cancellation and shutdown mode.", "Return finalization observations and staged failures."],
    "GUIDE-015": ["Build ConnectorManifest with node and configuration schemas.", "Validate values and keep secrets in ConnectorSecret.", "Implement finite delivery outcomes.", "Register, declare, and connect the endpoint.", "Run conformance before provider qualification."],
    "GUIDE-016": ["Declare every connector configuration field.", "Use Secret value kind for secret material.", "Construct ConnectorSecret instead of ordinary text values.", "Read validated values during preparation.", "Keep diagnostics on redacted representations."],
    "GUIDE-017": ["Create ../protocol/conformance/connector/v1 from the repository root.", "Copy scripts/fixtures/connector-v1-vectors.json to ../protocol/conformance/connector/v1/vectors.json.", "Run connector contract and grouping tests.", "Run portable semantics with the materialized canonical vector.", "Keep portable conformance and provider qualification as separate evidence."],
    "GUIDE-018": ["Build a dynamic library exporting pks_extension_library_v1.", "Use a canonical absolute path to a trusted regular file.", "Return a compatible descriptor and callbacks.", "Load through Session and retain the receipt.", "Handle registration rollback and executable-code lifetime."],
    "GUIDE-019": ["Include pocketstation.h and use its ABI version.", "Create handles through exported functions.", "Check every PksSessionStatus.", "Stop before releasing runtime ownership.", "Release each handle with its matching ABI function."],
    "GUIDE-020": ["Declare SidecarProcessSpec with bounded limits and deadlines.", "Start the child through SidecarHost.", "Exchange only declared message kinds.", "Apply cancellation, drain, or abort through lifecycle state.", "Inspect host snapshot and terminal error before restart."],
    "GUIDE-021": ["Acquire observation handles before the period you need to inspect.", "Snapshot metrics by typed SessionComponentId values and preserve the observation boundary for each counter or timestamp.", "Record versioned SessionTraceRecord values for durable lifecycle and component-failure evidence.", "Stop and include SessionTraceTerminal plus the independent recording outcome.", "Validate trace structure before diagnosis."],
    "GUIDE-022": ["Retain RunningSession as runtime owner.", "Request stop once application work ends.", "Read component failures in SessionStopOutcome.", "Read recording and trace finalization separately.", "Preserve diagnostics before releasing ownership."],
    "GUIDE-023": ["Create AudioInputConfig matching the producer.", "Acquire a bounded AudioInputBuffer.", "Write only within declared capacity and format.", "Submit through AudioInputWriter and route the source.", "Handle acquire, write, cancellation, and runtime errors separately."],
    "GUIDE-024": ["Choose an evidenced Opus profile and SampleSpec.", "Construct a stateful encoder.", "Encode only accepted frame formats.", "Construct the matching decoder and decode packets.", "Use the round-trip test as executable compatibility evidence."],
    "GUIDE-025": ["Run the repository protocol script.", "Run C ABI and compatibility tests selected by CI.", "Provide required sibling or private fixtures.", "Distinguish absent prerequisites from assertion failures.", "Record command, target, and fixture revision."],
    "GUIDE-026": ["Read the nested example prerequisites.", "Build its Cargo manifest.", "Choose required capture sources.", "Run the external process integration.", "Preserve and validate process evidence."],
    "GUIDE-027": ["Use defaults for native capture applications.", "Disable defaults for contract-only consumers.", "Enable conformance-fixtures only for fixture APIs.", "Reserve internal-testing for repository checks.", "Rebuild after feature changes."],
    "GUIDE-028": ["Build a supported system-capture query.", "Resolve or prepare it through the source provider.", "Attach a bounded consumer.", "Start and observe the typed open result.", "Keep implementation and qualification claims separate."],
    "GUIDE-029": ["Retain source clock-domain identity and timestamp.", "Update TimelineMapping with observed source and Session time.", "Map into the Session domain.", "Observe drift and discontinuity without rewriting lineage.", "Apply correction only through evidenced controller bounds."],
    "GUIDE-030": ["Compile the immutable Session declaration and retain SessionCompileDiagnostic if validation fails.", "Prepare resources and retain identity mappings.", "Handle source and endpoint preparation errors.", "Start with the intended cancellation option.", "Preserve rollback failures alongside a primary start failure."],
}


TASK_COMMANDS = {
    "GUIDE-017": "mkdir -p ../protocol/conformance/connector/v1\ncp scripts/fixtures/connector-v1-vectors.json ../protocol/conformance/connector/v1/vectors.json\ncargo test --all-features connector",
    "GUIDE-025": "./scripts/check_protocol.sh\ncargo test --test abi_c_conformance --all-features\ncargo test --test connector_portable_semantics --all-features",
    "GUIDE-026": "cargo check --manifest-path examples/whisper-transcribe/Cargo.toml\ncargo run --manifest-path examples/whisper-transcribe/Cargo.toml",
    "GUIDE-027": "cargo check --all-features\ncargo check --no-default-features",
}


def concrete_task_example(page: dict[str, Any]) -> str:
    command = TASK_COMMANDS.get(page["page_id"])
    if command:
        return (
            f"Run the {page['title'].lower()} commands from the PocketStation checkout. Each command is part of this task's documented validation surface.\n\n"
            f"{FENCE}bash\n{command}\n{FENCE}"
        )
    example_path = next(
        (
            item["path"] for item in page["evidence"]
            if item["path"].startswith("examples/") and item["path"].endswith((".rs", ".c", ".cpp"))
        ),
        None,
    )
    if example_path:
        source = frozen_text(example_path).rstrip()
        language = "rust" if example_path.endswith(".rs") else "c"
        example = next(record for record in EXAMPLES if record["path"] == example_path)
        return (
            f"This is the frozen, repository-owned example {code(example['example_id'])} at {code(example_path)}. It is validated by the examples checkpoint.\n\n"
            f"{FENCE}{language}\n{source}\n{FENCE}"
        )
    tests = page_tests(page, 1)
    if tests:
        test = tests[0]
        lines = frozen_text(test["path"]).splitlines()
        excerpt = "\n".join(lines[test["lines"][0] - 1:test["lines"][1]])
        language = "rust" if test["path"].endswith(".rs") else "c"
        location = code(f"{test['path']}:{test['lines'][0]}")
        return (
            f"The executable repository test {code(test['name'])} ({code(test['test_id'])}) shows the concrete API sequence and asserted outcome at {location}.\n\n"
            f"{FENCE}{language}\n{excerpt}\n{FENCE}\n\n"
            f"{FENCE}bash\ncargo test --all-features {test['name']}\n{FENCE}"
        )
    return (
        "No standalone repository example is assigned to this task. Use the exact declarations in the API table and verify the owning target directly.\n\n"
        f"{FENCE}bash\ncargo test --all-features\n{FENCE}"
    )


def workspace_qualification_section(page: dict[str, Any]) -> str:
    if page["page_id"] != "GUIDE-025" or not WORKSPACE_QUALIFICATION:
        return ""
    checkpoint_link = "`.doc-intel/checkpoints/workspace-qualification.json`"
    if not WORKSPACE_QUALIFICATION.get("observed"):
        return (
            "## Current workspace qualification\n\n"
            "The sibling `pks` repository was not present when this documentation model was built, so the single-engine ownership boundary is explicitly not observable. "
            f"Run the check with both repositories present and inspect the {checkpoint_link}; absence is not passing workspace evidence."
        )
    commit = WORKSPACE_QUALIFICATION.get("external_commit") or "unknown"
    if WORKSPACE_QUALIFICATION.get("passed"):
        return (
            "## Current workspace qualification\n\n"
            f"The sibling `pks` checkout at commit `{commit}` passed the single-engine ownership check. "
            f"The exact command and output digest are recorded in the {checkpoint_link}."
        )
    occurrences = WORKSPACE_QUALIFICATION.get("violation_occurrences", 0)
    paths = len(WORKSPACE_QUALIFICATION.get("violation_paths", []))
    return (
        "## Current workspace qualification\n\n"
        f"The sibling `pks` checkout at commit `{commit}` failed the single-engine ownership check with {occurrences} matched occurrences across {paths} source paths. "
        "It still imports PocketStation internal engine machinery and retains retired parallel command paths. "
        "This is an explicit `CONFLICTED` workspace result: this documentation branch does not claim that the multi-repository single-engine boundary passes. "
        f"The exact command, checked commit, output digest, and affected paths are recorded in the {checkpoint_link}."
    )


def how_to_body(page: dict[str, Any]) -> str:
    detail = GUIDES.get(page["page_id"])
    if not detail:
        raise RuntimeError(f"reviewed guide content is absent for {page['page_id']}")
    prerequisite, success, failure_focus, references = detail
    steps = PROCEDURES.get(page["page_id"], [])
    if not steps:
        matching = [journey for journey in JOURNEYS if set(page["capability_ids"]).intersection(journey["capability_ids"])]
        raw = matching[0]["steps"] if matching else ["inspect the contract", "perform the operation", "verify the outcome"]
        steps = [step.replace("_", " ").capitalize() + "." for step in raw]
    procedure = "\n".join(f"{number}. {step}" for number, step in enumerate(steps, 1))
    errors = page_errors(page, 10)
    failure_lines = []
    for record in errors:
        label = code(record["type"])
        if record.get("variant"):
            label += " / " + code(record["variant"])
        failure_lines.append(f"- {label} — {code(record['error_id'])}")
    return join_sections(
        scope_section(page),
        "## Prerequisites\n\n" + prerequisite,
        "## Procedure\n\n" + procedure,
        "## Concrete repository example\n\n" + concrete_task_example(page),
        workspace_qualification_section(page),
        "## Important consequence\n\n" + failure_focus,
        "## Verify the outcome\n\n" + success + "\n\n" + tests_text(page),
        "## Failure signals\n\n" + ("\n".join(failure_lines) if failure_lines else f"No task-specific public error was resolved for {page['title'].lower()}; preserve the owning API's returned error."),
        "## API reference\n\n" + "\n".join(f"- [{target.rsplit('/', 1)[-1].removesuffix('.md').replace('-', ' ').title()}]({target})" for target in references) + "\n\n" + symbols_table(page, 8),
        related_text(page), evidence_boundary(page),
    )


def exhaustive_table(title: str, headers: list[str], rows: list[list[Any]]) -> str:
    lines = [f"## {title}", "", "| " + " | ".join(headers) + " |", "|" + "|".join("---" for _ in headers) + "|"]
    for row in rows:
        lines.append("| " + " | ".join(md(value) for value in row) + " |")
    return "\n".join(lines)


def reference_body(page: dict[str, Any]) -> str:
    path = page["path"]
    if path == "docs/reference/behavior-evidence.md":
        rows = [[record["behavior_id"], record["classification"], record["domain"], code(record["name"]), source_ref(record)] for record in BEHAVIORS]
        table = exhaustive_table("Inventory", ["Evidence record", "Class", "Domain", "Behavior", "Source"], rows)
    elif path == "docs/reference/protocol-surface.md":
        rows = [[record["protocol_id"], record["kind"], code(record["name"]), source_ref(record)] for record in PROTOCOLS]
        table = exhaustive_table("Inventory", ["Evidence record", "Boundary", "Declaration", "Source"], rows)
    elif path == "docs/reference/test-evidence.md":
        rows = [[record["test_id"], code(record["name"]), record["behavior_under_test"], code(f"{record['path']}:{record['lines'][0]}")] for record in TESTS]
        table = exhaustive_table("Inventory", ["Evidence record", "Test", "Narrow behavior", "Source"], rows)
    elif path == "docs/reference/lifecycle-evidence.md":
        rows = [[
            record["lifecycle_id"], code(record["operation"]), record["trigger"],
            record["source_state"], record["destination_state"], record["guard"],
            record["action"], record["possible_error"], record["recovery"],
            record["idempotence"], record["observable_signal"],
            ", ".join(record["tests"]) or record.get("test_coverage_status", "no direct test link"),
            source_ref(record),
        ] for record in LIFECYCLES]
        table = exhaustive_table(
            "Inventory",
            ["Evidence record", "Operation", "Trigger", "From", "To", "Guard", "Action", "Possible error", "Recovery", "Idempotence", "Observable signal", "Tests", "Source"],
            rows,
        )
    elif page["doc_class"] == "error-reference":
        records = [record for record in ERRORS if relevant(page, record)]
        rows = []
        for record in records:
            variant = code(record["variant"]) if record.get("variant") else "type"
            rows.append([
                record["error_id"], code(record["type"]), variant,
                record["trigger_condition"], record["developer_action"],
                record["retryable"], record["retry_basis"], record["recoverable"],
                record["recovery_action"], record["test_coverage_status"],
                ", ".join(record["tests"]) or "none linked", source_ref(record),
            ])
        table = exhaustive_table(
            "Error inventory",
            ["Evidence record", "Type", "Variant", "Trigger", "Developer action", "Retryable", "Retry basis", "Recoverable", "Recovery action", "Test status", "Tests", "Defined"],
            rows,
        )
    elif page["doc_class"] == "config-reference" and path.endswith("configuration.md"):
        rows = [[
            record["config_id"], record["kind"], code(record["name"]), code(record.get("parent")),
            record["value_type"], record["required"], record["default"], record["units"],
            record["valid_values"], record["minimum"], record["maximum"], record["when_read"],
            record["precedence"], record["invalid_value_behavior"],
            record.get("test_coverage_status", "no direct test link extracted"),
            ", ".join(record.get("tests", [])) or "none linked", source_ref(record),
        ] for record in CONFIGURATION]
        table = exhaustive_table(
            "Configuration inventory",
            ["Evidence record", "Kind", "Name", "Parent", "Type", "Required", "Default", "Units", "Valid values", "Minimum", "Maximum", "When read", "Precedence", "Invalid value", "Test status", "Tests", "Source"],
            rows,
        )
    else:
        symbols = [symbol for symbol in SYMBOLS if symbol["public_api"] and relevant(page, symbol)]
        symbols.sort(key=lambda record: (record["kind"], record["qualified_name"]))
        rows = []
        selected = symbols if path == "docs/reference/rust-api.md" else symbols[:300]
        for symbol in selected:
            purpose = NATIVE_DOCS.get(symbol["symbol_id"], symbol["summary"]).split("\n\n", 1)[0]
            rows.append([symbol["symbol_id"], code(symbol["qualified_name"]), symbol["kind"], purpose, code(f"{symbol['source_file']}:{symbol['source_lines'][0]}")])
        table = exhaustive_table("Public surface", ["Evidence record", "Declaration", "Kind", "Purpose", "Source"], rows)
    native = f"For **{page['title']}**, the generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) provides compiler-rendered signatures and navigation. This repository page adds the frozen evidence identifiers, responsibilities, and cross-boundary interpretation used by the documentation verifier."
    return join_sections(
        scope_section(page),
        "## Reference authority\n\n" + native,
        table,
        f"## Interpretation\n\nThe **{page['title']}** inventory records compiler-visible or extracted evidence at the frozen snapshot. A field marked unknown or not declared remains outside the published guarantee; use the native signature, owning error type, and cited test before relying on panic, blocking, cancellation, ordering, limits, retry, or recovery behavior.",
        related_text(page), evidence_boundary(page),
    )


TROUBLESHOOTING_ACTIONS = {
    "TRBL-001": "Separate declaration and compile errors from resource-preparation and start errors. Preserve rollback failures as additional evidence rather than replacing the primary start failure.",
    "TRBL-002": "Confirm that selection resolves the intended source, then distinguish permission, open, lifecycle, and delivery observations. A running Session with no frames does not prove that capture opened the intended application.",
    "TRBL-003": "Check non-prompting permission observation, the host-owned prompt result, selected input identity, and source-open outcome in that order.",
    "TRBL-004": "Treat NotObservable as neither grant nor denial. The source-open result is authoritative where the platform has no single process-wide observation.",
    "TRBL-005": "Inspect source lifecycle event kind, source generation, and permission epoch. Preserve lineage from frames received before the change.",
    "TRBL-006": "Compare queue depth, saturation, and drop observations by route. Change capacity only after identifying the constrained consumer and its declared loss and backpressure policy.",
    "TRBL-007": "Inspect overall recording state and every stem outcome after Session stop. A successful source run does not imply every file finalized.",
    "TRBL-008": "Separate manifest validation, configuration validation, readiness timeout, delivery failure, retry-budget exhaustion, and endpoint finalization stages.",
    "TRBL-009": "Check absolute path, canonicalization, regular-file, library-load, entrypoint, ABI-version, descriptor, and registration errors in their reported order.",
    "TRBL-010": "Inspect sidecar state, message kind, protocol limit, and the deadline that expired before choosing drain, abort, or restart.",
    "TRBL-011": "Verify profile, sample rate, channel count, frame size, packet bounds, and encoder or decoder state expected by the selected API.",
    "TRBL-012": "Read component, rollback, endpoint, source, recording, sidecar, and finalization failures from the terminal outcome instead of reducing stop to one status flag.",
    "TRBL-013": "Resolve the external fixture path and version first. An absent sibling vector or private artifact is a prerequisite failure, not passing product evidence.",
    "TRBL-014": "Verify that the child process ran, its output was captured, and the evidence artifact corresponds to this source revision.",
    "TRBL-015": "Reproduce without default features to separate Core contracts from native dependencies, then restore native capture and diagnose the selected target dependency.",
    "TRBL-016": "Compare clock-domain IDs, source and Session timestamps, drift snapshots, corrections, generations, and discontinuity records before altering a mapping.",
    "TRBL-017": "Use the trace validation error and record index to distinguish ordering, identity, and terminal-record failures. Do not rewrite a trace to make validation pass.",
    "TRBL-018": "Distinguish buffer acquisition exhaustion from write format or capacity errors and cancellation. Use observations before changing bounded capacity.",
}


def troubleshooting_body(page: dict[str, Any]) -> str:
    detail = TROUBLESHOOTING.get(page["page_id"])
    if not detail:
        raise RuntimeError(f"reviewed troubleshooting content is absent for {page['page_id']}")
    symptom, causes, distinguish, corrective, retry_state, references = detail
    errors = page_errors(page, 12)
    signals = []
    for record in errors:
        label = code(record["type"])
        if record.get("variant"):
            label += " / " + code(record["variant"])
        signals.append(f"- {label} ({code(record['error_id'])})")
    tests = page_tests(page, 15)
    test_lines = [f"- {code(record['name'])} exercises {record['behavior_under_test']} under its recorded setup ({code(record['test_id'])})." for record in tests]
    return join_sections(
        "## Symptom\n\n" + symptom,
        "## Evidenced causes\n\n" + "\n".join(f"- {cause}" for cause in causes),
        "## Distinguish the causes\n\n" + distinguish,
        "## Diagnostic signals\n\n" + ("\n".join(signals) if signals else f"No error declaration is tied directly to {page['title'].lower()}; use the owning component's typed outcome and observations."),
        "## Executable evidence\n\n" + ("\n".join(test_lines) if test_lines else "No directly matching executable test is assigned to this symptom. Keep diagnosis within fields returned by the API."),
        "## Corrective action\n\n" + corrective,
        "## Retry and incomplete state\n\n" + retry_state,
        "## Related reference\n\n" + "\n".join(f"- [{target.rsplit('/', 1)[-1].removesuffix('.md').replace('-', ' ').title()}]({target})" for target in references),
        related_text(page), evidence_boundary(page),
    )


def best_practice_body(page: dict[str, Any]) -> str:
    detail = BEST_PRACTICES.get(page["page_id"])
    if not detail:
        raise RuntimeError(f"reviewed best-practice content is absent for {page['page_id']}")
    problem, recommendation, reason, tradeoff, exception = detail
    evidence_paths = {item["path"] for item in page["evidence"]}
    pattern_names = {
        "BEST-001": {"bounded_queue", "buffer_pool"},
        "BEST-002": {"bounded_queue", "buffer_pool"},
        "BEST-003": {"clock_correlation"},
        "BEST-004": {"typed_error", "transactional_registration"},
        "BEST-005": {"typed_error", "bounded_queue"},
        "BEST-006": {"transactional_registration", "sidecar_isolation"},
        "BEST-007": {"typed_error", "clock_correlation"},
    }[page["page_id"]]
    patterns = [
        record for record in PATTERNS
        if record["where_implemented"] in evidence_paths and record["name"] in pattern_names
    ]
    pattern_lines = [f"- {code(record['name'])} at {code(record['where_implemented'])} ({code(record['pattern_id'])})." for record in patterns]
    return join_sections(
        "## Problem\n\n" + problem,
        "## Recommendation\n\n" + recommendation,
        "## Reason\n\n" + reason,
        "## Tradeoff\n\n" + tradeoff,
        "## When it does not apply\n\n" + exception,
        "## Repository evidence\n\n" + ("\n".join(pattern_lines) if pattern_lines else "This recommendation is tied directly to the page's source evidence."),
        "## Executable evidence\n\n" + tests_text(page),
        related_text(page), evidence_boundary(page),
    )


def internals_body(page: dict[str, Any]) -> str:
    files = sorted({item["path"] for item in page["evidence"]})
    patterns = [record for record in PATTERNS if relevant(page, record)][:30]
    pattern_lines = [f"- {code(record['name'])} — {code(record['where_implemented'])} ({code(record['classification'])})." for record in patterns]
    return join_sections(
        scope_section(page),
        "## Ownership map\n\n" + "\n".join(f"- {code(path)} owns part of this boundary." for path in files),
        "## Compiler-visible surface\n\n" + symbols_table(page, 24),
        "## Observed implementation patterns\n\n" + ("\n".join(pattern_lines) if pattern_lines else "No automated pattern record is assigned to this boundary."),
        "## Behavioral evidence\n\n" + tests_text(page),
        f"## Stability boundary\n\n**{page['title']}** describes internal ownership. Its private module layout is not a compatibility promise; compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts.",
        related_text(page), evidence_boundary(page),
    )


def platform_body(page: dict[str, Any]) -> str:
    return join_sections(
        scope_section(page),
        "## Implemented boundary\n\n" + symbols_table(page, 20),
        f"## Permission and source opening\n\nFor **{page['title']}**, permission observation and source opening remain separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.",
        f"## Qualification boundary\n\nThe target-specific files, Cargo dependencies, and CI cited by **{page['title']}** establish implementation or build evidence only. They do not qualify every device, operating-system revision, packaging context, permission state, or physical path.",
        "## Executable evidence\n\n" + tests_text(page),
        related_text(page), evidence_boundary(page),
    )


def glossary_body(page: dict[str, Any]) -> str:
    rows = [[f"**{term['canonical_name']}**", code(term["code_spelling"]), term["definition"], ", ".join(term["aliases"]) or "—", ", ".join(term["forbidden_aliases"]) or "—"] for term in TERMINOLOGY]
    return join_sections(
        "Use these terms consistently. Code spelling anchors a term to a compiler symbol; it does not imply that every related type uses that name.",
        exhaustive_table("Terms", ["Term", "Code spelling", "Definition", "Aliases", "Avoid"], rows),
        "## Terminology conflicts\n\nAliases are permitted only where listed. A forbidden alias usually collapses a distinction such as source versus stem, endpoint versus connector, or wall clock versus clock domain.",
        evidence_boundary(page),
    )


def release_body(page: dict[str, Any]) -> str:
    original = subprocess.check_output(["git", "show", f"{SNAPSHOT}:RELEASE_NOTES.md"], cwd=ROOT, text=True)
    original = re.sub(r"^#\s+Release notes\s*", "", original, count=1).strip()
    return join_sections(
        f"PocketStation's package version at the analyzed snapshot is {PACKAGE_VERSION}. Release automation and package metadata remain the publication authority; this page preserves the repository's declared release record.",
        scope_section(page),
        "## Snapshot release record\n\n" + original,
        "## Evidence scope\n\nRelease notes are declared evidence. They do not replace executable checks, physical qualification artifacts, or the compatibility baseline.",
        related_text(page), evidence_boundary(page),
    )


def security_body(page: dict[str, Any]) -> str:
    return join_sections(
        scope_section(page),
        "## Secret values\n\nConnector configuration distinguishes secret values from ordinary text. The repository's secret owner overwrites initialized string bytes during clearing, and connector diagnostics use redacted representations. This does not claim that every upstream or downstream copy is erased.",
        "## Executable extension trust\n\nNative loading requires an absolute path, canonicalizes it, checks for a regular file, validates the ABI descriptor, and imports registrations transactionally. The host still decides whether the file is trusted executable code.",
        "## Process boundary\n\nSidecars are executable process and protocol boundaries. Configure finite messages and deadlines. Do not infer authentication or sandbox guarantees absent from the contract.",
        "## C ABI ownership\n\nUse header-defined handle and callback ownership. Keep libraries alive while callback contexts remain reachable, and release handles through matching functions.",
        related_text(page), evidence_boundary(page),
    )


def body_for(page: dict[str, Any]) -> str:
    doc_class = page["doc_class"]
    if doc_class == "overview":
        return overview_body(page)
    if doc_class == "getting-started":
        return getting_started_body(page)
    if doc_class == "concept":
        return concept_body(page)
    if doc_class == "lifecycle":
        return lifecycle_body(page)
    if doc_class == "how-to":
        return how_to_body(page)
    if doc_class in {"reference", "protocol-reference", "config-reference", "error-reference"}:
        return reference_body(page)
    if doc_class == "troubleshooting":
        return troubleshooting_body(page)
    if doc_class == "best-practice":
        return best_practice_body(page)
    if doc_class == "internals":
        return internals_body(page)
    if doc_class in {"platform", "compatibility"}:
        return platform_body(page)
    if doc_class == "glossary":
        return glossary_body(page)
    if doc_class == "release":
        return release_body(page)
    if doc_class == "security":
        return security_body(page)
    return concept_body(page)


def write_navigation() -> None:
    groups: dict[int, list[dict[str, Any]]] = defaultdict(list)
    for page in PAGES:
        if page["path"] != "README.md":
            groups[page["gate"]].append(page)
    labels = {
        7: "Learn and understand",
        8: "How-to guides",
        9: "Reference",
        10: "Failures, troubleshooting, and practices",
    }
    lines = ["# Summary", "", "- [PocketStation](../README.md)", "- [Documentation home](README.md)"]
    for gate in sorted(groups):
        lines.append(f"- {labels[gate]}")
        for page in sorted(groups[gate], key=lambda record: (record["doc_class"], record["title"])):
            target = page["path"][5:] if page["path"].startswith("docs/") else "../" + page["path"]
            if target != "README.md":
                lines.append(f"  - [{page['title']}]({target})")
    summary = ROOT / "docs" / "SUMMARY.md"
    summary.parent.mkdir(parents=True, exist_ok=True)
    summary.write_text("\n".join(lines) + "\n")
    llms = [
        "# PocketStation documentation",
        f"> Evidence snapshot: {SNAPSHOT}",
        "> Canonical human and reference documentation for the PocketStation Rust SDK.",
        "",
    ]
    for page in sorted(PAGES, key=lambda record: (record["gate"], record["doc_class"], record["title"])):
        capability_names = ", ".join(CAP_BY_ID[item]["name"] for item in page["capability_ids"][:4])
        llms.append(f"- [{page['title']}]({page['path']}): {capability_names}")
    (ROOT / "llms.txt").write_text("\n".join(llms) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--batch", type=int, default=0, help="Maximum pages to write; zero writes all")
    parser.add_argument("--start", help="Start at this page ID")
    arguments = parser.parse_args()
    pages = PAGES
    if arguments.start:
        positions = [index for index, page in enumerate(pages) if page["page_id"] == arguments.start]
        if not positions:
            raise SystemExit(f"unknown page ID: {arguments.start}")
        pages = pages[positions[0]:]
    if arguments.batch > 0:
        pages = pages[:arguments.batch]
    selected_ids = {page["page_id"] for page in pages}
    existing_claims = read_jsonl(DB / "claims.jsonl")
    claims = [record for record in existing_claims if record["documentation_page"] not in selected_ids]
    for page in pages:
        body = body_for(page).strip()
        page_claims = claims_for(page, body)
        text = f"# {page['title']}\n\n{claim_marker(page_claims)}\n\n{body}\n"
        path = ROOT / page["path"]
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
        claims.extend(page_claims)
        print(f"AUTHORED {page['page_id']} {page['path']} claims={len(page_claims)} words={len(text.split())}")
    write_jsonl(DB / "claims.jsonl", sorted(claims, key=lambda record: record["claim_id"]))
    write_navigation()
    print(f"pages_written={len(pages)} claims={len(claims)}")


if __name__ == "__main__":
    main()
