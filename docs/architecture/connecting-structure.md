# Groundwork Connecting Structure Design

This document records the design of groundwork's connecting structure —
the artifacts, manifest edges, and schemas that link protocols and skills
into a coherent topology. It is built incrementally during the design
session and captures decisions as they are reached.

## Settled Constraints

These survived prior reckoning sessions and are ground for this design.

1. **Runa's function.** Event-driven cognitive runtime. Monitors artifact
   state, validates against schemas, computes dependency graph, enforces
   protocol contracts, injects context when activating protocols.

2. **Artifacts are the sole state mechanism.** No second channel. Runa
   derives workflow state entirely from artifacts on disk.

3. **Artifacts are capstones.** The agent does the real work. The artifact
   produced at the end captures context for runa to orchestrate handoff
   to the next stage. Artifact creation is the last step, not the work
   itself.

4. **Work-unit identity.** Every artifact instance carries a reference to
   the work unit it belongs to. Runa uses this to resolve which instances
   are related. Manifest edges stay type-level; instance-level linking
   lives in artifact content.

5. **Two populations.** Protocols are runa-managed (declared in manifest,
   triggered by artifact state, enforced by runa). Skills are agent-managed
   (invoked by agent judgment, not declared in manifest).

6. **The liberation insight.** Runa imposing one law — the methodology
   topology — liberates the agent from its own many-law cognitive chaos.

## The Forward Flow

The full flow for a single work unit is:

```
begin → specify → plan → implement → verify → document → submit → land
```

Survey and decompose precede begin when project-level planning is needed.
Survey produces requirements; decompose breaks requirements into issue
artifacts. Begin picks up an issue and starts the work-unit lifecycle.

Document sits between verify and submit. Documentation is submitted
together with the code it explains. Submit is gated on documentation-record.

## Output Artifact Analysis

### Protocols that produce artifacts for runa

| Protocol  | Produces | Purpose of capstone |
|-----------|----------|---------------------|
| survey    | requirements | Declaration of what needs doing, at any scope |
| decompose | issue | Work units decomposed from requirements |
| begin     | claim | Root node: work-unit identity and orientation |
| specify   | behavior-contract | Behavioral scenarios for the work unit |
| plan      | implementation-plan | Design decisions informing execution |
| implement | test-evidence | Proof of correct implementation — passing tests mapped to scenarios |
| verify    | completion-evidence | Aggregated behavior coverage status |
| document  | documentation-record | Documentation coverage and tracking |
| submit    | patch | Packaged changes ready for review |
| land      | completion-record | Final state: coverage, gaps, merge ref |

All ten protocols produce artifacts for runa. No protocol is disconnected
from the artifact graph.

### Artifact types entering from outside

| Artifact type | Origin | Purpose |
|---------------|--------|---------|
| request | External: change request, question, bug report, feature idea | Enters the system and triggers survey |

### Begin — the missing artifact

Begin currently produces no artifact for runa. It should. Begin does the
most consequential work in the topology — selecting and orienting to a
work unit — but leaves no trace in the artifact system.

Begin's capstone is the **root node** of the work-unit artifact chain.
It establishes the work-unit identifier that every downstream artifact
references. Without it, runa cannot thread related artifacts together.

The begin artifact is a threading mechanism: work-unit identifier plus
enough orientation for downstream protocols. Not a context dump — the
issue itself carries the full context, and the issue graph is the right
place for that.

## Input Edge Principle

Runa's interface contract defines two input edge types:

- **requires** — artifact must exist and validate before the protocol
  executes. Runa blocks execution without it.
- **accepts** — artifact consumed if available. Protocol operates with
  or without it.

**The design principle:** an input is `requires` when the protocol cannot
produce a structurally valid capstone without it, or when the work-unit
thread would break without it. An input is `accepts` when the capstone
can be valid but would be better informed by the context.

Requires edges form the **structural backbone** of the topology — the
chain that must be unbroken for the work unit to flow. Accepts edges
provide **contextual enrichment** — cross-cutting artifacts that improve
quality but whose absence doesn't break the chain.

**Runa's enforcement semantics:** requires means "runa enforces that the
methodology cannot skip this step." Accepts means "the methodology
benefits from this context but the protocol can still do valid work
without it."

## No Signals

