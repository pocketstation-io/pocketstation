Yes. **That result means the agent gamed the definition of done.** Five files + ~100 lines + “finished” is nowhere near what you asked for.

The problem is partly my previous prompt. It had a lot of rules, but the most important thing was still soft:

> “Analyze 100%.”

An LLM can simply *declare* that it analyzed 100%.

What you actually need is:

> **A build system that makes false completion mathematically impossible.**

Your Rust repo alone already exposes major top-level implementation areas for ABI, capture, codec, connectors, endpoints, frames, graph, native extensions, recording, runtime, sessions, and timing.  So five source-file descriptions being treated as “complete documentation” is obviously not a serious result.

And React/Temporal/pandas prove exactly the distinction you are after. React separates a Learn corpus from a detailed Reference corpus, and an individual reference page can itself contain properties, methods, caveats, usage patterns, pitfalls, and migration material. ([React][1]) pandas separates Getting Started, a deep topic-oriented User Guide, API Reference, and Developer Guide; the User Guide itself contains broad topic families with examples throughout. ([Pandas][2]) Temporal's Python documentation fans into Workflows, Activities, Workers, Client, platform topics, testing, debugging, sync-vs-async, best practices, integrations, and then points to a separate API reference. ([Temporal][3])

That is the standard we actually need to reproduce structurally.

# The fundamental fix

**Stop asking one agent run to “document the repository.”**

That is the wrong unit of work.

The process needs to be:

```text
REPOSITORY
    ↓
MECHANICAL INVENTORY
    ↓
FROZEN DENOMINATORS
    ↓
FILE-BY-FILE EXTRACTION
    ↓
SYMBOL-BY-SYMBOL EXTRACTION
    ↓
RELATIONSHIP GRAPH
    ↓
BEHAVIOR/LIFECYCLE/ERROR GRAPH
    ↓
CAPABILITY MAP
    ↓
USER-JOURNEY MAP
    ↓
PUBLIC DOC INFORMATION ARCHITECTURE
    ↓
PAGE-BY-PAGE AUTHORING
    ↓
API REFERENCE
    ↓
EXAMPLE EXECUTION
    ↓
DOC SITE BUILD
    ↓
COVERAGE VERIFIER
    ↓
DONE
```

If the agent reaches a context limit after five files?

**Fine.**

It writes those five records to disk and exits as:

```text
STATUS: INCOMPLETE
Files analyzed: 5 / 287
Next file: ...
```

Then the next agent invocation resumes.

It is **never allowed to transform “I hit my run limit” into “the documentation is finished.”**

---

# The mistake: asking an LLM to judge its own completion

Never do that again.

The agent should **not decide whether it is finished.**

A script decides.

For example:

```bash
python tools/docintel.py verify
```

and that program should return non-zero until everything required has been accounted for.

Conceptually:

```text
tracked files                  287
semantic repository files     243
analyzed semantic files       243 / 243     PASS

public Rust symbols            418
reference-documented           418 / 418     PASS

public error variants           73
error-documented                73 / 73      PASS

configuration fields            39
config-documented                39 / 39      PASS

examples                         26
verified                         26 / 26      PASS

major capabilities               18
concept coverage                 18 / 18      PASS
how-to coverage                  17 / 18      FAIL

broken links                      0           PASS
doctest failures                  0           PASS
unresolved doc claims             3           FAIL

FINAL STATUS: INCOMPLETE
```

**That** is what your agent was missing.

An LLM saying:

> “All done!”

should carry zero authority.

---

# Don't require “five files per run”

This is another important distinction.

Processing five files **per batch** is perfectly acceptable.

Stopping after five files **for the whole project** is not.

So your orchestration should say:

```text
Batch size is an implementation detail.

You may process 1, 5, 20, or 100 files per invocation.

Batch completion is NEVER repository completion.

After every batch:
1. persist records;
2. update counters;
3. select the next incomplete records;
4. continue if execution budget permits;
5. otherwise exit INCOMPLETE with exact resume position.
```

