# Workflow Core Reference

## Behavioral Contract: BDD Thread

### Pipeline Phases

`bdd` runs through the full pipeline:
- **Specification**: define behavior in Given/When/Then terms.
- **Execution**: `test-driven-development` implements those behaviors via RED-GREEN-REFACTOR.
- **Verification**: completion evidence is checked against those behaviors, not just command success.

### Handoff Rules

1. `bdd -> writing-plans`: every plan item maps to one or more behavior statements.
2. `bdd -> test-driven-development`: each RED test corresponds to a named behavior scenario.
3. `bdd -> verification-before-completion`: completion claims require behavior-level evidence.
4. `bdd -> land`: closure records behavior coverage status and remaining gaps, if any.

### Canonical Reference

[docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md)

## Skill Routing Table

| Skill | Trigger |
|-------|---------|
| `using-groundwork` | session start, task initiation, or any moment requiring methodology orientation |
| `ground` | before creating designs/specs/architectures/processes |
| `research` | when reliable external evidence is needed for decisions |
| `bdd` | when defining or refining behavior expectations |
| `planning` | selecting session-sized work from issue graph, or when a task feels too big to hold in one session |
| `issue-craft` | creating/refining task/epic/bug/spike issues |
| `brainstorming` | before designing a solution or making a significant architectural choice |
| `writing-plans` | when you have a spec or requirements and need a structured implementation plan before touching code |
| `subagent-driven-development` | when executing a plan whose tasks are independent and can run in parallel |
| `test-driven-development` | when implementing any feature or bugfix — RED → GREEN → REFACTOR |
| `systematic-debugging` | when a test fails or behavior is unexpected, before proposing any fix |
| `verification-before-completion` | before claiming work is complete, fixed, or passing — evidence first |
| `requesting-code-review` | after implementation, before merging |
| `receiving-code-review` | when receiving review feedback, before implementing suggestions |
| `land` | merge-and-close completion events: `land`, `merge and close`, `ship it` |

## Issue-Based Development

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

## Dependency Graph Format

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

## Graph Maintenance

- **Stale detection**: flag issues with no progress comment for 14+ days. Resolution: resume, split into smaller work, or close as wont-fix with rationale.
- **Splitting**: when an in-progress issue exceeds session size, split remaining work into new issues and close the original with a pointer.
- **Merging**: when two issues converge on the same deliverable, merge into one and close the duplicate with a cross-reference.
- **Validation after mutation**: after adding, closing, splitting, or merging issues, verify the graph has no orphaned dependencies or cycles.

## Local Issue Mirroring

Issues are mirrored locally via `gh-issue-sync`. The `.issues/` directory is gitignored — it is a working surface, not a second source of truth. Skills sync at natural boundaries: pull before reading, push after writing.

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