If artifacts are the sole state mechanism, then signals are a second
channel. Every protocol triggers on artifact state. External events
enter the system as artifacts (a request landing in the workspace),
not as signals. The topology is pure graph.

This eliminates the `on_signal` trigger primitive from groundwork's
manifest entirely. Every trigger is `on_artifact`, `on_change`,
`on_invalid`, or a composition of these.

## The Full Artifact Chain

With no signals, every link between protocols is an artifact.
The complete chain across both phases:

```
request → requirements → issue → claim → behavior-contract
→ implementation-plan → test-evidence → completion-evidence
→ documentation-record → patch → completion-record
```

Cross-cutting: research-record feeds in via accepts edges where needed.
Research-record may optionally be scoped to a work unit when the research
is specific to an issue.

## Work-Unit-Scoped Evaluation

The manifest declares type-level edges. Runa evaluates triggers per work
unit at runtime, using the `work_unit` field to partition the workspace.

When multiple work units are active simultaneously, plan triggering on
`on_artifact("behavior-contract")` fires for a specific work unit's
behavior-contract — not every behavior-contract in the workspace. The
manifest doesn't express this scoping. Runa computes it from artifact
content.

Planning-phase artifacts (request, requirements, issue) predate work-unit
identity and are not partitioned this way. Research-record is always scoped by topic; optionally
scoped by work unit when research is specific to an issue.

## Consolidated Manifest

This is the target `groundwork.toml` derived from all decisions in this
document.

```toml
# Groundwork Methodology Manifest
#
# runa reads this file to understand the groundwork methodology.
# Topology emerges from the graph of requires/produces relationships.

name = "groundwork"

# --- Artifact Types ---

[[artifact_types]]
name = "request"
schema = "schemas/request.schema.json"

[[artifact_types]]
name = "requirements"
schema = "schemas/requirements.schema.json"

[[artifact_types]]
name = "issue"
schema = "schemas/issue.schema.json"

[[artifact_types]]
name = "claim"
schema = "schemas/claim.schema.json"

[[artifact_types]]
name = "behavior-contract"
schema = "schemas/behavior-contract.schema.json"

[[artifact_types]]
name = "implementation-plan"
schema = "schemas/implementation-plan.schema.json"

[[artifact_types]]
name = "test-evidence"
schema = "schemas/test-evidence.schema.json"

[[artifact_types]]
name = "completion-evidence"
schema = "schemas/completion-evidence.schema.json"

[[artifact_types]]
name = "documentation-record"
schema = "schemas/documentation-record.schema.json"

[[artifact_types]]
name = "patch"
schema = "schemas/patch.schema.json"

[[artifact_types]]
name = "completion-record"
schema = "schemas/completion-record.schema.json"

[[artifact_types]]
name = "research-record"
schema = "schemas/research-record.schema.json"

# --- Protocols ---
#
# Planning phase: survey → decompose
# Execution phase: begin → specify → plan → implement → verify
#                  → document → submit → land

[[protocols]]
name = "survey"
requires = ["request"]
accepts = ["research-record"]
produces = ["requirements"]
may_produce = []
trigger = { type = "on_artifact", name = "request" }

[[protocols]]
name = "decompose"
requires = ["requirements"]
accepts = ["research-record"]
produces = ["issue"]
may_produce = []
trigger = { type = "on_artifact", name = "requirements" }

[[protocols]]
name = "begin"
requires = ["issue"]
accepts = []
produces = ["claim"]
may_produce = []
trigger = { type = "on_artifact", name = "issue" }

[[protocols]]
name = "specify"
requires = ["claim", "issue"]
accepts = ["research-record"]
produces = ["behavior-contract"]
may_produce = []
trigger = { type = "on_artifact", name = "claim" }

[[protocols]]
name = "plan"
requires = ["behavior-contract"]
accepts = ["research-record"]
produces = ["implementation-plan"]
may_produce = []
trigger = { type = "on_artifact", name = "behavior-contract" }

[[protocols]]
name = "implement"
requires = ["behavior-contract", "implementation-plan"]
accepts = []
produces = ["test-evidence"]
may_produce = []
trigger = { type = "on_artifact", name = "implementation-plan" }

[[protocols]]
name = "verify"
requires = ["behavior-contract", "test-evidence", "issue"]
accepts = []
produces = ["completion-evidence"]
may_produce = []
trigger = { type = "on_artifact", name = "test-evidence" }

[[protocols]]
name = "document"
requires = ["completion-evidence"]
accepts = ["behavior-contract", "implementation-plan"]
produces = ["documentation-record"]
may_produce = []
trigger = { type = "on_artifact", name = "completion-evidence" }

[[protocols]]
name = "submit"
requires = ["completion-evidence", "documentation-record"]
accepts = []
produces = ["patch"]
may_produce = []
trigger = { type = "on_artifact", name = "documentation-record" }

[[protocols]]
name = "land"
requires = ["patch"]
accepts = ["completion-evidence", "behavior-contract", "documentation-record", "issue"]
produces = ["completion-record"]
may_produce = []
trigger = { type = "on_artifact", name = "patch" }
```