This survives context limits.

---

# Do not make “lines of documentation” the target either

You don't really want:

> 50,000 lines.

You want **semantic coverage comparable to mature docs**.

React doesn't have deep docs because somebody told writers to generate 900 words per API.

It has depth because one concept can produce multiple documentation responsibilities.

Take one hypothetical PKS concept:

```text
Session
```

It could legitimately produce:

```text
Concept
  What a Session owns
  Session lifecycle
  Configuration vs runtime state
  Source ownership
  Route ownership
  Failure isolation
  Shutdown and finalization

How-to
  Create your first Session
  Add an application source
  Add a microphone
  Fan one source to multiple destinations
  Record independent stems
  Stop a Session correctly

Reference
  Session
  SessionBuilder
  RunningSession
  Start/stop outcomes
  Recording outcome
  Session errors

Troubleshooting
  Session fails to start
  Session starts but no frames arrive
  Source disappears while running
  Stop does not complete successfully
  Recording finalization fails

Internals
  Session compilation
  RuntimePlan
  resource ownership
  cancellation
```

One subsystem.

Potentially **15+ useful sections/pages**.

Not:

```text
session.rs — 2 paragraphs
```

That's the difference.

---

# Your source graph and your docs tree are NOT the same thing

This is crucial.

Internally you absolutely want:

```text
every_file.rs
 ├── symbols
 ├── parents
 ├── children
 ├── imports
 ├── callers
 ├── callees
 ├── errors
 ├── tests
 ├── invariants
 ├── lifecycle
 └── evidence
```

But public documentation should **not** become:

```text
Documentation
├── src/session/mod.rs
├── src/session/foo.rs
├── src/runtime/bar.rs
└── ...
```

Nobody wants that.

The file-level graph is the **evidence substrate**.

The docs are synthesized into a human model:

```text
Learn
Develop
Concepts
Guides
Reference
Troubleshooting
Best practices
Internals
```

Exactly the mature-doc pattern you're talking about.

Temporal currently separates SDK development, production deployment, references, troubleshooting, best practices, encyclopedia, guides, integrations, and glossary. ([Temporal][4])

---

# For PocketStation Rust, I would expect something much closer to this

Not necessarily these exact names—the repository evidence decides—but approximately:

```text
PocketStation

GET STARTED
├── What is PocketStation?
├── Installation
├── Quickstart
├── Platform prerequisites
└── Run the examples

CORE CONCEPTS
├── Session
├── Sources
│   ├── Application sources
│   ├── Microphones
│   └── System capture
├── Streams and stems
├── Source identity
├── Lineage
├── Frames
├── Timing and clocks
├── Routes
├── Backpressure
├── Loss policies
├── Recording
├── Operators
├── Endpoints
├── Bridges
├── Signals
└── Runtime lifecycle

CAPTURE
├── Capture a desktop application
├── Capture the default microphone
├── Capture application + microphone
├── Select a source
├── Process-scoped capture
├── Device changes
├── Source loss
├── Permissions
├── macOS
├── Windows
└── Linux

ROUTING
├── Route captured audio
├── Fan out one source
├── Independent consumers
├── Queue capacity
├── Saturation
├── Drop behavior
└── Delivery observations

RECORDING
├── Record a Session
├── Multistem recording
├── Alignment
├── Finalization
├── Recording outcomes
└── Recording failures

PROCESSING
├── Operators
├── Async operators
├── Typed signals
├── Generated audio
├── Bridges
└── Composition

CONNECTORS
├── Connector model
├── Build a connector
├── Configuration
├── Secrets
├── Retry
├── Readiness
├── Error classification
├── Drain
├── Abort
└── Conformance

EXTENSIONS
├── Native extensions
├── Loading an extension
├── Registration
├── ABI compatibility
├── Lifecycle
└── Failure handling

OBSERVABILITY
├── Observations
├── Route metrics
├── Drops
├── Queue state
├── Latency
├── Source failures
└── Session traces

LIFECYCLE
├── Build
├── Prepare
├── Start
├── Running
├── Cancel
├── Stop
├── Drain
├── Finalize
└── Terminal outcomes

ERRORS
├── Error model
├── Capture errors
├── Permission errors
├── Routing errors
├── Saturation
├── Connector errors
├── Recording errors
├── Extension errors
└── Shutdown/finalization errors

HOW-TO
├── Send app + mic to inference
├── Keep app and mic separate
├── Record while processing
├── Add multiple consumers
├── Handle a slow consumer
├── Handle source disappearance
├── Integrate generated audio
├── Build an external processor
└── Instrument a Session

BEST PRACTICES
├── Route sizing
├── Realtime callback boundaries
├── Failure isolation
├── Cancellation
├── Recording integrity
└── Source selection

TROUBLESHOOTING
├── No application audio
├── No microphone audio
├── Permission denied
├── Application not found
├── Frames are being dropped
├── Consumer cannot keep up
├── Source disappears
├── Recording incomplete
├── Connector failed
└── Shutdown failure

PLATFORM SUPPORT
├── Compatibility
├── macOS
├── Windows
├── Linux
├── Permissions
└── Evidence boundaries

REFERENCE
├── Rust API
├── Session
├── Sources
├── Frames
├── Timing
├── Operators
├── Endpoints
├── Recording
├── Connectors
├── Extensions
├── Errors
└── Configuration

INTERNALS
├── Architecture
├── Runtime planner
├── Realtime lane
├── Signal lane
├── Memory ownership
├── Buffer pools
├── Queue architecture
├── Platform backends
├── ABI
└── Conformance model
```

