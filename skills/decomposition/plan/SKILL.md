---
name: plan
description: >-
  Use when an implementation task needs design convergence before code
  changes — multiple valid approaches exist, scope is unclear, or
  modifications cut across multiple files or subsystems.
metadata:
  version: "1.0.0"
  updated: "2026-03-04"
  origin: >-
    Adapted from Codex CLI plan mode (MIT license,
    github.com/openai/codex). The original is a human-interactive UI
    mode with 3-phase conversational workflow. This skill preserves
    the core discipline — explore before deciding, decision-complete
    plans, read-only until converged — and adapts it for autonomous
    execution where ambiguity is resolved through exploration and
    explicit assumptions rather than human conversation.
---

# Plan

Converge from exploration to a decision-complete implementation design before
modifying code. A decision-complete plan leaves no design choices to the
implementer — every approach, interface, data flow, edge case, and test
strategy is resolved.

For first-principles constraint framing, use `ground`.
For behavior contracts, use `bdd`.
For step-by-step execution breakdown after design convergence, use
`writing-plans`.

## Goal

Produce a plan detailed enough that a separate agent could implement it
without making any design decisions.

## Constraints

- `read-only-until-converged`: do not modify repo-tracked files during
  planning. Exploration — reads, searches, static analysis, dry-runs,
  builds, tests that write only to caches or build artifacts — is allowed
  and encouraged. Mutations — edits, codegen, formatters that rewrite
  files, patches — are not. When in doubt: if the action would be described
  as "doing the work" rather than "planning the work," do not do it.

- `decision-complete`: the plan must resolve every design choice — approach,
  interfaces, data flow, edge cases, test strategy. If the implementer
  would need to decide something, the plan is not done.

- `explore-before-assuming`: eliminate unknowns by reading code, not by
  guessing. Search relevant files, inspect entrypoints, confirm current
  implementation shape before forming any opinion.

- `assumptions-are-explicit`: when exploration cannot resolve an unknown,
  decide with rationale and record it as an explicit assumption. Never
  leave a design choice implicit.

## Requirements

- `plan-has-title-and-summary`: the plan opens with a clear title and brief
  summary of what changes and why.
- `plan-names-key-changes`: list important interface, API, schema, or
  behavioral changes. Group by subsystem or behavior, not file-by-file.
  Mention files only to disambiguate non-obvious changes — prefer
  behavior-level descriptions over file inventories.
- `plan-includes-test-strategy`: describe how to verify the changes
  end-to-end — which tests to run, what new tests to write, what
  behaviors to confirm.
- `plan-records-assumptions`: list every assumption made where exploration
  could not determine the answer, with rationale for each choice.
- `plan-reuses-existing-patterns`: identify and reference existing
  functions, utilities, and conventions found during exploration.

## Procedures

### phase-1-ground-in-environment

Explore the codebase to build a concrete understanding of current state.
Eliminate unknowns by discovering facts, not by guessing.

1. Read the issue, task, or request to identify what must change.
2. Search for relevant files: entrypoints, configs, schemas, types,
   manifests, docs, and existing implementations of similar behavior.
3. Trace the code paths that the change will touch or interact with.
4. Identify existing patterns, utilities, and conventions to reuse.
5. Note what you still do not know after exploration.

Do not skip this phase. Planning from imagination produces plans that
collide with the actual codebase.

### phase-2-resolve-intent

Clarify what the change must achieve and what is out of scope.

1. State the goal and success criteria derived from the issue and
   exploration.
2. Identify in-scope and out-of-scope boundaries.
3. Surface constraints from the codebase: dependencies, API contracts,
   performance budgets, existing tests that must keep passing.
4. For each remaining ambiguity, classify and resolve:
   - **Discoverable fact** (repo/system truth): explore further. Search
     configs, manifests, entrypoints, schemas, types.
   - **Preference or tradeoff** (not discoverable): choose the option best
     supported by codebase evidence, record it as an explicit assumption
     with rationale.

### phase-3-converge-implementation

Design the implementation until decision-complete.

1. Choose the approach. When multiple valid approaches exist, compare
   trade-offs against the constraints from phase 2 and select one.
2. Specify interfaces: APIs, schemas, function signatures, data flow.
3. Identify edge cases and failure modes. Decide handling for each.
4. Define the test strategy: existing tests to preserve, new tests to
   write, verification commands to run.
5. Note any migration or compatibility concerns.
6. Decision-completeness check: does the plan leave any design choice
   to the implementer? If yes, resolve it or record an explicit
   assumption with rationale.

### write-plan

Record the converged design.

1. **Title** — what this plan achieves.
2. **Summary** — 1-3 sentences on the change and its rationale.
3. **Key changes** — grouped by subsystem or behavior. Mention paths
   only when needed to prevent ambiguity. Compress related changes into
   high-signal bullets.
4. **Test plan** — how to verify correctness end-to-end.
5. **Assumptions** — every unresolvable unknown with rationale for the
   chosen default.

Keep it concise. Prefer the minimum detail needed for implementation
safety. Omit repeated repo facts and edge cases that cannot cause
implementation mistakes.

## Two Kinds of Unknowns

1. **Discoverable facts** (repo/system truth): explore first. Before
   assuming, run targeted searches and check likely sources of truth —
   configs, manifests, entrypoints, schemas, types, constants. Never
   guess what you can read.

2. **Preferences and tradeoffs** (not discoverable): intent or
   implementation preferences that cannot be derived from exploration.
   Choose the option best supported by codebase evidence. Provide
   rationale. Record as explicit assumption.

## Corruption Modes

- `imagination-planning`: designing without reading the codebase first.
  The plan describes a system that does not match the actual code.
- `premature-mutation`: editing files before the plan is decision-complete.
  Implementation starts leaking into the planning phase.
- `implicit-assumption`: making a design choice without recording it.
  The implementer will face the same ambiguity the planner avoided.
- `analysis-paralysis`: exploring indefinitely without converging. Three
  rounds of targeted exploration is usually sufficient. Decide and move.
- `file-inventory-plan`: listing every file to touch instead of describing
  behavioral changes. Files are implementation detail; behavior is the
  contract.
- `incomplete-plan`: leaving a design choice unresolved ("TBD", "decide
  later", "depends"). Either resolve it or record an explicit assumption
  with rationale.

## Principles

- `facts-before-opinions`: exploration-derived evidence outweighs
  intuition. Read the code before forming an opinion about it.
- `decision-completeness-is-the-bar`: the plan is done when a separate
  agent could implement it without asking any questions.
- `minimum-viable-detail`: include enough to prevent implementation
  mistakes, no more. Expand only when ambiguity is dangerous.

## Cross-References

- `ground`: first-principles constraint framing — runs before plan when
  the problem space itself is unclear.
- `bdd`: behavior contract — provides the behavior statements the plan
  must implement.
- `writing-plans`: step-by-step execution breakdown — takes a
  decision-complete design and produces ordered implementation steps.
- `next-issue`: work selection — identifies which issue to plan for.
- `issue-craft`: issue quality — ensures the issue is agent-executable
  before planning begins.
