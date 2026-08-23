#!/usr/bin/env python3
"""Author evidence-backed pages from the reviewed documentation work queue."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from collections import defaultdict
from pathlib import Path, PurePosixPath
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / ".doc-intel"
BT = chr(96)
FENCE = BT * 3


def read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def write_jsonl(path: Path, records: list[dict[str, Any]]) -> None:
    with path.open("w") as handle:
        for record in records:
            handle.write(json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n")


STATE = read_json(DB / "state.json")
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
ERRORS = read_jsonl(DB / "errors.jsonl")
CONFIGURATION = read_jsonl(DB / "configuration.jsonl")
BEHAVIORS = read_jsonl(DB / "behaviors.jsonl")
LIFECYCLES = read_jsonl(DB / "lifecycles.jsonl")
PROTOCOLS = read_jsonl(DB / "protocols.jsonl")
PATTERNS = read_jsonl(DB / "patterns.jsonl")
TERMINOLOGY = read_json(DB / "terminology.json")["terms"]
SNAPSHOT = STATE["snapshot"]


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


def source_ref(record: dict[str, Any]) -> str:
    evidence = record.get("evidence") or []
    if evidence:
        item = evidence[0]
        return code(f"{item.get('path')}:{item.get('lines', ['?'])[0]}")
    return code(f"{record.get('source_file', 'unknown')}:{record.get('source_lines', ['?'])[0]}")


def page_symbols(page: dict[str, Any], limit: int = 18) -> list[dict[str, Any]]:
    candidates = [symbol for symbol in SYMBOLS if symbol["public_api"] and relevant(page, symbol)]
    words = {word.lower() for word in re.findall(r"[A-Za-z]+", page["title"]) if len(word) > 3}
    rank = {"module": 0, "trait": 1, "struct": 2, "enum": 3, "function": 4, "type_alias": 5, "constant": 6, "variant": 7, "struct_field": 8}
    candidates.sort(key=lambda symbol: (
        -sum(word in symbol["name"].lower() or word in symbol["qualified_name"].lower() for word in words),
        0 if symbol["source_documented"] else 1,
        rank.get(symbol["kind"], 20),
        symbol["qualified_name"],
    ))
    return candidates[:limit]


def page_tests(page: dict[str, Any], limit: int = 12) -> list[dict[str, Any]]:
    records = [record for record in TESTS if relevant(page, record)]
    words = {word.lower() for word in re.findall(r"[A-Za-z]+", page["title"]) if len(word) > 3}
    records.sort(key=lambda record: (-sum(word in record["name"].lower() for word in words), record["path"], record["name"]))
    return records[:limit]


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


def claims_for(page: dict[str, Any]) -> list[dict[str, Any]]:
    claims = []
    for number, capability_id in enumerate(page["capability_ids"], 1):
        capability = CAP_BY_ID[capability_id]
        claims.append(make_claim(
            f"CLM-{page['page_id']}-CAP-{number:03d}",
            page,
            "Scope",
            capability["description"],
            capability["evidence"],
        ))
    claims.append(make_claim(
        f"CLM-{page['page_id']}-SOURCE-001",
        page,
        "Evidence boundary",
        "The page is scoped to the listed repository evidence at the frozen snapshot.",
        page["evidence"],
    ))
    if page["path"] == "docs/reference/behavior-evidence.md":
        claims.extend(add_surface_claims(page, BEHAVIORS, "BEHAVIOR", "behavior_id", "name"))
    elif page["path"] == "docs/reference/protocol-surface.md":
        claims.extend(add_surface_claims(page, PROTOCOLS, "PROTOCOL", "protocol_id", "name"))
    elif page["path"] == "docs/reference/test-evidence.md":
        claims.extend(add_surface_claims(page, TESTS, "TEST", "test_id", "name"))
    elif page["path"] == "docs/reference/lifecycle-evidence.md":
        claims.extend(add_surface_claims(page, LIFECYCLES, "LIFECYCLE", "lifecycle_id", "operation"))
    elif page["doc_class"] == "error-reference":
        claims.extend(add_surface_claims(page, [record for record in ERRORS if relevant(page, record)], "ERROR", "error_id", "variant"))
    elif page["doc_class"] == "config-reference" and page["path"].endswith("configuration.md"):
        claims.extend(add_surface_claims(page, CONFIGURATION, "CONFIG", "config_id", "name"))
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
        "These statements describe repository contracts at the documented snapshot. They do not extend platform qualification, performance, retry, or delivery guarantees beyond the native API contracts and executable evidence.",
    ])
    return "\n".join(lines)


def symbols_table(page: dict[str, Any], limit: int = 18) -> str:
    symbols = page_symbols(page, limit)
    if not symbols:
        return "No intentionally public Rust declaration is owned directly by this evidence domain. Use the linked protocol or repository reference."
    lines = ["| Public declaration | Kind | Declared purpose | Source |", "|---|---|---|---|"]
    for symbol in symbols:
        purpose = symbol["summary"] if symbol["source_documented"] else "The compiler exposes this declaration; its native description remains a Gate 9 obligation."
        location = f"{symbol['source_file']}:{symbol['source_lines'][0]}"
        lines.append(f"| {code(symbol['qualified_name'])} | {md(symbol['kind'])} | {md(purpose)[:240]} | {code(location)} |")
    return "\n".join(lines)


def tests_text(page: dict[str, Any]) -> str:
    tests = page_tests(page)
    if not tests:
        return "No directly matching executable test is assigned to this page. That absence does not prove unsupported behavior; consult the unknowns ledger before making a guarantee."
    lines = ["The following test bodies are evidence only for their recorded setup:", ""]
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
        f"This page was verified against Git snapshot {code(SNAPSHOT)} and these primary files:",
        "",
    ]
    for item in page["evidence"]:
        location = f"{item['path']}:{item['lines'][0]}-{item['lines'][1]}"
        lines.append(f"- {code(location)} ({code(item['classification'])})")
    lines.extend([
        "",
        "A file's presence proves implementation or declaration at this snapshot. It does not by itself prove physical-device qualification, operational performance, retry safety, or behavior outside the recorded test conditions.",
    ])
    return "\n".join(lines)


def join_sections(*sections: str) -> str:
    return "\n\n".join(section.strip() for section in sections if section and section.strip())


def overview_body(page: dict[str, Any]) -> str:
    if page["path"] == "README.md":
        example = (ROOT / "examples/product_quickstart.rs").read_text().rstrip()
        install = f"{FENCE}toml\n[dependencies]\npocketstation = \"1.1.1\"\n{FENCE}"
        contracts = f"{FENCE}toml\npocketstation = {{ version = \"1.1.1\", default-features = false }}\n{FENCE}"
        # The crate root includes README.md as rustdoc. Keep the hardware-
        # dependent quickstart compiled as a doctest without executing capture.
        sample = f"{FENCE}rust,no_run\n{example}\n{FENCE}"
        commands = f"{FENCE}bash\ncargo test --examples --all-features\ncargo run --example product_quickstart\n{FENCE}"
        return join_sections(
            "PocketStation is a Rust library for declaring and running source-aware desktop audio Sessions. A Session can keep application and microphone sources separate, route them through bounded paths, expose polled audio, and finalize multistem recording. Extensions, connectors, the C ABI, and sidecars participate through the same declaration and lifecycle model.",
            "## Install\n\nPocketStation 1.1.1 requires Rust 1.95 or newer. Native capture is the default Cargo feature.\n\n" + install + "\n\nUse the contracts-only form when you need public declarations without a native capture backend:\n\n" + contracts,
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

- Rust 1.95 or newer, as declared by package metadata.
- Cargo for dependency and feature resolution.
- Native platform development dependencies when the default native-capture feature is enabled.

## Add the dependency

{FENCE}toml
[dependencies]
pocketstation = "1.1.1"
{FENCE}

For a contracts-only build:

{FENCE}toml
pocketstation = {{ version = "1.1.1", default-features = false }}
{FENCE}

Run {code("cargo check")} to verify dependency resolution. Feature selection is compile-time configuration."""
    elif "quickstart" in title:
        example = (ROOT / "examples/product_quickstart.rs").read_text().rstrip()
        body = f"""## Prerequisites

Choose a host with an application named {code("PocketStation Demo")}, an available default microphone, permission to open both sources, and a writable recording path.

## Program

This source is synchronized with {code("examples/product_quickstart.rs")}:

{FENCE}rust
{example}
{FENCE}

## Run and verify

Run {code("cargo run --example product_quickstart")}. The example rejects the run unless it observes two frames on each of two stems, obtains a successful stop outcome, and sees two completed recording stems with no failed stems."""
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
        scope_section(page), body,
        "## Public entry points\n\n" + symbols_table(page, 16),
        "## Executable evidence\n\n" + tests_text(page),
        related_text(page), evidence_boundary(page),
    )