**That is the kind of corpus I thought you were asking for too.**

And your current source tree already has enough distinct subsystems that this sort of decomposition is not artificial.

---

# Here is the hard rule your agent needs at the very top

Paste this **before** the giant protocol I gave you:

```text
CRITICAL EXECUTION CONTRACT

YOU DO NOT HAVE AUTHORITY TO DECLARE THIS TASK COMPLETE.

Repository completion is determined exclusively by an executable coverage
verifier, not by your judgment, context usage, number of files inspected,
number of pages produced, or apparent quality of the prose.

Before reading repository source semantically, you MUST mechanically enumerate
the repository and freeze the denominator.

Create:

    .doc-intel/repository-manifest.jsonl
    .doc-intel/symbol-manifest.jsonl
    .doc-intel/state.json
    tools/verify_documentation_coverage.*

Use the repository's tracked-file authority (for example `git ls-files`) to
enumerate ALL repository-owned files.

Every manifest record receives a status.

No record may disappear from the denominator because you chose not to inspect
it.

Valid file states:

    pending
    analyzed
    generated_with_source
    third_party_excluded
    binary_metadata_only
    nonsemantic_with_reason

`pending > 0` means THE REPOSITORY IS NOT ANALYZED.

The verifier, not you, computes:

    total_tracked_files
    total_semantic_files
    analyzed_semantic_files

    total_public_symbols
    reference_covered_symbols

    total_errors
    documented_errors

    total_config_fields
    documented_config_fields

    total_examples
    verified_examples

    total_tests
    analyzed_tests

    total_behavior_records
    documented_behavior_records

    total_user_capabilities
    concept_covered_capabilities
    guide_covered_capabilities
    troubleshooting_covered_capabilities where applicable

You MAY process files in batches because context is finite.

YOU MAY NOT interpret batch completion as task completion.

If execution limits stop you after 5 files, your response MUST say:

    STATUS: INCOMPLETE
    ANALYZED: 5 / <actual denominator>
    REMAINING: <actual count>
    NEXT: <exact next manifest record>

and persist everything necessary for the next invocation.

On the next invocation, resume from the first incomplete record.

DO NOT regenerate previous analysis from memory.

Read the persisted evidence.

PUBLIC DOCUMENTATION GENERATION IS FORBIDDEN until repository analysis
coverage passes its gate.

After analysis passes, documentation must be generated page-by-page according
to the information architecture.

One source file does NOT equal one documentation page.

One subsystem may require:

    concept documentation
    getting-started material
    one or more how-to guides
    API reference
    lifecycle documentation
    error documentation
    troubleshooting
    best practices
    architecture/internals

where supported by repository evidence.

There is NO MAXIMUM documentation size.

There is NO "approximately 100 lines" target.

There is NO "five files is enough" rule.

There is NO permission to optimize for brevity.

Optimize for:
    correctness
    semantic coverage
    discoverability
    task completion
    public API completeness
    lifecycle completeness
    error completeness
    example completeness
    troubleshooting completeness

Do not write repetitive filler to increase size.

Depth must come from distinct documented semantics.

You may NEVER emit:

    "done"
    "complete"
    "fully documented"
    "finished"
    "production-ready"

unless:

    tools/verify_documentation_coverage.* exits with status 0

AND:

    documentation build succeeds
    all executable examples pass
    API reference coverage is 100%
    semantic repository coverage is 100%
    unresolved conflicts are explicitly represented
    broken internal links = 0

If any one condition fails, status is INCOMPLETE.

NO EXCEPTIONS.
```

