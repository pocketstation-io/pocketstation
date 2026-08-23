#!/usr/bin/env python3
"""Build and validate the PocketStation Markdown publication as a static site."""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import posixpath
import re
import shutil
from pathlib import Path, PurePosixPath
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / ".doc-intel"
OUTPUT = ROOT / "target" / "pocketstation-docs"
LINK = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")
CLAIM_MARKER = re.compile(r"<!--\s*claims:.*?-->", re.DOTALL)


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def output_path(markdown_path: str) -> PurePosixPath:
    source = PurePosixPath(markdown_path)
    if source.name == "README.md":
        return source.parent / "index.html"
    if markdown_path == "RELEASE_NOTES.md":
        return PurePosixPath("release-notes.html")
    return source.with_suffix(".html")


def rewrite_target(target: str, current_markdown: str) -> str:
    if target.startswith(("http://", "https://", "mailto:", "#")):
        return target
    base, separator, fragment = target.partition("#")
    if not base:
        return target
    if target.startswith("/"):
        markdown_target = posixpath.normpath(base.lstrip("/"))
        rendered = "/" + output_path(markdown_target).as_posix()
    else:
        markdown_target = posixpath.normpath(
            posixpath.join(posixpath.dirname(current_markdown), base)
        )
        rendered_target = output_path(markdown_target).as_posix()
        current_output_dir = output_path(current_markdown).parent.as_posix()
        rendered = posixpath.relpath(rendered_target, current_output_dir or ".")
    return rendered + ((separator + fragment) if separator else "")


def inline(value: str, current_markdown: str) -> str:
    pieces = value.split("`")
    rendered: list[str] = []
    for index, piece in enumerate(pieces):
        if index % 2:
            rendered.append(f"<code>{html.escape(piece)}</code>")
            continue
        escaped = html.escape(piece)

        def link(match: re.Match[str]) -> str:
            label = html.escape(html.unescape(match.group(1)))
            href = html.escape(rewrite_target(html.unescape(match.group(2)), current_markdown), quote=True)
            return f'<a href="{href}">{label}</a>'

        escaped = LINK.sub(link, escaped)
        escaped = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", escaped)
        rendered.append(escaped)
    return "".join(rendered)


def heading_id(value: str, used: set[str]) -> str:
    base = re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-") or "section"
    candidate = base
    number = 2
    while candidate in used:
        candidate = f"{base}-{number}"
        number += 1
    used.add(candidate)
    return candidate


def markdown_to_html(markdown: str, current_markdown: str) -> str:
    lines = CLAIM_MARKER.sub("", markdown).splitlines()
    blocks: list[str] = []
    paragraph: list[str] = []
    used_headings: set[str] = set()
    in_code = False
    code_language = ""
    code_lines: list[str] = []
    list_kind: str | None = None

    def flush_paragraph() -> None:
        if paragraph:
            blocks.append(f"<p>{inline(' '.join(part.strip() for part in paragraph), current_markdown)}</p>")
            paragraph.clear()

    def close_list() -> None:
        nonlocal list_kind
        if list_kind:
            blocks.append(f"</{list_kind}>")
            list_kind = None

    index = 0
    while index < len(lines):
        line = lines[index]
        fence = re.match(r"^```\s*([\w+-]*)", line)
        if fence:
            flush_paragraph()
            close_list()
            if in_code:
                language = f' class="language-{html.escape(code_language)}"' if code_language else ""
                blocks.append(f"<pre><code{language}>{html.escape(chr(10).join(code_lines))}</code></pre>")
                code_lines.clear()
                code_language = ""
                in_code = False
            else:
                in_code = True
                code_language = fence.group(1)
            index += 1
            continue
        if in_code:
            code_lines.append(line)
            index += 1
            continue
        heading = re.match(r"^(#{1,6})\s+(.+)$", line)
        if heading:
            flush_paragraph()
            close_list()
            level = len(heading.group(1))
            label = heading.group(2).strip()
            anchor = heading_id(re.sub(r"[`*_]", "", label), used_headings)
            blocks.append(f'<h{level} id="{anchor}">{inline(label, current_markdown)}</h{level}>')
            index += 1
            continue
        if line.startswith("|") and index + 1 < len(lines) and re.match(r"^\|(?:\s*:?-+:?\s*\|)+$", lines[index + 1]):
            flush_paragraph()
            close_list()
            headers = [cell.strip() for cell in line.strip().strip("|").split("|")]
            index += 2
            rows: list[list[str]] = []
            while index < len(lines) and lines[index].startswith("|"):
                rows.append([cell.strip() for cell in lines[index].strip().strip("|").split("|")])
                index += 1
            table = ['<div class="table-wrap"><table><thead><tr>']
            table.extend(f"<th>{inline(cell, current_markdown)}</th>" for cell in headers)
            table.append("</tr></thead><tbody>")
            for row in rows:
                table.append("<tr>")
                row += [""] * (len(headers) - len(row))
                table.extend(f"<td>{inline(cell, current_markdown)}</td>" for cell in row[: len(headers)])
                table.append("</tr>")
            table.append("</tbody></table></div>")
            blocks.append("".join(table))
            continue
        bullet = re.match(r"^\s*[-*]\s+(.+)$", line)
        ordered = re.match(r"^\s*\d+\.\s+(.+)$", line)
        if bullet or ordered:
            flush_paragraph()
            wanted = "ul" if bullet else "ol"
            if list_kind != wanted:
                close_list()
                list_kind = wanted
                blocks.append(f"<{wanted}>")
            blocks.append(f"<li>{inline((bullet or ordered).group(1), current_markdown)}</li>")
            index += 1
            continue
        quote = re.match(r"^>\s?(.*)$", line)
        if quote:
            flush_paragraph()
            close_list()
            blocks.append(f"<blockquote>{inline(quote.group(1), current_markdown)}</blockquote>")
            index += 1
            continue
        if not line.strip():
            flush_paragraph()
            close_list()
        else:
            close_list()
            paragraph.append(line)
        index += 1
    flush_paragraph()
    close_list()
    if in_code:
        blocks.append(f"<pre><code>{html.escape(chr(10).join(code_lines))}</code></pre>")
    return "\n".join(blocks)


