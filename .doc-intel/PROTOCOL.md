# Repository Documentation Intelligence and Publication Protocol

## Mission

You are responsible for producing production-grade technical documentation for the repository provided to you.

This is **not a summarization task**.

Do not read a subset of the repository, compress it into an architectural narrative, and then write documentation from memory.

Your job has two strictly separated stages:

1. **Repository intelligence and evidence construction**
2. **Documentation generation from verified evidence**

Stage 2 is forbidden until Stage 1 satisfies every required coverage gate.

The repository itself is the primary authority. Existing documentation, comments, tests, examples, configuration, generated declarations, and manifests are evidence, but they may disagree. Never silently reconcile conflicting evidence.

The finished result must be usable by a developer who has never spoken with the repository authors and has never read the source code.

---

# 1. Core operating principle

Treat your context window as temporary working memory.

Do **not** attempt to keep the entire repository in model context.

Persist your understanding as structured repository artifacts as you work.

For every file, symbol, relationship, runtime behavior, error path, configuration option, lifecycle operation, test, example, and public documentation claim, preserve enough evidence to reconstruct how you reached the conclusion.

At all times, maintain:

```text
source code
    ↓
file records
    ↓
symbol records
    ↓
relationship graph
    ↓
behavior and lifecycle records
    ↓
validated claims
    ↓
documentation pages
```

The reverse direction must also work:

```text
documentation statement
    ↓
claim ID
    ↓
behavior/symbol record
    ↓
source file + symbol + line range + content hash
```

No important public documentation statement may exist without this reverse provenance.

---

# 2. Absolute anti-hallucination rules

These rules are mandatory.

1. Never document behavior solely because it is conventional, expected, likely, implied by a name, or similar to another library.

2. Never convert an inference into a fact.

3. Classify knowledge as:

   * `DIRECT`: directly visible in code, schema, manifest, or public signature.
   * `TESTED`: demonstrated by an executable test or example.
   * `DECLARED`: stated in an authoritative contract or source comment.
   * `INFERRED`: follows from multiple pieces of evidence but is not directly declared.
   * `UNKNOWN`: insufficient evidence.
   * `CONFLICTED`: authoritative evidence disagrees.

4. Public documentation may use `DIRECT`, `TESTED`, and appropriate `DECLARED` facts.

5. `INFERRED` facts must either:

   * be validated before publication, or
   * remain explicitly qualified in internal engineering documentation.

6. `UNKNOWN` facts must never be filled in creatively.

7. `CONFLICTED` facts must be recorded in a documentation finding. Do not silently choose one side.

8. Never invent:

   * supported platforms;
   * guarantees;
   * latency claims;
   * performance characteristics;
   * retry behavior;
   * thread safety;
   * ordering guarantees;
   * delivery guarantees;
   * memory behavior;
   * security behavior;
   * permissions behavior;
   * error recovery;
   * default values;
   * configuration precedence;
   * API stability;
   * backwards compatibility.

9. A source comment does not automatically prove that an implementation satisfies the comment. Preserve both the declared contract and implemented evidence.

10. A test proves only the behavior exercised under the conditions of that test.

11. An implementation existing for an operating system does not prove that the feature has been physically qualified on that operating system.

12. Never change runtime semantics merely to make documentation easier to write.

---

# 3. Establish the repository snapshot

Before semantic analysis, record the repository state.

Capture:

```text
repository name
absolute repository path
current branch
HEAD commit SHA
git status
tracked files
relevant untracked files
submodules
workspace members
package members
repository language(s)
build system(s)
package manager(s)
CI definitions
release configuration
documentation framework if present
```

Generate stable content hashes for every repository-owned file.

Use the Git snapshot and file hashes in all evidence records.

If the working tree contains modifications, distinguish:

```text
HEAD implementation
working-tree implementation
```

Do not conflate them.

---

# 4. Define “all files” correctly

Build the canonical inventory from repository-owned files.

Every repository-owned textual file must be opened and inspected.

This includes, where present:

```text
source
tests
integration tests
examples
benchmarks
build scripts
package manifests
workspace manifests
feature declarations
CI
Dockerfiles
deployment manifests
configuration
schemas
protocol definitions
FFI headers
generated-interface definitions
scripts
documentation
ADRs
examples
fixtures with semantic significance
security configuration
release automation
lint configuration
```

Do not waste semantic analysis on external dependency trees such as:

```text
.git/
target/
node_modules/
.venv/
vendor/     when truly third-party vendored content
dist/       when completely generated
build/      when completely generated
```

However, record that these paths exist and why they were excluded.

Lockfiles are part of the inventory. Extract dependency/version information when useful, but do not pretend a lockfile contains application semantics that it does not.

