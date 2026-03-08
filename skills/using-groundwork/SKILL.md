---
name: using-groundwork
description: >-
  Use when working in a groundwork-equipped project, at session start, task
  initiation, or any moment requiring methodology orientation. Activates the
  full skill system as one connected methodology rather than isolated skills.
metadata:
  version: "3.0.0"
  updated: "2026-03-07"
---

# Using Groundwork

## Overview

Groundwork is one connected methodology, not a skill collection. Every skill closes a specific failure mode in the pipeline from problem framing to shipped change. This skill is the map that shows how they connect.

## Entry Point

Read the issue graph first. Whether starting a session, picking up work, or orienting mid-task, the issue graph tells you where you are: what's in progress, what's blocked, what's next. Work from the graph, not from memory.

Why the issue graph matters: agent sessions are bounded — context windows end, sessions close, agents rotate. The issue graph is the persistence layer that survives those boundaries. It holds what remains to be done, what blocks what, and what state each piece of work is in. Working from the graph instead of from memory is what makes multi-session progress reliable.

See [`WORKFLOW.md` § Issue-Based Development](https://github.com/pentaxis93/groundwork/blob/main/WORKFLOW.md#issue-based-development) for operational definitions and local issue mirroring.

## The Flow

Five stages, in dependency order. Each produces what the next consumes.

1. **Frame constraints.** `ground` establishes what the work must enable — verified constraints, not inherited assumptions.

2. **Define behavior.** `bdd` defines the behavior contract in Given/When/Then scenarios. This contract threads through every subsequent stage.

3. **Decompose.** Converge to a decision-complete design (`plan`), break the design into agent-executable issues (`issue-craft`), and select session-sized work (`next-issue`).

4. **Execute and verify.** Implement through RED-GREEN-REFACTOR (`test-driven-development`), parallelize independent tasks (`subagent-driven-development`), find root cause before fixing (`systematic-debugging`), review code (`requesting-code-review`, `receiving-code-review`), verify behavior-level evidence before claiming done (`verification-before-completion`), ensure documentation accuracy (`documentation`), and package verified changes into a PR (`propose`).

5. **Land.** `land` closes the loop: merge, cleanup, behavior coverage record, documentation coverage status, and issue closure.

The stages are in order but not all required for every piece of work. Enter the pipeline where the work needs you. A bug fix with an existing issue enters at Execute. A new capability enters at Frame. The constraint is sequence — you can't land before executing — not completeness.

See [`WORKFLOW.md`](https://github.com/pentaxis93/groundwork/blob/main/WORKFLOW.md) for detailed skill descriptions and triggers.

## Integration Principles

These thread across the pipeline. They aren't phases — they're disciplines that engage when relevant and stay active from that point forward.

**Sovereignty.** Every handoff passes outcomes (WHAT must be true), never implementation steps (HOW to achieve it). Issues define acceptance criteria, not procedure. Plans define interfaces and decisions, not copy-paste instructions. Example: Issue #5 prescribed "Replace ATTRIBUTION.md with a standard NOTICE file." An implementing agent planned exactly that — but NOTICE is an Apache convention (wrong for MIT), and the file should have been deleted. This skill teaches the map; agent judgment navigates it.

**Behavior traceability.** The behavior contract from stage 2 should be traceable at every subsequent stage. Plans link design decisions to behavior statements. Issues map acceptance criteria to behaviors. Tests correspond to named scenarios. Verification cites behavior-level evidence. Landing records what coverage shipped.

**Documentation obligation.** User-facing changes carry documentation obligations: acceptance criteria include doc updates, completion claims include doc accuracy evidence, and landing records documentation coverage status. User-visible changes require a CHANGELOG entry.

**Ground re-fires.** `ground` is not step-one-once. New generative work mid-session requires re-grounding. The trigger is creation, not sequence position.

**Research fires at any stage.** `research` provides reliable external evidence when decisions depend on facts outside the codebase — framing, design, decomposition, and implementation can all require it.

**Introduce third force on friction.** Friction is a two-force collision: task momentum vs obstacle. Routing around is the collapsed triad — both forces lose. When operational friction appears — a missing tool, broken config, stale convention, undocumented requirement — stop and introduce the reconciling move: resolve it structurally before continuing. `third-force` provides the assessment methodology and scope guidance. Friction that exceeds side-quest scope becomes an issue via `issue-craft`. Unresolved friction compounds.

For the formal handoff contracts, fail conditions, and anti-divergence rules, see [`pipeline-contract.md`](https://github.com/pentaxis93/groundwork/blob/main/docs/architecture/pipeline-contract.md).

## Corruption Modes

**Methodology as gates.** Recognition: you're checking off skills in order regardless of whether the work needs them, refusing to use a later-stage skill because you haven't completed an earlier stage that doesn't apply, or invoking a skill because its name matches a keyword in the conversation rather than because the pipeline calls for it. The Flow is a connected path with entry points, not a checklist where every box must be ticked.

**Behavior traceability loss.** Recognition: your tests pass but you can't name which behavior scenario each test verifies, or your completion claim says "all tests pass" without mapping results to named behaviors. Treating `bdd` as a one-time preface rather than a contract that threads through execution.

**Issue discipline failure.** Recognition: you're deciding what to work on from the conversation or your own reasoning instead of reading the issue graph, or you've started implementation without checking whether the issue is blocked. The issue graph is the project's working memory — working from anything else means you're navigating from a snapshot that may already be stale.

**Sovereignty violation.** Recognition: your issue's acceptance criteria describe steps to perform rather than outcomes to verify, or your plan reads like a script to follow rather than decisions that constrain a solution space. When this fires, agents execute instructions instead of solving problems — and prescribed steps that encode wrong assumptions propagate unchallenged.

**Documentation drift.** Recognition: you're claiming completion but haven't checked whether the change affects any documentation, or you're aware of drifted docs but treating the update as separate future work. Drifted docs compound — each one trains readers to distrust all docs.