def nav_html(pages: list[dict[str, Any]], current: dict[str, Any]) -> str:
    labels = {
        7: "Learn and understand",
        8: "How-to guides",
        9: "Reference",
        10: "Failures and operations",
    }
    current_output = output_path(current["path"])
    groups: list[str] = []
    for gate in sorted(labels):
        entries = []
        for page in sorted((item for item in pages if item["gate"] == gate), key=lambda item: (item["doc_class"], item["title"])):
            destination = output_path(page["path"])
            href = posixpath.relpath(destination.as_posix(), current_output.parent.as_posix() or ".")
            active = ' aria-current="page" class="active"' if page["page_id"] == current["page_id"] else ""
            entries.append(f'<li><a href="{html.escape(href, quote=True)}"{active}>{html.escape(page["title"])}</a></li>')
        groups.append(f'<details open><summary>{labels[gate]}</summary><ul>{"".join(entries)}</ul></details>')
    home = posixpath.relpath("index.html", current_output.parent.as_posix() or ".")
    return f'<a class="brand" href="{home}">PocketStation</a>{"".join(groups)}'


STYLE = """
:root{color-scheme:light dark;--bg:#0b1020;--panel:#121a2e;--text:#edf2ff;--muted:#aab5d0;--line:#293556;--accent:#77b7ff;--code:#0a0e18}*{box-sizing:border-box}html{scroll-behavior:smooth}body{margin:0;background:var(--bg);color:var(--text);font:16px/1.65 system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}a{color:var(--accent)}a:focus-visible,input:focus-visible{outline:3px solid #f8c555;outline-offset:3px}.skip{position:absolute;left:-9999px}.skip:focus{left:1rem;top:1rem;z-index:9;background:#fff;color:#000;padding:.75rem}.layout{display:grid;grid-template-columns:minmax(16rem,22rem) minmax(0,1fr);min-height:100vh}nav{border-right:1px solid var(--line);padding:1.25rem;overflow:auto;max-height:100vh;position:sticky;top:0}.brand{font-size:1.4rem;font-weight:800;text-decoration:none}nav summary{cursor:pointer;font-weight:700;margin-top:1rem}nav ul{list-style:none;padding-left:.75rem}nav li{margin:.35rem 0}nav a.active{color:#fff;font-weight:700}main{max-width:78rem;padding:2.5rem clamp(1.25rem,5vw,5rem)}h1{font-size:clamp(2.2rem,5vw,4rem);line-height:1.05}h2{margin-top:2.8rem;border-bottom:1px solid var(--line);padding-bottom:.35rem}code{background:var(--code);padding:.12rem .35rem;border-radius:.3rem}pre{background:var(--code);padding:1rem;overflow:auto;border:1px solid var(--line);border-radius:.6rem}pre code{padding:0}.table-wrap{overflow:auto;border:1px solid var(--line);border-radius:.5rem}table{border-collapse:collapse;width:100%;font-size:.92rem}th,td{text-align:left;vertical-align:top;padding:.55rem .7rem;border-bottom:1px solid var(--line)}th{position:sticky;top:0;background:var(--panel)}blockquote{margin-left:0;border-left:4px solid var(--accent);padding-left:1rem;color:var(--muted)}.search{margin:1.25rem 0;width:100%;padding:.65rem;background:var(--panel);color:var(--text);border:1px solid var(--line);border-radius:.35rem}.meta{color:var(--muted);font-size:.9rem}@media(max-width:850px){.layout{display:block}nav{position:relative;max-height:none;border-right:0;border-bottom:1px solid var(--line)}main{padding:1.5rem}}
""".strip()