Binary files must be inventoried with type, size, path, hash, and known purpose. Do not hallucinate binary contents.

Coverage is not complete until every inventory entry has one of these states:

```text
SEMANTICALLY_ANALYZED
METADATA_ONLY_WITH_JUSTIFICATION
GENERATED_WITH_SOURCE_IDENTIFIED
THIRD_PARTY_WITH_PROVENANCE
BINARY_WITH_METADATA
```

No file may remain merely `UNREAD`.

---

# 5. Create a durable intelligence workspace

Create a working directory such as:

```text
.doc-intel/
    snapshot.json
    inventory.jsonl
    files/
    symbols.jsonl
    edges.jsonl
    behaviors.jsonl
    errors.jsonl
    configuration.jsonl
    protocols.jsonl
    tests.jsonl
    examples.jsonl
    claims.jsonl
    terminology.json
    coverage.json
    conflicts.jsonl
    unknowns.jsonl
    doc-map.json
    checkpoints/
```

Do not use prose summaries as the only persistent representation.

Prefer records that can be queried, joined, diffed, and regenerated.

The final public documentation may exclude `.doc-intel/` from publication, but the generation process depends on it.

---

# 6. Pass 1 — Repository skeleton

Construct the complete physical and logical repository skeleton.

Identify:

```text
workspace
packages/crates/modules
libraries
binaries
entry points
public packages
private packages
generated bindings
FFI boundaries
protocol boundaries
services
CLI programs
examples
tests
deployment units
configuration domains
```

For every directory, establish its responsibility from evidence.

For every package/crate/module, identify:

```text
what it owns
what owns it
its public boundary
its private boundary
its dependencies
its dependents
its runtime role
its build role
its deployment role
```

Do not yet write user documentation.

---

# 7. Pass 2 — File dossiers

Create one complete dossier for every repository-owned semantic file.

Each dossier must include, where applicable:

```yaml
file_id:
path:
content_hash:
language:
file_kind:
line_count:
package:
module:
visibility:
generated:
generated_from:
platform_gate:
feature_gate:

purpose:
responsibilities:
non_responsibilities:

defines:
imports:
imported_by:
reexports:
calls:
called_by:
constructs:
constructed_by:

implements:
implemented_by:
extends:
extended_by:

entry_points:
public_surface:
private_surface:

inputs:
outputs:
side_effects:

filesystem_io:
network_io:
device_io:
process_io:
ffi_io:

threads:
tasks:
async_boundaries:
queues:
channels:
locks:
atomics:
callbacks:

resource_ownership:
startup_behavior:
shutdown_behavior:
cancellation_behavior:
drop_cleanup:
finalization:

state_machine:
invariants:

errors_defined:
errors_created:
errors_wrapped:
errors_propagated:
errors_translated:
retry_behavior:
recovery_behavior:

configuration_read:
environment_variables:
feature_flags:
defaults:

protocol_messages:
endpoints:
serialization:

tests_covering:
examples_using:

related_docs:

observed_patterns:
potential_pitfalls:

evidence:
  - path:
    lines:
    symbol:
    classification:
```

Do not omit fields simply because they are inconvenient. Use `not_applicable` or `unknown` when appropriate.

---

# 8. Pass 3 — Symbol registry

Use language-aware parsers, compiler metadata, ASTs, language servers, or reflection metadata where available.

Do not rely only on regex.

Also read the raw source around every symbol because an AST alone does not capture all semantics.

Create a symbol record for every relevant:

```text
crate
package
module
namespace
class
struct
interface
trait
protocol
enum
enum variant
union
type alias
generic type
function
method
constructor
destructor/drop implementation
property
field
constant
static
macro
decorator
annotation
callback
event
CLI command
CLI flag
environment variable
configuration property
HTTP endpoint
WebSocket message
RPC method
FFI function
schema object
manifest field
plugin/connector registration
```

For each symbol record, capture:

```yaml
symbol_id:
qualified_name:
kind:
source_file:
source_lines:
visibility:
signature:
parent:
children:

summary:
responsibility:
when_to_use:
when_not_to_use:

parameters:
type_parameters:
return_value:
yield_value:

preconditions:
postconditions:
side_effects:

errors:
panic_behavior:
exceptions:
safety_contract:

blocking_behavior:
async_behavior:
cancellation:
thread_safety:
ordering:
backpressure:

ownership:
lifetime:
mutability:

defaults:
valid_values:
units:
limits:

implemented_by:
implements:
overrides:
calls:
called_by:
creates:
created_by:
reads:
writes:

platforms:
feature_flags:

tests:
examples:

deprecation:
replacement:

evidence:
```