def concept_body(page: dict[str, Any]) -> str:
    intro = " ".join(CAP_BY_ID[item]["description"] for item in page["capability_ids"])
    journeys = [journey for journey in JOURNEYS if set(page["capability_ids"]).intersection(journey["capability_ids"])]
    encountered = "\n".join(f"- **{journey['name']}** — {journey['outcome']}" for journey in journeys[:8])
    return join_sections(
        intro,
        scope_section(page),
        "## Contract surface\n\n" + symbols_table(page, 20),
        "## Where you encounter it\n\n" + (encountered or "The current capability model has no separate end-to-end journey for this concept."),
        "## Behavior established by tests\n\n" + tests_text(page),
        "## Boundaries\n\nThe compiler inventory establishes names, kinds, visibility, and signatures. Tests establish only their exercised conditions. Where retryability, ordering, cancellation, physical qualification, or recovery is not declared, this page leaves it unspecified.",
        related_text(page), evidence_boundary(page),
    )


def lifecycle_body(page: dict[str, Any]) -> str:
    records = [record for record in LIFECYCLES if relevant(page, record)][:100]
    lines = ["| Operation | Trigger | Source state | Destination state | Evidence record |", "|---|---|---|---|---|"]
    for record in records:
        lines.append(f"| {code(record['operation'])} | {code(record['trigger'])} | {md(record['source_state'])} | {md(record['destination_state'])} | {code(record['lifecycle_id'])} |")
    return join_sections(
        scope_section(page),
        "## Ownership transition\n\nPocketStation uses distinct declaration, compilation, preparation, running, cancellation, rollback, stop, and terminal-result types where the source exposes them. Do not collapse a stop outcome into a Boolean assumption: component and finalization failures remain structured data.",
        "## Extracted lifecycle operations\n\n" + ("\n".join(lines) if records else "No lifecycle record matches this evidence domain."),
        "## Failure handling\n\nA transition whose guard, idempotence, or recovery is recorded as unknown has no published guarantee here. Preserve the returned error or terminal outcome, inspect component and stage fields, and consult the error reference before retrying.",
        "## Executable evidence\n\n" + tests_text(page),
        related_text(page), evidence_boundary(page),
    )