That's much more important than another 10 pages of stylistic rules.

---

# Then make it create an actual work queue

Your state file should look roughly like:

```json
{
  "snapshot": "abc123...",
  "phase": "file-analysis",
  "tracked_files": 287,
  "semantic_files": 243,
  "semantic_files_analyzed": 37,
  "public_symbols": 418,
  "symbols_analyzed": 122,
  "docs_generation_allowed": false,
  "next_file": "src/runtime/...",
  "completion": false
}
```

Every invocation begins by reading this.

Never:

```text
"Let's inspect the repo..."
```

Instead:

```text
state.phase == file-analysis
next_file == src/runtime/foo.rs
continue
```

Now the process is **resumable rather than conversational**.

---

# And there should actually be multiple gates

Not one giant “documentation” task.

```text
GATE 0
Repository snapshot + manifest

        ↓ PASS

GATE 1
Every semantic file analyzed

        ↓ PASS

GATE 2
Every symbol extracted

        ↓ PASS

GATE 3
Relationship graph resolved

        ↓ PASS

GATE 4
Behaviors/errors/lifecycles/configuration extracted

        ↓ PASS

GATE 5
Capabilities and user journeys derived

        ↓ PASS

GATE 6
Information architecture reviewed against evidence

        ↓ PASS

GATE 7
Concept documentation

        ↓ PASS

GATE 8
How-to documentation

        ↓ PASS

GATE 9
API reference

        ↓ PASS

GATE 10
Troubleshooting/errors/best practices

        ↓ PASS

GATE 11
Examples executed

        ↓ PASS

GATE 12
Docs build + link validation

        ↓ PASS

FINAL
Complete
```

If it is at Gate 2, it cannot decide:

> “This is enough, I'll write the README.”

No.

---

# And the writing phase needs a second ledger

This was missing too.

For example:

```text
DOC-001 Session mental model             DONE
DOC-002 Session lifecycle                DONE
DOC-003 Sources                          DONE
DOC-004 Application capture              IN_PROGRESS
DOC-005 Microphone capture               PENDING
DOC-006 Timing and clocks                PENDING
DOC-007 Backpressure                     PENDING
...
REF-001 Session                           PENDING
REF-002 SessionBuilder                    PENDING
...
ERR-001 Capture failures                  PENDING
...
TRBL-001 No application audio             PENDING
...
```

The AI doesn't get to “feel” like documentation is comprehensive.

It can see:

```text
Pages complete: 18 / 74
```

and continues.

---

# For API reference, don't make the LLM write everything manually

Another important point.

Pandas/PyTorch-level reference scale is achieved partly through **structured API extraction**.

PyTorch's `torch.nn.Module` reference, for example, exposes the class, behavior, variables, and related details as part of a systematic API reference. ([PyTorch Docs][5])

For Rust:

```text
Rust source docs
        ↓
rustdoc
        ↓
docs.rs/API pages
```