---

# 9. Language-specific extraction rules

## Rust

Use repository evidence including:

```text
Cargo.toml
Cargo.lock
workspace Cargo metadata
crate roots
module tree
pub and pub(crate) exports
reexports
features
cfg attributes
traits
trait implementations
structs
enums
error types
unsafe blocks
FFI
Drop implementations
Send/Sync implications when explicitly knowable
examples
benches
tests
doctests
```

Public Rust documentation must eventually account for all intentionally public:

```text
crates
modules
traits
structs
enums
functions
methods
macros
type definitions
important constants
```

Document applicable:

```text
Errors
Panics
Safety
Examples
platform restrictions
feature requirements
lifecycle expectations
```

Runnable Rust documentation examples must be doctested or compiled where possible.

## Python

Inspect:

```text
package exports
__init__.py
__all__
modules
classes
functions
async functions
properties
context managers
iterators
async iterators
exceptions
dataclasses
enums
typing.Protocol
type aliases
decorators
default arguments
keyword-only arguments
```

Document:

```text
arguments
return values
yield values
exceptions
side effects
async requirements
context-manager lifecycle
thread/process expectations when evidenced
```

Follow idiomatic Python docstring conventions.

## JavaScript / TypeScript

Inspect:

```text
package.json exports
entry points
ESM/CJS behavior
TypeScript declarations
interfaces
types
classes
functions
methods
properties
events
callbacks
Promises
async iterators
native bindings
browser/Node distinctions
bundler conditions
platform packages
```

Document public export paths, not merely source-file locations.

Explicitly record whether an operation:

```text
returns synchronously
returns a Promise
invokes a callback
emits an event
streams values
can reject
can throw synchronously
supports cancellation
```

Do not treat a TypeScript type as proof of runtime validation unless runtime validation exists.

## Go

Inspect:

```text
go.mod
packages
commands
exported symbols
interfaces
structs
goroutines
channels
context.Context usage
HTTP handlers
WebSocket handlers
protocol messages
configuration
environment variables
signals
shutdown paths
metrics
```

Every exported package/symbol requires appropriate documentation.

For server repositories, trace request and connection lifecycles end to end.

---

# 10. Pass 4 — Build the relationship graph

A flat symbol inventory is insufficient.

Build explicit directed edges between repository entities.

Supported edge classes must include, when applicable:

```text
CONTAINS
IMPORTS
IMPORTED_BY
EXPORTS
REEXPORTS
CALLS
CALLED_BY
IMPLEMENTS
IMPLEMENTED_BY
EXTENDS
OVERRIDES
CONSTRUCTS
OWNS
BORROWS
READS
WRITES
SENDS
RECEIVES
ENCODES
DECODES
REGISTERS
LOADS
SPAWNS
JOINS
CANCELS
RETRIES
WRAPS_ERROR
MAPS_ERROR
EMITS
OBSERVES
CONFIGURES
GATED_BY
TESTED_BY
EXEMPLIFIED_BY
DOCUMENTED_BY
```

Resolve relationships to stable symbol IDs whenever possible.

Do not store merely `"calls": "start"` when there are multiple possible `start` symbols.

For dynamic dispatch, plugin loading, reflection, FFI callbacks, registries, and runtime-selected implementations, use explicit dynamic relationship types and preserve the mechanism that creates the relationship.

---

# 11. Pass 5 — Runtime lineage

Static structure is not enough.

Trace important execution paths end to end.

For every externally meaningful operation, determine:

```text
entry point
validation
configuration resolution
resource acquisition
object/session construction
runtime preparation
start
steady-state processing
queue/channel crossings
callback crossings
async boundaries
network/device/filesystem boundaries
error branches
retry branches
cancellation
source loss
shutdown
drain
finalization
cleanup
terminal outcome
```

Represent these paths as machine-readable behavior records.

Example shape:

```text
public call
→ builder/configuration
→ validation
→ runtime plan
→ implementation selection
→ resource open
→ worker/task/thread
→ bounded queue
→ consumer
→ stop request
→ drain
→ finalize
→ result
```

Do not assume this exact sequence. Discover the repository's actual sequence.

Create separate lineages for materially different platform implementations.

---

# 12. Pass 6 — State and lifecycle extraction

Find every important lifecycle and state machine.

Examples include:

```text
Created
Configured
Prepared
Started
Running
Stopping
Draining
Stopped
Failed
Finalized
Cancelled
Disconnected
Reconnecting
```

Again: use only states that exist or are directly evidenced.

For every transition, capture:

```text
source state
trigger
guard/precondition
action
side effects
destination state
possible error
recovery
idempotence
observable signal
```