PROCEDURES = {
    "GUIDE-001": ["Create a Session declaration.", "Build an ApplicationSelector whose evidence matches your selection need.", "Declare the application Source and attach a consumer route.", "Start the Session and retain RunningSession.", "Observe frames or typed capture failures, then stop and inspect the outcome."],
    "GUIDE-002": ["Observe permission without prompting when the target exposes that operation.", "Let the host application own any permission prompt.", "Declare the default or identified microphone Source.", "Attach a consumer before start.", "Treat preparation or source opening as the authoritative result."],
    "GUIDE-003": ["Declare application and microphone sources in one Session.", "Give each source an independent endpoint or route.", "Retain stem and source identity from frame lineage.", "Start once and consume both bounded routes.", "Stop once and inspect Session plus recording outcomes."],
    "GUIDE-004": ["Discover candidates through the source provider.", "Build a process or application query with the required scope.", "Resolve the query and retain stable source identity.", "Observe generation changes instead of assuming process identity is permanent.", "Handle empty or ambiguous resolution as a typed result."],
    "GUIDE-005": ["Call microphone_permission_observation before opening a source when preflight information helps the UI.", "Interpret NotObservable as neither allowed nor denied.", "Request permission only through the host application's platform UI.", "Prepare or start the selected source.", "Use the open result as the authoritative decision."],
    "GUIDE-006": ["Declare a separate polled_audio endpoint for each independent route.", "Send the source or stream to that endpoint.", "Call try_poll_audio from non-realtime application code.", "Iterate only indices below the returned batch length.", "Release the lease promptly and inspect polling observations."],
    "GUIDE-007": ["Declare the source once.", "Create each consumer endpoint independently.", "Connect the same source output to each endpoint.", "Set explicit edge policy where the default is unsuitable.", "Observe each route separately so saturation remains attributable."],
    "GUIDE-008": ["Identify producer and consumer partitions.", "Choose finite capacity.", "Select backpressure, loss, copy, delivery, and observation policies.", "Compile and handle rejected contracts.", "Measure queue depth, saturation, and drops before changing capacity."],
    "GUIDE-009": ["Set the recording root on SessionBuilder.", "Call record with a label for each stem.", "Start and run the Session.", "Stop to trigger endpoint finalization.", "Inspect overall and per-stem recording outcomes."],
    "GUIDE-010": ["Retain RunningSession until stop returns.", "Preserve SessionStopOutcome.", "Read recording_outcome after stop.", "Check overall state plus completed and failed stem counts.", "Use error codes and per-stem results to diagnose partial finalization."],
    "GUIDE-011": ["Define named ports in AsyncOperatorManifest.", "Implement factory preparation.", "Return an async node that observes cancellation and declared policies.", "Register before Session compilation.", "Connect named ports and run the separate consumer example."],
    "GUIDE-012": ["Retain typed output and input declaration handles.", "Connect handles with compatible signal specifications.", "Use exact port names from the manifest.", "Compile and handle unknown, duplicate, or incompatible port errors.", "Confirm the compiled binding targets the intended instance."],
    "GUIDE-013": ["Declare generated-audio output.", "Prepare the bounded audio-reentry bridge.", "Produce PCM matching the target sample specification.", "Write from the asynchronous lane.", "Observe accepted, saturated, closed, or cancelled outcomes."],
    "GUIDE-014": ["Implement EndpointDriverFactory preparation.", "Return a prepared driver with its start gate.", "Consume matching audio or signal inputs.", "Honor cancellation and shutdown mode.", "Return finalization observations and staged failures."],
    "GUIDE-015": ["Build ConnectorManifest with node and configuration schemas.", "Validate values and keep secrets in ConnectorSecret.", "Implement finite delivery outcomes.", "Register, declare, and connect the endpoint.", "Run conformance before provider qualification."],
    "GUIDE-016": ["Declare every connector configuration field.", "Use Secret value kind for secret material.", "Construct ConnectorSecret instead of ordinary text values.", "Read validated values during preparation.", "Keep diagnostics on redacted representations."],
    "GUIDE-017": ["Obtain the versioned connector vector file required by portable semantics.", "Place it at the sibling path expected by the test workflow.", "Run connector contract and grouping tests.", "Run portable semantics with the external vector present.", "Keep local conformance and provider qualification as separate evidence."],
    "GUIDE-018": ["Build a dynamic library exporting pks_extension_library_v1.", "Use a canonical absolute path to a trusted regular file.", "Return a compatible descriptor and callbacks.", "Load through Session and retain the receipt.", "Handle registration rollback and executable-code lifetime."],
    "GUIDE-019": ["Include pocketstation.h and use its ABI version.", "Create handles through exported functions.", "Check every PksSessionStatus.", "Stop before releasing runtime ownership.", "Release each handle with its matching ABI function."],
    "GUIDE-020": ["Declare SidecarProcessSpec with bounded limits and deadlines.", "Start the child through SidecarHost.", "Exchange only declared message kinds.", "Apply cancellation, drain, or abort through lifecycle state.", "Inspect host snapshot and terminal error before restart."],
    "GUIDE-021": ["Acquire observation handles before the period you need to inspect.", "Snapshot metrics by stable component IDs.", "Record a trace for durable lifecycle evidence.", "Stop and include the terminal outcome.", "Validate trace structure before diagnosis."],
    "GUIDE-022": ["Retain RunningSession as runtime owner.", "Request stop once application work ends.", "Read component failures in SessionStopOutcome.", "Read recording and trace finalization separately.", "Preserve diagnostics before releasing ownership."],
    "GUIDE-023": ["Create AudioInputConfig matching the producer.", "Acquire a bounded AudioInputBuffer.", "Write only within declared capacity and format.", "Submit through AudioInputWriter and route the source.", "Handle acquire, write, cancellation, and runtime errors separately."],
    "GUIDE-024": ["Choose an evidenced Opus profile and SampleSpec.", "Construct a stateful encoder.", "Encode only accepted frame formats.", "Construct the matching decoder and decode packets.", "Use the round-trip test as executable compatibility evidence."],
    "GUIDE-025": ["Run the repository protocol script.", "Run C ABI and compatibility tests selected by CI.", "Provide required sibling or private fixtures.", "Distinguish absent prerequisites from assertion failures.", "Record command, target, and fixture revision."],
    "GUIDE-026": ["Read the nested example prerequisites.", "Build its Cargo manifest.", "Choose required capture sources.", "Run the external process integration.", "Preserve and validate process evidence."],
    "GUIDE-027": ["Use defaults for native capture applications.", "Disable defaults for contract-only consumers.", "Enable conformance-fixtures only for fixture APIs.", "Reserve internal-testing for repository checks.", "Rebuild after feature changes."],
    "GUIDE-028": ["Build a supported system-capture query.", "Resolve or prepare it through the source provider.", "Attach a bounded consumer.", "Start and observe the typed open result.", "Keep implementation and qualification claims separate."],
    "GUIDE-029": ["Retain source clock-domain identity and timestamp.", "Update TimelineMapping with observed source and Session time.", "Map into the Session domain.", "Observe drift and discontinuity without rewriting lineage.", "Apply correction only through evidenced controller bounds."],
    "GUIDE-030": ["Compile the immutable Session declaration.", "Prepare resources and retain identity mappings.", "Handle source and endpoint preparation errors.", "Start with the intended cancellation option.", "Preserve rollback failures alongside a primary start failure."],
}