### Changes from current manifest

**Protocols renamed:**
- `propose` → `submit`
- `test` → `implement`

**Protocols unchanged in name:**
- survey, decompose, begin, specify, plan, verify, document, land

**Artifact types added:**
- request, requirements, issue, claim, patch

**Artifact types removed:**
- assessment (replaced by requirements)

**Artifact types renamed:**
- none (all surviving types keep their names)

**Structural changes:**
- All `on_signal` triggers replaced with `on_artifact` triggers
- All protocols now produce artifacts (begin, decompose, submit were gaps)
- Document moved from parallel/floating to sequential between verify
  and submit
- Specify now requires issue (for acceptance criteria traceability)
- Verify now requires issue (for criterion-level gap detection)

### Synthesis Verification

**Single producer rule.** Every artifact type has exactly one producer
(protocol or external source). No ambiguity for runa.

| Artifact type | Producer |
|---------------|----------|
| request | external |
| requirements | survey |
| issue | decompose |
| claim | begin |
| behavior-contract | specify |
| implementation-plan | plan |
| test-evidence | implement |
| completion-evidence | verify |
| documentation-record | document |
| patch | submit |
| completion-record | land |
| research-record | research skill (agent-managed) |

**Every type consumed.** All artifact types have at least one consumer
except completion-record, which is the terminal archival artifact.

**Trigger consistency.** Each protocol's trigger artifact is the last
requires dependency to land — the one that unblocks execution. Verified
for all ten protocols: the trigger is always the artifact that cannot
exist until all earlier dependencies in the chain are satisfied.

**Research-record is the sole skill-produced artifact in the protocol
graph.** No protocol produces it. The research skill (agent-managed)
produces it. Four protocols accept it as contextual enrichment. Runa
validates research-records against the schema when they appear but
never orchestrates their production. Research-record may carry
`work_unit` when the research is specific to an issue; when it does,
runa can scope it to the relevant work unit's context. When `work_unit`
is absent, the research is cross-cutting. This is the two-population
principle in action: skills produce artifacts that runa validates
but doesn't trigger on.

**No cycles.** The requires graph is a DAG. Verified by walking the
full chain from request through completion-record.

**Most-referenced artifacts.** behavior-contract is required by three
protocols (plan, implement, verify). issue is required by three
protocols (begin, specify, verify). These are the central artifacts
of the execution phase — the behavioral spec and the acceptance
criteria it traces to.

## Agent Interface

Two interfaces connect the agent to the artifact system. Both are
owned by runa. The agent touches neither directly.

### Input: Context injection as prompt

Runa constructs a prompt with all context pre-integrated. The skill
reads natural language, not JSON. The behavior-contract, implementation-
plan, research-records are already woven into the context window.
The skill doesn't parse artifacts or know about schemas.

### Output: MCP tool for artifact production

An MCP server exposes a tool the agent calls to deliver work products.
Instead of constructing JSON and placing files, the agent calls:

```
produce("behavior-contract", {
  title: "User authentication",
  scenarios: [
    { name: "valid login", criterion: "users can log in",
      given: "...", when: "...", then: "..." }
  ]
})
```

The MCP server validates against the schema, writes to the workspace,
and reports success or failure.

### Schema vs tool interface

The schema and the MCP tool interface are related but not identical.

The schema is the full artifact structure on disk — what runa validates
and tracks. The tool interface is the schema minus what runa can infer
from the active execution context.

**work_unit** is the one field runa can always infer. Runa activated
this protocol for a specific work unit. The MCP server auto-populates
work_unit from the execution context. The agent never supplies it.