Identify invalid transitions.

This information becomes the foundation for lifecycle documentation and troubleshooting.

---

# 13. Pass 7 — Error intelligence

Build a repository-wide error catalog.

Do not document errors only where error classes are defined.

Trace their full lineage.

For each error or failure condition, record:

```yaml
error_id:
type:
variant:
defined_at:
created_at:
trigger_condition:
propagates_through:
wrapped_by:
translated_to:
external_representation:

retryable:
retry_evidence:

recoverable:
recovery_action:

fatal_to_operation:
fatal_to_session:
fatal_to_process:

logged:
metric:
event:
status_code:
exit_code:

user_action:
developer_action:

tests:
evidence:
```

Distinguish:

```text
validation failure
permission failure
source unavailable
source lost
consumer saturation
timeout
network failure
protocol failure
authentication failure
authorization failure
configuration failure
resource exhaustion
cancellation
shutdown failure
finalization failure
internal invariant violation
```

Use only categories evidenced by the repository.

---

# 14. Pass 8 — Implementation strategy extraction

Identify deliberate implementation patterns without marketing them prematurely as “best practices.”

Examples may include:

```text
bounded queues
ownership transfer
copy-on-branch
buffer pools
retry budgets
cancellation tokens
RAII cleanup
transactional registration
process isolation
sidecars
feature-gated implementations
platform abstraction
typed errors
state machines
backpressure policies
clock correlation
immutable lineage
capability boundaries
```

For every observed strategy explain internally:

```text
problem it solves
where implemented
how implemented
constraints
tradeoffs
failure behavior
tests/evidence
alternative paths present in the repository
```

Label it `OBSERVED_IMPLEMENTATION_PATTERN`.

Only call something a `RECOMMENDED_PATTERN` or `BEST_PRACTICE` if the repository has evidence that it is an intentional, validated recommendation.

---

# 15. Pass 9 — Configuration and precedence

Find every source of configuration:

```text
constructor arguments
builder methods
config files
environment variables
CLI arguments
manifest properties
feature flags
compile-time flags
defaults
platform defaults
remote configuration
```

Determine exact precedence when the code makes it knowable.

Document each field with:

```text
name
type
default
required/optional
valid values
units
minimum/maximum
scope
when read
whether mutable
whether restart is required
security implications
platform restrictions
precedence
failure on invalid value
```

Never infer a default from an example if the implementation has a different default.

---

# 16. Pass 10 — Tests as behavioral evidence

Read all repository-owned tests.

For every test identify:

```text
behavior under test
setup/preconditions
inputs
expected outputs
failure expectation
platform
feature gate
timing assumption
mocked boundary
real boundary
relevant production symbols
```

Connect tests to symbols and behavior records.

A test name is not sufficient evidence. Read the test body.

Do not generalize beyond what a test proves.

---

# 17. Pass 11 — Examples as intended-use evidence

Read every example.

Determine:

```text
what user task it demonstrates
which public APIs it exercises
prerequisites
platform requirements
failure handling
whether it compiles/runs
whether it represents the preferred current API
```

Flag examples that use obsolete or private APIs.

Do not copy an example into documentation unless it is verified against the repository snapshot.

Prefer documentation snippets extracted from real, tested example files rather than duplicated manually.

---

# 18. Coverage gate before writing documentation

Documentation generation is prohibited until the intelligence stage produces a coverage report.

Minimum gate:

```text
Repository-owned files classified:               100%
Repository-owned semantic text files analyzed:   100%
Packages/modules discovered:                      100%
Public symbols inventoried:                       100%
Public configuration surface inventoried:        100%
Public error surface inventoried:                 100%
Public protocol/API surface inventoried:          100%
Examples analyzed:                                100%
Tests analyzed/classified:                        100%
Dynamic/FFI/plugin boundaries identified:         100%
Unresolved evidence conflicts:                    explicitly listed
Unknown public behavior:                          explicitly listed
```

`Unknown` does not mean failure.

Silent unknowns are failure.

Provide the coverage report before beginning documentation.

---

# 19. Documentation architecture

Once the evidence gate passes, design documentation for people rather than for the filesystem.

The all-files intelligence graph remains the source of truth but must **not become the primary navigation of the public documentation site**.

Build these documentation classes as appropriate to the repository:

```text
Home / overview
Quickstart
Getting started
Concepts
Developer guides
How-to guides
API reference
Configuration reference
Protocol reference
Error/failure reference
Troubleshooting
Best practices
Integrations
Compatibility
Deployment / operations
Security
Architecture / internals
Glossary
Release/version information
```

Not every repository needs every category. Do not create empty taxonomy.

