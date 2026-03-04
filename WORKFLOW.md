# Groundwork Workflow

This is the integration manual for operating Groundwork's skills as one connected pipeline from problem framing to shipped change.

## Pipeline

There is one path, not a menu. Every piece of work flows through five stages: frame constraints, define behavior, decompose, execute and verify, land. Skills from different sources slot into this sequence. Each step produces an artifact or state change that the next step consumes.

### 1. Frame constraints

Invoke `ground` before creating any design, spec, architecture, or process. It establishes what the work must enable, decomposes to verified constraints, and strips assumed constraints. Its output — a set of grounded constraints — is the input to every subsequent design decision.

`ground` is not step-one-once. New generative work mid-session requires re-grounding. The trigger is creation, not sequence position.

### 2. Define the behavior contract

Invoke `bdd` to define the behavior contract in Given/When/Then scenarios. Each scenario names an observable behavior the system must exhibit. This contract is not a one-time artifact — it threads through every subsequent step. See [BDD Thread](#bdd-thread) for how it integrates with execution and verification.

`bdd` and `test-driven-development` are not alternatives. `bdd` defines what behavior must exist. `test-driven-development` is the execution discipline that realizes it.

### 3. Decompose into executable work

Use `next-issue` to select session-sized work from the issue graph. It reads unblocked issues, ranks by value and unblock leverage, and declares a session goal with a binary done condition and explicit scope gate.

Use `issue-craft` to create, decompose, refine, and close issues. It produces agent-executable issues with binary acceptance criteria, explicit dependencies, and bounded scope. For epics with 4+ tasks, it builds dependency graphs with execution layers.

Use `brainstorming` before designing a solution or making a significant architectural choice. It explores 2-3 approaches with trade-offs and produces an approved design document.

Use `plan` to converge from exploration to a decision-complete implementation design before modifying code. It explores the codebase, resolves intent, and produces a plan where every design choice is explicit — the implementer does not need to make any decisions. Based on Codex CLI plan mode (MIT), adapted for autonomous execution.

Use `writing-plans` when you have a decision-complete design and need a structured implementation plan before touching code. It translates the design into bite-sized implementation steps — each step names exact files, code, commands, and expected output. Every plan item maps to one or more behavior statements from the `bdd` contract.

### 4. Execute and verify

Use `test-driven-development` to implement each plan step through RED-GREEN-REFACTOR. Each RED test corresponds to a named behavior scenario from `bdd`. Write one failing test, verify it fails for the right reason, write the simplest code to pass it, verify it passes, refactor.

Use `subagent-driven-development` when the plan contains independent tasks that can run in parallel. It dispatches a fresh subagent per task with two-stage review (spec compliance, then code quality).

Use `systematic-debugging` when a test fails or behavior is unexpected. It finds root cause before proposing fixes — no symptom-patching.

Use `requesting-code-review` after implementation, before merging. Use `receiving-code-review` when processing feedback — verify technically before implementing.

Use `verification-before-completion` before claiming any work is complete. It requires running the actual verification command and confirming the output matches the claim. Completion evidence must be behavior-level — not just "tests pass" but explicit behavior coverage.

### 5. Land

Invoke `land` to close the loop. It merges the feature branch to main, pushes, deletes the branch (remote and local), posts a completion comment on the issue, and closes the issue. Closure records behavior coverage status and any remaining gaps.

Do not stop after merge. `land` means complete delivery: merged code, clean branches, closed issue.

For the formal handoff contracts, anti-divergence rules, and compliance checklist, see [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md).

## BDD Thread

BDD is a cross-cutting integration mechanism, not a pipeline phase. It runs through specification, execution, and verification:

- **Specification**: define behavior in Given/When/Then terms.
- **Execution**: `test-driven-development` implements those behaviors via RED-GREEN-REFACTOR.
- **Verification**: completion evidence is checked against those behaviors, not just command success.

### Handoff Rules

1. `bdd -> writing-plans`: every plan item maps to one or more behavior statements.
2. `bdd -> test-driven-development`: each RED test corresponds to a named behavior scenario.
3. `bdd -> verification-before-completion`: completion claims require behavior-level evidence.
4. `bdd -> land`: closure records behavior coverage status and remaining gaps, if any.

For fail conditions and anti-divergence rules, see [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md).

## Documentation Thread

Documentation is a cross-cutting communication discipline, not a pipeline phase. It threads through every stage:

- **Frame constraints**: `ground`'s Orient identifies who this serves and what it must enable — that output defines the documentation audience. Capture grounded constraints in ARCHITECTURE.md. Record significant decisions as ADRs.
- **Define behavior**: behavior contracts from `bdd` are the authoritative source for API documentation. Public behaviors should be reflected in user-facing docs, not only test files.
- **Decompose**: `issue-craft` requires `criteria-include-docs`. The `documentation` skill defines what that means: identify which artifacts need creation or update and include those as acceptance criteria.
- **Execute**: inline documentation (doc comments, type annotations) is written alongside code during implementation. Doc comments are implementation work, not afterthought.
- **Verify**: `documentation-review` fires before `verification-before-completion`. Documentation accuracy is completion evidence.
- **Land**: CHANGELOG entry required for user-visible changes. Documentation coverage is recorded alongside behavior coverage.

### Handoff Rules

1. `documentation -> issue-craft`: user-facing changes include documentation updates as acceptance criteria.
2. `documentation -> verification-before-completion`: completion claims include documentation accuracy evidence.
3. `documentation -> land`: landing records documentation coverage status.

For fail conditions and anti-divergence rules, see [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md).

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

### Local Issue Mirroring

Issues are mirrored locally via `gh-issue-sync`. The `.issues/` directory is gitignored — it is a working surface, not a second source of truth. Sync at natural boundaries: pull before reading, push after writing.

## Skill Routing Table

| Skill | Trigger |
|-------|---------|
| `using-groundwork` | session start, task initiation, or any moment requiring methodology orientation |
| `ground` | before creating designs/specs/architectures/processes |
| `research` | when reliable external evidence is needed for decisions |
| `bdd` | when defining or refining behavior expectations |
| `next-issue` | selecting session-sized work from issue graph, or when a task feels too big to hold in one session |
| `issue-craft` | creating/refining task/epic/bug/spike issues |
| `brainstorming` | before designing a solution or making a significant architectural choice |
| `plan` | implementation needs design convergence — multiple approaches, unclear scope, or cross-cutting changes |
| `writing-plans` | when you have a decision-complete design and need a structured implementation plan before touching code |
| `test-driven-development` | when implementing any feature or bugfix — RED → GREEN → REFACTOR |
| `subagent-driven-development` | when executing a plan whose tasks are independent and can run in parallel |
| `systematic-debugging` | when a test fails or behavior is unexpected, before proposing any fix |
| `requesting-code-review` | after implementation, before merging |
| `receiving-code-review` | when receiving review feedback, before implementing suggestions |
| `documentation` | after code changes that may cause drift, at project initialization, when architectural decisions are made, or when docs fail the audience test |
| `verification-before-completion` | before claiming work is complete, fixed, or passing — evidence first |
| `land` | merge-and-close completion events: `land`, `merge and close`, `ship it` |

## Install

```bash
groundwork init
```
