# Groundwork Workflow (v0.1)

Groundwork is a full-path methodology from first problem framing to shipped change.

This workflow intentionally combines:
- **Original Groundwork skills** for cognitive discipline, research discipline, behavioral contracts, decomposition, and closure.
- **Upstream execution skills** for high-discipline implementation and verification.

## Philosophy

Groundwork gives you a full-path methodology from first problem framing to shipped change:

- Start from what the work must enable, not from inherited implementation patterns.
- Maintain a stable behavioral contract from first framing through verification.
- Persist intent and progress across sessions via the issue graph, so any agent can resume from truthful state.
- Execute and verify against that contract with implementation discipline.
- Close cleanly with evidence of what shipped and what remains.

## Cross-Cutting Thread: Behavioral Contract (`bdd`)

`bdd` is not a one-time pre-implementation step.

`bdd` runs through the full pipeline:
- **Specification**: define behavior in Given/When/Then terms.
- **Execution**: `test-driven-development` implements those behaviors via RED-GREEN-REFACTOR.
- **Verification**: completion evidence is checked against those behaviors, not just command success.

Handoff rules:
1. `bdd -> writing-plans`: every plan item maps to one or more behavior statements.
2. `bdd -> test-driven-development`: each RED test corresponds to a named behavior scenario.
3. `bdd -> verification-before-completion`: completion claims require behavior-level evidence.
4. `bdd -> land`: closure records behavior coverage status and remaining gaps, if any.

Canonical contract: [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md)

## Methodology Orientation

### `using-groundwork` (original)
Trigger: session start, task initiation, or any moment requiring methodology orientation.

Purpose:
- activate the full skill system as one connected methodology
- show how skills relate and hand off to each other
- provide routing guidance for which skill fits which situation

## Issue-Based Development

Issues are the persistence layer for multi-session work. Agent sessions are
bounded; the issue graph is not. Every meaningful intent, decision, and
dependency lives in issues so that any session — by any agent — can resume
from truthful state without relying on memory or context windows.

### Definitions

- **Issue graph**: the set of open issues and their dependency edges. It is the
  working memory of the project — not a backlog to be groomed, but the live map
  of what remains and what blocks what.
- **Unblocked**: an issue whose hard dependencies are all closed. Transitively
  unblocked means every ancestor in the dependency chain is closed.
- **Execution layer**: a set of issues that share no mutual dependencies and can
  be worked in parallel once their shared ancestors are closed. Layer 0 has no
  dependencies; layer 1 depends only on layer 0; and so on.
- **Session-sized**: an issue that one agent can complete — from reading context
  through passing verification — in a single focused session.

### Issue Working States

State is determined by reading issue content, not forge metadata (labels,
columns). An issue's state is what its body and comments say it is.

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

1. **Mermaid diagram** for visual reading (arrows mean "must complete before"):
   ```mermaid
   graph TD
     A[#1 task-title] --> B[#2 task-title]
     A --> C[#3 task-title]
     B --> D[#4 task-title]
     C --> D
   ```

2. **Layered text summary** for machine readability:
   ```
   Layer 0 (no deps):  #1
   Layer 1 (needs 0):  #2, #3
   Layer 2 (needs 1):  #4
   ```

### Graph Maintenance

- **Stale detection**: flag issues with no progress comment for 14+ days.
  Resolution: resume, split into smaller work, or close as wont-fix with
  rationale.
- **Splitting**: when an in-progress issue exceeds session size, split remaining
  work into new issues and close the original with a pointer.
- **Merging**: when two issues converge on the same deliverable, merge into one
  and close the duplicate with a cross-reference.
- **Validation after mutation**: after adding, closing, splitting, or merging
  issues, verify the graph has no orphaned dependencies or cycles.

## 1. Foundation

### `ground` (original)
Trigger: before creating designs/specs/architectures/processes.

Purpose:
- establish normative constraints (what must be true)
- separate requirements from inherited conventions
- reconstruct solutions from verified constraints

### `research` (original)
Trigger: when reliable external evidence is needed for decisions.

Purpose:
- run systematic multi-source research with citation discipline
- synthesize findings into actionable constraints for design and execution

## 2. Specification

### `bdd` (original, cross-cutting)
Trigger: when defining or refining behavior expectations.

Purpose:
- define behavior as executable expectations (Given/When/Then style)
- maintain a stable behavior contract from design through verification

## 3. Decomposition

### `planning` (original)
Trigger: selecting session-sized work from issue graph, or when a task feels too big to hold in one session.

Purpose:
- choose the next meaningful unit of progress and confirm it fits one session
- establish clear scope boundaries that prevent context overrun and task switching

### `issue-craft` (original)
Trigger: creating/refining task/epic/bug/spike issues.

Purpose:
- create agent-executable issues with binary acceptance criteria
- preserve dependency clarity

### Local Issue Mirroring

Issues are mirrored locally via `gh-issue-sync`. The `.issues/` directory is
gitignored — it is a working surface, not a second source of truth. Skills
sync at natural boundaries: pull before reading, push after writing.

## 4. Execution + Verification

Groundwork v0.1 includes these upstream Superpowers execution skills. Each has a Trigger — use it when you reach that moment in your session.

### `brainstorming`
Trigger: before designing a solution or making a significant architectural choice.

### `writing-plans`
Trigger: when you have a spec or requirements and need a structured implementation plan before touching code.

### `subagent-driven-development`
Trigger: when executing a plan whose tasks are independent and can run in parallel.

### `test-driven-development`
Trigger: when implementing any feature or bugfix. RED → GREEN → REFACTOR.

### `systematic-debugging`
Trigger: when a test fails or behavior is unexpected, before proposing any fix.

### `verification-before-completion`
Trigger: before claiming work is complete, fixed, or passing. Evidence first.

### `requesting-code-review`
Trigger: after implementation, before merging.

### `receiving-code-review`
Trigger: when receiving review feedback, before implementing suggestions.

### Contract Model

- Groundwork skills define constraints and the behavioral contract.
- Execution skills implement and verify that contract.
- Avoid coding without design pressure-testing, task execution drift, or TDD skipping.
- Require root-cause debugging, command evidence, and behavior mapping before completion claims.
- Seek and respond to code review with structural rigor.

## 5. Completion

### `land` (original)
Trigger: merge-and-close completion events (`land`, `merge and close`, `ship it`).

Purpose:
- close the branch and issue lifecycle cleanly
- ensure shipped changes are discoverable and traceable
- preserve what behavior coverage shipped and what remains

## Original vs Upstream Skills

- Original skills are maintained in this repository.
- Upstream skills are fetched from their source repositories at install time.

## Install and Run

```bash
groundwork init
```

Then run your task through one coherent pipeline:
1. Ground with `ground`
2. Define/refine behavior contract with `bdd`
3. Decompose with `planning` + `issue-craft`
4. Execute and verify the behavior contract with execution skills
5. Complete with `land`