---

# 20. The two documentation products

Produce two distinct documentation layers.

## A. Source/API documentation

This is exhaustive.

Document public:

```text
packages
modules
classes
interfaces
traits
structs
enums
variants
functions
methods
constructors
properties
fields
configuration
errors
protocol types
endpoints
CLI commands
```

Use the native ecosystem tooling:

```text
rustdoc / docs.rs for Rust
Python docstrings plus an API renderer for Python
TSDoc / TypeDoc or equivalent for TypeScript
Go doc / pkg.go.dev style documentation for Go
```

## B. Human developer documentation

This is task- and concept-centered.

It answers questions such as:

```text
What is this?
Why would I use it?
How do I install it?
How do I perform my first useful operation?
How do I select/configure X?
What happens when Y fails?
What guarantees exist?
What does not have a guarantee?
How do I observe the system?
How do I shut it down correctly?
How do I deploy it?
How do I debug it?
What should I use in production?
Where is the precise API reference?
```

Never replace layer B with generated API pages.

Never replace layer A with prose guides.

---

# 21. Required writing style

Use a people-first technical style inspired by the strongest qualities of Temporal, Google developer documentation, GitHub Docs, Microsoft developer documentation, and language-native documentation conventions.

Do not imitate sentences or branding from those sites.

Apply these rules.

### Voice

Write directly to the developer.

Prefer:

```text
Create a `Session`.
Call `start()` after you configure all routes.
The method returns...
PocketStation rejects...
The route drops...
```

Avoid:

```text
One may...
The user should...
It can be seen that...
This class basically...
This functionality allows for...
PocketStation handles things...
```

Use second person for instructions.

Use active voice unless passive voice materially improves clarity.

### Precision

Use the exact code term whenever one exists.

Do not substitute vague synonyms.

Do not say:

```text
thing
stuff
data handling
manages
processes
takes care of
works with
handles errors
handles audio
```

when an exact verb exists.

Prefer verbs such as:

```text
captures
opens
validates
registers
allocates
queues
drops
copies
borrows
encodes
decodes
retries
cancels
drains
finalizes
emits
records
returns
rejects
reports
closes
```

### Modality

Use:

```text
must     = required for correctness
can      = capability exists
may      = permitted/possible where appropriate
will     = deterministic future behavior only when guaranteed
```

Avoid ambiguous `should` when the distinction between requirement and recommendation matters.

Use:

```text
We recommend...
```

for a recommendation.

### Structure

Start every page by making its purpose obvious.

Do not begin with project history.

Do not begin with internal architecture unless the page is an architecture page.

Put prerequisites before steps.

Put conditions before the instruction they constrain.

Use numbered procedures for ordered tasks.

Keep one conceptual responsibility per section.

Use sentence-case headings.

Define specialized terminology before relying on it.

Avoid unexplained acronyms.

Avoid idioms, jokes, regional expressions, hype, and unnecessary adjectives.

Avoid:

```text
easy
simple
obvious
trivial
magic
blazing fast
seamless
powerful
revolutionary
enterprise-grade
production-ready
```

unless a statement is objectively defined and evidenced.

---

# 22. Exact description patterns by symbol kind

Do not mechanically start every description with “This X does...”.

Use the semantic role of the symbol.

## Package / module

First sentence:

```text
Provides <capability> for <purpose/context>.
```

Then explain:

```text
what belongs here
public entry points
important lifecycle
relationship to neighboring modules
```

## Class / struct

Describe what an instance represents or owns.

Good pattern:

```text
Represents a running capture session and owns the resources required to stop and finalize it.
```

Not:

```text
This class is used for sessions.
```

## Interface / trait / protocol

Describe the contract.

Pattern:

```text
Defines the operations a capture backend must provide to prepare, start, stop, and release a source.
```

State:

```text
who implements it
who calls it
lifecycle expectations
threading/cancellation rules
failure contract
```

## Function / method

Begin with the observable operation.

Examples:

```text
Returns...
Creates...
Starts...
Stops...
Registers...
Removes...
Reports whether...
Attempts to...
Converts...
Writes...
Waits for...
```

Then document:

```text
parameters
return value
side effects
preconditions
errors
cancellation
blocking/async behavior
special cases
```

## Property / field

State what the value represents.

Always document applicable:

```text
units
valid range
default
mutability
ownership
whether optional
meaning of absence
when read
```

## Boolean

Use wording such as:

```text
Indicates whether...
Reports whether...
```

Make the meaning of both `true` and `false` unambiguous when necessary.

## Enum

Explain the decision space represented by the enum.

Document every variant and exactly when it occurs.

## Error

Use:

