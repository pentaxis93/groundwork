---
name: using-groundwork
description: >-
  Use when working in a groundwork-equipped project, at session start, task
  initiation, or any moment requiring methodology orientation. Activates the
  full skill system as one connected methodology rather than isolated skills.
metadata:
  version: "2.1.0"
  updated: "2026-03-07"
---

# Using Groundwork

## Overview

Groundwork is one connected methodology, not a skill collection. Every skill closes a specific failure mode in the pipeline from problem framing to shipped change. This skill is the map that shows how they connect.

## Entry Point

Read the issue graph first. Whether starting a session, picking up work, or orienting mid-task, the issue graph tells you where you are: what's in progress, what's blocked, what's next. Work from the graph, not from memory.

## The Flow

There is one path, not a menu. Every piece of work flows through five stages:

1. **Frame constraints.** `ground` establishes what the work must enable. Local issues (`.issues/`) mirror the forge — `gh-issue-sync pull` before reading, `push` after writing. Trigger: you're about to design, spec, architect, or frame a problem.

2. **Define behavior.** `bdd` defines the behavior contract in Given/When/Then scenarios. This contract threads through every subsequent stage — planning, decomposition, testing, verification, and landing should all maintain behavior traceability. Trigger: outcomes are unclear or behavior is undefined.

3. **Decompose.** `brainstorming` explores approaches before committing to a design — when it completes, continue to `plan` (for design convergence) or `issue-craft` (for decomposition) according to need. `plan` converges from exploration to a decision-complete implementation design. `issue-craft` decomposes that design into agent-executable issues with binary acceptance criteria. `next-issue` selects session-sized work from the issue graph. Trigger: multiple approaches exist, scope is unclear, or work needs breaking down.

4. **Execute and verify.** `test-driven-development` implements through RED-GREEN-REFACTOR — each RED test maps to a named behavior scenario. When tasks are independent, `subagent-driven-development` parallelizes them. When something breaks unexpectedly, `systematic-debugging` finds root cause before proposing fixes — don't guess. `requesting-code-review` and `receiving-code-review` handle review. Before any completion claim, `verification-before-completion` demands behavior-level evidence, and `documentation` ensures doc accuracy is verified. Trigger: you're writing code, debugging, or claiming done.

5. **Land.** `land` closes the loop: merge, cleanup, behavior coverage record, documentation coverage status, and issue closure. Trigger: work is verified and ready to ship.

## Why Issues Are Central

Agent sessions are bounded — context windows end, sessions close, agents rotate. Issues are the persistence layer that survives those boundaries. The issue graph is the project's working memory: it holds what remains to be done, what blocks what, and what state each piece of work is in. Working from the issue graph instead of from memory is what makes multi-session progress reliable. See [`WORKFLOW.md` § Issue-Based Development](https://github.com/pentaxis93/groundwork/blob/main/WORKFLOW.md#issue-based-development) for operational definitions.

## Cross-Cutting Disciplines

**Ground re-fires.** `ground` is not step-one-once. New generative work mid-session requires re-grounding. The trigger is creation, not sequence position.

**Research fires at any stage.** `research` provides reliable external evidence when decisions depend on facts outside the codebase. Framing, design, decomposition, and implementation can all require it.

**BDD threads the full pipeline.** The behavior contract defined in stage 2 should be traceable at every subsequent stage. If you're planning, decomposing, testing, verifying, or landing, check that behavior traceability is intact — tests map to named scenarios, verification evidence is behavior-level, and landing records what coverage shipped.

**Documentation threads the full pipeline.** User-facing changes carry documentation obligations through the pipeline: acceptance criteria include doc updates, completion claims include doc accuracy evidence, and landing records documentation coverage status. User-visible changes require a CHANGELOG entry.

For the formal handoff contracts, fail conditions, and anti-divergence rules, see [`docs/architecture/pipeline-contract.md`](https://github.com/pentaxis93/groundwork/blob/main/docs/architecture/pipeline-contract.md).

## Sovereignty

Every handoff in the pipeline passes **outcomes** (WHAT must be true), never **implementation steps** (HOW to achieve it). Issues define acceptance criteria, not procedure. Plans define interfaces and decisions, not copy-paste instructions.

When this boundary breaks, agents execute instructions instead of solving problems — and prescribed steps that encode wrong assumptions propagate unchallenged. Example: Issue #5 prescribed "Replace ATTRIBUTION.md with a standard NOTICE file." An implementing agent planned exactly that — but NOTICE is an Apache convention (wrong for MIT), and the file should have been deleted.

This skill teaches the map; agent judgment navigates it. The behavior contract flows from `bdd` through execution to `land`, but skip what the work does not need, return to what it does.

## Corruption Modes

**Methodology misuse:** Treating groundwork as fixed sequential gates, or using individual skills by keyword match without methodology context. The flow is a connected path, not a checklist.

**Behavior traceability loss:** Treating `bdd` as a one-time preface, writing tests that pass but don't map to named behaviors, or claiming completion with command output but no behavior-level evidence.

**Issue discipline failure:** Working from memory instead of reading the issue graph, starting work without checking issue state and dependencies, or treating issues as documentation artifacts rather than the project's working memory.

**Sovereignty violation:** Prescribing HOW in handoffs instead of passing WHAT — issues with implementation steps instead of acceptance criteria.

**Documentation drift:** Claiming completion without documentation review — drifted docs remain untracked.
