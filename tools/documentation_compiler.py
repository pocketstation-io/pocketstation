#!/usr/bin/env python3
"""Evidence compiler and completion authority for PocketStation documentation.

The compiler reads the frozen Git snapshot rather than the mutable working tree.
Its verifier is the only mechanism allowed to set ``state.completion`` to true.
Every other command persists resumable work and leaves the final status false.
"""

from __future__ import annotations

import argparse
import datetime as dt
import functools
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tomllib
from collections import Counter, defaultdict
from pathlib import Path, PurePosixPath
from typing import Any, Iterable, Iterator


ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / ".doc-intel"
STATE = DB / "state.json"
REPOSITORY_MANIFEST = DB / "repository-manifest.jsonl"
SYMBOL_MANIFEST = DB / "symbol-manifest.jsonl"
EDGE_MANIFEST = DB / "edges.jsonl"
PAGE_MANIFEST = DB / "page-manifest.jsonl"

PROTOCOL_SHA256 = "457a830edf137459bd4192b4a8584257f2d323bd4af91b5e65e5079c2ae2a8e2"
CONTRACT_SHA256 = "4f6e8d2ed0a52306a9f2e110100af3a15b942331544dd9a4ea3b719fd558453d"

VALID_FILE_STATES = {
    "pending",
    "analyzed",
    "generated_with_source",
    "third_party_excluded",
    "binary_metadata_only",
    "nonsemantic_with_reason",
}
ANALYSIS_STAGES = {
    "discovered": 1,
    "enriched": 2,
    "relationships_resolved": 3,
    "behavior_validated": 4,
    "doc_ready": 5,
}
TEXT_SUFFIXES = {
    "",
    ".baseline",
    ".c",
    ".cmake",
    ".cpp",
    ".gitignore",
    ".h",
    ".json",
    ".lock",
    ".m",
    ".md",
    ".plist",
    ".rs",
    ".sh",
    ".toml",
    ".txt",
    ".yaml",
    ".yml",
}
LANGUAGES = {
    ".baseline": "ABI baseline",
    ".c": "C",
    ".cmake": "CMake",
    ".cpp": "C++",
    ".gitignore": "Git ignore rules",
    ".h": "C header",
    ".json": "JSON",
    ".lock": "Cargo lockfile",
    ".m": "Objective-C",
    ".md": "Markdown",
    ".plist": "Property list",
    ".rs": "Rust",
    ".sh": "Shell",
    ".toml": "TOML",
    ".txt": "Text",
    ".yaml": "YAML",
    ".yml": "YAML",
}
RUST_ITEM = re.compile(
    r"(?m)^(?P<indent>\s*)(?P<vis>pub(?:\([^)]*\))?\s+)?"
    r"(?:(?:async|unsafe|const|extern\s+\"[^\"]+\")\s+)*"
    r"(?P<kind>struct|enum|trait|union|type|fn|const|static|macro_rules!|mod)\s+"
    r"(?P<name>[A-Za-z_][A-Za-z0-9_]*)"
)
TEST_ITEM = re.compile(
    r"(?ms)(?P<attrs>(?:\s*#\[[^]]*\]\s*)+)"
    r"(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+"
    r"(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\("
)
EXECUTABLE_RUST_TEST_ATTRIBUTE = re.compile(
    r"#\[\s*(?:(?:[A-Za-z_][A-Za-z0-9_]*::)*test|(?:[A-Za-z_][A-Za-z0-9_]*::)*rstest)\b"
)
ENV_VAR = re.compile(
    r"(?:std::env::var|env::var|var_os|option_env!|env!)\s*\(\s*\"([A-Z][A-Z0-9_]*)\""
)
MARKDOWN_LINK = re.compile(r"(?<!!)\[[^]]+\]\(([^)]+)\)")
PLACEHOLDER = re.compile(r"(?i)\b(?:TODO|TBD|FIXME|XXX|lorem ipsum|coming soon)\b")

PUBLICATION_BOILERPLATE = (
    "The compiler exposes this declaration; its native description remains a Gate 9 obligation.",
    "Read the linked concept and confirm that target platform, Cargo features, source or provider dependencies, and application-owned permission work match this task.",
    "The following test bodies are evidence only for their recorded setup:",
    "An inventory row establishes that a declaration, test, lifecycle operation, configuration surface, or protocol element exists at the frozen snapshot.",
    "These statements describe repository contracts at the documented snapshot.",
    "A file's presence proves implementation or declaration at this snapshot.",
    "Apply only the action implied by the typed failure or violated precondition.",
    "Treat frames, signals, files, acknowledgements, and finalization results produced before failure as potentially partial",
)

REQUIRED_PAGE_SECTIONS = {
    "concept": {
        "What it is", "Why it exists", "Relationships", "Invariants and guarantees",
        "When you encounter it", "Use it",
    },
    "how-to": {"Prerequisites", "Procedure", "Verify the outcome", "Failure signals", "API reference"},
    "troubleshooting": {
        "Symptom", "Evidenced causes", "Distinguish the causes", "Corrective action",
        "Retry and incomplete state", "Related reference",
    },
    "best-practice": {
        "Problem", "Recommendation", "Reason", "Tradeoff", "When it does not apply",
        "Repository evidence",
    },
}


class CompilerError(RuntimeError):
    """Signals a deterministic compiler or verifier failure."""


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def stable_id(prefix: str, value: str) -> str:
    return f"{prefix}-{sha256_bytes(value.encode())[:20]}"


def run(*args: str, check: bool = True, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    merged = os.environ.copy()
    if env:
        merged.update(env)
    command = list(args)
    if command and command[0] in {"cargo", "rustc", "rustdoc"} and shutil.which(command[0]) is None:
        rustup = shutil.which("rustup") or "/opt/homebrew/bin/rustup"
        binary = run_rustup_which(rustup, command[0])
        toolchain_bin = str(Path(binary).parent)
        merged["PATH"] = toolchain_bin + os.pathsep + merged.get("PATH", "")
        command[0] = binary
    return subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=check,
        env=merged,
    )


def run_rustup_which(rustup: str, binary: str) -> str:
    result = subprocess.run(
        [rustup, "which", binary], cwd=ROOT, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True,
    )
    return result.stdout.strip()


def git(*args: str) -> str:
    return run("git", *args).stdout.strip()


@functools.lru_cache(maxsize=None)
def git_bytes(snapshot: str, path: str) -> bytes:
    result = subprocess.run(
        ["git", "show", f"{snapshot}:{path}"],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    )
    return result.stdout


def read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    records: list[dict[str, Any]] = []
    for number, line in enumerate(path.read_text().splitlines(), 1):
        if not line.strip():
            continue
        try:
            value = json.loads(line)
        except json.JSONDecodeError as error:
            raise CompilerError(f"{path}:{number}: invalid JSON: {error}") from error
        if not isinstance(value, dict):
            raise CompilerError(f"{path}:{number}: record is not an object")
        records.append(value)
    return records