```text
Returned when...
Raised when...
Reported when...
```

Then explain:

```text
cause
scope of failure
retryability
recovery
observable consequences
```

## Configuration property

Use:

```text
Sets...
Controls...
Limits...
Selects...
```

Then document:

```text
type
default
valid values
units
precedence
reload/restart behavior
invalid-input behavior
```

## Event

Use:

```text
Emitted when...
```

Document:

```text
trigger
payload
ordering
frequency
delivery semantics
```

## Endpoint / RPC

Document:

```text
purpose
method/path/name
authentication
authorization
request
response
status/error mapping
idempotency
retry semantics
limits
examples
```

## CLI command

Document:

```text
purpose
syntax
arguments
options
environment dependencies
output
exit codes
examples
failure cases
```

---

# 23. Quickstart standard

A quickstart must prove that a stranger can reach useful behavior without reading implementation source.

It must contain:

```text
audience
prerequisites
supported environment
installation
smallest useful program
how to run it
what success looks like
common first-run failure
next steps
```

Every command must be executable.

Every code sample must come from, or be synchronized with, tested repository code wherever feasible.

A quickstart must not introduce every architecture concept.

Its purpose is first success.

---

# 24. Concept page standard

A concept page explains one coherent idea.

It must answer:

```text
What is it?
Why does it exist?
How does it relate to nearby concepts?
What invariant or behavior matters to the developer?
When would the developer encounter it?
Where do they go to use it?
```

Do not turn concept pages into API encyclopedias.

---

# 25. How-to guide standard

A how-to guide solves one concrete task.

Use a task-oriented title.

State prerequisites.

Provide the preferred path first.

Explain important consequences next to the relevant action.

End with a verifiable outcome.

Link to reference material rather than duplicating the entire reference.

---

# 26. Reference standard

Reference documentation must favor completeness and predictability over narrative.

For every public API element document applicable:

```text
signature
purpose
parameters
type parameters
return/yield
errors
panics
safety
defaults
side effects
lifecycle
thread safety
blocking/async behavior
cancellation
ordering
limits
platform restrictions
feature requirements
deprecation
examples
related APIs
```

Use consistent headings across equivalent reference pages.

---

# 27. Troubleshooting standard

Do not make a generic FAQ.

Organize troubleshooting around observable symptoms.

For each problem document:

```text
symptom
likely evidenced causes
how to distinguish the causes
diagnostic signal
corrective action
whether retry is safe
whether data/state may be incomplete
related error/reference
```

Never invent a fix because it sounds plausible.

---

# 28. Best-practice standard

Best-practice documentation is prescriptive.

It must not simply describe implementation.

For every recommendation establish:

```text
problem
recommended action
reason
tradeoff
cases where recommendation does not apply
evidence or contract supporting it
```

Separate best practices from conceptual explanation.

---

# 29. Error and lifecycle documentation

Lifecycle and failure behavior are first-class documentation.

Users must be able to answer:

```text
What can fail before start?
What can fail while running?
What happens to other consumers when one consumer fails?
What happens during cancellation?
What happens during shutdown?
What is drained?
What is discarded?
What is finalized?
What constitutes successful completion?
What state is safe to retry from?
```

Only include questions relevant to the repository.

---

# 30. Examples are executable product surface

Every public example must be treated as code, not decorative prose.

Validate examples in CI where feasible.

For Rust, prefer doctests and compiling examples.

For Python, execute example tests in an isolated environment.

For TypeScript/JavaScript, type-check and execute applicable examples.

For Go, compile or test examples.

Do not publish pseudocode as executable code.

If pseudocode is necessary, label it explicitly.

---

# 31. Documentation claim ledger

Every generated page must have hidden or machine-readable provenance.

For each meaningful technical claim, maintain:

```yaml
claim_id:
documentation_page:
section:
claim_summary:

evidence:
  - file:
    content_hash:
    lines:
    symbol:
    evidence_class:

last_verified_commit:
status:
```

Do not necessarily show source-line citations to end users because line numbers are unstable.

The provenance ledger exists so maintainers and future agents can prove and refresh every statement.

---

# 32. Bidirectional documentation coverage

Build both mappings:

```text
public symbol → documentation pages
documentation claim → source evidence
```

Report:

```text
undocumented public symbols
symbols documented only by generated reference
symbols lacking examples
symbols lacking failure documentation
documentation pages referencing deleted symbols
claims whose source hash changed
orphan documentation pages
orphan glossary terms
```

A future source change must make it possible to identify which documentation may now be stale.

---

# 33. Terminology authority

Generate a terminology registry before writing prose.

For every important term record:

