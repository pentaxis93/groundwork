---
name: using-groundwork
description: Methodology bootstrap for groundwork-equipped projects. Use at session start, task initiation, or any approach-framing moment to map the connected groundwork workflow, select relevant skills with judgment, and preserve handoffs between ground, bdd, decomposition, execution, and land.
metadata:
  version: "1.0.0"
  updated: "2026-03-03"
  origin: "Groundwork meta-skill connecting original and curated layers into one methodology map."
---

# Using Groundwork

Groundwork is one connected methodology, not a folder of isolated skills.
Use this skill to orient before choosing specific skills.

## Goal

Build a reliable map of how skills hand off so you can choose the right next skill
for the situation without mechanical phase enforcement.

## Methodology Map

1. `ground`: fire before any generative design move (spec, plan, architecture,
   process, migration approach, solution framing).
2. `bdd`: define behavior contracts and keep them live across the pipeline.
3. `planning` + `issue-craft`: choose and shape session-sized executable work
   with clear done conditions.
4. Curated execution layer (Superpowers): implement and verify using disciplined
   execution, including TDD and verification against behavior contracts.
5. `land`: close branch and issue lifecycle with behavior coverage visibility.

## Key Relationship Contracts

- `ground` is not "step 1 once". Re-fire whenever you are about to generate a
  new design-level artifact or framing decision.
- `bdd` is not specification-only. Behavior contracts remain active through
  implementation and verification.
- `bdd -> writing-plans`: plan items map to behavior statements.
- `bdd -> test-driven-development`: RED tests map to named behavior scenarios.
- `bdd -> verification-before-completion`: completion evidence includes behavior
  coverage, not only command status.
- `bdd -> land`: closure communicates shipped coverage and deferred gaps.

Canonical integration contract:
- [WORKFLOW.md](../../../WORKFLOW.md)
- [docs/architecture/pipeline-contract.md](../../../docs/architecture/pipeline-contract.md)

## How to Apply in Groundwork-Equipped Projects

Use this routing logic:

- If work requires new framing or design choices, run `ground` first.
- If expected outcomes are unclear or untestable, run `bdd` to define behavior.
- If work is too broad for a single session, run `planning` and/or `issue-craft`.
- If behavior is defined and work is executable, use curated execution skills
  while preserving behavior traceability.
- If implementation is complete and verified, run `land` for full closure.

## Sovereignty Guardrail

This skill teaches the map; it does not command rigid sequencing.
Agent judgment decides how to traverse the map based on the task, while
preserving handoff integrity and contract continuity.

## Corruption Modes

- Treating groundwork as fixed phase gates with no situational judgment.
- Treating `bdd` as a one-time preface and dropping behavior traceability.
- Executing tasks without mapping work and evidence back to behavior contracts.
- Forgetting to re-fire `ground` before new generative moves.

## Cross-References

- `ground`
- `bdd`
- `planning`
- `issue-craft`
- `land`
- [WORKFLOW.md](../../../WORKFLOW.md)
- [docs/architecture/pipeline-contract.md](../../../docs/architecture/pipeline-contract.md)
- [CURATED.md](../../../CURATED.md)