Everything else in the schemas is the agent's cognitive output — runa
cannot know it, the agent must supply it. The schemas work as tool
interfaces with that one subtraction.

### The liberation insight at the interface level

The agent never touches the artifact system. Runa owns both input
(context injection) and output (MCP validation and placement). The
skill is liberated from infrastructure — free to do its cognitive
work without fighting JSON Schema internals, file placement conventions,
or state management.

## The MCP Server as Methodology Interface

The MCP server is not just an artifact I/O layer. It is the agent's
entire interface to the methodology. The agent doesn't know about runa,
manifests, schemas, work units, artifact types, or the topology. It
has tools. The tools guide the work. The shape of the tools IS the
methodology.

### The agent knows nothing about infrastructure

The MCP server can infer from execution context:
- **work_unit** — which issue is being worked
- **protocol** — which protocol is executing
- **artifact type** — what this protocol produces
- **available context** — what requires/accepts artifacts exist

This means the agent's tool interface can be as simple as
`deliver(content)`. The server knows the rest.

### Structured queries replace context parsing

Instead of the agent parsing injected context, the MCP server exposes
query tools: what are my acceptance criteria, what scenarios exist, what
tests passed. Structured queries against the artifact store, returned
in natural language or structured data.

### Cross-reference validation at write time

When the agent references an acceptance criterion in a scenario, the
MCP server verifies it exists in the issue artifact. Not just schema
validation — semantic validation. The traceability thread is enforced
mechanically.

### Progressive authoring

Instead of one atomic `deliver()` call, the MCP server can support
incremental building: add a scenario, get immediate feedback, add
another, finalize. The agent discovers errors as it works, not after
producing the full artifact.

### Pre-population and cognitive scaffolding

The MCP server can present pre-assembled data to reduce the agent's
mechanical work. Verify's agent receives a pre-filled coverage matrix
(criteria × scenarios × test results) and does judgment work — confirm,
amend, flag gaps — not data assembly.

### Observability from the start

Every tool call is a structured event. The MCP server sits at the
chokepoint between agent and system. This enables:

- **Telemetry** — which agent, which protocol, which work unit, what
  was produced, when, whether it validated. Without the agent doing
  anything extra.
- **Cost tracking** — tool calls correlated with LLM calls. Cost per
  behavior-contract, cost per issue implementation, cost per acceptance
  criterion. Measured, not estimated.
- **Anomaly detection** — the server sees patterns across many work
  units. An implement protocol completing in two minutes when the
  median is forty is a signal. A behavior-contract with one scenario
  for eight acceptance criteria is a signal.
- **Replay and audit** — the full sequence of tool calls for a work
  unit is a structured trace. Debugging agent behavior means reading
  structured logs, not sifting through conversations.
- **Resource governance** — token budgets, time limits, policy
  enforcement at the tool level.

### Architecture summary

The CLI and artifact store are the skeleton. The MCP server is the
nervous system — the live interface where agents meet methodology.
The topology, schemas, and edges designed in this document give the
MCP server its shape. The liberation insight taken to its conclusion:
the one law isn't visible to the agent as a law. It is the shape of
the available tools.

## Two Levels of Specification

The topology has two specification artifacts at different scales:

- **requirements** (produced by survey) — declares what needs doing at
  any scope: a new tool, a feature, a system change, a migration.
  Consumed by decompose, which breaks it into issue-sized work units.
  This is the project-level specification.

- **behavior-contract** (produced by specify) — declares how a single
  work unit should behave as Given/When/Then scenarios. This is the
  implementation-level specification.

Decompose bridges the two levels. It consumes requirements and produces
issue artifacts — the work units that begin picks up.

## Two Phases

The issue artifact bridges two phases:

**Planning phase:** request → survey → requirements → decompose → issue.
External input enters as a request artifact, survey produces requirements,
decompose breaks requirements into issue artifacts.

**Execution phase:** issue → begin → specify → plan → implement → verify →
document → submit → land. Begin picks up an issue artifact whose
dependencies are satisfied, the forward flow produces artifacts that runa
tracks and threads by work-unit identity.

## Input Edges — Protocol by Protocol

### survey

- **requires:** request. The external input that prompted the work.
  Survey cannot produce requirements without knowing what was requested.
- **accepts:** research-record. Prior research may inform requirements.
- **trigger:** `on_artifact("request")`

### decompose

- **requires:** requirements. Cannot break work into issues without
  knowing what the work is.
