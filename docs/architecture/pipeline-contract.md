# Groundwork Pipeline Contract (v0.1)

This file defines the canonical integration contract for Groundwork's methodology pipeline.

## One Pipeline Rule

Groundwork has one coherent path:
1. `ground` frames constraints.
2. `bdd` defines and maintains behavior contract.
3. `planning` + `issue-craft` decompose executable work.
4. Curated middle skills implement and verify the same behavior contract.
5. `land` closes work with behavior coverage visibility.

`bdd` and `test-driven-development` are not alternatives.
- `bdd` defines behavior.
- `test-driven-development` is the execution discipline that realizes behavior.

## Handoff Contracts

## `bdd -> writing-plans`
Requirement:
- every implementation task maps to one or more behavior statements.

Fail condition:
- plan tasks exist but no behavior mapping is stated.

## `bdd -> test-driven-development`
Requirement:
- each RED test corresponds to a named behavior scenario.

Fail condition:
- tests are created from implementation convenience rather than behavior contract.

## `bdd -> verification-before-completion`
Requirement:
- completion claims include behavior-level evidence, not only command status.

Fail condition:
- "tests pass" without explicit behavior coverage statement.

## `bdd -> land`
Requirement:
- closure output identifies behavior coverage status and any deferred behavior gaps.

Fail condition:
- merged work with no behavior coverage summary.

## Anti-Divergence Rules

1. Do not document BDD as specification-only.
2. Do not present BDD and TDD as user-selectable alternatives.
3. Do not accept completion evidence that lacks behavior mapping.
4. Do not allow plan decomposition to lose behavior traceability.

## Quick Compliance Checklist

- [ ] Behavior statements are explicitly defined.
- [ ] Plan steps reference behavior statements.
- [ ] Execution tests map to behavior statements.
- [ ] Verification cites behavior-level evidence.
- [ ] Completion records behavior coverage/gaps.
