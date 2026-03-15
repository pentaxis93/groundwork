# Groundwork Workflow

This is the integration manual for operating Groundwork's skills as one connected topology from problem framing to shipped change.

For the canonical inventory and interface declarations, see `groundwork.toml`. This document is the narrative explanation of how those skills interact.

## Topology

Every piece of work flows through five stages: frame constraints, define behavior, decompose, execute and verify, land. Skills from different sources slot into this sequence. Each step produces an artifact or state change that the next step consumes.

### 1. Frame constraints

Invoke `ground` before creating any design, spec, architecture, or process. It establishes what the work must enable, decomposes to verified constraints, and strips assumed constraints. Its output — a set of grounded constraints — is the input to every subsequent design decision.

`ground` is not step-one-once. New generative work mid-session requires re-grounding. The trigger is creation, not sequence position.

### 2. Define the behavior contract

Invoke `bdd` to define the behavior contract in Given/When/Then scenarios. Each scenario names an observable behavior the system must exhibit. This contract is not a one-time artifact — it threads through every subsequent step. See [BDD Thread](#bdd-thread) for how it integrates with execution and verification.

`bdd` and `test-first` are not alternatives. `bdd` defines what behavior must exist. `test-first` is the execution discipline that realizes it.

### 3. Decompose into executable work

Use `issue-craft` to create, decompose, refine, and close issues. It produces agent-executable issues with binary acceptance criteria, explicit dependencies, and bounded scope. For epics with 4+ tasks, it builds dependency graphs with execution layers.

Use `begin` to initiate a work session. It selects session-sized work from the issue graph, prepares the workspace (feature branch, issue context), and declares the session's starting direction with an explicit scope gate.

Use `plan` to converge from exploration to a decision-complete implementation design before modifying code. It explores the codebase, resolves intent, and produces a plan where every design choice is explicit — the implementer does not need to make any decisions. Based on Codex CLI plan mode (MIT), adapted for autonomous execution.

After a design is approved, use `issue-craft` to express the implementation as agent-executable work units with binary acceptance criteria. Approved designs flow into `plan` for convergence and into `issue-craft` for decomposition.

### 4. Execute and verify

Use `test-first` to implement each plan step through RED-GREEN-REFACTOR. Each RED test corresponds to a named behavior scenario from `bdd`. Write one failing test, verify it fails for the right reason, write the simplest code to pass it, verify it passes, refactor.

When the plan contains independent tasks, dispatch a fresh subagent per task to keep execution context clean — stale context from earlier tasks pollutes later ones. Match subagent model to task complexity: use cheaper/faster models for straightforward work, reserve the most capable model for subtle or cross-cutting changes.

Use `systematic-debugging` when a test fails or behavior is unexpected. It finds root cause before proposing fixes — no symptom-patching. Although listed here because most debugging occurs during execution, it is a cross-cutting discipline that fires at any stage when failures appear.

Code review is handled by CI/CD infrastructure, not a methodology skill. The methodology requires review before landing but does not prescribe the mechanism.

Use `verification-before-completion` before claiming any work is complete. It requires running the actual verification command and confirming the output matches the claim. Completion evidence must be behavior-level — not just "tests pass" but explicit behavior coverage.

Use `propose` to package verified changes into a PR: ensure feature branch, analyze and commit changes, push, and create PR with derived title/body linked to issue(s). This produces the open PR that `land` will merge in stage 5. The session lifecycle is: `begin` (initiate session) → implement → `propose` (package for review) → review → `land` (merge and close).

### 5. Land

Invoke `land` to close the loop. It merges the feature branch to main, pushes, deletes the branch (remote and local), posts a completion comment on the issue, and closes the issue. Closure records behavior coverage status and any remaining gaps.

Do not stop after merge. `land` means complete delivery: merged code, clean branches, closed issue.

For the formal handoff contracts, anti-divergence rules, and compliance checklist, see [docs/architecture/topology-contract.md](docs/architecture/topology-contract.md).

## BDD Thread

BDD is a cross-cutting integration mechanism, not a phase. It runs through specification, execution, and verification:

- **Specification**: define behavior in Given/When/Then terms.
- **Execution**: `test-first` implements those behaviors via RED-GREEN-REFACTOR.
- **Verification**: completion evidence is checked against those behaviors, not just command success.

### Handoff Rules

1. `bdd -> plan`: design decisions preserve explicit behavior traceability.
2. `bdd -> issue-craft`: acceptance criteria and work units map back to behavior statements.
3. `bdd -> test-first`: each RED test corresponds to a named behavior scenario.
4. `bdd -> verification-before-completion`: completion claims require behavior-level evidence.
5. `bdd -> land`: closure records behavior coverage status and remaining gaps, if any.

For fail conditions and anti-divergence rules, see [docs/architecture/topology-contract.md](docs/architecture/topology-contract.md).

## Documentation Thread

Documentation is a cross-cutting communication discipline, not a phase. It threads through every stage:

- **Frame constraints**: `ground`'s Orient identifies who this serves and what it must enable — that output defines the documentation audience. Capture grounded constraints in ARCHITECTURE.md. Record significant decisions as ADRs.
- **Define behavior**: behavior contracts from `bdd` are the authoritative source for API documentation. Public behaviors should be reflected in user-facing docs, not only test files.
- **Decompose**: `issue-craft` requires that user-facing changes include documentation expectations in acceptance criteria. The `documentation` skill defines what that means: identify which artifacts need creation or update and include those as acceptance criteria.
- **Execute**: inline documentation (doc comments, type annotations) is written alongside code during implementation. Doc comments are implementation work, not afterthought.
- **Verify**: `documentation-review` fires before `verification-before-completion`. Documentation accuracy is completion evidence.
- **Land**: CHANGELOG entry required for user-visible changes. Documentation coverage is recorded alongside behavior coverage.

### Handoff Rules

1. `documentation -> issue-craft`: user-facing changes include documentation updates as acceptance criteria.
2. `documentation -> verification-before-completion`: completion claims include documentation accuracy evidence.
3. `documentation -> land`: landing records documentation coverage status.

For fail conditions and anti-divergence rules, see [docs/architecture/topology-contract.md](docs/architecture/topology-contract.md).

## Root-Cause Investigation Thread

Root-cause investigation is a cross-cutting discipline. It fires at any stage when a test fails, behavior is unexpected, or any failure occurs — before proposing fixes. See `systematic-debugging` for the full methodology.

### Handoff Rules

1. `systematic-debugging -> test-first`: once root cause is established, hand off to `test-first` fix-bug to write a failing test and implement the fix.
2. `systematic-debugging -> ground`: when the 3-fix escalation rule fires, invoke `ground` to re-examine architectural assumptions.
3. `systematic-debugging -> third-force`: when investigation reveals the root cause is operational friction (missing tool, broken config, stale convention), hand off to `third-force`.

For fail conditions and anti-divergence rules, see [docs/architecture/topology-contract.md](docs/architecture/topology-contract.md).

## Friction Resolution Thread

Friction resolution is a cross-cutting discipline. It fires at any stage when the operational environment impedes progress. See `third-force` for the full methodology.

### Handoff Rules

1. `third-force -> documentation`: structural fixes that change operational instructions are reflected in CLAUDE.md or WORKFLOW.md.
2. `third-force -> issue-craft`: friction exceeding side-quest scope is filed as an issue, not deferred silently.

For fail conditions and anti-divergence rules, see [docs/architecture/topology-contract.md](docs/architecture/topology-contract.md).

## Issue-Based Development

The issue graph is the persistence layer across sessions. Agent sessions end, context windows close, agents rotate — the issue graph survives. Work from the graph, not from memory.

### Definitions

- **Issue graph**: the set of open issues and their dependency edges. It is the working memory of the project — not a backlog to be groomed, but the live map of what remains and what blocks what.
- **Unblocked**: an issue whose hard dependencies are all closed. Transitively unblocked means every ancestor in the dependency chain is closed.
- **Execution layer**: a set of issues that share no mutual dependencies and can be worked in parallel once their shared ancestors are closed. Layer 0 has no dependencies; layer 1 depends only on layer 0; and so on.
- **Session-sized**: an issue that one agent can complete — from reading context through passing verification — in a single focused session.

### State Source Rule

State is determined by reading issue content, not forge metadata (labels, columns). An issue's state is what its body and comments say it is.

### Issue Working States

| State       | Meaning                                      | Enters when                        | Exits when                          |
|-------------|----------------------------------------------|------------------------------------|-------------------------------------|
| draft       | Intent captured, not yet agent-executable     | Issue created without full criteria | Criteria, scope, and size filled in |
| ready       | Agent-executable and unblocked                | All fields complete, deps closed   | Session claims it                   |
| in-progress | Active session is working on it               | Session declares goal against it   | Session closes or blocks            |
| blocked     | Waiting on one or more open dependencies      | Dependency discovered or reopened  | All blocking issues closed          |
| closed      | All acceptance criteria verified              | Verified and merged                | Reopened for regression             |
| stale       | No progress for 14+ days while still open     | Clock expires                      | Resumed, split, or closed as wont-fix |

### Dependency Graph Format

Epics with 4+ tasks include a dependency graph in two representations:

1. **Mermaid diagram** (arrows mean "must complete before"):
   ```mermaid
   graph TD
     A[#1 task-title] --> B[#2 task-title]
     A --> C[#3 task-title]
     B --> D[#4 task-title]
     C --> D
   ```

2. **Layered text summary** (for machine readability):
   ```
   Layer 0 (no deps):  #1
   Layer 1 (needs 0):  #2, #3
   Layer 2 (needs 1):  #4
   ```

### Graph Maintenance

- **Stale detection**: flag issues with no progress comment for 14+ days. Resolution: resume, split into smaller work, or close as wont-fix with rationale.
- **Splitting**: when an in-progress issue exceeds session size, split remaining work into new issues and close the original with a pointer.
- **Merging**: when two issues converge on the same deliverable, merge into one and close the duplicate with a cross-reference.
- **Validation after mutation**: after adding, closing, splitting, or merging issues, verify the graph has no orphaned dependencies or cycles.

## Skill Routing Table

| Skill | Trigger |
|-------|---------|
| `using-groundwork` | session start, task initiation, or any moment requiring methodology orientation |
| `ground` | before creating designs/specs/architectures/processes |
| `research` | when reliable external evidence is needed for decisions |
| `bdd` | when defining or refining behavior expectations |
| `issue-craft` | creating/refining task/epic/bug/spike issues |
| `begin` | initiating a work session: selecting work, preparing workspace, declaring direction |
| `plan` | implementation needs design convergence — multiple approaches, unclear scope, or cross-cutting changes |
| `test-first` | when implementing any feature or bugfix — RED → GREEN → REFACTOR |
| `systematic-debugging` | when a test fails or behavior is unexpected, before proposing any fix |
| `third-force` | operational friction — missing tools, broken configs, stale conventions, undocumented requirements |
| `documentation` | after code changes that may cause drift, at project initialization, when architectural decisions are made, or when docs fail the audience test |
| `verification-before-completion` | before claiming work is complete, fixed, or passing — evidence first |
| `propose` | packaging changes for review: `propose`, `submit pr`, `create pr`, `open pr`, `send for review` |
| `land` | merge-and-close completion events: `land`, `merge and close`, `ship it` |