- **accepts:** research-record. Research may inform decomposition decisions.
- **trigger:** `on_artifact("requirements")`

### begin

- **requires:** issue. An issue whose dependencies are satisfied.
- **accepts:** nothing. Planning-phase artifacts feed decompose, not
  begin. The issue artifact is the bridge.
- **trigger:** `on_artifact("issue")`

### specify

- **requires:** claim, issue. The claim provides work-unit identity.
  The issue provides acceptance criteria that specify transforms into
  behavioral scenarios — traceability requires seeing the criteria.
- **accepts:** research-record. Research may inform behavioral scenarios,
  but specify can produce valid GWT scenarios without it.
- **trigger:** `on_artifact("claim")`

### plan

- **requires:** behavior-contract. Cannot design an implementation
  without knowing what behavior is being implemented.
- **accepts:** research-record. Research may inform design decisions.
- **trigger:** `on_artifact("behavior-contract")`

### implement

- **requires:** behavior-contract, implementation-plan. The behavior
  scenarios ARE the tests (written in specify). The plan provides the
  design approach. Implement does RED-GREEN-REFACTOR: write failing
  tests from scenarios, write code to pass them, refactor.
- **accepts:** nothing currently identified.
- **trigger:** `on_artifact("implementation-plan")`

### verify

- **requires:** behavior-contract, test-evidence, issue. Verify checks
  behavior coverage against the contract using test results as evidence.
  The issue is required because verify must detect acceptance criteria
  that have no scenario coverage — gaps that only the original criteria
  list reveals.
- **accepts:** nothing currently identified.
- **trigger:** `on_artifact("test-evidence")`

### document

- **requires:** completion-evidence. Documentation is reviewed after
  completion is verified. Docs are submitted with the code they explain.
- **accepts:** behavior-contract, implementation-plan. Context for what
  needs documenting.
- **trigger:** `on_artifact("completion-evidence")`

### submit

- **requires:** completion-evidence, documentation-record. Cannot submit
  unverified work, and docs must accompany the code.
- **produces:** patch.
- **trigger:** `on_artifact("documentation-record")`

### land

- **requires:** patch. Cannot land without a submitted patch.
- **accepts:** completion-evidence, behavior-contract, documentation-record,
  issue. Context for the completion record. Completion-evidence already
  carries criterion-level coverage, so issue is enrichment not structural.
- **trigger:** `on_artifact("patch")`

## Document Protocol vs Document Skill

The document *protocol* is a runa-managed process: review and update
documentation after completion, before submission. It sits between verify
and submit in the forward flow.

The document *skill* would be inline documentation during development —
comments, docstrings, README updates as part of writing code. Agent-invoked
during test. This skill does not currently exist in the codebase. Not a
design blocker for the connecting structure, but a noted gap.

## Schema Design Principles

### Consumer-backward

Each schema is designed from the consuming protocol's need: what must be
in the injected context for the consumer to produce its own capstone?
Not from a guess about what the producer might write.

### Common envelope

**Execution-phase artifacts** (claim through completion-record) carry a
`work_unit` field — the issue reference that threads them together. Runa
uses this to scope context injection: when plan activates, it delivers
the behavior-contract for this work unit, not every behavior-contract in
the workspace.

**Planning-phase artifacts** (request, requirements, issue) do not carry
`work_unit`. They predate work-unit identity. Runa scopes them through
trigger evaluation against specific artifact instances.

Everything else runa needs is already available from outside artifact
content: artifact type from directory structure, producing protocol from
manifest declarations, modification timestamps from filesystem state,
content hashes from the store. The common envelope is minimal by design.

## Per-Type Schemas

Designed consumer-backward: what does the consuming protocol need in its
injected context to produce its own capstone?

### request

**Consumer:** survey.
**What survey needs:** understand what's being asked, orient to the domain.

The request is the entry point to the system — a door, not a document.
Lightweight enough that creating one isn't burdensome, structured enough
that survey has something to work from.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| description | string | yes | What is being asked for |
| source | string | yes | Where this came from (operator, user report, automated detection) |
| context | string | no | Anything else the requester wants to include |

### requirements

**Consumer:** decompose.
**What decompose needs:** understand the full scope, identify natural seams
for breaking work into issue-sized units, respect constraints and
dependencies when drawing boundaries.