```text
canonical name
code spelling
human spelling
definition
aliases
forbidden/obsolete aliases
first defining symbol
related concepts
```

Use one term consistently.

If the repository uses multiple terms for the same concept, record the conflict rather than arbitrarily choosing one.

Once a canonical public term is established, enforce it across documentation.

---

# 34. Documentation structure for SDK repositories

For a language SDK, prefer a structure resembling:

```text
Overview
Quickstart

Develop
  Capture / sources / primary primitives
  Routing / delivery
  Recording
  Processing / operators
  Lifecycle
  Observability
  Errors and cancellation

Guides
  concrete user tasks

Best practices
  validated recommendations

Integrations

Reference
  API reference
  configuration
  errors
  compatibility

Troubleshooting

Glossary
```

Adapt those names to the actual repository semantics.

Do not create concepts the repository does not contain.

---

# 35. PocketStation Rust profile

When this protocol is applied to the PocketStation Rust repository, pay special attention to discovering and documenting, if actually present:

```text
Session lifecycle
source selection
application capture
microphone capture
system capture
source/stem identity
lineage
clock/timestamp semantics
routing
bounded queues
backpressure
drop behavior
recording
multistem behavior
operators
bridges
endpoints
extensions
C ABI
sidecars
permissions
platform implementations
platform qualification boundaries
observations/metrics
cancellation
shutdown
finalization
errors
```

Do not treat this list as proof that any capability exists.

It is an investigation checklist only.

Keep implementation evidence distinct from qualification evidence.

---

# 36. PocketStation Python profile

When applied to the Python SDK, establish first what the package actually binds.

Do not assume architectural parity with Rust.

Determine from code:

```text
what runtime authority Python controls
what is native
what is remote
what is local
whether capture exists
whether Session semantics exist
how native libraries are loaded
async behavior
thread behavior
callbacks/iterators
exceptions
resource ownership
shutdown
package installation
supported platforms
```

Then align terminology with Rust only where the underlying contracts are actually equivalent.

A shared brand must not cause you to pretend two different APIs have identical semantics.

---

# 37. PocketStation JavaScript / TypeScript profile

Determine:

```text
actual package exports
Node versions
ESM/CJS behavior
native binding mechanism
binary distribution
platform artifacts
Session ownership
callbacks/events/async iterators
Promise rejection
synchronous exceptions
Buffer/TypedArray ownership
copying versus borrowed memory
worker-thread behavior
shutdown
native resource cleanup
Electron/Tauri/Node boundaries if supported
```

Do not promise Electron, browsers, Bun, Deno, or other environments unless they are explicitly supported and evidenced.

---

# 38. PocketStation Relay profile

For a relay/server repository, the public documentation should cover the operational product, not merely Go packages.

Discover and document, if present:

```text
server startup
configuration
ports
listeners
sessions/rooms
authentication
authorization
token lifecycle
signaling
media transport
WebRTC
TURN/STUN
WebSocket
HTTP APIs
protocol messages
connection lifecycle
reconnection
limits
rate limits
timeouts
metrics
health
readiness
logging
graceful shutdown
deployment
Docker/container behavior
security boundaries
failure modes
```

Produce separate:

```text
operator guide
protocol/API reference
deployment guide
troubleshooting
configuration reference
security guidance
```

---

# 39. PocketStation Connectors profile

For a connector repository, discover and document:

```text
connector contract
authoring lifecycle
registration
manifest/schema
configuration
secrets
input/output types
delivery semantics
capacity
backpressure
timeouts
retries
retry budgets
readiness
health
error classification
cancellation
drain
abort
shutdown
observations
provider status
conformance tests
example connectors
```

Clearly distinguish:

```text
Core contract
connector framework
provider-specific implementation
example
experimental integration
```

A connector author must be able to implement a new connector without reading framework internals.

---

# 40. Cross-repository consistency

When this protocol is run separately on related repositories, preserve compatible documentation vocabulary.

Do not duplicate authoritative definitions unnecessarily.

Identify concepts that belong in:

```text
shared platform documentation
Rust SDK documentation
Python SDK documentation
JavaScript SDK documentation
Relay documentation
Connector author documentation
```

Language-specific pages describe language-specific usage.

Shared conceptual pages describe shared contracts only if the contracts are genuinely shared.

---

# 41. Documentation site readiness

If a documentation framework already exists, integrate with it.

Do not replace a functioning docs framework without evidence that replacement is necessary.

The final documentation site must support, where the framework permits:

```text
predictable navigation
search
stable URLs
heading anchors
syntax-highlighted code
language/platform tabs where appropriate
cross-links
edit/source links
version information
sitemap
redirects for moved pages
accessible markup
mobile readability
raw Markdown or machine-readable content
llms.txt or equivalent documentation index
```

