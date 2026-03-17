---
name: decompose
description: >-
  Transfer problem understanding across context boundaries through well-formed
  issues. Use for creating, decomposing, refining, triaging, and closing issues
  in GitHub projects.
requires: []
accepts: ["assessment", "behavior-contract", "implementation-plan"]
produces: []
may_produce: []
trigger:
  any_of:
    - on_artifact: "assessment"
    - on_signal: "decompose-work"
---

# Issue Craft

An issue transfers problem understanding across a context boundary — from the
person who sees the problem to the agent who will solve it. The agent has no
access to your context, your codebase familiarity, or your unstated assumptions.
Everything it needs must be in the issue.

For concrete templates, see [references/templates.md](references/templates.md).
For the issue state model and dependency graph format, see
[`issue-model.md`](../../docs/architecture/issue-model.md).

## The Central Discipline

**Issues describe what must be true, not how to get there.**

An issue that says "Replace X with Y" has already made the design decision. If
that decision is wrong, the implementing agent will faithfully execute the wrong
solution — and the further the prescription travels through decomposition,
planning, and implementation, the harder it is to catch. The issue author's job
is to describe the problem and the desired end state. The implementer's job is
to find the path.

This is not a stylistic preference. It is the structural defense against the
most common failure mode in issue-driven development.

## Guidelines

### Scope and sizing

**One concern per issue.** An issue that touches unrelated modules forces the
implementer to hold multiple problem contexts simultaneously and makes partial
completion ambiguous. When you notice scope creeping across boundaries, split.

**Session-sized work.** Each task issue should be completable in one focused
agent session — from reading context through passing verification. Oversized
issues cause context loss mid-execution; undersized issues create coordination
overhead that exceeds the work itself.

### Acceptance criteria

**Criteria describe outcomes, not activities.** "Refactor the parser" is an
activity — it passes when someone did something, not when something is true.
"Parser returns typed errors for malformed input" is an outcome — it passes when
a specific observable behavior exists. Every criterion should be verifiable by
running a command or inspecting an artifact.

**Include testing and documentation expectations.** If the change needs tests,
say what kind (unit, integration, behavior). If it affects user-facing behavior,
include documentation updates as criteria. Making these explicit prevents the
implementer from treating them as optional.

### Dependencies

**Explicit and hard only.** Dependencies are issue references (`#N`) that
represent true blockers — work that literally cannot proceed without the
dependency being complete. Preferred ordering is not a dependency. Soft
dependencies create false bottlenecks that serialize work unnecessarily.

### Epics and decomposition

**Vertical slices with dependency graphs.** Decompose epics into independently
shippable slices, not horizontal layers. Each slice delivers observable value.
For epics with 4+ tasks, include a dependency graph showing execution layers
(see [`issue-model.md`](../../docs/architecture/issue-model.md) § Dependency
Graph Format) so implementers can parallelize independent work.

### The issue as contract

**Must stand alone without external context.** An issue is a contract, not a
conversation. The implementer may be a different agent, in a different session,
with no access to the discussion that produced the issue. Summary states what
and why. Scope names concrete files or modules. Criteria are binary pass/fail.
Task issues reference their parent epic or milestone so the work has context
in the issue graph. If the issue requires reading a Slack thread or "seeing
the earlier discussion" to understand, it is incomplete.

## Procedures

### create-issue

1. Classify issue type (`task`, `epic`, `bug`, `spike`).
2. Write summary: what needs to exist and why. Not how.
3. Define scope with concrete files or modules.
4. Write acceptance criteria as observable outcomes — functional behavior,
   testing expectations, documentation updates where applicable.
5. Identify dependencies by searching existing issues. Link with `#N`.
6. Assemble using template from `references/templates.md`. Title format:
   `<type>(<scope>): <what>`.

A structural linter is available at `scripts/issue_lint.py` for validating
issue bodies against template schemas.

### decompose-epic

1. Extract deliverables — artifacts that must exist when done.
2. Split into vertical slices that are independently verifiable.
3. Group by module boundary where it clarifies ownership.
4. Build dependency graph (Mermaid `graph TD` + layered text summary —
   see [`issue-model.md`](../../docs/architecture/issue-model.md) § Dependency Graph Format).
5. Size-check each candidate: split if oversized, merge if trivial.
6. Create task issues in topological order (lowest execution layer first).
7. Create or update parent epic with task checklist and dependency graph.

### define-task-boundary

A well-bounded task has:

- **Title**: verb + object + short outcome
- **Scope**: concrete files or modules touched
- **Goal**: one sentence describing the observable outcome
- **Acceptance criteria**: binary pass/fail checks describing end state
- **Test plan**: exact verification command or scenario
- **Effort**: `small`, `medium`, or `large`

### refine-issue

1. Diagnose: vague summary, missing scope, untestable criteria, implicit
   dependencies, oversized scope, or prescription leaking into criteria.
2. Apply targeted fixes only where weak. Keep already-strong sections unchanged.
3. Re-verify the central discipline — does any criterion or scope statement
   prescribe an implementation approach?

### triage-issues

1. Refine non-ready issues first.
2. Build dependency graph for the backlog.
3. Create topological execution layers.
4. Apply labels (`size:*`, module/area).
5. Flag stale issues (no progress for 14+ days) for review. Resolution:
   resume, split, or close as wont-fix with rationale.

### close-issue

1. Verify all acceptance criteria against implementation.
2. Check scope deviations — split unintended extra work into new issues.
3. Update parent epic checklist.
4. Close with commit/PR reference (`Closes #N`).

## Triggers

- creating or refining issues
- decomposing large goals into executable work
- triaging or prioritizing a backlog
- closing completed work
- planning milestones or releases

## Corruption Modes

- `implicit-how`: implementation prescription leaks into scope or criteria.
  The issue says "use library X" or "replace A with B" instead of describing
  the required end state.
  *Recognition: read scope and criteria aloud — if they name tools, patterns,
  or implementation steps rather than observable outcomes, prescription has
  leaked in.*

- `activity-criteria`: criteria describe activities ("refactor", "clean up",
  "investigate") rather than outcomes. They pass when someone did something,
  not when something is true.
  *Recognition: ask "can I verify this by running a command or inspecting an
  artifact?" If no, it is an activity.*

- `dependency-blindness`: blockers exist but are not surfaced. The implementer
  discovers mid-session that prerequisite work is incomplete.
  *Recognition: before filing, ask "what must already be true for this work
  to start?" If the answer references unfinished work, that is a dependency.*

- `kitchen-sink-epic`: an epic that accumulates loosely related work until it
  is too large to reason about or track. No clear deliverable boundary.
  *Recognition: if you cannot state the epic's done condition in one sentence,
  it is too broad.*

- `premature-issues`: filing detailed task issues for work that depends on
  unresolved design decisions. The issues will need rewriting when the
  decisions land.
  *Recognition: if the issue's scope would change based on an open question,
  the question must be answered first (spike issue).*

- `graph-omission`: an epic with 4+ tasks has no dependency graph or layered
  execution order, forcing implementers to discover sequencing by reading
  every task.
  *Recognition: if someone asked "what can I work on right now?" and you
  cannot answer without reading all task bodies, the graph is missing.*

## Cross-References

- `begin`: session-level prioritization and execution discipline.
- `bdd`: behavior framing and test naming discipline.
- `plan`: design convergence before implementation.
- `land`: merge-and-close completion events.
- `documentation`: documentation updates as acceptance criteria for user-facing
  changes.