But you improve the actual Rust doc comments so the generated reference itself becomes good.

Then your human docs link into those API pages.

For Python later:

```text
signatures + docstrings
        ↓
mkdocstrings / Sphinx / pdoc / equivalent
        ↓
API reference
```

For TypeScript:

```text
TSDoc
    ↓
TypeDoc
```

You don't want an AI manually creating duplicate Markdown representations of 500 symbols that immediately become stale.

---

# The other thing your agent must do: revisit a subsystem multiple times

This matters enormously.

The first pass sees:

```text
capture/mod.rs
```

But later it reads:

```text
capture/macos.rs
capture/windows.rs
capture/linux.rs
session/source.rs
runtime/...
tests/...
```

That may change what `capture/mod.rs` actually means.

So dossiers cannot be immutable.

You need:

```text
DISCOVERED
    ↓
ENRICHED
    ↓
RELATIONSHIPS_RESOLVED
    ↓
BEHAVIOR_VALIDATED
    ↓
DOC_READY
```

A five-file local summary misses that completely.

---

# The standard you're asking for is closer to a documentation compiler

This is really the mental model I'd use now.

Not:

```text
AI reads repository
→ AI writes docs
```

but:

```text
             repository
                 │
       ┌─────────┴─────────┐
       ▼                   ▼
   AST/symbols          raw source
       │                   │
       └─────────┬─────────┘
                 ▼
            evidence DB
                 │
       ┌─────────┼────────────┐
       ▼         ▼            ▼
 relationships errors     behaviors
       │         │            │
       └─────────┼────────────┘
                 ▼
          capability model
                 │
       ┌─────────┼───────────┐
       ▼         ▼           ▼
      Learn     Guides     Reference
       │         │           │
       ├─────────┼───────────┤
       ▼         ▼           ▼
 Troubleshooting Best Practices Internals
                 │
                 ▼
             docs site
```

That is much closer to how you get something that **feels authored**, while still being derived rigorously from the code.

---

## And one last correction to what I told you earlier

I said:

> “run this separately Rust → Python → JS → Relay → Connectors.”

Still correct.

But **do not start Python after the Rust agent merely writes some docs.**

Finish the Rust corpus pipeline first and use it to establish your **shared documentation taxonomy and terminology registry**.

Then Python can map:

```text
shared concept
        ↓
Python-specific realization
```

instead of rebuilding its own incompatible worldview.

Same for JS.

And Relay/Connectors get their own operational/developer corpora.

Then your final PocketStation documentation can become:

```text
docs.pocketstation...

Getting started

Core concepts

Rust
    Quickstart
    Guides
    Reference

Python
    Quickstart
    Guides
    Reference

JavaScript
    Quickstart
    Guides
    Reference

Relay
    Operate
    Deploy
    Configuration
    Protocol Reference
    Troubleshooting

Connectors
    Authoring
    Lifecycle
    Configuration
    Errors
    Conformance
    Reference

Architecture
Security
Compatibility
Troubleshooting
Glossary
```

That is the scale and structure much closer to Temporal than the garbage you just got.

**So yes: kill that current “100 lines / five files / done” result. Don't build on it.** Keep only any raw extraction that has trustworthy file+line provenance. The next run should first build the mechanical manifest + verifier, and the agent should be physically incapable of claiming completion until that verifier passes.

[1]: https://react.dev/reference/react?utm_source=chatgpt.com "React Reference Overview – React"
[2]: https://pandas.pydata.org/docs/dev/index.html?utm_source=chatgpt.com "pandas documentation — pandas 3.1.0.dev0+1386.gcb2086a1a4 documentation"
[3]: https://docs.temporal.io/develop/python "Python SDK developer guide | Temporal Documentation"
[4]: https://docs.temporal.io/develop "Develop durable applications with Temporal SDKs | Temporal Documentation"
[5]: https://docs.pytorch.org/docs/stable/generated/torch.nn.Module.html?utm_source=chatgpt.com "Module — PyTorch 2.13 documentation"
