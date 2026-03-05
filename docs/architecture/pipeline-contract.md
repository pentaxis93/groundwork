# Groundwork Pipeline Contract (v0.1)

This file defines the canonical integration contract for Groundwork's methodology pipeline.

## One Pipeline Rule

Groundwork has one coherent path:
1. `ground` frames constraints.
2. `bdd` defines and maintains behavior contract.
3. `issue-craft` decomposes executable work; `next-issue` initiates a work session (select, branch, draft PR); `plan` converges the implementation design.
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

## `documentation -> issue-craft`
Requirement:
- user-facing changes include documentation updates as explicit acceptance criteria.

Fail condition:
- issue for a user-facing change has no documentation criterion.

## `documentation -> verification-before-completion`
Requirement:
- completion claims include documentation accuracy evidence from `documentation-review`.

Fail condition:
- work claimed complete without documentation review; drifted docs remain untracked.

## `documentation -> land`
Requirement:
- landing records documentation coverage status: which docs were updated, which were verified accurate, which were flagged with tracking issues.

Fail condition:
- user-visible change landed without CHANGELOG entry or documentation coverage statement.

## `next-issue -> execution`
Requirement:
- workspace is prepared (feature branch and draft PR exist) before implementation begins.

Fail condition:
- implementation starts on main or without a draft PR referencing the issue(s).

## `next-issue <-> land`
Requirement:
- `next-issue` and `land` are symmetric bookends. `next-issue` opens work (branch, draft PR, direction). `land` closes it (merge, cleanup, close issue).

Fail condition:
- work initiated without preparation (no branch/PR) or closed without full delivery (merge without issue close).

## Anti-Divergence Rules

1. Do not document BDD as specification-only.
2. Do not present BDD and TDD as user-selectable alternatives.
3. Do not accept completion evidence that lacks behavior mapping.
4. Do not allow plan decomposition to lose behavior traceability.
5. Do not accept completion evidence that lacks documentation review.
6. Do not land user-visible changes without a CHANGELOG entry.
7. Do not treat stale documentation as authoritative over code behavior.
8. Do not begin implementation without a prepared workspace (branch and draft PR).

## Quick Compliance Checklist

- [ ] Behavior statements are explicitly defined.
- [ ] Plan steps reference behavior statements.
- [ ] Execution tests map to behavior statements.
- [ ] Verification cites behavior-level evidence.
- [ ] Completion records behavior coverage/gaps.
- [ ] Documentation review completed before verification.
- [ ] User-facing changes include CHANGELOG entry.
- [ ] Documentation coverage status recorded at completion.
- [ ] Feature branch and draft PR exist before implementation begins.
- [ ] Work initiated with preparation (next-issue) and closed with full delivery (land).
