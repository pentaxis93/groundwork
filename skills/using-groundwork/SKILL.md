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

`ground` fires first — establishing what the work must enable before any design begins. From grounded constraints, `bdd` defines the behavior contract: executable expectations that thread through every subsequent step. `planning` and `issue-craft` decompose that contracted behavior into session-sized, agent-executable work. The curated execution layer — `test-driven-development`, `writing-plans`, `subagent-driven-development`, `systematic-debugging` — implements and verifies the behavior contract through disciplined RED-GREEN-REFACTOR loops. `verification-before-completion` demands behavior-level evidence before any completion claim. `land` closes the loop: merge, cleanup, and behavior coverage record.

Each handoff carries the behavior contract forward.

## Cross-Cutting Disciplines

**Ground re-fires.** `ground` is not step-one-once. New generative work mid-session — a new design, a shifted scope, a fresh architecture decision — requires re-grounding. The trigger is creation, not sequence position.

**BDD threads the full pipeline:**
- `bdd` → `writing-plans`: every plan item maps to named behavior
- `bdd` → `test-driven-development`: each RED test implements a behavior scenario
- `bdd` → `verification-before-completion`: evidence must be behavior-level, not just green tests
- `bdd` → `land`: closure records what behavior coverage shipped and what remains

## Routing

- About to design, spec, or architect? → `ground`
- Outcomes unclear or behavior undefined? → `bdd`
- Need reliable external evidence? → `research`
- Work too broad for one session? → `planning` + `issue-craft`
- Behavior defined, work decomposed? → curated execution layer
- Bug or unexpected failure? → `systematic-debugging`
- Creative exploration needed? → `brainstorming`
- Done? → `verification-before-completion`, then `land`

## Sovereignty

This skill teaches the map. Agent judgment navigates it. Handoff integrity matters — the behavior contract must flow from `bdd` through execution to `land`. Rigid phase gates do not matter — skip what the work does not need, return to what it does.

## Corruption Modes

- Treating groundwork as fixed sequential gates instead of a connected methodology
- Treating `bdd` as a one-time preface, ignoring its threading through execution and verification
- Executing without behavior traceability — tests pass but do not map to named behaviors
- Forgetting `ground` re-fires on new generative work mid-session
- Using individual skills by keyword match without methodology context
- Claiming completion with command output but no behavior-level evidence
