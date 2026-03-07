---
name: using-groundwork
description: >-
  Use when working in a groundwork-equipped project, at session start, task
  initiation, or any moment requiring methodology orientation. Activates the
  full skill system as one connected methodology rather than isolated skills.
metadata:
  version: "2.0.0"
  updated: "2026-03-07"
---

# Using Groundwork

## Overview

Groundwork is one connected methodology, not a skill collection. Every skill closes a specific failure mode in the pipeline from problem framing to shipped change. This skill is the map that shows how they connect.

## The Flow

There is one path, not a menu. Every piece of work flows through five stages:

1. **Frame constraints.** `ground` establishes what the work must enable. Local issues (`.issues/`) mirror the forge — `gh-issue-sync pull` before reading, `push` after writing.

2. **Define behavior.** `bdd` defines the behavior contract in Given/When/Then scenarios. This contract threads through every subsequent step (see [Cross-Cutting Disciplines](#cross-cutting-disciplines)).

3. **Decompose.** `brainstorming` explores approaches before committing to a design. `plan` converges from exploration to a decision-complete implementation design. `issue-craft` decomposes that design into agent-executable issues with binary acceptance criteria. `next-issue` selects session-sized work from the issue graph.

4. **Execute and verify.**
   - `test-driven-development` implements through RED-GREEN-REFACTOR — each RED test maps to a named behavior scenario.
   - `subagent-driven-development` parallelizes independent tasks.
   - `systematic-debugging` finds root cause before proposing fixes.
   - `requesting-code-review` and `receiving-code-review` handle review.
   - `verification-before-completion` demands behavior-level evidence before any completion claim.
   - `documentation` ensures doc accuracy is verified before completion.

5. **Land.** `land` closes the loop: merge, cleanup, behavior coverage record, documentation coverage status, and issue closure.

## Why Issues Are Central

Agent sessions are bounded — context windows end, sessions close, agents
rotate. Issues are the persistence layer that survives those boundaries. The
issue graph is the project's working memory: it holds what remains to be done,
what blocks what, and what state each piece of work is in. Working from the
issue graph instead of from memory is what makes multi-session progress
reliable. See [`WORKFLOW.md` § Issue-Based Development](../../WORKFLOW.md#issue-based-development) for operational definitions.

## Cross-Cutting Disciplines

**Ground re-fires.** `ground` is not step-one-once. New generative work mid-session requires re-grounding. The trigger is creation, not sequence position.

**Research fires at any stage.** `research` provides reliable external evidence when decisions depend on facts outside the codebase. It is not bound to a pipeline stage — framing, design, decomposition, and implementation can all require it.

**BDD threads the full pipeline:**
- `bdd` → `plan`: design decisions keep behavior coverage explicit
- `bdd` → `issue-craft`: work units map to named behavior
- `bdd` → `test-driven-development`: each RED test implements a behavior scenario
- `bdd` → `verification-before-completion`: evidence must be behavior-level, not just green tests
- `bdd` → `land`: closure records what behavior coverage shipped and what remains

**Documentation threads the full pipeline:**
- `documentation` → `issue-craft`: user-facing changes include doc updates as acceptance criteria
- `documentation` → `verification-before-completion`: completion claims include documentation accuracy evidence
- `documentation` → `land`: landing records documentation coverage status; user-visible changes require a CHANGELOG entry

For the formal handoff contracts, fail conditions, and anti-divergence rules, see [`docs/architecture/pipeline-contract.md`](../../docs/architecture/pipeline-contract.md).

## Sovereignty

Every handoff in the pipeline passes **outcomes** (WHAT must be true), never **implementation steps** (HOW to achieve it). Issues define acceptance criteria, not procedure. Plans define interfaces and decisions, not copy-paste instructions. This WHAT/HOW boundary is the anti-prescription guardrail.

When this boundary breaks, agents execute instructions instead of solving problems — and prescribed steps that encode wrong assumptions propagate unchallenged. Example: Issue #5 prescribed "Replace ATTRIBUTION.md with a standard NOTICE file." An implementing agent planned exactly that. But NOTICE is an Apache convention (wrong for an MIT project), and the file should have been deleted — the opposite of the prescription.

This skill teaches the map; agent judgment navigates it. The behavior contract must flow from `bdd` through execution to `land`, but skip what the work does not need, return to what it does.

## Routing

| Situation | Skill |
|-----------|-------|
| About to design, spec, or architect | `ground` |
| Outcomes unclear or behavior undefined | `bdd` |
| Need reliable external evidence | `research` |
| Creating, decomposing, or refining issues | `issue-craft` |
| Selecting next work from the issue graph | `next-issue` |
| Multiple approaches or unclear scope | `brainstorming`, then `plan` |
| Converging a design from explored options | `plan` |
| Implementing behavior | `test-driven-development` |
| Multiple independent tasks to parallelize | `subagent-driven-development` |
| Bug or unexpected failure | `systematic-debugging` |
| Preparing or responding to review | `requesting-code-review`, `receiving-code-review` |
| Documentation needs creation, review, or update | `documentation` |
| Done | `verification-before-completion`, then `land` |

**Curated skill override:** `brainstorming`'s upstream terminal handoff is overridden in this pipeline. After brainstorming, continue to `plan` (for design convergence) or `issue-craft` (for decomposition) according to need.

## Corruption Modes

- Treating groundwork as fixed sequential gates instead of a connected methodology
- Treating `bdd` as a one-time preface, ignoring its threading through execution and verification
- Executing without behavior traceability — tests pass but do not map to named behaviors
- Forgetting `ground` re-fires on new generative work mid-session
- Using individual skills by keyword match without methodology context
- Claiming completion with command output but no behavior-level evidence
- Working from memory or context instead of reading the issue graph
- Treating issues as documentation artifacts rather than the project's working memory
- Starting work without checking issue state and dependencies
- Prescribing HOW in handoffs instead of passing WHAT — issues with implementation steps instead of acceptance criteria
- Claiming completion without documentation review — drifted docs remain untracked