def how_to_body(page: dict[str, Any]) -> str:
    steps = PROCEDURES.get(page["page_id"], [])
    if not steps:
        matching = [journey for journey in JOURNEYS if set(page["capability_ids"]).intersection(journey["capability_ids"])]
        raw = matching[0]["steps"] if matching else ["inspect the contract", "perform the operation", "verify the outcome"]
        steps = [step.replace("_", " ").capitalize() + "." for step in raw]
    procedure = "\n".join(f"{number}. {step}" for number, step in enumerate(steps, 1))
    errors = [record for record in ERRORS if relevant(page, record)][:12]
    failure_lines = []
    for record in errors:
        label = code(record["type"])
        if record.get("variant"):
            label += " / " + code(record["variant"])
        failure_lines.append(f"- {label} — {code(record['error_id'])}")
    return join_sections(
        scope_section(page),
        "## Prerequisites\n\nRead the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task. Keep returned typed errors and outcomes available for verification.",
        "## Procedure\n\n" + procedure,
        "## APIs used\n\n" + symbols_table(page, 16),
        "## Verify the outcome\n\n" + tests_text(page),
        "## Failure signals\n\n" + ("\n".join(failure_lines) if failure_lines else "No domain-specific error record is assigned. Preserve the returned error and use the general error index."),
        "Retry only when the relevant API or error contract explicitly permits it. An error name, a transient-looking message, or a successful prior run is not retry evidence.",
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
        rows = [[record["lifecycle_id"], code(record["operation"]), record["trigger"], record["source_state"], record["destination_state"], source_ref(record)] for record in LIFECYCLES]
        table = exhaustive_table("Inventory", ["Evidence record", "Operation", "Trigger", "From", "To", "Source"], rows)
    elif page["doc_class"] == "error-reference":
        records = [record for record in ERRORS if relevant(page, record)]
        rows = []
        for record in records:
            variant = code(record["variant"]) if record.get("variant") else "type"
            rows.append([record["error_id"], code(record["type"]), variant, record["retryable"], record["recoverable"], source_ref(record)])
        table = exhaustive_table("Error inventory", ["Evidence record", "Type", "Variant", "Retryable", "Recoverable", "Defined"], rows)
    elif page["doc_class"] == "config-reference" and path.endswith("configuration.md"):
        rows = [[record["config_id"], record["kind"], code(record["name"]), code(record.get("parent")), record["default"], record["when_read"], source_ref(record)] for record in CONFIGURATION]
        table = exhaustive_table("Configuration inventory", ["Evidence record", "Kind", "Name", "Parent", "Default", "When read", "Source"], rows)
    else:
        symbols = [symbol for symbol in SYMBOLS if symbol["public_api"] and relevant(page, symbol)]
        symbols.sort(key=lambda record: (record["kind"], record["qualified_name"]))
        rows = []
        for symbol in symbols[:300]:
            purpose = (
                symbol["summary"]
                if symbol["source_documented"]
                else NATIVE_DOCS.get(symbol["symbol_id"], "See the native Rust API reference")
            )
            rows.append([code(symbol["qualified_name"]), symbol["kind"], purpose, code(f"{symbol['source_file']}:{symbol['source_lines'][0]}")])
        table = exhaustive_table("Public surface", ["Declaration", "Kind", "Purpose", "Source"], rows)
    native = "The generated [docs.rs API](https://docs.rs/pocketstation/latest/pocketstation/) is the exhaustive symbol-level Rust reference. Human reference pages organize responsibilities and cross-boundary behavior; they do not duplicate every rustdoc signature."
    return join_sections(
        scope_section(page),
        "## Reference authority\n\n" + native,
        table,
        "## Interpretation\n\nAn inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot. Fields shown as unknown remain deliberately unspecified. Consult the native API and error contract before relying on panic behavior, blocking, cancellation, ordering, limits, retry, or recovery.",
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
    errors = [record for record in ERRORS if relevant(page, record)][:30]
    signals = []
    for record in errors:
        label = code(record["type"])
        if record.get("variant"):
            label += " / " + code(record["variant"])
        signals.append(f"- {label} ({code(record['error_id'])})")
    tests = page_tests(page, 15)
    test_lines = [f"- {code(record['name'])} exercises {record['behavior_under_test']} under its recorded setup ({code(record['test_id'])})." for record in tests]
    action = TROUBLESHOOTING_ACTIONS.get(page["page_id"], "Preserve the typed error, lifecycle stage, component identity, and available observations before changing configuration or retrying.")
    return join_sections(
        f"Use this page when you observe **{page['title'].lower()}**. Diagnose the reported stage and identity before changing route, source, connector, or lifecycle policy.",
        "## Distinguish the cause\n\n" + action,
        "## Diagnostic signals\n\n" + ("\n".join(signals) if signals else "No domain-specific error variant is assigned. Use the stable error-code index and terminal outcome."),
        "## Executable evidence\n\n" + ("\n".join(test_lines) if test_lines else "No directly matching executable test is assigned to this symptom. Keep diagnosis within fields returned by the API."),
        "## Corrective action and retry\n\nApply only the action implied by the typed failure or violated precondition. Retry is not safe merely because a failure appears transient. When retryability or recovery is unknown, preserve the failure for application policy or maintainer review.",
        "## Data and state\n\nTreat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial unless the terminal contract says otherwise. Inspect per-route, per-stem, and per-component outcomes.",
        related_text(page), evidence_boundary(page),
    )


def best_practice_body(page: dict[str, Any]) -> str:
    recommendations = {
        "BEST-001": "Measure each bounded route's queue, saturation, drop, and latency observations before changing capacity.",
        "BEST-002": "Keep allocation, blocking, locks, async scheduling, logging, and application callbacks outside the realtime callback path where architecture checks enforce that boundary.",
        "BEST-003": "Retain source, stream, stem, generation, clock, sequence, and derivation identity instead of flattening frames into anonymous PCM.",
        "BEST-004": "Preserve structured stop, component, recording, sidecar, and trace outcomes before releasing runtime ownership.",
        "BEST-005": "Declare finite attempts and time in connector retry policy, and expose readiness and exhaustion through observations.",
        "BEST-006": "Load executable extensions only from a canonical absolute path whose trust decision belongs to the host application.",
        "BEST-007": "Label build, virtual-machine, conformance, and physical-device evidence separately; never promote one scope into another.",
    }
    patterns = [record for record in PATTERNS if relevant(page, record)][:20]
    pattern_lines = [f"- {code(record['name'])} at {code(record['where_implemented'])} ({code(record['pattern_id'])})." for record in patterns]
    return join_sections(
        "## Recommendation\n\n" + recommendations.get(page["page_id"], "Follow the explicit contract and keep unsupported behavior out of application assumptions."),
        "## Why\n\nThe repository makes capacity, ownership, identity, lifecycle, and evidence boundaries explicit so failures remain attributable. Bypassing them removes observations and typed outcomes needed for diagnosis.",
        "## Tradeoff\n\nThe recommendation requires explicit configuration and result handling. It does not promise that one capacity, retry budget, selector, or shutdown policy fits every workload. Measure within the API's stated scope.",
        "## When it does not apply\n\nDo not apply a realtime, connector, capture, or extension rule to another lane or boundary unless it exposes the same contract. An internal pattern is not automatically a public recommendation.",
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
        "## Stability boundary\n\nThis page explains internals. Public compatibility comes from exported Rust declarations, the C header, manifests, error codes, and explicit compatibility artifacts—not private module layout.",
        related_text(page), evidence_boundary(page),
    )


def platform_body(page: dict[str, Any]) -> str:
    return join_sections(
        scope_section(page),
        "## Implemented boundary\n\n" + symbols_table(page, 20),
        "## Permission and source opening\n\nPermission observation and source opening are separate. The host application owns prompts and selection UI. A non-prompting observation is advisory where implemented; preparation or open returns the authoritative result.",
        "## Qualification boundary\n\nTarget-specific files, Cargo dependencies, or CI establish implementation or build evidence only. They do not establish that every device, operating-system revision, packaging context, permission state, or physical path was qualified.",
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
        "PocketStation's package version at the analyzed snapshot is 1.1.1. Release automation and package metadata remain the publication authority; this page preserves the repository's declared release record.",
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
        page_claims = claims_for(page)
        text = f"# {page['title']}\n\n{claim_marker(page_claims)}\n\n{body_for(page).strip()}\n"
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