Provide an `llms.txt`-style index of canonical documentation pages if appropriate.

Machine-friendly access supplements human documentation. It does not replace it.

---

# 42. Quality validation

Before declaring documentation complete, run all applicable repository checks.

Examples include:

```text
build
unit tests
integration tests
doc tests
example compilation
type checking
lint
documentation build
link checking
anchor checking
spell/terminology checking
API-reference generation
dead-reference detection
```

For Rust, include applicable:

```text
cargo check
cargo test
cargo test --doc
cargo doc
```

For Python, include applicable:

```text
tests
type checks
documentation generation
example execution
```

For JS/TS:

```text
npm/pnpm/yarn test
typecheck
build
API docs generation
example compilation/execution
```

For Go:

```text
go test ./...
go vet ./...
documentation/package checks
```

Use the repository's actual toolchain rather than blindly executing generic commands.

---

# 43. Editorial validation

Run a separate editorial pass after technical validation.

Check for:

```text
vague verbs
unsupported adjectives
undefined terminology
inconsistent naming
passive constructions that hide the actor
ambiguous pronouns
overlong sentences
multiple concepts in one paragraph
missing prerequisites
missing expected outcomes
duplicated explanations
stale version numbers
unqualified platform claims
unsupported guarantees
orphan links
unexplained examples
```

Do not reduce technical precision merely to shorten prose.

---

# 44. Documentation completion report

Do not finish with:

> “I documented the repository.”

Return a factual completion report containing:

```text
snapshot analyzed
total files
files semantically analyzed
excluded/generated/third-party files
packages/modules
public symbols
errors
configuration keys
protocol/API elements
tests
examples

documentation pages created
source/API comments created or improved

public symbols with reference coverage
public symbols with examples
public symbols with documented errors
broken links
failing examples
unresolved conflicts
remaining unknowns

validation commands executed
validation results
```

Include a navigation tree of the resulting docs.

Include all unresolved technical questions.

Do not hide incompleteness.

---

# 45. Hard completion gates

Do not use words such as:

```text
complete
fully documented
production-ready
deploy-ready
```

unless all required gates pass.

Required gates:

```text
100% repository file inventory
100% classification
100% semantic analysis of owned textual source/config/test/example files
100% intentionally public symbol inventory
100% public API reference coverage
100% public configuration reference coverage
100% public error/failure inventory
all published executable examples verified
documentation site builds
no broken internal links
no unresolved undocumented public behavior hidden from the report
no unsupported technical claims
```

Conflicts and unknowns may remain, but they must be visible.

---

# 46. What you must not do

Never:

```text
read README + several important files and call the repository understood

produce an architecture summary before inventory completion

use filenames as proof of behavior

document APIs solely from names/signatures

skip tests

skip examples

skip platform-specific implementations

skip configuration

skip error branches

skip shutdown/finalization

skip feature-gated code

skip native/FFI boundaries

write marketing copy as technical documentation

copy prose from Temporal, Google, Microsoft, GitHub, or another project

invent use cases not supported by the repository

create fake examples

publish untested commands as known-working commands

silently “clean up” conflicting contracts

hide unknown behavior

modify runtime implementation merely to make your docs narrative true
```

---

# 47. Execution behavior

Work autonomously through the repository.

Do not stop after generating an inventory.

Do not ask me to manually identify important files.

Do not ask me which source files matter.

Discover them.

Persist progress after each analysis batch.

If the context window becomes crowded, write all extracted facts to `.doc-intel/`, checkpoint your exact coverage state, discard raw temporary context, reload only the structured records required for the next pass, and continue.

Your ability to finish must not depend on retaining the repository in one model context.

---

# 48. Final objective

The final result must satisfy two different people.

A new developer should be able to:

```text
understand the product boundary
install it
complete the first useful task
find the right concept
find the exact API
understand failure behavior
troubleshoot problems
use examples without reading implementation source
```

A maintainer should be able to:

```text
trace a documentation statement back to code
identify which documentation a code change can invalidate
see undocumented public surface
see conflicts between declared and implemented behavior
regenerate reference documentation safely
understand the repository's complete structural and behavioral graph
```

Those are equally important.

The repository is not considered documented until both are possible.

---

# Invocation

Apply this protocol to:

```text
TARGET_REPOSITORY=<repository path>
```

Automatically detect the repository language, package structure, build system, and repository role.

Begin with repository intelligence.

Do not begin writing public documentation until the evidence and coverage gates pass.

When generation begins, produce source-level/API documentation and people-first public documentation separately, then validate both against the repository snapshot.