def write_jsonl(path: Path, records: Iterable[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        for record in records:
            handle.write(json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n")


def load_state() -> dict[str, Any]:
    if not STATE.exists():
        raise CompilerError("state is absent; run `documentation_compiler.py init` first")
    state = read_json(STATE)
    if not isinstance(state, dict):
        raise CompilerError("state.json must contain an object")
    return state


def save_state(state: dict[str, Any]) -> None:
    state["updated_at"] = now()
    # Only verify may replace this with true.
    if state.get("last_command") != "verify-pass":
        state["completion"] = False
    write_json(STATE, state)


def line_number(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def brace_end(text: str, start: int) -> int:
    opening = text.find("{", start)
    if opening < 0:
        semicolon = text.find(";", start)
        return semicolon + 1 if semicolon >= 0 else start
    depth = 0
    in_string = False
    escaped = False
    line_comment = False
    block_comment = 0
    index = opening
    while index < len(text):
        char = text[index]
        following = text[index + 1] if index + 1 < len(text) else ""
        if line_comment:
            line_comment = char != "\n"
        elif block_comment:
            if char == "/" and following == "*":
                block_comment += 1
                index += 1
            elif char == "*" and following == "/":
                block_comment -= 1
                index += 1
        elif in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
        elif char == "/" and following == "/":
            line_comment = True
            index += 1
        elif char == "/" and following == "*":
            block_comment = 1
            index += 1
        elif char == '"':
            in_string = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    return len(text)


def path_language(path: str) -> str:
    suffix = PurePosixPath(path).suffix.lower()
    return LANGUAGES.get(suffix, "Text" if not suffix else "Unknown")


def classify_file(path: str, data: bytes) -> dict[str, Any]:
    suffix = PurePosixPath(path).suffix.lower()
    if suffix not in TEXT_SUFFIXES or b"\x00" in data:
        return {
            "semantic": False,
            "file_kind": "binary",
            "status": "binary_metadata_only",
            "reason": "Binary or unsupported media is inventoried by Git object, size, type, and hash.",
        }
    if suffix == ".lock":
        return {
            "semantic": False,
            "file_kind": "lockfile",
            "status": "nonsemantic_with_reason",
            "reason": "Generated dependency resolution is inventory evidence, not application behavior.",
        }
    if path == ".gitignore":
        return {
            "semantic": False,
            "file_kind": "repository_metadata",
            "status": "nonsemantic_with_reason",
            "reason": "Ignore patterns are repository metadata and contain no product behavior.",
        }
    if path.startswith("src/"):
        kind = "source"
    elif path.startswith("tests/"):
        kind = "test"
    elif path.startswith("examples/"):
        kind = "example"
    elif path.startswith("benches/"):
        kind = "benchmark"
    elif path.startswith("native/"):
        kind = "native_source"
    elif path.startswith("include/"):
        kind = "public_ffi"
    elif path.startswith(("scripts/", ".github/")) or path == "build.rs":
        kind = "automation"
    elif path.startswith("docs/") or suffix == ".md":
        kind = "documentation"
    else:
        kind = "semantic_metadata"
    return {"semantic": True, "file_kind": kind, "status": "pending", "reason": None}


def snapshot_tree(snapshot: str) -> list[dict[str, str]]:
    result = subprocess.run(
        ["git", "ls-tree", "-r", "-z", "--full-tree", snapshot],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        check=True,
    ).stdout
    records: list[dict[str, str]] = []
    for raw in result.split(b"\0"):
        if not raw:
            continue
        header, encoded_path = raw.split(b"\t", 1)
        mode, object_type, git_object = header.decode().split()
        records.append(
            {
                "path": encoded_path.decode("utf-8", errors="surrogateescape"),
                "mode": mode,
                "object_type": object_type,
                "git_object": git_object,
            }
        )
    return sorted(records, key=lambda record: record["path"])


def repository_languages(records: list[dict[str, Any]]) -> dict[str, int]:
    counts: Counter[str] = Counter()
    for record in records:
        counts[record["language"]] += 1
    return dict(sorted(counts.items()))


def refresh_state_counts(state: dict[str, Any]) -> None:
    files = read_jsonl(REPOSITORY_MANIFEST)
    symbols = read_jsonl(SYMBOL_MANIFEST)
    state["tracked_files"] = len(files)
    state["semantic_files"] = sum(bool(record.get("semantic")) for record in files)
    state["semantic_files_analyzed"] = sum(
        bool(record.get("semantic")) and record.get("status") == "analyzed" for record in files
    )
    pending_files = [record["path"] for record in files if record.get("status") == "pending"]
    state["next_file"] = pending_files[0] if pending_files else None
    state["public_symbols"] = sum(bool(record.get("public_api")) for record in symbols)
    state["symbols_analyzed"] = sum(record.get("status") == "analyzed" for record in symbols)
    pending_symbols = [record["symbol_id"] for record in symbols if record.get("status") == "pending"]
    state["next_symbol"] = pending_symbols[0] if pending_symbols else None
    pages = read_jsonl(PAGE_MANIFEST)
    state["pages"] = len(pages)
    state["pages_validated"] = sum(record.get("status") == "validated" for record in pages)
    pending_pages = [record["page_id"] for record in pages if record.get("status") != "validated"]
    state["next_page"] = pending_pages[0] if pending_pages else None


def cmd_init(arguments: argparse.Namespace) -> None:
    DB.mkdir(parents=True, exist_ok=True)
    if STATE.exists() and not arguments.force:
        raise CompilerError("state already exists; init is intentionally non-destructive")
    if arguments.force:
        dossier_dir = DB / "files"
        if dossier_dir.exists():
            for dossier_path in dossier_dir.glob("file-*.json"):
                if dossier_path.is_file():
                    dossier_path.unlink()
        checkpoint_dir = DB / "checkpoints"
        if checkpoint_dir.exists():
            for checkpoint_path in checkpoint_dir.iterdir():
                if checkpoint_path.is_file():
                    checkpoint_path.unlink()
        coverage_path = DB / "coverage.json"
        if coverage_path.exists():
            coverage_path.unlink()
    snapshot = arguments.snapshot or git("rev-parse", "HEAD")
    if run("git", "cat-file", "-e", f"{snapshot}^{{commit}}", check=False).returncode:
        raise CompilerError(f"snapshot is not a commit: {snapshot}")
    snapshot = git("rev-parse", snapshot)
    manifest: list[dict[str, Any]] = []
    for number, item in enumerate(snapshot_tree(snapshot), 1):
        data = git_bytes(snapshot, item["path"])
        classification = classify_file(item["path"], data)
        manifest.append(
            {
                "file_id": f"file-{number:04d}-{stable_id('p', item['path'])[2:10]}",
                "path": item["path"],
                "snapshot": snapshot,
                "git_object": item["git_object"],
                "git_mode": item["mode"],
                "object_type": item["object_type"],
                "sha256": sha256_bytes(data),
                "bytes": len(data),
                "language": path_language(item["path"]),
                **classification,
                "dossier": None,
                "analysis_stage": None,
                "analyzed_at": None,
            }
        )
    write_jsonl(REPOSITORY_MANIFEST, manifest)
    write_jsonl(DB / "inventory.jsonl", manifest)
    status = git("status", "--porcelain=v1", "--untracked-files=all").splitlines()
    submodules = run("git", "submodule", "status", check=False).stdout.splitlines()
    cargo = tomllib.loads(git_bytes(snapshot, "Cargo.toml").decode()) if any(
        item["path"] == "Cargo.toml" for item in manifest
    ) else {}
    snapshot_record = {
        "repository_name": ROOT.name,
        "absolute_repository_path": str(ROOT),
        "branch_at_initialization": git("branch", "--show-current"),
        "snapshot": snapshot,
        "head_at_initialization": git("rev-parse", "HEAD"),
        "working_tree_status_at_initialization": status,
        "submodules": submodules,
        "workspace_members": cargo.get("workspace", {}).get("members", [cargo.get("package", {}).get("name", ROOT.name)]),
        "package_members": [cargo.get("package", {}).get("name")] if cargo.get("package") else [],
        "languages": repository_languages(manifest),
        "build_systems": ["Cargo"] if cargo else [],
        "package_managers": ["Cargo"] if cargo else [],
        "ci_definitions": [record["path"] for record in manifest if record["path"].startswith(".github/workflows/")],
        "release_configuration": [
            record["path"] for record in manifest
            if record["path"] == "Cargo.toml" or "release" in record["path"].lower() or "publish" in record["path"].lower()
        ],
        "documentation_frameworks": ["rustdoc", "repository Markdown"],
        "manifest_sha256": sha256_file(REPOSITORY_MANIFEST),
        "governing_inputs": read_json(DB / "GOVERNING-INPUTS.json"),
        "created_at": now(),
    }
    write_json(DB / "snapshot.json", snapshot_record)
    empty_ledgers = [
        "symbol-manifest.jsonl",
        "edges.jsonl",
        "behaviors.jsonl",
        "lifecycles.jsonl",
        "errors.jsonl",
        "configuration.jsonl",
        "protocols.jsonl",
        "tests.jsonl",
        "examples.jsonl",
        "claims.jsonl",
        "conflicts.jsonl",
        "unknowns.jsonl",
        "capabilities.jsonl",
        "user-journeys.jsonl",
        "page-manifest.jsonl",
        "doc-map.jsonl",
        "native-docs.jsonl",
        "patterns.jsonl",
    ]
    for name in empty_ledgers:
        write_jsonl(DB / name, [])
    write_json(DB / "terminology.json", {"status": "pending", "terms": []})
    state: dict[str, Any] = {
        "schema_version": 1,
        "snapshot": snapshot,
        "phase": "file-analysis",
        "gate": 0,
        "docs_generation_allowed": False,
        "completion": False,
        "created_at": now(),
        "last_command": "init",
        "gates": {str(number): {"status": "pending", "failures": []} for number in range(13)},
    }
    refresh_state_counts(state)
    save_state(state)
    print_status(state)


def rust_module(path: str) -> str:
    if path == "src/lib.rs":
        return "pocketstation"
    if not path.startswith("src/") or not path.endswith(".rs"):
        return "not_applicable"
    relative = path[4:-3]
    if relative.endswith("/mod"):
        relative = relative[:-4]
    return "pocketstation::" + relative.replace("/", "::")


def source_purpose(path: str, text: str, kind: str) -> tuple[str, str]:
    if path.endswith(".md"):
        heading = re.search(r"(?m)^#\s+(.+)$", text)
        if heading:
            return f"Documents {heading.group(1).strip()}.", "DECLARED"
    if path.endswith(".rs"):
        docs = [line.strip()[3:].strip() for line in text.splitlines()[:100] if line.strip().startswith("//!")]
        if docs:
            return " ".join(docs)[:1200], "DECLARED"
        return f"Defines the Rust module and items owned by `{rust_module(path)}`.", "INFERRED"
    descriptions = {
        "test": "Provides executable repository-owned behavioral evidence.",
        "example": "Provides repository-owned intended-use evidence.",
        "benchmark": "Defines a repository-owned performance measurement workload.",
        "native_source": "Implements a native or platform boundary.",
        "public_ffi": "Declares the public foreign-function interface.",
        "automation": "Defines a repository build, validation, CI, or release operation.",
        "semantic_metadata": "Defines repository-owned build, package, or semantic fixture data.",
    }
    return descriptions.get(kind, f"Provides repository-owned content at `{path}`."), "INFERRED"


def cfg_evidence(text: str) -> tuple[list[str], list[str]]:
    attrs = re.findall(r"#\s*\[\s*cfg(?:_attr)?\s*\(([^]]+)\)\s*\]", text)
    platforms: set[str] = set()
    features: set[str] = set()
    for attr in attrs:
        platforms.update(re.findall(r'target_(?:os|arch|family)\s*=\s*"([^"]+)"', attr))
        features.update(re.findall(r'feature\s*=\s*"([^"]+)"', attr))
    return sorted(platforms), sorted(features)


def rust_imports(text: str) -> list[str]:
    found = {
        " ".join(value.split())
        for value in re.findall(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);", text)
    }
    found.update(
        f"module:{name}" for name in re.findall(
            r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*);", text
        )
    )
    return sorted(found)


def call_tokens(text: str) -> list[str]:
    excluded = {"if", "for", "while", "match", "return", "sizeof", "Some", "Ok", "Err"}
    return sorted(
        {
            match.group(1)
            for match in re.finditer(r"\b([A-Za-z_][A-Za-z0-9_:]*)\s*\(", text)
            if match.group(1) not in excluded
        }
    )[:1000]


def line_matches(text: str, patterns: tuple[str, ...], limit: int = 200) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for number, line in enumerate(text.splitlines(), 1):
        if any(re.search(pattern, line) for pattern in patterns):
            records.append({"line": number, "text": line.strip()[:500]})
            if len(records) == limit:
                break
    return records


def make_dossier(record: dict[str, Any], data: bytes) -> dict[str, Any]:
    text = data.decode("utf-8", errors="replace")
    path = record["path"]
    platforms, features = cfg_evidence(text)
    rust_items = [
        {
            "kind": match.group("kind"),
            "name": match.group("name"),
            "visibility": (match.group("vis") or "private").strip(),
            "line": line_number(text, match.start()),
        }
        for match in RUST_ITEM.finditer(text)
    ] if path.endswith(".rs") else []
    purpose, purpose_class = source_purpose(path, text, record["file_kind"])
    public_items = [item for item in rust_items if item["visibility"].startswith("pub")]
    imports = rust_imports(text) if path.endswith(".rs") else []
    reexports = [value for value in imports if text.find(f"pub use {value}") >= 0]
    filesystem = line_matches(text, (r"\b(?:File|OpenOptions|read_to_string|write_all|fs::)",))
    network = line_matches(text, (r"\b(?:Tcp|Udp|Socket|WebSocket|http|https|connect\()",))
    device = line_matches(text, (r"\b(?:cpal|pipewire|wasapi|CoreAudio|AudioDevice|device)\b",))
    process = line_matches(text, (r"\b(?:Command|Child|process::|spawn\()",))
    ffi = line_matches(text, (r"extern\s+\"C\"|unsafe\s*\{|\bffi\b|\bABI\b",))
    concurrency = line_matches(
        text,
        (r"\b(?:thread|spawn|tokio|async|await|channel|Sender|Receiver|Mutex|RwLock|Atomic|callback)\b",),
    )
    error_lines = line_matches(
        text,
        (r"\b(?:Error|Result|Err\(|map_err|panic!|assert!|expect\(|unwrap\()",),
    )
    config_lines = line_matches(
        text,
        (r"\b(?:Config|Configuration|Options|Policy|DEFAULT|feature|env::|cfg\()",),
    )
    lifecycle_lines = line_matches(
        text,
        (r"\b(?:prepare|start|running|cancel|stop|drain|finalize|shutdown|drop|abort|join)\b",),
    )
    evidence = [{
        "path": path,
        "content_hash": record["sha256"],
        "lines": [1, max(1, text.count("\n") + 1)],
        "symbol": None,
        "classification": "DIRECT",
    }]
    return {
        "file_id": record["file_id"],
        "path": path,
        "content_hash": record["sha256"],
        "git_object": record["git_object"],
        "snapshot": record["snapshot"],
        "language": record["language"],
        "file_kind": record["file_kind"],
        "line_count": max(1, text.count("\n") + 1),
        "package": "pocketstation" if path.endswith(".rs") or path == "Cargo.toml" else "repository",
        "module": rust_module(path),
        "visibility": "mixed" if public_items else "private_or_not_applicable",
        "generated": False,
        "generated_from": "not_applicable",
        "platform_gate": platforms or ["not_explicitly_gated"],
        "feature_gate": features or ["not_explicitly_gated"],
        "purpose": {"text": purpose, "classification": purpose_class},
        "responsibilities": [item["name"] for item in rust_items[:100]],
        "non_responsibilities": ["unknown"],
        "defines": rust_items,
        "imports": imports,
        "imported_by": [],
        "reexports": reexports,
        "calls": call_tokens(text),
        "called_by": [],
        "constructs": [value for value in call_tokens(text) if value.endswith(("new", "builder", "from"))],
        "constructed_by": [],
        "implements": [match.group(1).strip() for match in re.finditer(r"(?m)^\s*impl(?:<[^>]+>)?\s+([^\{]+)\{", text)],
        "implemented_by": [],
        "extends": [],
        "extended_by": [],
        "entry_points": public_items,
        "public_surface": public_items,
        "private_surface": [item for item in rust_items if not item["visibility"].startswith("pub")],
        "inputs": ["See compiler-backed symbol signatures"],
        "outputs": ["See compiler-backed symbol signatures"],
        "side_effects": [item for group in (filesystem, network, device, process, ffi) for item in group],
        "filesystem_io": filesystem,
        "network_io": network,
        "device_io": device,
        "process_io": process,
        "ffi_io": ffi,
        "threads": [item for item in concurrency if "thread" in item["text"] or "spawn" in item["text"]],
        "tasks": [item for item in concurrency if "tokio" in item["text"] or "async" in item["text"] or "await" in item["text"]],
        "async_boundaries": [item for item in concurrency if "async" in item["text"] or "await" in item["text"]],
        "queues": [item for item in concurrency if "queue" in item["text"].lower()],
        "channels": [item for item in concurrency if "channel" in item["text"].lower() or "Sender" in item["text"] or "Receiver" in item["text"]],
        "locks": [item for item in concurrency if "Mutex" in item["text"] or "RwLock" in item["text"]],
        "atomics": [item for item in concurrency if "Atomic" in item["text"]],
        "callbacks": [item for item in concurrency if "callback" in item["text"].lower()],
        "resource_ownership": line_matches(text, (r"\b(?:Box|Arc|Rc|Owned|owner|Drop)\b",)),
        "startup_behavior": [item for item in lifecycle_lines if "start" in item["text"] or "prepare" in item["text"]],
        "shutdown_behavior": [item for item in lifecycle_lines if "stop" in item["text"] or "shutdown" in item["text"]],
        "cancellation_behavior": [item for item in lifecycle_lines if "cancel" in item["text"]],
        "drop_cleanup": [item for item in lifecycle_lines if re.search(r"\b[Dd]rop\b", item["text"])],
        "finalization": [item for item in lifecycle_lines if "final" in item["text"] or "drain" in item["text"]],
        "state_machine": line_matches(text, (r"\b(?:State|Status|Phase|Transition)\b",)),
        "invariants": line_matches(text, (r"\b(?:assert|invariant|must|cannot|never)\b",)),
        "errors_defined": [item for item in rust_items if item["kind"] in {"enum", "struct"} and "error" in item["name"].lower()],
        "errors_created": [item for item in error_lines if "Err(" in item["text"]],
        "errors_wrapped": [item for item in error_lines if "map_err" in item["text"]],
        "errors_propagated": [item for item in error_lines if "?" in item["text"]],
        "errors_translated": [item for item in error_lines if "from" in item["text"].lower() or "map_err" in item["text"]],
        "retry_behavior": line_matches(text, (r"\b(?:retry|attempt|backoff)\b",)),
        "recovery_behavior": line_matches(text, (r"\b(?:recover|rollback|restore|resume)\b",)),
        "configuration_read": config_lines,
        "environment_variables": sorted(set(ENV_VAR.findall(text))),
        "feature_flags": features,
        "defaults": [item for item in config_lines if "DEFAULT" in item["text"] or "default" in item["text"].lower()],
        "protocol_messages": line_matches(text, (r"\b(?:Protocol|Message|Request|Response|Command|Event)\b",)),
        "endpoints": line_matches(text, (r"\b(?:Endpoint|endpoint)\b",)),
        "serialization": line_matches(text, (r"\b(?:Serialize|Deserialize|serde|encode|decode)\b",)),
        "tests_covering": [],
        "examples_using": [],
        "related_docs": [],
        "observed_patterns": [],
        "potential_pitfalls": [],
        "evidence": evidence,
        "analysis_stage": "discovered",
    }


def cmd_analyze_files(arguments: argparse.Namespace) -> None:
    state = load_state()
    records = read_jsonl(REPOSITORY_MANIFEST)
    remaining = [record for record in records if record.get("status") == "pending"]
    selected = remaining if arguments.batch <= 0 else remaining[: arguments.batch]
    for record in selected:
        data = git_bytes(state["snapshot"], record["path"])
        if sha256_bytes(data) != record["sha256"]:
            raise CompilerError(f"snapshot hash drift for {record['path']}")
        dossier = make_dossier(record, data)
        dossier_path = DB / "files" / f"{record['file_id']}.json"
        write_json(dossier_path, dossier)
        record["status"] = "analyzed"
        record["dossier"] = dossier_path.relative_to(ROOT).as_posix()
        record["analysis_stage"] = "discovered"
        record["analyzed_at"] = now()
    write_jsonl(REPOSITORY_MANIFEST, records)
    write_jsonl(DB / "inventory.jsonl", records)
    state["last_command"] = "analyze-files"
    state["phase"] = "file-analysis" if len(selected) < len(remaining) else "symbol-extraction"
    refresh_state_counts(state)
    save_state(state)
    print_status(state)


def rustdoc_kind(item: dict[str, Any]) -> str:
    inner = item.get("inner") or {}
    return next(iter(inner), "unknown")


def rustdoc_child_ids(item: dict[str, Any]) -> list[int]:
    inner = item.get("inner") or {}
    kind = next(iter(inner), None)
    value = inner.get(kind, {}) if kind else {}
    children: list[int] = []
    if kind in {"module", "trait", "impl"}:
        children.extend(value.get("items", []))
    elif kind == "enum":
        children.extend(value.get("variants", []))
        children.extend(value.get("impls", []))
    elif kind == "variant":
        variant_kind = value.get("kind", {})
        if isinstance(variant_kind, dict):
            detail = next(iter(variant_kind.values()), {})
            if isinstance(detail, list):
                children.extend(detail)
            elif isinstance(detail, dict):
                children.extend(detail.get("fields", []))
    elif kind == "union":
        children.extend(value.get("fields", []))
        children.extend(value.get("impls", []))
    elif kind == "struct":
        struct_kind = value.get("kind", {})
        if isinstance(struct_kind, dict):
            detail = next(iter(struct_kind.values()), {})
            if isinstance(detail, dict):
                children.extend(detail.get("fields", []))
        children.extend(value.get("impls", []))
    return [child for child in children if isinstance(child, int)]


def rustdoc_references(value: Any) -> Iterator[int]:
    if isinstance(value, dict):
        for key, child in value.items():
            if key == "id" and isinstance(child, int):
                yield child
            else:
                yield from rustdoc_references(child)
    elif isinstance(value, list):
        for child in value:
            yield from rustdoc_references(child)


def source_excerpt(snapshot: str, path: str, begin: int, end: int) -> str:
    lines = git_bytes(snapshot, path).decode("utf-8", errors="replace").splitlines()
    return "\n".join(lines[max(0, begin - 1): min(len(lines), end)])


def rustdoc_signature(item: dict[str, Any]) -> Any:
    value = item.get("inner", {}).get(rustdoc_kind(item), {})
    if not isinstance(value, dict):
        return "not_applicable"
    for key in ("sig", "type", "generics"):
        if key in value:
            return value[key]
    return "See source excerpt and compiler item kind."


def rustdoc_stable_key(item: dict[str, Any]) -> tuple[Any, ...] | None:
    span = item.get("span")
    if not isinstance(span, dict) or not item.get("name"):
        return None
    return (
        span.get("filename"),
        tuple(span.get("begin", [])),
        tuple(span.get("end", [])),
        item.get("name"),
        rustdoc_kind(item),
    )


def visibility_name(value: Any) -> str:
    if isinstance(value, str):
        return value
    if isinstance(value, dict) and "restricted" in value:
        restriction = value["restricted"]
        return f"restricted:{restriction.get('path', restriction.get('parent', 'unknown'))}"
    return "unknown"


def parent_map(index: dict[str, dict[str, Any]]) -> dict[int, int]:
    parents: dict[int, int] = {}
    for encoded_id, item in index.items():
        item_id = int(encoded_id)
        for child in rustdoc_child_ids(item):
            child_item = index.get(str(child))
            if child_item and rustdoc_kind(item) != "impl":
                parents.setdefault(child, item_id)
    return parents


def qualified_name(
    item_id: int,
    item: dict[str, Any],
    index: dict[str, dict[str, Any]],
    parents: dict[int, int],
    paths: dict[str, Any],
) -> str:
    path_record = paths.get(str(item_id))
    if isinstance(path_record, dict) and path_record.get("path"):
        return "::".join(path_record["path"])
    components: list[str] = []
    seen: set[int] = set()
    cursor: int | None = item_id
    while cursor is not None and cursor not in seen:
        seen.add(cursor)
        current = index.get(str(cursor), {})
        if current.get("name"):
            components.append(current["name"])
        cursor = parents.get(cursor)
    components.reverse()
    return "::".join(components) or f"rustdoc::{item_id}"


def cmd_extract_symbols(arguments: argparse.Namespace) -> None:
    state = load_state()
    files = read_jsonl(REPOSITORY_MANIFEST)
    if any(record.get("semantic") and record.get("status") != "analyzed" for record in files):
        raise CompilerError("Gate 1 has not passed: semantic file records remain pending")
    private_path = Path(arguments.private_json).resolve()
    public_path = Path(arguments.public_json).resolve()
    private_bytes = private_path.read_bytes()
    public_bytes = public_path.read_bytes()
    private = json.loads(private_bytes)
    public = json.loads(public_bytes)
    if not private.get("includes_private"):
        raise CompilerError("private rustdoc JSON was not built with --document-private-items")
    if public.get("includes_private"):
        raise CompilerError("public rustdoc JSON unexpectedly includes private items")
    if private.get("format_version") != public.get("format_version"):
        raise CompilerError("rustdoc JSON format versions differ")
    snapshot = state["snapshot"]
    file_by_path = {record["path"]: record for record in files}
    index: dict[str, dict[str, Any]] = private["index"]
    public_index: dict[str, dict[str, Any]] = public["index"]
    parents = parent_map(index)
    public_keys: set[tuple[Any, ...]] = set()
    public_qualified: dict[tuple[Any, ...], str] = {}
    public_parents = parent_map(public_index)
    for encoded_id, public_item in public_index.items():
        if public_item.get("crate_id") != 0:
            continue
        key = rustdoc_stable_key(public_item)
        if key is None or key[0] not in file_by_path:
            continue
        public_keys.add(key)
        public_qualified[key] = qualified_name(
            int(encoded_id), public_item, public_index, public_parents, public.get("paths", {})
        )
    compiler_to_symbol: dict[int, str] = {}
    candidates: list[tuple[int, dict[str, Any], str, int, int, bool]] = []
    allowed_kinds = {
        "module", "struct", "struct_field", "union", "enum", "variant", "function",
        "trait", "type_alias", "constant", "static", "macro", "assoc_const", "assoc_type",
    }
    for encoded_id, item in index.items():
        item_id = int(encoded_id)
        if item.get("crate_id") != 0 or not item.get("name"):
            continue
        kind = rustdoc_kind(item)
        if kind not in allowed_kinds:
            continue
        span = item.get("span")
        if not isinstance(span, dict):
            continue
        path = span.get("filename")
        if path not in file_by_path:
            continue
        begin = int(span["begin"][0])
        end = int(span["end"][0])
        excerpt = source_excerpt(snapshot, path, begin, end)
        # Rustdoc emits derived methods at the type declaration span. They are not
        # authored public declarations and do not belong in the intentional API denominator.
        authored = bool(re.search(rf"\b{re.escape(item['name'])}\b", excerpt))
        stable_key = rustdoc_stable_key(item)
        public_api = stable_key in public_keys and authored
        candidates.append((item_id, item, path, begin, end, public_api))
        compiler_to_symbol[item_id] = stable_id("sym", f"{snapshot}:{item_id}")
    records: list[dict[str, Any]] = []
    for item_id, item, path, begin, end, public_api in sorted(
        candidates, key=lambda value: (value[2], value[3], value[0])
    ):
        kind = rustdoc_kind(item)
        name = item["name"]
        docs = item.get("docs")
        platforms, features = cfg_evidence(source_excerpt(snapshot, path, begin, end))
        parent_id = parents.get(item_id)
        child_ids = [compiler_to_symbol[child] for child in rustdoc_child_ids(item) if child in compiler_to_symbol]
        record = {
            "symbol_id": compiler_to_symbol[item_id],
            "compiler_id": item_id,
            "snapshot": snapshot,
            "qualified_name": public_qualified.get(
                rustdoc_stable_key(item),
                qualified_name(item_id, item, index, parents, private.get("paths", {})),
            ),
            "name": name,
            "kind": kind,
            "source_file": path,
            "source_file_sha256": file_by_path[path]["sha256"],
            "source_lines": [begin, end],
            "visibility": visibility_name(item.get("visibility")),
            "public_api": public_api,
            "intentionally_public_rule": "stable span+name+kind present in public rustdoc and name lexically authored at compiler span",
            "signature": rustdoc_signature(item),
            "parent": compiler_to_symbol.get(parent_id),
            "children": child_ids,
            "summary": docs.split("\n\n", 1)[0] if docs else "unknown",
            "responsibility": "unknown" if not docs else docs.split("\n\n", 1)[0],
            "when_to_use": "unknown",
            "when_not_to_use": "unknown",
            "parameters": item.get("inner", {}).get(kind, {}).get("sig", {}).get("inputs", []) if kind == "function" else [],
            "type_parameters": item.get("inner", {}).get(kind, {}).get("generics", {}).get("params", []) if isinstance(item.get("inner", {}).get(kind), dict) else [],
            "return_value": item.get("inner", {}).get(kind, {}).get("sig", {}).get("output") if kind == "function" else "not_applicable",
            "yield_value": "not_applicable",
            "preconditions": [],
            "postconditions": [],
            "side_effects": [],
            "errors": [],
            "panic_behavior": "unknown",
            "exceptions": "not_applicable",
            "safety_contract": "unknown" if "unsafe" in json.dumps(item.get("inner", {})) else "not_applicable",
            "blocking_behavior": "unknown",
            "async_behavior": "async" if '"is_async":true' in json.dumps(item.get("inner", {}), separators=(",", ":")) else "synchronous_or_not_applicable",
            "cancellation": "unknown",
            "thread_safety": "unknown",
            "ordering": "unknown",
            "backpressure": "unknown",
            "ownership": "See signature and source.",
            "lifetime": "See compiler signature.",
            "mutability": "See compiler signature.",
            "defaults": [],
            "valid_values": [],
            "units": "unknown",
            "limits": "unknown",
            "implemented_by": [],
            "implements": [],
            "overrides": [],
            "calls": [],
            "called_by": [],
            "creates": [],
            "created_by": [],
            "reads": [],
            "writes": [],
            "platforms": platforms or ["not_explicitly_gated"],
            "feature_flags": features,
            "tests": [],
            "examples": [],
            "deprecation": item.get("deprecation"),
            "replacement": "unknown" if item.get("deprecation") else "not_applicable",
            "source_documented": bool(docs and docs.strip()),
            "reference_status": "pending" if public_api else "not_required",
            "reference_pages": [],
            "status": "analyzed",
            "evidence": [{
                "path": path,
                "content_hash": file_by_path[path]["sha256"],
                "lines": [begin, end],
                "symbol": name,
                "classification": "DIRECT",
            }],
        }
        records.append(record)
    write_jsonl(SYMBOL_MANIFEST, records)
    write_jsonl(DB / "symbols.jsonl", records)
    symbol_ids = {record["symbol_id"] for record in records}
    edges: list[dict[str, Any]] = []
    seen_edges: set[tuple[str, str, str]] = set()

    def add_edge(source: str, kind: str, target: str | None, mechanism: str, status: str) -> None:
        key = (source, kind, target or mechanism)
        if key in seen_edges:
            return
        seen_edges.add(key)
        edges.append({
            "edge_id": stable_id("edge", ":".join(key)),
            "source": source,
            "kind": kind,
            "target": target,
            "mechanism": mechanism,
            "status": status,
        })

    for item_id, item, _path, _begin, _end, _public in candidates:
        source = compiler_to_symbol[item_id]
        parent = parents.get(item_id)
        if parent in compiler_to_symbol:
            add_edge(compiler_to_symbol[parent], "CONTAINS", source, "rustdoc containment", "resolved")
        if rustdoc_kind(item) == "trait":
            for implementation in item.get("inner", {}).get("trait", {}).get("implementations", []):
                if implementation in compiler_to_symbol:
                    add_edge(source, "IMPLEMENTED_BY", compiler_to_symbol[implementation], "rustdoc trait implementation", "resolved")
        for reference in set(rustdoc_references(item.get("inner", {}))):
            if reference in compiler_to_symbol and compiler_to_symbol[reference] != source:
                add_edge(source, "REFERENCES", compiler_to_symbol[reference], "compiler type/signature reference", "resolved")
            elif str(reference) in index and index[str(reference)].get("crate_id") != 0:
                add_edge(source, "REFERENCES_EXTERNAL", None, f"rustdoc external item {reference}", "external")
    file_id_by_path = {record["path"]: record["file_id"] for record in files}
    for file_record in files:
        if not file_record.get("dossier"):
            continue
        dossier_path = ROOT / file_record["dossier"]
        dossier = read_json(dossier_path)
        if file_record["path"].endswith(".rs"):
            directory = PurePosixPath(file_record["path"]).parent
            for imported in dossier["imports"]:
                if not imported.startswith("module:"):
                    continue
                module_name = imported.split(":", 1)[1]
                candidates_for_module = [
                    (directory / f"{module_name}.rs").as_posix(),
                    (directory / module_name / "mod.rs").as_posix(),
                ]
                target_path = next((candidate for candidate in candidates_for_module if candidate in file_id_by_path), None)
                add_edge(
                    file_record["file_id"],
                    "IMPORTS",
                    file_id_by_path.get(target_path) if target_path else None,
                    imported,
                    "resolved" if target_path else "unresolved_explicit",
                )
        dossier["analysis_stage"] = "relationships_resolved"
        write_json(dossier_path, dossier)
        file_record["analysis_stage"] = "relationships_resolved"
    write_jsonl(EDGE_MANIFEST, sorted(edges, key=lambda record: record["edge_id"]))
    write_jsonl(REPOSITORY_MANIFEST, files)
    write_jsonl(DB / "inventory.jsonl", files)
    import gzip
    checkpoint_dir = DB / "checkpoints"
    checkpoint_dir.mkdir(parents=True, exist_ok=True)
    (checkpoint_dir / "rustdoc-private.json.gz").write_bytes(gzip.compress(private_bytes, compresslevel=9))
    (checkpoint_dir / "rustdoc-public.json.gz").write_bytes(gzip.compress(public_bytes, compresslevel=9))
    write_json(checkpoint_dir / "rustdoc-extraction.json", {
        "snapshot": snapshot,
        "format_version": private.get("format_version"),
        "crate_version": private.get("crate_version"),
        "private_json_sha256": sha256_bytes(private_bytes),
        "public_json_sha256": sha256_bytes(public_bytes),
        "private_records": len(private.get("index", {})),
        "public_records": len(public.get("index", {})),
        "owned_named_symbols": len(records),
        "intentionally_public_symbols": sum(record["public_api"] for record in records),
        "rule": "Compiler-owned named items with snapshot spans; public denominator joins public/private rustdoc by stable span+name+kind and requires lexical authorship at the compiler span.",
        "created_at": now(),
    })
    state["phase"] = "surface-extraction"
    state["last_command"] = "extract-symbols"
    refresh_state_counts(state)
    save_state(state)
    print_status(state)


def test_records_for_file(
    path: str,
    text: str,
    file_hash: str,
    snapshot: str,
    symbols_by_name: dict[str, list[dict[str, Any]]],
) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    if path.endswith(".rs"):
        for match in TEST_ITEM.finditer(text):
            if not EXECUTABLE_RUST_TEST_ATTRIBUTE.search(match.group("attrs")):
                continue
            end = brace_end(text, match.end())
            body = text[match.start():end]
            begin_line = line_number(text, match.start())
            end_line = line_number(text, end)
            linked: list[str] = []
            for token in call_tokens(body):
                name = token.rsplit("::", 1)[-1]
                choices = symbols_by_name.get(name, [])
                if len(choices) == 1:
                    linked.append(choices[0]["symbol_id"])
            records.append({
                "test_id": stable_id("test", f"{snapshot}:{path}:{match.group('name')}:{begin_line}"),
                "name": match.group("name"),
                "path": path,
                "source_hash": file_hash,
                "lines": [begin_line, end_line],
                "attributes": " ".join(match.group("attrs").split()),
                "body_hash": sha256_bytes(body.encode()),
                "behavior_under_test": match.group("name").replace("_", " "),
                "setup_preconditions": line_matches(body, (r"\b(?:let|given_|fixture|new\()",), 50),
                "inputs": [token for token in call_tokens(body) if any(word in token.lower() for word in ("new", "from", "build", "fixture", "source"))],
                "expected_outputs": line_matches(body, (r"\bassert(?:_eq|_ne|_matches)?!",), 50),
                "failure_expectation": line_matches(body, (r"\b(?:Err|is_err|unwrap_err|panic)\b",), 50),
                "platform": cfg_evidence(match.group("attrs"))[0] or ["not_explicitly_gated"],
                "feature_gate": cfg_evidence(match.group("attrs"))[1],
                "timing_assumption": line_matches(body, (r"\b(?:sleep|timeout|Duration|Instant)\b",), 50),
                "mocked_boundary": line_matches(body, (r"\b(?:Mock|Fake|Stub|fixture)\b",), 50),
                "real_boundary": line_matches(body, (r"\b(?:File|Socket|device|native|ffi|Command)\b",), 50),
                "production_symbols": sorted(set(linked)),
                "status": "analyzed",
                "documentation_status": "pending",
                "evidence": [{"path": path, "content_hash": file_hash, "lines": [begin_line, end_line], "classification": "TESTED"}],
            })
    elif path.startswith("tests/") and path.endswith((".c", ".cpp")) and re.search(r"\bmain\s*\(", text):
        records.append({
            "test_id": stable_id("test", f"{snapshot}:{path}:main"),
            "name": PurePosixPath(path).stem,
            "path": path,
            "source_hash": file_hash,
            "lines": [1, max(1, text.count("\n") + 1)],
            "attributes": "native conformance executable",
            "body_hash": sha256_bytes(text.encode()),
            "behavior_under_test": PurePosixPath(path).stem.replace("_", " "),
            "setup_preconditions": [], "inputs": [], "expected_outputs": line_matches(text, (r"\bassert\s*\(",), 100),
            "failure_expectation": [], "platform": ["native"], "feature_gate": [], "timing_assumption": [],
            "mocked_boundary": [], "real_boundary": [{"line": 1, "text": "C/C++ ABI boundary"}],
            "production_symbols": [], "status": "analyzed", "documentation_status": "pending",
            "evidence": [{"path": path, "content_hash": file_hash, "lines": [1, max(1, text.count("\n") + 1)], "classification": "TESTED"}],
        })
    elif path.startswith("scripts/test-") and path.endswith(".sh"):
        records.append({
            "test_id": stable_id("test", f"{snapshot}:{path}:script"), "name": PurePosixPath(path).stem,
            "path": path, "source_hash": file_hash, "lines": [1, max(1, text.count("\n") + 1)],
            "attributes": "repository test script", "body_hash": sha256_bytes(text.encode()),
            "behavior_under_test": PurePosixPath(path).stem.replace("-", " "), "setup_preconditions": [],
            "inputs": [], "expected_outputs": line_matches(text, (r"\b(?:diff|cmp|test|cargo)\b",), 100),
            "failure_expectation": line_matches(text, (r"\b(?:exit 1|fail|error)\b",), 100),
            "platform": ["shell environment"], "feature_gate": [], "timing_assumption": [],
            "mocked_boundary": [], "real_boundary": [], "production_symbols": [], "status": "analyzed",
            "documentation_status": "pending",
            "evidence": [{"path": path, "content_hash": file_hash, "lines": [1, max(1, text.count("\n") + 1)], "classification": "TESTED"}],
        })
    return records


def domain_for_path(path: str) -> str:
    parts = PurePosixPath(path).parts
    if path == "src/lib.rs":
        # The crate-root facade owns the public Session lifecycle. Treat its
        # lifecycle methods and dispositions as Session behavior, not as an
        # artificial domain named after the filename.
        return "session"
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
    if len(parts) > 1 and parts[0] == "docs":
        return "documentation"
    return parts[0].removesuffix(".rs") if parts else "repository"


def cmd_extract_surfaces(_arguments: argparse.Namespace) -> None:
    state = load_state()
    files = read_jsonl(REPOSITORY_MANIFEST)
    symbols = read_jsonl(SYMBOL_MANIFEST)
    if not symbols:
        raise CompilerError("Gate 2 has not run: symbol manifest is empty")
    by_symbol_id = {record["symbol_id"]: record for record in symbols}
    symbols_by_name: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for symbol in symbols:
        symbol["tests"] = []
        symbol["examples"] = []
        symbols_by_name[symbol["name"]].append(symbol)
    tests: list[dict[str, Any]] = []
    examples: list[dict[str, Any]] = []
    env_records: dict[str, dict[str, Any]] = {}
    for file_record in files:
        if not file_record.get("semantic"):
            continue
        text = git_bytes(state["snapshot"], file_record["path"]).decode("utf-8", errors="replace")
        tests.extend(test_records_for_file(
            file_record["path"], text, file_record["sha256"], state["snapshot"], symbols_by_name
        ))
        if file_record["path"].startswith("examples/") and file_record["path"].endswith(".rs"):
            calls_in_example = call_tokens(text)
            linked: list[str] = []
            for token in calls_in_example:
                choices = symbols_by_name.get(token.rsplit("::", 1)[-1], [])
                if len(choices) == 1 and choices[0]["public_api"]:
                    linked.append(choices[0]["symbol_id"])
            examples.append({
                "example_id": stable_id("example", f"{state['snapshot']}:{file_record['path']}"),
                "path": file_record["path"],
                "source_hash": file_record["sha256"],
                "task": PurePosixPath(file_record["path"]).stem.replace("_", " ").replace("-", " "),
                "public_apis": sorted(set(linked)),
                "prerequisites": line_matches(text, (r"(?i)\b(?:require|prerequisite|feature|platform)\b",), 50),
                "platform_requirements": cfg_evidence(text)[0] or ["not_explicitly_gated"],
                "feature_requirements": cfg_evidence(text)[1],
                "failure_handling": line_matches(text, (r"\b(?:Result|Err|map_err|expect\(|unwrap\()",), 100),
                "preferred_api_status": "current_snapshot_public_api",
                "status": "analyzed",
                "verification_status": "pending",
                "verification_checkpoint": None,
                "evidence": [{
                    "path": file_record["path"], "content_hash": file_record["sha256"],
                    "lines": [1, max(1, text.count("\n") + 1)], "classification": "DECLARED",
                }],
            })
        for match in ENV_VAR.finditer(text):
            variable = match.group(1)
            env_records.setdefault(variable, {
                "config_id": stable_id("cfg", f"env:{variable}"), "kind": "environment_variable", "name": variable,
                "parent": None, "source_file": file_record["path"], "source_hash": file_record["sha256"],
                "source_lines": [line_number(text, match.start()), line_number(text, match.end())],
                "value_type": "string_or_os_string", "default": "unknown", "required": "unknown",
                "valid_values": "unknown", "units": "not_applicable", "minimum": "unknown", "maximum": "unknown",
                "scope": "process", "when_read": "See source evidence.", "mutable": "unknown",
                "restart_required": "unknown", "security_implications": "unknown", "platforms": cfg_evidence(text)[0],
                "precedence": "unknown", "invalid_value_behavior": "unknown", "status": "analyzed",
                "documentation_status": "pending",
                "evidence": [{"path": file_record["path"], "content_hash": file_record["sha256"],
                              "lines": [line_number(text, match.start()), line_number(text, match.end())], "classification": "DIRECT"}],
            })
    test_by_id = {record["test_id"]: record for record in tests}
    for test in tests:
        for symbol_id in test["production_symbols"]:
            if symbol_id in by_symbol_id:
                by_symbol_id[symbol_id]["tests"].append(test["test_id"])
    for example in examples:
        for symbol_id in example["public_apis"]:
            if symbol_id in by_symbol_id:
                by_symbol_id[symbol_id]["examples"].append(example["example_id"])
    error_types = {
        symbol["symbol_id"]: symbol for symbol in symbols
        if symbol["public_api"] and symbol["kind"] in {"enum", "struct", "type_alias"}
        and any(word in symbol["name"].lower() for word in ("error", "failure"))
    }
    errors: list[dict[str, Any]] = []
    for symbol in symbols:
        parent = symbol.get("parent")
        is_error_type = symbol["symbol_id"] in error_types
        is_variant = symbol["kind"] == "variant" and parent in error_types
        if not (is_error_type or is_variant):
            continue
        error_type = symbol if is_error_type else error_types[parent]
        errors.append({
            "error_id": stable_id("error", symbol["symbol_id"]),
            "symbol_id": symbol["symbol_id"], "type": error_type["qualified_name"],
            "variant": symbol["name"] if is_variant else None,
            "defined_at": {"path": symbol["source_file"], "lines": symbol["source_lines"]},
            "created_at": [], "trigger_condition": "unknown", "propagates_through": [], "wrapped_by": [],
            "translated_to": [], "external_representation": "unknown", "retryable": "unknown",
            "retry_evidence": [], "recoverable": "unknown", "recovery_action": "unknown",
            "fatal_to_operation": "unknown", "fatal_to_session": "unknown", "fatal_to_process": "unknown",
            "logged": "unknown", "metric": "unknown", "event": "unknown", "status_code": "not_applicable",
            "exit_code": "not_applicable", "user_action": "unknown", "developer_action": "unknown",
            "tests": symbol["tests"], "status": "analyzed", "documentation_status": "pending",
            "evidence": symbol["evidence"],
        })
    config: list[dict[str, Any]] = list(env_records.values())
    config_parent_words = ("config", "configuration", "options", "policy", "selector", "settings")
    for symbol in symbols:
        if not symbol["public_api"]:
            continue
        parent = by_symbol_id.get(symbol.get("parent"), {})
        parent_name = parent.get("name", "").lower()
        qualifies = symbol["kind"] in {"struct_field", "variant"} and any(
            word in parent_name for word in config_parent_words
        )
        if not qualifies:
            continue
        config.append({
            "config_id": stable_id("cfg", symbol["symbol_id"]), "kind": symbol["kind"], "name": symbol["name"],
            "parent": parent.get("qualified_name"), "symbol_id": symbol["symbol_id"],
            "source_file": symbol["source_file"], "source_hash": symbol["source_file_sha256"],
            "source_lines": symbol["source_lines"], "value_type": symbol["signature"], "default": "unknown",
            "required": "unknown", "valid_values": "See compiler signature and parent type.", "units": "unknown",
            "minimum": "unknown", "maximum": "unknown", "scope": domain_for_path(symbol["source_file"]),
            "when_read": "unknown", "mutable": symbol["mutability"], "restart_required": "unknown",
            "security_implications": "unknown", "platforms": symbol["platforms"], "precedence": "unknown",
            "invalid_value_behavior": "unknown", "status": "analyzed", "documentation_status": "pending",
            "evidence": symbol["evidence"],
        })
    cargo_text = git_bytes(state["snapshot"], "Cargo.toml").decode()
    cargo = tomllib.loads(cargo_text)
    cargo_record = next(record for record in files if record["path"] == "Cargo.toml")
    for feature, members in cargo.get("features", {}).items():
        config.append({
            "config_id": stable_id("cfg", f"cargo-feature:{feature}"), "kind": "cargo_feature", "name": feature,
            "parent": "Cargo.toml [features]", "source_file": "Cargo.toml", "source_hash": cargo_record["sha256"],
            "source_lines": [1, max(1, cargo_text.count("\n") + 1)], "value_type": "compile_time_feature",
            "default": feature in cargo.get("features", {}).get("default", []), "required": False,
            "valid_values": members, "units": "not_applicable", "minimum": "not_applicable", "maximum": "not_applicable",
            "scope": "crate compilation", "when_read": "Cargo feature resolution", "mutable": False,
            "restart_required": "rebuild_required", "security_implications": "unknown", "platforms": [],
            "precedence": "Cargo feature unification", "invalid_value_behavior": "Cargo rejects unknown features",
            "status": "analyzed", "documentation_status": "pending",
            "evidence": [{"path": "Cargo.toml", "content_hash": cargo_record["sha256"],
                          "lines": [1, max(1, cargo_text.count("\n") + 1)], "classification": "DIRECT"}],
        })
    lifecycle_verbs = re.compile(r"^(?:prepare|prepared|start|started|run|running|cancel|cancelled|stop|stopped|drain|drained|finalize|finalized|shutdown|abort|join|close|drop)(?:_|$)", re.I)
    lifecycles: list[dict[str, Any]] = []
    for symbol in symbols:
        if symbol["public_api"] and symbol["kind"] in {"function", "variant"} and lifecycle_verbs.search(symbol["name"]):
            lifecycles.append({
                "lifecycle_id": stable_id("life", symbol["symbol_id"]), "operation": symbol["qualified_name"],
                "source_state": "unknown", "trigger": symbol["name"], "guard": "unknown", "action": symbol["summary"],
                "side_effects": symbol["side_effects"], "destination_state": "unknown", "possible_error": "unknown",
                "recovery": "unknown", "idempotence": "unknown", "observable_signal": "unknown",
                "symbol_id": symbol["symbol_id"], "status": "analyzed", "documentation_status": "pending",
                "evidence": symbol["evidence"],
            })
    behaviors: list[dict[str, Any]] = []
    for test in tests:
        behaviors.append({
            "behavior_id": stable_id("behavior", test["test_id"]), "name": test["behavior_under_test"],
            "domain": domain_for_path(test["path"]), "classification": "TESTED", "entry_points": test["production_symbols"],
            "steps": [{"operation": call} for call in call_tokens(git_bytes(state["snapshot"], test["path"]).decode("utf-8", errors="replace"))[:100]],
            "errors": test["failure_expectation"], "tests": [test["test_id"]], "status": "analyzed",
            "documentation_status": "pending", "evidence": test["evidence"],
        })
    for lifecycle in lifecycles:
        behaviors.append({
            "behavior_id": stable_id("behavior", lifecycle["lifecycle_id"]), "name": lifecycle["operation"],
            "domain": domain_for_path(lifecycle["evidence"][0]["path"]), "classification": "DIRECT",
            "entry_points": [lifecycle["symbol_id"]], "steps": [{"operation": lifecycle["trigger"]}],
            "errors": [lifecycle["possible_error"]], "tests": [], "status": "analyzed",
            "documentation_status": "pending", "evidence": lifecycle["evidence"],
        })
    protocols: list[dict[str, Any]] = []
    for symbol in symbols:
        if not symbol["public_api"]:
            continue
        path = symbol["source_file"]
        if path.startswith(("src/abi/", "include/", "src/connector/")) or symbol["kind"] == "macro":
            protocols.append({
                "protocol_id": stable_id("protocol", symbol["symbol_id"]), "name": symbol["qualified_name"],
                "kind": "ffi" if path.startswith(("src/abi/", "include/")) else "connector_or_extension_contract",
                "symbol_id": symbol["symbol_id"], "status": "analyzed", "documentation_status": "pending",
                "evidence": symbol["evidence"],
            })
    patterns: list[dict[str, Any]] = []
    pattern_terms = {
        "bounded_queue": ("bounded", "queue", "capacity"), "buffer_pool": ("pool", "buffer"),
        "transactional_registration": ("rollback", "transaction"), "sidecar_isolation": ("sidecar",),
        "clock_correlation": ("clock", "timestamp"), "typed_error": ("Error", "Result"),
    }
    for file_record in files:
        if not file_record.get("dossier"):
            continue
        dossier = read_json(ROOT / file_record["dossier"])
        content = git_bytes(state["snapshot"], file_record["path"]).decode(
            "utf-8", errors="replace"
        )
        observed = []
        for pattern_name, tokens in pattern_terms.items():
            if all(token.lower() in content.lower() for token in tokens):
                record = {
                    "pattern_id": stable_id("pattern", f"{file_record['path']}:{pattern_name}"),
                    "name": pattern_name, "classification": "OBSERVED_IMPLEMENTATION_PATTERN",
                    "problem": "unknown", "where_implemented": file_record["path"], "how_implemented": "See dossier evidence.",
                    "constraints": "unknown", "tradeoffs": "unknown", "failure_behavior": "unknown",
                    "tests": [], "alternatives": [], "evidence": dossier["evidence"],
                }
                patterns.append(record)
                observed.append(record["pattern_id"])
        dossier["observed_patterns"] = observed
        dossier["analysis_stage"] = "behavior_validated"
        write_json(ROOT / file_record["dossier"], dossier)
        file_record["analysis_stage"] = "behavior_validated"
    conflicts: list[dict[str, Any]] = []
    unknowns: list[dict[str, Any]] = [
        {
            "unknown_id": "unknown-platform-qualification", "scope": "platform support",
            "question": "Which implemented platform paths have been physically qualified for this release?",
            "reason": "Implementation and CI presence do not by themselves prove physical qualification.",
            "status": "explicit", "evidence": [],
        },
        {
            "unknown_id": "unknown-public-thread-safety", "scope": "public API",
            "question": "Which public operations have thread-safety guarantees beyond compiler-enforced Send/Sync properties?",
            "reason": "The compiler graph does not establish operational concurrency guarantees.",
            "status": "explicit", "evidence": [],
        },
        {
            "unknown_id": "unknown-retry-policy", "scope": "errors",
            "question": "Which public failures are safe to retry when no explicit retry contract exists?",
            "reason": "Retry safety cannot be inferred from error names.",
            "status": "explicit", "evidence": [],
        },
    ]
    files_by_path = {record["path"]: record for record in files}
    fixture_record = files_by_path.get("scripts/fixtures/connector-v1-vectors.json")
    publish_record = files_by_path.get(".github/workflows/publish.yml")
    publish_materializes_fixture = False
    if publish_record:
        publish_text = git_bytes(state["snapshot"], publish_record["path"]).decode()
        publish_materializes_fixture = (
            "scripts/fixtures/connector-v1-vectors.json" in publish_text
            and "../protocol/conformance/connector/v1/vectors.json" in publish_text
        )
    for file_record in files:
        if file_record["path"] == "tests/connector_portable_semantics.rs":
            text = git_bytes(state["snapshot"], file_record["path"]).decode()
            if (
                "protocol/conformance/connector/v1/vectors.json" in text
                and not (fixture_record and publish_materializes_fixture)
            ):
                conflicts.append({
                    "conflict_id": "conflict-external-connector-vectors", "kind": "external_prerequisite",
                    "summary": "Connector portable-semantics tests require a sibling vector path without a repository-owned materialization source.",
                    "status": "explicit_unresolved", "sides": ["repository test", "external sibling artifact"],
                    "evidence": [{"path": file_record["path"], "content_hash": file_record["sha256"],
                                  "lines": [1, max(1, text.count("\n") + 1)], "classification": "DIRECT"}],
                })
    write_jsonl(DB / "tests.jsonl", sorted(tests, key=lambda record: record["test_id"]))
    write_jsonl(DB / "examples.jsonl", sorted(examples, key=lambda record: record["example_id"]))
    write_jsonl(DB / "errors.jsonl", sorted(errors, key=lambda record: record["error_id"]))
    write_jsonl(DB / "configuration.jsonl", sorted(config, key=lambda record: record["config_id"]))
    write_jsonl(DB / "lifecycles.jsonl", sorted(lifecycles, key=lambda record: record["lifecycle_id"]))
    write_jsonl(DB / "behaviors.jsonl", sorted(behaviors, key=lambda record: record["behavior_id"]))
    write_jsonl(DB / "protocols.jsonl", sorted(protocols, key=lambda record: record["protocol_id"]))
    write_jsonl(DB / "patterns.jsonl", sorted(patterns, key=lambda record: record["pattern_id"]))
    write_jsonl(DB / "conflicts.jsonl", conflicts)
    write_jsonl(DB / "unknowns.jsonl", unknowns)
    write_jsonl(SYMBOL_MANIFEST, list(by_symbol_id.values()))
    write_jsonl(DB / "symbols.jsonl", list(by_symbol_id.values()))
    write_jsonl(REPOSITORY_MANIFEST, files)
    write_jsonl(DB / "inventory.jsonl", files)
    edges = [
        edge for edge in read_jsonl(EDGE_MANIFEST)
        if edge.get("kind") not in {"TESTED_BY", "EXEMPLIFIED_BY"}
    ]
    existing = {(edge["source"], edge["kind"], edge.get("target")) for edge in edges}
    for test in tests:
        for symbol_id in test["production_symbols"]:
            key = (symbol_id, "TESTED_BY", test["test_id"])
            if key not in existing:
                edges.append({"edge_id": stable_id("edge", ":".join(key)), "source": symbol_id,
                              "kind": "TESTED_BY", "target": test["test_id"],
                              "mechanism": "test body call resolved to a unique compiler symbol name", "status": "resolved"})
    for example in examples:
        for symbol_id in example["public_apis"]:
            key = (symbol_id, "EXEMPLIFIED_BY", example["example_id"])
            if key not in existing:
                edges.append({"edge_id": stable_id("edge", ":".join(key)), "source": symbol_id,
                              "kind": "EXEMPLIFIED_BY", "target": example["example_id"],
                              "mechanism": "example call resolved to a unique public compiler symbol name", "status": "resolved"})
    write_jsonl(EDGE_MANIFEST, sorted(edges, key=lambda record: record["edge_id"]))
    state["phase"] = "capability-model"
    state["last_command"] = "extract-surfaces"
    refresh_state_counts(state)
    save_state(state)
    print_status(state)


def validate_evidence(evidence: Any, file_by_path: dict[str, dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    if not isinstance(evidence, list) or not evidence:
        return ["evidence list is absent or empty"]
    for number, item in enumerate(evidence):
        if not isinstance(item, dict):
            failures.append(f"evidence[{number}] is not an object")
            continue
        path = item.get("path") or item.get("file")
        if path not in file_by_path:
            failures.append(f"evidence[{number}] path is outside the snapshot: {path!r}")
            continue
        expected_hash = file_by_path[path]["sha256"]
        actual_hash = item.get("content_hash")
        if actual_hash != expected_hash:
            failures.append(f"evidence[{number}] hash differs for {path}")
        lines = item.get("lines")
        if not (
            isinstance(lines, list) and len(lines) == 2 and all(isinstance(value, int) for value in lines)
            and 1 <= lines[0] <= lines[1]
        ):
            failures.append(f"evidence[{number}] has invalid line range for {path}")
        if item.get("classification") not in {"DIRECT", "TESTED", "DECLARED", "INFERRED", "UNKNOWN", "CONFLICTED"}:
            failures.append(f"evidence[{number}] has invalid classification for {path}")
    return failures


def model_records(path: Path, id_field: str, required: set[str]) -> list[dict[str, Any]]:
    records = read_jsonl(path)
    identifiers: set[str] = set()
    for number, record in enumerate(records, 1):
        missing = sorted(required - record.keys())
        if missing:
            raise CompilerError(f"{path}:{number}: missing fields: {', '.join(missing)}")
        identifier = record[id_field]
        if identifier in identifiers:
            raise CompilerError(f"{path}:{number}: duplicate {id_field}: {identifier}")
        identifiers.add(identifier)
    return records


def cmd_review_model(_arguments: argparse.Namespace) -> None:
    state = load_state()
    if state.get("phase") not in {
        "capability-model", "information-architecture", "concept-documentation", "how-to-documentation",
        "api-reference", "failure-documentation", "documentation", "example-validation", "validation",
    }:
        raise CompilerError(f"model review is not available during phase {state.get('phase')!r}")
    files = read_jsonl(REPOSITORY_MANIFEST)
    file_by_path = {record["path"]: record for record in files}
    capabilities = model_records(
        DB / "capabilities.jsonl", "capability_id",
        {"capability_id", "name", "description", "domains", "evidence", "status", "guide_required", "troubleshooting_applicable"},
    )
    journeys = model_records(
        DB / "user-journeys.jsonl", "journey_id",
        {"journey_id", "name", "audience", "outcome", "capability_ids", "steps", "evidence", "status"},
    )
    pages = model_records(
        PAGE_MANIFEST, "page_id",
        {"page_id", "title", "path", "doc_class", "gate", "domains", "capability_ids", "evidence", "status"},
    )
    capability_ids = {record["capability_id"] for record in capabilities}
    failures: list[str] = []
    for capability in capabilities:
        failures.extend(f"{capability['capability_id']}: {failure}" for failure in validate_evidence(capability["evidence"], file_by_path))
        if capability["status"] != "analyzed":
            failures.append(f"{capability['capability_id']}: status is not analyzed")
    for journey in journeys:
        failures.extend(f"{journey['journey_id']}: {failure}" for failure in validate_evidence(journey["evidence"], file_by_path))
        unknown = sorted(set(journey["capability_ids"]) - capability_ids)
        if unknown:
            failures.append(f"{journey['journey_id']}: unknown capabilities: {unknown}")
        if journey["status"] != "analyzed":
            failures.append(f"{journey['journey_id']}: status is not analyzed")
    allowed_classes = {
        "overview", "getting-started", "concept", "how-to", "lifecycle", "best-practice",
        "troubleshooting", "platform", "reference", "error-reference", "config-reference",
        "protocol-reference", "compatibility", "security", "internals", "glossary", "release",
    }
    paths: set[str] = set()
    for page in pages:
        if page["doc_class"] not in allowed_classes:
            failures.append(f"{page['page_id']}: invalid doc_class {page['doc_class']!r}")
        if not isinstance(page["gate"], int) or not 7 <= page["gate"] <= 10:
            failures.append(f"{page['page_id']}: gate must be 7 through 10")
        if not (page["path"].endswith(".md") and (page["path"].startswith("docs/") or page["path"] in {"README.md", "RELEASE_NOTES.md"})):
            failures.append(f"{page['page_id']}: page path must be site Markdown, README.md, or RELEASE_NOTES.md")
        if page["path"] in paths:
            failures.append(f"{page['page_id']}: duplicate page path {page['path']}")
        paths.add(page["path"])
        unknown = sorted(set(page["capability_ids"]) - capability_ids)
        if unknown:
            failures.append(f"{page['page_id']}: unknown capabilities: {unknown}")
        failures.extend(f"{page['page_id']}: {failure}" for failure in validate_evidence(page["evidence"], file_by_path))
    terminology = read_json(DB / "terminology.json")
    if terminology.get("status") != "reviewed" or not terminology.get("terms"):
        failures.append("terminology registry is not reviewed or contains no terms")
    term_names: set[str] = set()
    for term in terminology.get("terms", []):
        required = {"canonical_name", "code_spelling", "human_spelling", "definition", "aliases", "forbidden_aliases", "first_defining_symbol", "related_concepts"}
        missing = sorted(required - term.keys())
        if missing:
            failures.append(f"terminology {term.get('canonical_name', '<unnamed>')}: missing {missing}")
        if term.get("canonical_name") in term_names:
            failures.append(f"duplicate terminology name: {term.get('canonical_name')}")
        term_names.add(term.get("canonical_name"))
    if failures:
        raise CompilerError("model review failed:\n- " + "\n- ".join(failures))
    for file_record in files:
        if not file_record.get("dossier"):
            continue
        dossier_path = ROOT / file_record["dossier"]
        dossier = read_json(dossier_path)
        dossier["analysis_stage"] = "doc_ready"
        dossier["related_docs"] = [page["path"] for page in pages if domain_for_path(file_record["path"]) in page["domains"] or "all" in page["domains"]]
        write_json(dossier_path, dossier)
        file_record["analysis_stage"] = "doc_ready"
    write_jsonl(REPOSITORY_MANIFEST, files)
    write_jsonl(DB / "inventory.jsonl", files)
    state["phase"] = "information-architecture"
    state["last_command"] = "review-model"
    refresh_state_counts(state)
    save_state(state)
    print_status(state)


def page_claims(page_id: str, claims: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [claim for claim in claims if claim.get("documentation_page") == page_id]


def native_documentation_by_symbol() -> dict[str, str]:
    path = DB / "native-docs.jsonl"
    if not path.exists():
        return {}
    return {
        record["symbol_id"]: str(record.get("documentation", "")).strip()
        for record in read_jsonl(path)
    }


def native_documentation_failures(symbols: list[dict[str, Any]]) -> list[str]:
    """Reject symbol-count coverage that has no meaningful description behind it."""
    generated = native_documentation_by_symbol()
    failures: list[str] = []
    for symbol in symbols:
        if not symbol.get("public_api"):
            continue
        description = generated.get(symbol["symbol_id"], "")
        normalized = re.sub(r"\s+", " ", description).strip()
        if not normalized or normalized.lower() in {"unknown", "not_applicable", "not applicable"}:
            failures.append(f"{symbol['symbol_id']}: public symbol has no native description")
            continue
        if len(re.findall(r"[A-Za-z0-9]+", normalized)) < 4:
            failures.append(f"{symbol['symbol_id']}: native description is too small to state a purpose")
        if any(snippet.lower() in normalized.lower() for snippet in PUBLICATION_BOILERPLATE):
            failures.append(f"{symbol['symbol_id']}: native description contains publication boilerplate")
    rejected_source_patterns = (
        "Generated native API description",
        "Performs the ",
        " value exposed by the PocketStation API",
        " in the PocketStation API",
        "Defines the implementation contract for",
        "Stops and join",
        "Carries the typed state and values defined by",
        "Executes the `",
    )
    for source in sorted((ROOT / "src").rglob("*.rs")):
        text = source.read_text()
        for pattern in rejected_source_patterns:
            if pattern in text:
                failures.append(
                    f"{source.relative_to(ROOT)}: current source documentation contains rejected wording {pattern!r}"
                )
    return failures


def claim_marker_failures(pages: list[dict[str, Any]], claims: list[dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    claims_by_page: dict[str, set[str]] = defaultdict(set)
    for claim in claims:
        claims_by_page[str(claim.get("documentation_page"))].add(str(claim.get("claim_id")))
    for page in pages:
        path = ROOT / page["path"]
        if not path.exists():
            failures.append(f"{page['page_id']}: page absent while validating claim mappings")
            continue
        text = path.read_text()
        declared: set[str] = set()
        for value in re.findall(r"<!--\s*claims:\s*([^>]+?)\s*-->", text):
            declared.update(token.strip() for token in value.split(",") if token.strip())
        expected = claims_by_page.get(page["page_id"], set())
        if declared != expected:
            failures.append(
                f"{page['page_id']}: page-to-claim mapping differs "
                f"(declared {len(declared)}, ledger {len(expected)})"
            )
    manifest_paths = {page["path"] for page in pages}
    for claim in claims:
        if claim.get("documentation_path") not in manifest_paths:
            failures.append(f"{claim.get('claim_id')}: claim points outside the page manifest")
    return failures


def editorial_page_failures(pages: list[dict[str, Any]]) -> list[str]:
    """Apply deterministic page-shape and anti-template checks.

    This does not pretend to replace an editor. It does make the previously
    accepted failure modes—generic page shells and copied filler—non-publishable.
    """
    failures: list[str] = []
    paragraphs: dict[str, list[str]] = defaultdict(list)
    for page in pages:
        path = ROOT / page["path"]
        if not path.exists():
            failures.append(f"{page['page_id']}: page absent")
            continue
        text = path.read_text()
        headings = {
            match.group(1).strip()
            for match in re.finditer(r"(?m)^##\s+(.+?)\s*$", text)
        }
        required = set(REQUIRED_PAGE_SECTIONS.get(page.get("doc_class"), set()))
        if page.get("path") == "docs/getting-started/rust-quickstart.md":
            required.update({
                "Audience", "Prerequisites", "Supported environment", "Install",
                "Program", "Run it", "Success", "Common first-run failures", "Next steps",
            })
        missing = sorted(required - headings)
        if missing:
            failures.append(f"{page['page_id']}: required sections absent: {missing}")
        for snippet in PUBLICATION_BOILERPLATE:
            if snippet.lower() in text.lower():
                failures.append(f"{page['page_id']}: prohibited generic publication text: {snippet[:72]}")
        for raw in re.split(r"\n\s*\n", text):
            paragraph = re.sub(r"\s+", " ", raw).strip()
            if (
                len(paragraph.split()) < 18
                or paragraph.startswith(("#", "|", "- ", "<!--", "```"))
                or re.match(r"^\d+\.\s", paragraph)
            ):
                continue
            paragraphs[paragraph].append(page["page_id"])
    for paragraph, page_ids in paragraphs.items():
        if len(set(page_ids)) > 3:
            failures.append(
                f"repeated prose appears in {len(set(page_ids))} pages: "
                f"{paragraph[:120]}"
            )
    return failures


def dossier_semantic_failures(files: list[dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    for record in files:
        if not record.get("semantic") or not record.get("dossier"):
            continue
        dossier = read_json(ROOT / record["dossier"])
        purpose = dossier.get("purpose", {})
        purpose_text = purpose.get("text", "") if isinstance(purpose, dict) else str(purpose)
        if (
            len(purpose_text.split()) < 4
            or "not_applicable" in purpose_text
            or "Provides repository-owned content at" in purpose_text
        ):
            failures.append(f"{record['path']}: dossier purpose is a filename fallback")
        non_responsibilities = dossier.get("non_responsibilities")
        if not isinstance(non_responsibilities, list) or not non_responsibilities:
            failures.append(f"{record['path']}: dossier has no explicit non-responsibility")
        elif any(str(item).strip().lower() == "unknown" for item in non_responsibilities):
            failures.append(f"{record['path']}: dossier leaves non-responsibility as unknown")
        for field in ("inputs", "outputs"):
            values = dossier.get(field)
            if not isinstance(values, list) or not values:
                failures.append(f"{record['path']}: dossier {field} are absent")
            elif any("See compiler-backed symbol signatures" in str(value) for value in values):
                failures.append(f"{record['path']}: dossier {field} retain a schema placeholder")
        if not dossier.get("related_docs"):
            failures.append(f"{record['path']}: dossier has no reverse documentation mapping")
        if dossier.get("test_coverage_status") not in {"linked", "no_direct_test_link_extracted"}:
            failures.append(f"{record['path']}: dossier test coverage disposition is absent")
        if dossier.get("example_coverage_status") not in {"linked", "no_direct_example_link_extracted"}:
            failures.append(f"{record['path']}: dossier example coverage disposition is absent")
        if dossier.get("path") != record["path"] or dossier.get("file_id") != record["file_id"]:
            failures.append(f"{record['path']}: dossier identity differs from manifest")
    return failures


def command_checkpoint(name: str, commands: list[list[str]]) -> dict[str, Any]:
    results: list[dict[str, Any]] = []
    passed = True
    for command in commands:
        outcome = run(*command, check=False)
        result = {
            "command": command,
            "returncode": outcome.returncode,
            "stdout": outcome.stdout[-20000:],
            "stderr": outcome.stderr[-20000:],
        }
        results.append(result)
        passed = passed and outcome.returncode == 0
    checkpoint = {"name": name, "passed": passed, "commands": results, "created_at": now()}
    write_json(DB / "checkpoints" / f"{name}.json", checkpoint)
    return checkpoint


def cmd_sync_pages(_arguments: argparse.Namespace) -> None:
    state = load_state()
    if not state.get("docs_generation_allowed"):
        raise CompilerError("public documentation generation is forbidden until verifier Gates 0-6 pass")
    pages = read_jsonl(PAGE_MANIFEST)
    claims = read_jsonl(DB / "claims.jsonl")
    files = read_jsonl(REPOSITORY_MANIFEST)
    file_by_path = {record["path"]: record for record in files}
    claim_ids: set[str] = set()
    failures: list[str] = []
    for claim in claims:
        claim_id = claim.get("claim_id")
        if not claim_id or claim_id in claim_ids:
            failures.append(f"missing or duplicate claim ID: {claim_id!r}")
        claim_ids.add(claim_id)
        failures.extend(f"{claim_id}: {failure}" for failure in validate_evidence(claim.get("evidence"), file_by_path))
        if claim.get("status") not in {"verified", "conflicted", "unknown_explicit"}:
            failures.append(f"{claim_id}: claim status is not publishable")
    for page in pages:
        path = ROOT / page["path"]
        if not path.exists():
            page["status"] = "pending"
            continue
        text = path.read_text()
        headings = re.findall(r"(?m)^#{1,6}\s+(.+)$", text)
        page_failures: list[str] = []
        if not headings or headings[0].strip() != page["title"]:
            page_failures.append("first heading does not equal the ledger title")
        if len(headings) < 2:
            page_failures.append("page contains fewer than two headings")
        if PLACEHOLDER.search(text):
            page_failures.append("page contains a placeholder marker")
        if len(text.split()) < 80:
            page_failures.append("page is too small to establish its declared responsibility")
        owned_claims = page_claims(page["page_id"], claims)
        if not owned_claims:
            page_failures.append("page has no provenance claims")
        declared_ids = set(re.findall(r"<!--\s*claims:\s*([^>]+?)\s*-->", text))
        flattened: set[str] = set()
        for value in declared_ids:
            flattened.update(token.strip() for token in value.split(",") if token.strip())
        expected = {claim["claim_id"] for claim in owned_claims}
        if flattened != expected:
            page_failures.append(f"hidden claim marker differs: expected {sorted(expected)}, found {sorted(flattened)}")
        if page_failures:
            page["status"] = "in_progress"
            failures.extend(f"{page['page_id']}: {failure}" for failure in page_failures)
        else:
            page["status"] = "authored"
            page["content_sha256"] = sha256_file(path)
            page["word_count"] = len(text.split())
    failures.extend(editorial_page_failures(pages))
    failures.extend(claim_marker_failures(pages, claims))
    write_jsonl(PAGE_MANIFEST, pages)
    authored_pages = [page for page in pages if page.get("status") == "authored"]
    surface_rules = {
        "errors.jsonl": {"error-reference", "troubleshooting"},
        "configuration.jsonl": {"config-reference", "reference"},
        "behaviors.jsonl": {"concept", "how-to", "lifecycle", "internals", "reference"},
        "lifecycles.jsonl": {"lifecycle", "concept", "how-to", "reference"},
        "protocols.jsonl": {"protocol-reference", "reference", "internals"},
        "tests.jsonl": {"concept", "how-to", "internals", "troubleshooting", "reference"},
    }
    doc_map: list[dict[str, Any]] = []
    for ledger_name, classes in surface_rules.items():
        ledger_path = DB / ledger_name
        records = read_jsonl(ledger_path)
        id_field = next((field for field in ("error_id", "config_id", "behavior_id", "lifecycle_id", "protocol_id", "test_id") if records and field in records[0]), None)
        for record in records:
            evidence_path = (record.get("evidence") or [{}])[0].get("path", "")
            domain = record.get("domain") or domain_for_path(evidence_path)
            matching = []
            for page in authored_pages:
                if page["doc_class"] not in classes or not (domain in page["domains"] or "all" in page["domains"]):
                    continue
                identifier = record.get(id_field) if id_field else None
                if identifier and identifier in (ROOT / page["path"]).read_text():
                    matching.append(page["path"])
            record["documentation_status"] = "covered" if matching else "pending"
            if id_field:
                for page_path in matching:
                    doc_map.append({"entity_id": record[id_field], "entity_type": ledger_name.removesuffix(".jsonl"), "documentation_page": page_path})
        write_jsonl(ledger_path, records)
    for capability in read_jsonl(DB / "capabilities.jsonl"):
        for page in authored_pages:
            if capability["capability_id"] in page["capability_ids"]:
                doc_map.append({"entity_id": capability["capability_id"], "entity_type": "capability", "documentation_page": page["path"]})
    native_reference = next(
        (page for page in authored_pages if page["doc_class"] == "reference" and "rust-api" in page["path"]),
        None,
    )
    if native_reference:
        for symbol in read_jsonl(SYMBOL_MANIFEST):
            if symbol.get("public_api") and symbol.get("reference_status") == "covered":
                doc_map.append({
                    "entity_id": symbol["symbol_id"],
                    "entity_type": "symbol",
                    "documentation_page": native_reference["path"],
                })
    write_jsonl(DB / "doc-map.jsonl", sorted(doc_map, key=lambda record: (record["entity_type"], record["entity_id"], record["documentation_page"])))
    state["phase"] = "documentation"
    state["last_command"] = "sync-pages"
    refresh_state_counts(state)
    save_state(state)
    if failures:
        print("PAGE SYNC FINDINGS")
        for failure in failures:
            print(f"- {failure}")
    print_status(state)


def current_source_digest() -> str:
    paths = sorted(
        path for path in git("ls-files").splitlines()
        if path == "Cargo.toml" or path == "build.rs" or path.startswith(("src/", "include/", "native/"))
    )
    payload = bytearray()
    for path in paths:
        payload.extend(path.encode())
        payload.extend(b"\0")
        payload.extend((ROOT / path).read_bytes())
        payload.extend(b"\0")
    return sha256_bytes(bytes(payload))


def current_docs_digest(pages: list[dict[str, Any]]) -> str:
    payload = bytearray()
    for page in sorted(pages, key=lambda record: record["path"]):
        path = ROOT / page["path"]
        payload.extend(page["path"].encode())
        payload.extend(b"\0")
        if path.exists():
            payload.extend(path.read_bytes())
        payload.extend(b"\0")
    return sha256_bytes(bytes(payload))


def cmd_record_rustdoc(arguments: argparse.Namespace) -> None:
    state = load_state()
    if not state.get("docs_generation_allowed"):
        raise CompilerError("rustdoc coverage may only be recorded after verifier Gates 0-6 pass")
    commands = [
        ["cargo", "rustdoc", "--lib", "--no-default-features", "--", "-Z", "unstable-options", "--show-coverage"],
        ["cargo", "rustdoc", "--lib", "--all-features", "--", "-Z", "unstable-options", "--show-coverage"],
    ]
    matrix = []
    for command in commands:
        outcome = run(*command, check=False, env={"RUSTC_BOOTSTRAP": "1"})
        combined = outcome.stdout + "\n" + outcome.stderr
        # `rustdoc --show-coverage` prints item and example percentages. Parse
        # the Total row positionally so the example value cannot replace item
        # coverage.
        total_row = re.search(
            r"^\|\s*Total\s*\|\s*(\d+)\s*\|\s*([0-9]+(?:\.[0-9]+)?)%\s*\|\s*(\d+)\s*\|\s*([0-9]+(?:\.[0-9]+)?)%\s*\|$",
            combined,
            re.MULTILINE,
        )
        matrix.append({
            "command": command,
            "returncode": outcome.returncode,
            "stdout": outcome.stdout[-30000:],
            "stderr": outcome.stderr[-30000:],
            "documented": int(total_row.group(1)) if total_row else None,
            "percent": float(total_row.group(2)) if total_row else None,
        })
    percent = min((record["percent"] for record in matrix if record["percent"] is not None), default=None)
    checkpoint = {
        "commands": matrix,
        "documented": max((record["documented"] or 0 for record in matrix), default=0),
        "total": None,
        "percent": percent,
        "passed": bool(matrix) and all(record["returncode"] == 0 and record["percent"] == 100.0 for record in matrix),
        "source_digest": current_source_digest(),
        "created_at": now(),
    }
    write_json(DB / "checkpoints" / "rustdoc-coverage.json", checkpoint)
    symbols = read_jsonl(SYMBOL_MANIFEST)
    native_reference = next((page for page in read_jsonl(PAGE_MANIFEST) if page["doc_class"] == "reference" and "rust-api" in page["path"]), None)
    for symbol in symbols:
        if symbol["public_api"]:
            symbol["reference_status"] = "covered" if checkpoint["passed"] and native_reference else "pending"
            symbol["reference_pages"] = [native_reference["path"]] if checkpoint["passed"] and native_reference else []
    write_jsonl(SYMBOL_MANIFEST, symbols)
    write_jsonl(DB / "symbols.jsonl", symbols)
    doc_map = [
        record for record in read_jsonl(DB / "doc-map.jsonl")
        if record.get("entity_type") != "symbol"
    ]
    if checkpoint["passed"] and native_reference:
        doc_map.extend({
            "entity_id": symbol["symbol_id"],
            "entity_type": "symbol",
            "documentation_page": native_reference["path"],
        } for symbol in symbols if symbol["public_api"])
    write_jsonl(DB / "doc-map.jsonl", sorted(
        doc_map,
        key=lambda record: (record["entity_type"], record["entity_id"], record["documentation_page"]),
    ))
    state["last_command"] = "record-rustdoc"
    refresh_state_counts(state)
    save_state(state)
    print(json.dumps(checkpoint, indent=2))
    print_status(state)
    if not checkpoint["passed"]:
        raise SystemExit(1)


def cmd_verify_examples(_arguments: argparse.Namespace) -> None:
    state = load_state()
    if not state.get("docs_generation_allowed"):
        raise CompilerError("example validation may only run after verifier Gates 0-6 pass")
    commands = [
        ["cargo", "test", "--examples", "--all-features"],
        ["cargo", "check", "--manifest-path", "examples/operator-consumer/Cargo.toml"],
        ["cargo", "check", "--manifest-path", "examples/whisper-transcribe/Cargo.toml"],
    ]
    checkpoint = command_checkpoint("examples", commands)
    checkpoint["source_digest"] = current_source_digest()
    write_json(DB / "checkpoints" / "examples.json", checkpoint)
    examples = read_jsonl(DB / "examples.jsonl")
    for example in examples:
        if checkpoint["passed"]:
            example["verification_status"] = "verified"
            example["verification_checkpoint"] = ".doc-intel/checkpoints/examples.json"
        else:
            example["verification_status"] = "failed"
    write_jsonl(DB / "examples.jsonl", examples)
    state["last_command"] = "verify-examples"
    save_state(state)
    print(json.dumps(checkpoint, indent=2))
    print_status(state)
    if not checkpoint["passed"]:
        raise SystemExit(1)


def normalize_link_target(target: str) -> str:
    return target.split("#", 1)[0].split("?", 1)[0]


def validate_internal_links(pages: list[dict[str, Any]]) -> list[dict[str, Any]]:
    failures: list[dict[str, Any]] = []
    for page in pages:
        source = ROOT / page["path"]
        if not source.exists():
            failures.append({"source": page["path"], "target": None, "reason": "source page absent"})
            continue
        for target in MARKDOWN_LINK.findall(source.read_text()):
            if target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            cleaned = normalize_link_target(target)
            if not cleaned:
                continue
            destination = (ROOT / cleaned.lstrip("/")) if target.startswith("/") else (source.parent / cleaned)
            if not destination.exists():
                failures.append({"source": page["path"], "target": target, "reason": "target absent"})
    return failures


def cmd_validate_docs(_arguments: argparse.Namespace) -> None:
    state = load_state()
    pages = read_jsonl(PAGE_MANIFEST)
    acceptable_statuses = {"authored", "validated"}
    if not pages or any(page.get("status") not in acceptable_statuses for page in pages):
        raise CompilerError(
            "every page must have authored or previously validated status before documentation validation"
        )
    claims = read_jsonl(DB / "claims.jsonl")
    files = read_jsonl(REPOSITORY_MANIFEST)
    file_by_path = {record["path"]: record for record in files}
    claim_failures: list[str] = []
    for claim in claims:
        claim_failures.extend(f"{claim.get('claim_id')}: {failure}" for failure in validate_evidence(claim.get("evidence"), file_by_path))
    editorial_failures = editorial_page_failures(pages)
    mapping_failures = claim_marker_failures(pages, claims)
    api_failures = native_documentation_failures(read_jsonl(SYMBOL_MANIFEST))
    dossier_failures_found = dossier_semantic_failures(files)
    links = validate_internal_links(pages)
    commands = [
        ["python3", "tools/build_documentation.py", "--check"],
        ["cargo", "doc", "--no-deps", "--all-features"],
    ]
    build = command_checkpoint("documentation-build", commands)
    passed = not (
        claim_failures or editorial_failures or mapping_failures or api_failures
        or dossier_failures_found or links
    ) and build["passed"]
    checkpoint = {
        "passed": passed,
        "claim_failures": claim_failures,
        "editorial_failures": editorial_failures,
        "mapping_failures": mapping_failures,
        "api_documentation_failures": api_failures,
        "dossier_semantic_failures": dossier_failures_found,
        "broken_internal_links": links,
        "docs_digest": current_docs_digest(pages),
        "page_manifest_sha256": sha256_file(PAGE_MANIFEST),
        "claims_sha256": sha256_file(DB / "claims.jsonl"),
        "source_digest": current_source_digest(),
        "build_checkpoint": ".doc-intel/checkpoints/documentation-build.json",
        "created_at": now(),
    }
    write_json(DB / "checkpoints" / "documentation-validation.json", checkpoint)
    if passed:
        for page in pages:
            page["status"] = "validated"
            page["validated_sha256"] = sha256_file(ROOT / page["path"])
        write_jsonl(PAGE_MANIFEST, pages)
    state["phase"] = "validation"
    state["last_command"] = "validate-docs"
    refresh_state_counts(state)
    save_state(state)
    print(json.dumps(checkpoint, indent=2))
    print_status(state)
    if not checkpoint["passed"]:
        raise SystemExit(1)


def cmd_run_validation(_arguments: argparse.Namespace) -> None:
    state = load_state()
    commands = [
        ["cargo", "check", "--all-targets", "--all-features"],
        ["cargo", "test", "--lib", "--all-features"],
        ["cargo", "test", "--tests", "--all-features"],
        ["cargo", "test", "--doc", "--all-features"],
    ]
    checkpoint = command_checkpoint("repository-validation", commands)
    checkpoint["source_digest"] = current_source_digest()
    write_json(DB / "checkpoints" / "repository-validation.json", checkpoint)
    state["last_command"] = "run-validation"
    save_state(state)
    print(json.dumps(checkpoint, indent=2))
    print_status(state)
    if not checkpoint["passed"]:
        raise SystemExit(1)


def gate_result(number: int, name: str, failures: Iterable[str]) -> dict[str, Any]:
    unique = list(dict.fromkeys(str(failure) for failure in failures if failure))
    return {"gate": number, "name": name, "status": "pass" if not unique else "fail", "failures": unique}


def dossier_failures(files: list[dict[str, Any]], minimum_stage: str) -> list[str]:
    failures: list[str] = []
    required = {
        "file_id", "path", "content_hash", "language", "file_kind", "line_count", "package", "module",
        "visibility", "generated", "generated_from", "platform_gate", "feature_gate", "purpose",
        "responsibilities", "non_responsibilities", "defines", "imports", "imported_by", "reexports",
        "calls", "called_by", "constructs", "constructed_by", "implements", "implemented_by", "extends",
        "extended_by", "entry_points", "public_surface", "private_surface", "inputs", "outputs", "side_effects",
        "filesystem_io", "network_io", "device_io", "process_io", "ffi_io", "threads", "tasks",
        "async_boundaries", "queues", "channels", "locks", "atomics", "callbacks", "resource_ownership",
        "startup_behavior", "shutdown_behavior", "cancellation_behavior", "drop_cleanup", "finalization",
        "state_machine", "invariants", "errors_defined", "errors_created", "errors_wrapped", "errors_propagated",
        "errors_translated", "retry_behavior", "recovery_behavior", "configuration_read", "environment_variables",
        "feature_flags", "defaults", "protocol_messages", "endpoints", "serialization", "tests_covering",
        "examples_using", "related_docs", "observed_patterns", "potential_pitfalls", "evidence", "analysis_stage",
    }
    minimum = ANALYSIS_STAGES[minimum_stage]
    for record in files:
        if not record.get("semantic"):
            continue
        if record.get("status") != "analyzed":
            failures.append(f"{record['path']}: status {record.get('status')!r}")
            continue
        dossier_name = record.get("dossier")
        if not dossier_name or not (ROOT / dossier_name).exists():
            failures.append(f"{record['path']}: dossier absent")
            continue
        dossier = read_json(ROOT / dossier_name)
        missing = sorted(required - dossier.keys())
        if missing:
            failures.append(f"{record['path']}: dossier missing fields {missing}")
        if dossier.get("content_hash") != record["sha256"]:
            failures.append(f"{record['path']}: dossier content hash differs")
        stage = dossier.get("analysis_stage")
        if ANALYSIS_STAGES.get(stage, 0) < minimum:
            failures.append(f"{record['path']}: dossier stage {stage!r} is below {minimum_stage}")
    return failures


def frozen_manifest_failures(state: dict[str, Any], files: list[dict[str, Any]]) -> list[str]:
    failures: list[str] = []
    if not (DB / "PROTOCOL.md").exists() or sha256_file(DB / "PROTOCOL.md") != PROTOCOL_SHA256:
        failures.append("governing protocol is absent or its hash differs")
    if not (DB / "CRITICAL-EXECUTION-CONTRACT.md").exists() or sha256_file(DB / "CRITICAL-EXECUTION-CONTRACT.md") != CONTRACT_SHA256:
        failures.append("critical execution contract is absent or its hash differs")
    governing = DB / "GOVERNING-INPUTS.json"
    if not governing.exists():
        failures.append("governing-input provenance is absent")
    snapshot = state.get("snapshot")
    if not snapshot or run("git", "cat-file", "-e", f"{snapshot}^{{commit}}", check=False).returncode:
        failures.append("state snapshot is absent or is not a Git commit")
        return failures
    expected = snapshot_tree(snapshot)
    if len(expected) != len(files):
        failures.append(f"manifest denominator differs: expected {len(expected)}, found {len(files)}")
    by_path = {record.get("path"): record for record in files}
    if len(by_path) != len(files):
        failures.append("repository manifest contains duplicate paths")
    for item in expected:
        record = by_path.get(item["path"])
        if not record:
            failures.append(f"manifest path absent: {item['path']}")
            continue
        if record.get("git_object") != item["git_object"]:
            failures.append(f"{item['path']}: Git object differs")
        data = git_bytes(snapshot, item["path"])
        if record.get("sha256") != sha256_bytes(data):
            failures.append(f"{item['path']}: content hash differs")
        if record.get("status") not in VALID_FILE_STATES:
            failures.append(f"{item['path']}: invalid status {record.get('status')!r}")
        if record.get("semantic") and record.get("status") not in {"pending", "analyzed"}:
            failures.append(f"{item['path']}: semantic record has non-semantic status")
        if not record.get("semantic") and record.get("status") == "pending":
            failures.append(f"{item['path']}: non-semantic record is pending")
    snapshot_path = DB / "snapshot.json"
    if not snapshot_path.exists():
        failures.append("snapshot.json is absent")
    else:
        snapshot_record = read_json(snapshot_path)
        if snapshot_record.get("snapshot") != snapshot:
            failures.append("snapshot.json and state.json identify different commits")
    return failures


def checkpoint_current(path: Path, digest_key: str, digest: str) -> tuple[dict[str, Any] | None, list[str]]:
    if not path.exists():
        return None, [f"checkpoint absent: {path.relative_to(ROOT)}"]
    checkpoint = read_json(path)
    failures = []
    if not checkpoint.get("passed"):
        failures.append(f"checkpoint did not pass: {path.relative_to(ROOT)}")
    if checkpoint.get(digest_key) != digest:
        failures.append(f"checkpoint is stale for {digest_key}: {path.relative_to(ROOT)}")
    return checkpoint, failures


def verify_all(state: dict[str, Any]) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    files = read_jsonl(REPOSITORY_MANIFEST)
    symbols = read_jsonl(SYMBOL_MANIFEST)
    edges = read_jsonl(EDGE_MANIFEST)
    tests = read_jsonl(DB / "tests.jsonl")
    examples = read_jsonl(DB / "examples.jsonl")
    errors = read_jsonl(DB / "errors.jsonl")
    config = read_jsonl(DB / "configuration.jsonl")
    behaviors = read_jsonl(DB / "behaviors.jsonl")
    lifecycles = read_jsonl(DB / "lifecycles.jsonl")
    protocols = read_jsonl(DB / "protocols.jsonl")
    patterns = read_jsonl(DB / "patterns.jsonl")
    capabilities = read_jsonl(DB / "capabilities.jsonl")
    journeys = read_jsonl(DB / "user-journeys.jsonl")
    pages = read_jsonl(PAGE_MANIFEST)
    claims = read_jsonl(DB / "claims.jsonl")
    doc_map = read_jsonl(DB / "doc-map.jsonl")
    file_by_path = {record["path"]: record for record in files}
    dossier_quality = dossier_semantic_failures(files)
    native_doc_quality = native_documentation_failures(symbols)
    editorial_quality = editorial_page_failures(pages)
    claim_mapping_quality = claim_marker_failures(pages, claims)
    gates: list[dict[str, Any]] = []
    gates.append(gate_result(0, "Repository snapshot + manifest", frozen_manifest_failures(state, files)))
    gates.append(gate_result(
        1,
        "Every semantic file analyzed",
        [*dossier_failures(files, "discovered"), *dossier_quality],
    ))
    gate2_failures: list[str] = []
    if not symbols:
        gate2_failures.append("symbol manifest is empty")
    if len({record.get("symbol_id") for record in symbols}) != len(symbols):
        gate2_failures.append("symbol IDs are absent or duplicated")
    for symbol in symbols:
        if symbol.get("status") != "analyzed":
            gate2_failures.append(f"{symbol.get('symbol_id')}: status is not analyzed")
        source = file_by_path.get(symbol.get("source_file"))
        if not source or source.get("sha256") != symbol.get("source_file_sha256"):
            gate2_failures.append(f"{symbol.get('symbol_id')}: source provenance differs")
    rustdoc_extraction = DB / "checkpoints" / "rustdoc-extraction.json"
    if not rustdoc_extraction.exists():
        gate2_failures.append("rustdoc extraction checkpoint is absent")
    else:
        checkpoint = read_json(rustdoc_extraction)
        if checkpoint.get("snapshot") != state.get("snapshot"):
            gate2_failures.append("rustdoc extraction checkpoint targets another snapshot")
        import gzip
        for kind in ("private", "public"):
            artifact = DB / "checkpoints" / f"rustdoc-{kind}.json.gz"
            if not artifact.exists():
                gate2_failures.append(f"compressed rustdoc {kind} artifact is absent")
            elif sha256_bytes(gzip.decompress(artifact.read_bytes())) != checkpoint.get(f"{kind}_json_sha256"):
                gate2_failures.append(f"compressed rustdoc {kind} artifact hash differs")
        if checkpoint.get("owned_named_symbols") != len(symbols):
            gate2_failures.append("symbol manifest count differs from rustdoc checkpoint")
        if checkpoint.get("intentionally_public_symbols") != sum(bool(record.get("public_api")) for record in symbols):
            gate2_failures.append("public-symbol denominator differs from rustdoc checkpoint")
    gates.append(gate_result(2, "Every symbol extracted", gate2_failures))
    gate3_failures = dossier_failures(files, "relationships_resolved")
    if not edges:
        gate3_failures.append("relationship graph is empty")
    entity_ids = {
        *(record["file_id"] for record in files),
        *(record["symbol_id"] for record in symbols),
        *(record["test_id"] for record in tests),
        *(record["example_id"] for record in examples),
    }
    allowed_edge_states = {"resolved", "dynamic", "external", "unresolved_explicit"}
    for edge in edges:
        if edge.get("source") not in entity_ids:
            gate3_failures.append(f"{edge.get('edge_id')}: source is dangling")
        if edge.get("status") not in allowed_edge_states:
            gate3_failures.append(f"{edge.get('edge_id')}: invalid relationship status")
        if edge.get("status") == "resolved" and edge.get("target") not in entity_ids:
            gate3_failures.append(f"{edge.get('edge_id')}: resolved target is dangling")
        if edge.get("status") != "resolved" and not edge.get("mechanism"):
            gate3_failures.append(f"{edge.get('edge_id')}: non-resolved edge lacks mechanism")
    gates.append(gate_result(3, "Relationship graph resolved", gate3_failures))
    gate4_failures = dossier_failures(files, "behavior_validated")
    required_ledgers = {
        "tests": tests, "examples": examples, "errors": errors, "configuration": config,
        "behaviors": behaviors, "lifecycles": lifecycles, "protocols": protocols,
    }
    for name, records in required_ledgers.items():
        if not records:
            gate4_failures.append(f"{name} ledger is empty")
        for record in records:
            if record.get("status") != "analyzed":
                gate4_failures.append(f"{name} record is not analyzed: {record}")
            gate4_failures.extend(f"{name}: {failure}" for failure in validate_evidence(record.get("evidence"), file_by_path))
    for test in tests:
        if test.get("path", "").endswith(".rs") and not EXECUTABLE_RUST_TEST_ATTRIBUTE.search(test.get("attributes", "")):
            gate4_failures.append(f"{test.get('test_id')}: Rust test record lacks an executable test attribute")
    if not (DB / "conflicts.jsonl").exists() or not (DB / "unknowns.jsonl").exists():
        gate4_failures.append("conflicts or unknowns ledger is absent")
    refinement = DB / "checkpoints" / "intelligence-refinement.json"
    if not refinement.exists() or read_json(refinement).get("snapshot") != state.get("snapshot"):
        gate4_failures.append("intelligence refinement checkpoint is absent or targets another snapshot")
    for error in errors:
        for field in ("trigger_condition", "developer_action", "retryable", "recoverable"):
            if str(error.get(field, "")).strip().lower() in {"", "unknown"}:
                gate4_failures.append(f"{error.get('error_id')}: {field} is unresolved without disposition")
        if error.get("test_coverage_status") not in {"linked", "no_direct_test_link_extracted"}:
            gate4_failures.append(f"{error.get('error_id')}: test coverage disposition is absent")
    for item in config:
        for field in ("default", "when_read", "precedence", "invalid_value_behavior"):
            if str(item.get(field, "")).strip().lower() in {"", "unknown"}:
                gate4_failures.append(f"{item.get('config_id')}: {field} is unresolved without disposition")
    for lifecycle in lifecycles:
        for field in ("source_state", "destination_state", "guard", "recovery", "idempotence"):
            if str(lifecycle.get(field, "")).strip().lower() in {"", "unknown"}:
                gate4_failures.append(f"{lifecycle.get('lifecycle_id')}: {field} is unresolved without disposition")
    for pattern in patterns:
        for field in ("problem", "how_implemented", "constraints", "tradeoffs", "failure_behavior"):
            if str(pattern.get(field, "")).strip().lower() in {"", "unknown"}:
                gate4_failures.append(f"{pattern.get('pattern_id')}: {field} is unresolved without disposition")
    gates.append(gate_result(4, "Behaviors, errors, lifecycles, and configuration extracted", gate4_failures))
    gate5_failures: list[str] = []
    if not capabilities:
        gate5_failures.append("capability ledger is empty")
    if not journeys:
        gate5_failures.append("user-journey ledger is empty")
    capability_ids = {record.get("capability_id") for record in capabilities}
    for capability in capabilities:
        if capability.get("status") != "analyzed":
            gate5_failures.append(f"{capability.get('capability_id')}: status is not analyzed")
        gate5_failures.extend(f"{capability.get('capability_id')}: {failure}" for failure in validate_evidence(capability.get("evidence"), file_by_path))
    for journey in journeys:
        if journey.get("status") != "analyzed":
            gate5_failures.append(f"{journey.get('journey_id')}: status is not analyzed")
        unknown = set(journey.get("capability_ids", [])) - capability_ids
        if unknown:
            gate5_failures.append(f"{journey.get('journey_id')}: unknown capabilities {sorted(unknown)}")
        gate5_failures.extend(f"{journey.get('journey_id')}: {failure}" for failure in validate_evidence(journey.get("evidence"), file_by_path))
    behavior_domains = {record.get("domain") for record in behaviors}
    capability_domains = {domain for record in capabilities for domain in record.get("domains", [])}
    uncovered_domains = sorted(domain for domain in behavior_domains - capability_domains if domain)
    if uncovered_domains:
        gate5_failures.append(f"behavior domains absent from capability model: {uncovered_domains}")
    gates.append(gate_result(5, "Capabilities and user journeys derived", gate5_failures))
    gate6_failures = dossier_failures(files, "doc_ready")
    if not pages:
        gate6_failures.append("page ledger is empty")
    if not (DB / "terminology.json").exists() or read_json(DB / "terminology.json").get("status") != "reviewed":
        gate6_failures.append("terminology registry is not reviewed")
    else:
        known_symbols = {record["symbol_id"] for record in symbols}
        for term in read_json(DB / "terminology.json").get("terms", []):
            defining = term.get("first_defining_symbol")
            if defining not in known_symbols:
                gate6_failures.append(
                    f"terminology {term.get('canonical_name')}: first defining symbol is unresolved"
                )
    page_ids = {page.get("page_id") for page in pages}
    if len(page_ids) != len(pages):
        gate6_failures.append("page IDs are absent or duplicated")
    for capability in capabilities:
        related = [page for page in pages if capability["capability_id"] in page.get("capability_ids", [])]
        if not any(page.get("doc_class") == "concept" for page in related):
            gate6_failures.append(f"{capability['capability_id']}: concept page absent")
        if capability.get("guide_required") and not any(page.get("doc_class") == "how-to" for page in related):
            gate6_failures.append(f"{capability['capability_id']}: required how-to page absent")
        if capability.get("troubleshooting_applicable") and not any(page.get("doc_class") == "troubleshooting" for page in related):
            gate6_failures.append(f"{capability['capability_id']}: applicable troubleshooting page absent")
    required_classes = {"overview", "getting-started", "concept", "how-to", "reference", "error-reference", "config-reference", "troubleshooting", "internals", "glossary"}
    missing_classes = sorted(required_classes - {page.get("doc_class") for page in pages})
    if missing_classes:
        gate6_failures.append(f"information architecture lacks required evidence-supported classes: {missing_classes}")
    gates.append(gate_result(6, "Information architecture reviewed against evidence", gate6_failures))
    for gate_number, name in ((7, "Concept documentation"), (8, "How-to documentation")):
        target = [page for page in pages if page.get("gate") == gate_number]
        failures = editorial_page_failures(target)
        if not target:
            failures.append(f"no pages assigned to Gate {gate_number}")
        for page in target:
            if page.get("status") not in {"authored", "validated"}:
                failures.append(f"{page.get('page_id')}: status {page.get('status')!r}")
            elif not (ROOT / page["path"]).exists():
                failures.append(f"{page.get('page_id')}: page absent")
        gates.append(gate_result(gate_number, name, failures))
    gate9_failures: list[str] = []
    gate9_failures.extend(native_doc_quality)
    rustdoc_checkpoint_path = DB / "checkpoints" / "rustdoc-coverage.json"
    rustdoc_checkpoint, stale = checkpoint_current(rustdoc_checkpoint_path, "source_digest", current_source_digest())
    gate9_failures.extend(stale)
    if rustdoc_checkpoint and rustdoc_checkpoint.get("percent") != 100.0:
        gate9_failures.append(f"native Rust API coverage is {rustdoc_checkpoint.get('percent')}%, not 100%")
    symbol_mappings = {
        record.get("entity_id") for record in doc_map
        if record.get("entity_type") == "symbol"
    }
    native_reference = next(
        (page for page in pages if page.get("doc_class") == "reference" and "rust-api" in page.get("path", "")),
        None,
    )
    native_reference_text = (
        (ROOT / native_reference["path"]).read_text()
        if native_reference and (ROOT / native_reference["path"]).exists()
        else ""
    )
    for symbol in symbols:
        if not symbol.get("public_api"):
            continue
        if symbol.get("reference_status") != "covered":
            gate9_failures.append(f"{symbol['symbol_id']}: public symbol lacks reference coverage")
        if symbol["symbol_id"] not in symbol_mappings:
            gate9_failures.append(f"{symbol['symbol_id']}: public symbol lacks reverse documentation mapping")
        if symbol["symbol_id"] not in native_reference_text:
            gate9_failures.append(f"{symbol['symbol_id']}: public symbol is absent from the exhaustive Rust API page")
    reference_pages = [page for page in pages if page.get("gate") == 9]
    if not reference_pages:
        gate9_failures.append("no reference pages assigned to Gate 9")
    for page in reference_pages:
        if page.get("status") not in {"authored", "validated"}:
            gate9_failures.append(f"{page.get('page_id')}: status {page.get('status')!r}")
    gates.append(gate_result(9, "API reference", gate9_failures))
    gate10_failures: list[str] = []
    for name, records in (("error", errors), ("configuration", config), ("behavior", behaviors), ("lifecycle", lifecycles), ("protocol", protocols)):
        for record in records:
            if record.get("documentation_status") != "covered":
                identifier = next((record.get(key) for key in ("error_id", "config_id", "behavior_id", "lifecycle_id", "protocol_id") if record.get(key)), "unknown")
                gate10_failures.append(f"{name} {identifier}: documentation coverage pending")
    target10 = [page for page in pages if page.get("gate") == 10]
    gate10_failures.extend(editorial_page_failures(target10))
    if not target10:
        gate10_failures.append("no troubleshooting/error/best-practice pages assigned to Gate 10")
    for page in target10:
        if page.get("status") not in {"authored", "validated"}:
            gate10_failures.append(f"{page.get('page_id')}: status {page.get('status')!r}")
    gates.append(gate_result(10, "Troubleshooting, errors, and best practices", gate10_failures))
    gate11_failures: list[str] = []
    example_checkpoint, stale = checkpoint_current(DB / "checkpoints" / "examples.json", "source_digest", current_source_digest())
    gate11_failures.extend(stale)
    for example in examples:
        if example.get("verification_status") != "verified":
            gate11_failures.append(f"{example.get('example_id')}: example not verified")
    gates.append(gate_result(11, "Examples executed", gate11_failures))
    gate12_failures: list[str] = []
    doc_checkpoint_path = DB / "checkpoints" / "documentation-validation.json"
    if not doc_checkpoint_path.exists():
        gate12_failures.append("documentation validation checkpoint is absent")
    else:
        doc_checkpoint = read_json(doc_checkpoint_path)
        if not doc_checkpoint.get("passed"):
            gate12_failures.append("documentation validation did not pass")
        if doc_checkpoint.get("docs_digest") != current_docs_digest(pages):
            gate12_failures.append("documentation validation is stale for page content")
        if doc_checkpoint.get("claims_sha256") != sha256_file(DB / "claims.jsonl"):
            gate12_failures.append("documentation validation is stale for claims")
        if doc_checkpoint.get("broken_internal_links"):
            gate12_failures.append(f"broken internal links: {len(doc_checkpoint['broken_internal_links'])}")
    validation_checkpoint, stale = checkpoint_current(DB / "checkpoints" / "repository-validation.json", "source_digest", current_source_digest())
    gate12_failures.extend(stale)
    for page in pages:
        if page.get("status") != "validated":
            gate12_failures.append(f"{page.get('page_id')}: page has not passed final validation")
    for claim in claims:
        if claim.get("status") not in {"verified", "conflicted", "unknown_explicit"}:
            gate12_failures.append(f"{claim.get('claim_id')}: unresolved claim status")
        gate12_failures.extend(f"{claim.get('claim_id')}: {failure}" for failure in validate_evidence(claim.get("evidence"), file_by_path))
    for conflict in read_jsonl(DB / "conflicts.jsonl"):
        if conflict.get("status") not in {"explicit_unresolved", "resolved"}:
            gate12_failures.append(f"{conflict.get('conflict_id')}: conflict is hidden or unclassified")
    for unknown in read_jsonl(DB / "unknowns.jsonl"):
        if unknown.get("status") != "explicit":
            gate12_failures.append(f"{unknown.get('unknown_id')}: unknown is not explicit")
    gate12_failures.extend(editorial_quality)
    gate12_failures.extend(claim_mapping_quality)
    gate12_failures.extend(dossier_quality)
    gate12_failures.extend(native_doc_quality)
    gates.append(gate_result(12, "Documentation build + link validation", gate12_failures))
    capability_concept = {
        capability["capability_id"] for capability in capabilities
        if any(capability["capability_id"] in page.get("capability_ids", []) and page.get("doc_class") == "concept" and page.get("status") == "validated" for page in pages)
    }
    capability_guides = {
        capability["capability_id"] for capability in capabilities
        if any(capability["capability_id"] in page.get("capability_ids", []) and page.get("doc_class") == "how-to" and page.get("status") == "validated" for page in pages)
    }
    capability_troubleshooting = {
        capability["capability_id"] for capability in capabilities
        if any(capability["capability_id"] in page.get("capability_ids", []) and page.get("doc_class") == "troubleshooting" and page.get("status") == "validated" for page in pages)
    }
    metrics = {
        "total_tracked_files": len(files),
        "total_semantic_files": sum(bool(record.get("semantic")) for record in files),
        "analyzed_semantic_files": sum(bool(record.get("semantic")) and record.get("status") == "analyzed" for record in files),
        "total_public_symbols": sum(bool(record.get("public_api")) for record in symbols),
        "reference_covered_symbols": sum(bool(record.get("public_api")) and record.get("reference_status") == "covered" for record in symbols),
        "total_errors": len(errors), "documented_errors": sum(record.get("documentation_status") == "covered" for record in errors),
        "total_config_fields": len(config), "documented_config_fields": sum(record.get("documentation_status") == "covered" for record in config),
        "total_examples": len(examples), "verified_examples": sum(record.get("verification_status") == "verified" for record in examples),
        "total_tests": len(tests), "analyzed_tests": sum(record.get("status") == "analyzed" for record in tests),
        "total_behavior_records": len(behaviors), "documented_behavior_records": sum(record.get("documentation_status") == "covered" for record in behaviors),
        "total_user_capabilities": len(capabilities), "concept_covered_capabilities": len(capability_concept),
        "guide_covered_capabilities": len(capability_guides),
        "troubleshooting_applicable_capabilities": sum(bool(record.get("troubleshooting_applicable")) for record in capabilities),
        "troubleshooting_covered_capabilities": len(capability_troubleshooting),
        "total_pages": len(pages), "validated_pages": sum(record.get("status") == "validated" for record in pages),
        "total_claims": len(claims), "verified_or_explicit_claims": sum(record.get("status") in {"verified", "conflicted", "unknown_explicit"} for record in claims),
        "relationships": len(edges), "conflicts": len(read_jsonl(DB / "conflicts.jsonl")),
        "unknowns": len(read_jsonl(DB / "unknowns.jsonl")),
    }
    return gates, metrics


def first_resume_record() -> str:
    for record in read_jsonl(REPOSITORY_MANIFEST):
        if record.get("status") == "pending":
            return f"file:{record['file_id']}:{record['path']}"
    for record in read_jsonl(SYMBOL_MANIFEST):
        if record.get("status") == "pending":
            return f"symbol:{record['symbol_id']}:{record.get('qualified_name')}"
    ledgers = [
        ("capability", DB / "capabilities.jsonl", "capability_id", "status", "analyzed"),
        ("journey", DB / "user-journeys.jsonl", "journey_id", "status", "analyzed"),
        ("page", PAGE_MANIFEST, "page_id", "status", "validated"),
        ("example", DB / "examples.jsonl", "example_id", "verification_status", "verified"),
    ]
    for kind, path, identifier, status_field, expected in ledgers:
        for record in read_jsonl(path):
            if record.get(status_field) != expected:
                return f"{kind}:{record.get(identifier)}"
    return "verifier:first-failing-gate"


def print_status(state: dict[str, Any], metrics: dict[str, Any] | None = None, gates: list[dict[str, Any]] | None = None) -> None:
    if metrics is None:
        files = read_jsonl(REPOSITORY_MANIFEST)
        symbols = read_jsonl(SYMBOL_MANIFEST)
        pages = read_jsonl(PAGE_MANIFEST)
        metrics = {
            "total_tracked_files": len(files),
            "total_semantic_files": sum(bool(record.get("semantic")) for record in files),
            "analyzed_semantic_files": sum(bool(record.get("semantic")) and record.get("status") == "analyzed" for record in files),
            "total_public_symbols": sum(bool(record.get("public_api")) for record in symbols),
            "reference_covered_symbols": sum(bool(record.get("public_api")) and record.get("reference_status") == "covered" for record in symbols),
            "total_pages": len(pages), "validated_pages": sum(record.get("status") == "validated" for record in pages),
        }
    semantic_remaining = metrics["total_semantic_files"] - metrics["analyzed_semantic_files"]
    print("STATUS: " + ("COMPLETE" if state.get("completion") else "INCOMPLETE"))
    print(f"SNAPSHOT: {state.get('snapshot')}")
    print(f"PHASE: {state.get('phase')}")
    print(f"ANALYZED: {metrics['analyzed_semantic_files']} / {metrics['total_semantic_files']}")
    print(f"REMAINING: {semantic_remaining}")
    print(f"PUBLIC SYMBOL REFERENCE: {metrics['reference_covered_symbols']} / {metrics['total_public_symbols']}")
    print(f"PAGES VALIDATED: {metrics['validated_pages']} / {metrics['total_pages']}")
    print(f"NEXT: {'none' if state.get('completion') else first_resume_record()}")
    if gates:
        for gate in gates:
            print(f"GATE {gate['gate']}: {gate['status'].upper()} — {gate['name']}")
            for failure in gate["failures"][:20]:
                print(f"  - {failure}")
            if len(gate["failures"]) > 20:
                print(f"  - ... {len(gate['failures']) - 20} additional failures recorded in coverage.json")


def cmd_verify(_arguments: argparse.Namespace) -> None:
    state = load_state()
    gates, metrics = verify_all(state)
    all_pass = all(gate["status"] == "pass" for gate in gates)
    state["gates"] = {str(gate["gate"]): {"status": gate["status"], "failures": gate["failures"]} for gate in gates}
    state["docs_generation_allowed"] = all(gate["status"] == "pass" for gate in gates[:7])
    first_failed = next((gate["gate"] for gate in gates if gate["status"] != "pass"), None)
    phases = {
        0: "snapshot", 1: "file-analysis", 2: "symbol-extraction", 3: "relationship-resolution",
        4: "surface-extraction", 5: "capability-model", 6: "information-architecture",
        7: "concept-documentation", 8: "how-to-documentation", 9: "api-reference",
        10: "failure-documentation", 11: "example-validation", 12: "validation",
    }
    state["gate"] = 13 if all_pass else first_failed
    state["phase"] = "final" if all_pass else phases[first_failed]
    state["last_command"] = "verify-pass" if all_pass else "verify-fail"
    state["completion"] = all_pass
    if all_pass:
        state["last_error"] = None
    refresh_state_counts(state)
    save_state(state)
    coverage = {
        "snapshot": state["snapshot"], "generated_at": now(), "final_status": "complete" if all_pass else "incomplete",
        "metrics": metrics, "gates": gates, "next": None if all_pass else first_resume_record(),
    }
    write_json(DB / "coverage.json", coverage)
    print_status(state, metrics, gates)
    raise SystemExit(0 if all_pass else 1)


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    commands = result.add_subparsers(dest="command", required=True)
    initialize = commands.add_parser("init", help="Freeze the repository denominator")
    initialize.add_argument("--snapshot", help="Commit to freeze; defaults to HEAD")
    initialize.add_argument("--force", action="store_true", help="Reserved for controlled rebuilds")
    initialize.set_defaults(function=cmd_init)
    analyze = commands.add_parser("analyze-files", help="Analyze pending semantic files")
    analyze.add_argument("--batch", type=int, default=0, help="Maximum pending files; zero means all")
    analyze.set_defaults(function=cmd_analyze_files)
    symbols = commands.add_parser("extract-symbols", help="Load compiler-produced private and public rustdoc JSON")
    symbols.add_argument("--private-json", required=True)
    symbols.add_argument("--public-json", required=True)
    symbols.set_defaults(function=cmd_extract_symbols)
    surfaces = commands.add_parser("extract-surfaces", help="Extract tests, examples, errors, config, protocols, and behavior")
    surfaces.set_defaults(function=cmd_extract_surfaces)
    model = commands.add_parser("review-model", help="Validate capability, journey, terminology, and page ledgers")
    model.set_defaults(function=cmd_review_model)
    sync = commands.add_parser("sync-pages", help="Validate authored pages and compute surface mappings")
    sync.set_defaults(function=cmd_sync_pages)
    rustdoc = commands.add_parser("record-rustdoc", help="Run and record native Rust API documentation coverage")
    rustdoc.set_defaults(function=cmd_record_rustdoc)
    examples = commands.add_parser("verify-examples", help="Compile and test every example target")
    examples.set_defaults(function=cmd_verify_examples)
    docs = commands.add_parser("validate-docs", help="Build documentation, validate claims, and check links")
    docs.set_defaults(function=cmd_validate_docs)
    validation = commands.add_parser("run-validation", help="Run repository code and test validation")
    validation.set_defaults(function=cmd_run_validation)
    verify = commands.add_parser("verify", help="Compute all frozen-denominator gates")
    verify.set_defaults(function=cmd_verify)
    status = commands.add_parser("status", help="Print the persisted resume position")
    status.set_defaults(function=lambda _args: print_status(load_state()))
    return result


def main() -> None:
    arguments = parser().parse_args()
    try:
        arguments.function(arguments)
    except CompilerError as error:
        print(f"documentation compiler error: {error}", file=sys.stderr)
        if STATE.exists():
            state = load_state()
            state["last_error"] = str(error)
            state["last_command"] = f"{arguments.command}-failed"
            save_state(state)
            print_status(state)
        raise SystemExit(2) from error


if __name__ == "__main__":
    main()