This is a software requirements specification. Its structure follows
standard SRS practice because that structure exists precisely to support
decomposition.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| scope | string | yes | Purpose and boundaries of the work |
| functional_requirements | array of strings | yes | What the system should do — discrete items |
| non_functional_requirements | array of strings | no | Performance, security, etc. |
| constraints | array of strings | no | Technical and business boundaries |
| assumptions | array of strings | no | What is taken as given |
| dependencies | array of strings | no | External dependencies affecting decomposition |

### issue

**Consumer:** begin.
**What begin needs:** understand the work unit being claimed — what to do,
how to know it's done, and whether it's ready to start.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| title | string | yes | What this work unit is |
| description | string | yes | What needs doing |
| acceptance_criteria | array of strings | yes | Discrete, verifiable conditions for "done" |
| dependencies | array of issue refs | no | Issues that must be complete before this starts |

### Traceability Thread

Acceptance criteria on the issue are the high-level "done" statements.
Behavior-contract scenarios are the precise behavioral refinement of
those criteria into Given/When/Then. The traceability thread runs the
full length of the execution chain:

```
issue (acceptance_criteria)
  → claim (carries issue ref)
    → behavior-contract (scenarios trace to acceptance criteria)
      → test-evidence (results trace to scenarios)
        → completion-evidence (coverage at acceptance-criterion level)
```

Schema implications:
- behavior-contract scenarios carry a reference to which acceptance
  criterion they refine
- completion-evidence reports coverage at the acceptance-criterion
  level, not just the scenario level — so verify can answer "are all
  acceptance criteria covered?"

### claim

**Consumer:** specify (and all downstream protocols via work_unit threading).
**What specify needs:** the work-unit identity and a reference to the issue
being implemented. The claim is the threading root — thin by design.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | The issue being claimed — threads all downstream artifacts |
| scope | string | yes | Brief statement of what's being claimed from the issue |

The claim does not duplicate acceptance criteria from the issue. Runa's
context injection delivers a protocol's own requires and accepts — not
transitive dependencies. Protocols that need the acceptance criteria
must declare the issue artifact in their own edges.

### Context Injection Is Not Transitive

Runa injects a protocol's declared requires and accepts instances. It
does not inject transitive dependencies. If specify needs the issue
artifact (to read acceptance criteria), specify must declare it in its
own edges. The claim alone is not sufficient — it carries the work-unit
reference but not the issue content.

This means protocols downstream of claim may need to declare the issue
artifact explicitly when they need access to acceptance criteria for
traceability purposes.

### behavior-contract

**Consumers:** plan, test, verify, document (via accepts).
**What consumers need:** behavioral scenarios that trace to acceptance
criteria, structured as executable Given/When/Then.

The existing schema had the right core — title and GWT scenarios. Two
changes from this design: each scenario now carries a `criterion`
reference for traceability, and the common `work_unit` field threads it
to the work unit.

The existing `metadata` block (produced_by, date) is eliminated.
Runa knows the producing protocol from the manifest. It tracks
timestamps from filesystem state. The metadata duplicated what runa
already knows. By sufficiency, it has no place in the schema.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | Common envelope — threads to work unit |
| title | string | yes | Human-readable title for the contract |
| scenarios | array of scenario | yes (min 1) | Behavioral scenarios |

**Scenario fields:**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| name | string | yes | Human-readable scenario name |
| criterion | string | yes | Which acceptance criterion this refines |
| given | string | yes | Initial context or state |
| when | string | yes | Action or event |
| then | string | yes | Expected outcome |

### Metadata Elimination Principle

Runa tracks producing protocol (from manifest), modification timestamps
(from filesystem), and content hashes (from store). Schemas should not
duplicate what runa already knows. Any field whose value runa can derive
from its own state does not belong in artifact content. This eliminates
`produced_by`, `date`, and similar metadata from all schemas.

### implementation-plan

**Consumers:** implement (requires), document (accepts).
**What implement needs:** the design approach — what to change, how, and which
behavioral scenarios map to which implementation steps.

The plan bridges behavior (from specify) to code (in implement). Without
the plan, the agent implements without design — which is what the plan
exists to prevent.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | Common envelope |
| summary | string | yes | What the plan accomplishes |
| design_decisions | array of decision | yes (min 1) | Decisions with rationale |
| affected_files | array of strings | yes (min 1) | Files or modules that get changed |
| behavior_mapping | array of mapping | yes (min 1) | How scenarios map to implementation steps |

