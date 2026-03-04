---
name: using-groundwork
description: >-
  Use when working in a groundwork-equipped project, at session start, task
  initiation, or any moment requiring methodology orientation. Activates the
  full skill system as one connected methodology rather than isolated skills.
metadata:
  version: "1.0.0"
  updated: "2026-03-03"
---

# Using Groundwork

## Overview

Groundwork is one connected methodology, not a skill collection. Every skill closes a specific failure mode in the pipeline from problem framing to shipped change. This skill is the map that shows how they connect.

## The Flow

`ground` fires first — establishing what the work must enable. Local issues (`.issues/`) mirror the forge — `gh-issue-sync pull` before reading, `push` after writing. From grounded constraints, `bdd` defines the behavior contract — executable expectations threading through every step. `planning` and `issue-craft` decompose that contracted behavior into session-sized, agent-executable work. `writing-plans` translates behavior into implementation steps. `test-driven-development` implements them through RED-GREEN-REFACTOR — each RED test maps to a named behavior scenario. `subagent-driven-development` parallelizes independent tasks when the plan supports it. `verification-before-completion` demands behavior-level evidence before any completion claim. `land` closes the loop: merge, cleanup, and behavior coverage record.

## Why Issues Are Central

Agent sessions are bounded — context windows end, sessions close, agents
rotate. Issues are the persistence layer that survives those boundaries. The
issue graph is the project's working memory: it holds what remains to be done,
what blocks what, and what state each piece of work is in. Working from the
issue graph instead of from memory is what makes multi-session progress
reliable. See WORKFLOW.md § Issue-Based Development for operational definitions.

## Cross-Cutting Disciplines

**Ground re-fires.** `ground` is not step-one-once. New generative work mid-session requires re-grounding. The trigger is creation, not sequence position.

**BDD threads the full pipeline:**
- `bdd` → `writing-plans`: every plan item maps to named behavior
- `bdd` → `test-driven-development`: each RED test implements a behavior scenario
- `bdd` → `verification-before-completion`: evidence must be behavior-level, not just green tests
- `bdd` → `land`: closure records what behavior coverage shipped and what remains

## Routing

- About to design, spec, or architect? → `ground`
- Outcomes unclear or behavior undefined? → `bdd`
- Need reliable external evidence? → `research`
- Selecting next work from the issue graph? → `planning`
- Creating, decomposing, or refining issues? → `issue-craft`
- Ready to translate spec into steps? → `writing-plans`
- Implementing behavior? → `test-driven-development`
- Multiple independent tasks to parallelize? → `subagent-driven-development`
- Preparing or responding to review? → `requesting-code-review`, `receiving-code-review`
- Bug or unexpected failure? → `systematic-debugging`
- Creative exploration needed? → `brainstorming`
- Done? → `verification-before-completion`, then `land`

## Sovereignty

This skill teaches the map; agent judgment navigates it. The behavior contract must flow from `bdd` through execution to `land`, but skip what the work does not need, return to what it does.

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