SCRIPT = """
const box=document.querySelector('.search');box?.addEventListener('change',async()=>{const q=box.value.trim().toLowerCase();if(!q)return;const root=document.documentElement.dataset.root||'';const rows=await fetch(root+'search-index.json').then(r=>r.json());const hits=rows.filter(x=>(x.title+' '+x.text).toLowerCase().includes(q)).slice(0,20);const main=document.querySelector('main');main.innerHTML='<h1>Search results</h1>'+hits.map(x=>`<p><a href="${root+x.url}">${x.title}</a><br><span class="meta">${x.text}</span></p>`).join('');});
""".strip()


def page_document(page: dict[str, Any], pages: list[dict[str, Any]], body: str) -> str:
    depth = len(output_path(page["path"]).parent.parts)
    root_prefix = "../" * depth
    return f"""<!doctype html>
<html lang="en" data-root="{root_prefix}"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{html.escape(page['title'])} · PocketStation</title><meta name="description" content="Evidence-backed PocketStation documentation for {html.escape(page['title'])}."><link rel="stylesheet" href="{root_prefix}assets/site.css"></head>
<body><a class="skip" href="#content">Skip to content</a><div class="layout"><nav aria-label="Documentation">{nav_html(pages, page)}<label for="site-search">Search documentation</label><input class="search" id="site-search" type="search" placeholder="Type and press Enter"></nav><main id="content">{body}<p class="meta">Evidence snapshot: 3b7b970f6598239e5d435b60c8d132a955a1886c</p></main></div><script src="{root_prefix}assets/site.js"></script></body></html>"""


def build() -> tuple[list[dict[str, Any]], list[str]]:
    pages = read_jsonl(DB / "page-manifest.jsonl")
    if OUTPUT.exists():
        shutil.rmtree(OUTPUT)
    (OUTPUT / "assets").mkdir(parents=True)
    (OUTPUT / "assets" / "site.css").write_text(STYLE + "\n")
    (OUTPUT / "assets" / "site.js").write_text(SCRIPT + "\n")
    search: list[dict[str, str]] = []
    manifest: list[dict[str, Any]] = []
    failures: list[str] = []
    for page in pages:
        source = ROOT / page["path"]
        if not source.exists():
            failures.append(f"missing source: {page['path']}")
            continue
        markdown = source.read_text()
        if "Native description pending Gate" in markdown:
            failures.append(f"stale gate marker: {page['path']}")
        rendered = page_document(page, pages, markdown_to_html(markdown, page["path"]))
        destination = OUTPUT / output_path(page["path"])
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(rendered)
        plain = re.sub(r"\s+", " ", re.sub(r"[`#*|>\[\]()]", " ", CLAIM_MARKER.sub("", markdown))).strip()
        search.append({"title": page["title"], "url": output_path(page["path"]).as_posix(), "text": plain[:500]})
        manifest.append({
            "page_id": page["page_id"], "source": page["path"],
            "source_sha256": digest(source), "output": output_path(page["path"]).as_posix(),
            "output_sha256": digest(destination),
        })
    (OUTPUT / "search-index.json").write_text(json.dumps(search, indent=2, sort_keys=True) + "\n")
    (OUTPUT / "site-map.json").write_text(json.dumps([row["output"] for row in manifest], indent=2) + "\n")
    (OUTPUT / "build-manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    return manifest, failures


def validate(manifest: list[dict[str, Any]], failures: list[str]) -> list[str]:
    pages = read_jsonl(DB / "page-manifest.jsonl")
    if len(manifest) != len(pages):
        failures.append(f"built {len(manifest)} pages for {len(pages)} ledger entries")
    expected = {output_path(page["path"]).as_posix() for page in pages}
    actual = {row["output"] for row in manifest}
    for missing in sorted(expected - actual):
        failures.append(f"missing rendered page: {missing}")
    for row in manifest:
        path = OUTPUT / row["output"]
        text = path.read_text()
        if '<main id="content">' not in text or "<h1" not in text:
            failures.append(f"semantic page shell absent: {row['output']}")
        if row["output_sha256"] != digest(path):
            failures.append(f"output digest mismatch: {row['output']}")
    for asset in ("assets/site.css", "assets/site.js", "search-index.json", "site-map.json"):
        if not (OUTPUT / asset).exists():
            failures.append(f"missing publication asset: {asset}")
    return failures


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Fail unless every publication page and asset validates")
    args = parser.parse_args()
    manifest, failures = build()
    if args.check:
        failures = validate(manifest, failures)
    print(f"site_pages={len(manifest)} output={OUTPUT}")
    if failures:
        for failure in failures:
            print(f"ERROR: {failure}")
        raise SystemExit(1)


if __name__ == "__main__":
    main()
