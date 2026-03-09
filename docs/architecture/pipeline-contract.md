# Groundwork Pipeline Contract (v0.2)

This file defines the canonical integration contract for Groundwork's methodology pipeline.

## One Pipeline Rule

Groundwork has one coherent path:
1. `ground` frames constraints.
2. `bdd` defines and maintains behavior contract.
3. `plan` converges from exploration to a decision-complete implementation design. `issue-craft` decomposes that design into agent-executable issues. `begin` initiates the work session: selects session-sized work, prepares the workspace, and declares direction.
4. Middle skills implement and verify the same behavior contract.
5. `land` closes work with behavior coverage visibility.

`bdd` and `test-first` are not alternatives.
- `bdd` defines behavior.
- `test-first` is the execution discipline that realizes behavior.

## Handoff Contracts

### `bdd -> plan`
Requirement:
- design decisions preserve explicit links to the behavior statements they implement.

Fail condition:
- an implementation design exists but its behavior coverage is implicit.

### `bdd -> issue-craft`
Requirement:
- every executable work unit maps to one or more behavior statements.

Fail condition:
- issue acceptance criteria exist but no behavior mapping is stated.

### `bdd -> test-first`
Requirement:
- each RED test corresponds to a named behavior scenario.

Fail condition:
- tests are created from implementation convenience rather than behavior contract.

### `bdd -> verification-before-completion`
Requirement:
- completion claims include behavior-level evidence, not only command status.

Fail condition:
- "tests pass" without explicit behavior coverage statement.

### `bdd -> land`
Requirement:
- closure output identifies behavior coverage status and any deferred behavior gaps.

Fail condition:
- merged work with no behavior coverage summary.

### `documentation -> issue-craft`
Requirement:
- user-facing changes include documentation updates as explicit acceptance criteria.

Fail condition:
- issue for a user-facing change has no documentation criterion.

### `documentation -> verification-before-completion`
Requirement:
- completion claims include documentation accuracy evidence from the `documentation` skill's review mode.

Fail condition:
- work claimed complete without documentation review; drifted docs remain untracked.

### `documentation -> land`
Requirement:
- landing records documentation coverage status: which docs were updated, which were verified accurate, which were flagged with tracking issues.

Fail condition:
- user-visible change landed without CHANGELOG entry or documentation coverage statement.

### `verification-before-completion -> propose`
Requirement:
- changes are verified before being proposed for review.

Fail condition:
- PR created for unverified work — no behavior coverage evidence exists.

### `propose -> land`
Requirement:
- `propose` produces an open PR on a feature branch. `land` consumes that PR.

Fail condition:
- `land` invoked on a branch with no PR, falling back to local merge and losing PR merge metadata.

### `systematic-debugging -> test-first`
Requirement:
- root cause is established before a fix is attempted through `test-first` fix-bug. Investigation output identifies the specific cause and the transition point where valid data becomes invalid.

Fail condition:
- fix-bug entered without root-cause analysis when the cause was unclear — agent proposed a fix from symptoms alone.

### `systematic-debugging -> ground`
Requirement:
- the 3-fix escalation rule triggers architectural re-examination via `ground` when 3 fix attempts have failed. The debugging scope ends and the architectural scope begins.

Fail condition:
- fourth fix attempt without questioning the architecture — agent continued to apply fixes past the point where the methodology indicated an architectural problem.

### `third-force -> documentation`
Requirement:
- structural fixes that change operational instructions are reflected in CLAUDE.md, CONTRIBUTING.md, or WORKFLOW.md.

Fail condition:
- tool installed or config fixed without updating relevant documentation.

### `third-force -> issue-craft`
Requirement:
- friction that exceeds side-quest scope is filed as an issue, not deferred silently.

Fail condition:
- agent applied a workaround without filing an issue for the structural fix.

## Anti-Divergence Rules

1. Do not document BDD as specification-only.
2. Do not present BDD and TDD as user-selectable alternatives.
3. Do not accept completion evidence that lacks behavior mapping.
4. Do not allow plan decomposition to lose behavior traceability.
5. Do not accept completion evidence that lacks documentation review.
6. Do not land user-visible changes without a CHANGELOG entry.
7. Do not treat stale documentation as authoritative over code behavior.
8. Do not collapse the triad by routing around operational friction — resolve it structurally or file an issue.
9. Do not propose fixes without root-cause investigation when the cause is unclear.

## Quick Compliance Checklist

- [ ] Behavior statements are explicitly defined.
- [ ] Design and issue decomposition reference behavior statements.
- [ ] Execution tests map to behavior statements.
- [ ] Verification cites behavior-level evidence.
- [ ] Completion records behavior coverage/gaps.
- [ ] Documentation review completed before verification.
- [ ] User-facing changes include CHANGELOG entry.
- [ ] Documentation coverage status recorded at completion.
- [ ] Root-cause investigation completed before fix when cause was unclear.