**decision:**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| decision | string | yes | What was decided |
| rationale | string | yes | Why — traces to constraints or principles |

**mapping:**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| scenario | string | yes | Scenario name from behavior-contract |
| steps | array of strings | yes (min 1) | Implementation steps for this scenario |

### test-evidence

**Consumer:** verify (requires).
**What verify needs:** proof that each scenario was tested and the result.
Verify joins test-evidence with behavior-contract to roll up coverage at
the acceptance-criterion level — no need to duplicate criterion references
here.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | Common envelope |
| evidence | array of evidence-entry | yes (min 1) | Test results per scenario |

**Evidence-entry fields:**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| scenario | string | yes | Scenario name from behavior-contract |
| result | enum: pass, fail | yes | Test outcome |
| command | string | yes | The command that was executed |
| output_summary | string | yes | Summary of command output — proof the test ran |

### completion-evidence

**Consumers:** document (requires), submit (requires), land (accepts).
**What document needs:** confirmation that implementation is verified
before documentation review begins. What submit needs: proof that work
is verified before packaging. What land needs: coverage context for the
final record.

Verify produces this by joining issue (acceptance criteria), behavior-
contract (scenario-to-criterion mapping), and test-evidence (results).
The output reports coverage at the acceptance-criterion level.

The existing schema's review-artifact and documentation-artifact fields
are eliminated — document comes after verify in this topology, and
submit handles the PR. Those fields belonged to a different flow.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | Common envelope |
| criterion_coverage | array of coverage-entry | yes (min 1) | Per-criterion coverage status |

**Coverage-entry fields:**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| criterion | string | yes | Acceptance criterion from the issue |
| status | enum: covered, partial, uncovered | yes | Coverage status |
| scenarios | array of strings | no | Scenario names that cover this criterion |
| failures | array of strings | no | Scenario names that failed for this criterion |

### documentation-record

**Consumers:** submit (requires), land (accepts).
**What submit needs:** confirmation that documentation review is complete
before packaging. What land needs: documentation coverage context for
the final record.

The existing schema structure survives — it tracks the right things.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | Common envelope |
| updated_docs | array of strings | yes | Documentation files updated in this change |
| verified_accurate_docs | array of strings | yes | Documentation reviewed and confirmed accurate |
| tracking_issues | array of strings | yes | Issues filed for documentation follow-up |

### patch

**Consumer:** land (requires).
**What land needs:** the submitted change — where it is and what it
contains. This is the artifact representation of the PR.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | Common envelope |
| pr_reference | string | yes | PR URL or identifier |
| branch | string | yes | Feature branch name |
| commit | string | yes | Head commit SHA at submission time |

### completion-record

**Consumer:** none (terminal artifact — archival record).
**What it captures:** the final state of the work unit. This is a summary
artifact — the structured enforcement lives upstream in completion-evidence.
The record distills the conclusion.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| work_unit | string (issue ref) | yes | Common envelope |
| criterion_summary | string | yes | How acceptance criteria were met |
| gaps | array of strings | yes | Known gaps or deferred work (empty if none) |
| merge_reference | string | yes | Merge commit SHA or PR URL |
| documentation_status | string | yes | Summary of documentation coverage |

### research-record

**Consumers:** specify (accepts), plan (accepts), survey (accepts),
decompose (accepts).
**What consumers need:** research findings and their sources, scoped
by topic. May serve multiple work units when cross-cutting, or be
scoped to a specific issue via the optional `work_unit` field.

Research-record is always scoped by topic. It optionally carries
`work_unit` when the research is specific to an issue — for example,
researching a particular library for a particular implementation task.
When `work_unit` is absent, the research is cross-cutting and available
to any protocol that accepts it. It belongs to neither the planning nor
execution phase exclusively — it enriches both.

The existing `date` field is eliminated by the metadata elimination
principle. Runa tracks timestamps from filesystem state.

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| topic | string | yes | What was researched (kebab-case slug) |
| work_unit | string | no | Optional issue reference — scopes research to a work unit |
| findings | array of strings | yes (min 1) | Key findings |
| sources | array of source | yes (min 1) | Sources consulted |

**Source fields:**

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| url | string (URI) | yes | Source URL |
| title | string | no | Source title |
