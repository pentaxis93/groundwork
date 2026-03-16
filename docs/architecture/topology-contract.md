# Groundwork Topology Contract (v0.4)

The topology — stages, cross-cutting disciplines, protocol routing, and entry rules — is defined in [README.md](../../README.md). This document specifies what must hold at each protocol handoff for that topology to maintain coherence: what artifacts carry integration state, what each handoff requires, and what behaviors would break the system.

The authoritative source for protocol interfaces is `groundwork.toml`; this document captures the semantic contracts that the manifest's structural edges enforce.

## Two Visibility Channels

runa has two mechanisms for seeing workflow state. Each serves a distinct purpose.

**Artifacts** carry cognitive work products between protocols. A behavior contract captures the thinking that BDD performs; test-first consumes it to know what to test. The artifact's structure IS the cognitive act — filling in given/when/then IS thinking about behavior. Artifacts flow between methodology protocols and are validated by runa against JSON Schema.

**Signals** carry infrastructure events for lifecycle orchestration. A PR being created, a review completing, code being merged — these are external state changes, not cognitive work products. runa sees them through `on_signal` triggers (operator actions, webhooks, schedulers) without the methodology producing an artifact.

The distinction: if a protocol's primary output is a cognitive work product that a downstream protocol needs as input, it produces an artifact. If a protocol's primary output is an infrastructure state change (code merged, PR created), the infrastructure IS the record, and runa's visibility comes through signals. A methodology artifact duplicating infrastructure state would be a shadow copy — less authoritative than the source, prone to drift.

This is why `propose` and `land` produce no artifacts. Propose creates a PR; land merges code, closes issues, and updates documentation. Their outputs live in GitHub, which is more authoritative than any methodology artifact. runa reaches the next lifecycle stage through signals triggered by those infrastructure events, not by scanning for methodology artifacts.

## Artifact Flow

Five artifact types carry the integration contract. Each has a JSON Schema in `schemas/`.

The forward-flow artifact chain:

```
behavior-contract → implementation-plan → test-evidence → completion-evidence
     (bdd)              (plan)            (test-first)    (verification)
```

After completion-evidence, the workflow transitions from cognitive work products to infrastructure actions: `propose` (create PR), review (infrastructure), `land` (merge and close). These transitions are visible to runa through signals, not artifacts.

`research-record` is the cross-cutting artifact — produced by `research`, accepted (not required) by `ground`, `bdd`, and `plan`.

Edge semantics from `groundwork.toml`:
- **requires** — artifact must exist and validate before protocol executes (hard dependency)
- **accepts** — artifact may be consumed if available (soft dependency)
- **produces** — artifact will exist and validate after protocol executes

## Handoff Contracts

### Forward-flow handoffs

#### `bdd -> plan`
Requirement: design decisions preserve explicit links to the behavior statements they implement.
Fail condition: an implementation design exists but its behavior coverage is implicit.

#### `plan -> test-first`
Requirement: the implementation plan informs execution — `test-first` consumes design decisions, not just behavior statements.
Fail condition: execution proceeds without consulting the implementation plan when one exists.

#### `bdd -> test-first`
Requirement: each RED test corresponds to a named behavior scenario.
Fail condition: tests are created from implementation convenience rather than behavior contract.

#### `test-first -> verification-before-completion`
Requirement: `test-evidence` is produced and validated before verification begins. Verification consumes both `test-evidence` (required) and `behavior-contract` (accepted) to assess coverage.
Fail condition: completion claimed without `test-evidence` artifact, or verification checks only command exit status without behavior mapping.

#### `verification-before-completion -> propose`
Requirement: `completion-evidence` is produced and validated before changes are proposed for review. This is the last artifact handoff in the forward chain — propose consumes the artifact for enforcement (runa gates PR creation on verified completion) but produces no artifact itself.
Fail condition: PR created for unverified work — no `completion-evidence` exists.

#### `propose -> land`
Requirement: `propose` produces an open PR on a feature branch. `land` consumes that PR. This handoff crosses the artifact/signal boundary — the PR is an infrastructure object, not a methodology artifact. runa sees the transition through signals (e.g., `merge-ready`), not through artifact scanning.
Fail condition: `land` invoked on a branch with no PR, falling back to local merge and losing PR merge metadata.

### Behavior-contract thread

The `behavior-contract` produced by `bdd` is accepted or required by downstream protocols. These contracts ensure behavior traceability is not lost as work flows through the topology.

#### `bdd -> issue-craft`
Requirement: every executable work unit maps to one or more behavior statements.
Fail condition: issue acceptance criteria exist but no behavior mapping is stated.

#### `bdd -> verification-before-completion`
Requirement: completion claims include behavior-level evidence, not only command status.
Fail condition: "tests pass" without explicit behavior coverage statement.

### Cross-cutting handoffs

#### `research -> ground | bdd | plan`
Requirement: when `research-record` exists, consuming protocols incorporate its evidence rather than re-deriving or assuming.
Fail condition: a design or behavior decision contradicts available `research-record` without stated rationale.

#### `documentation -> issue-craft`
Requirement: user-facing changes include documentation updates as explicit acceptance criteria.
Fail condition: issue for a user-facing change has no documentation criterion.

#### `documentation -> verification-before-completion`
Requirement: completion claims include documentation accuracy evidence.
Fail condition: work claimed complete without documentation review; drifted docs remain untracked.

#### `documentation -> land`
Requirement: landing ceremony verifies documentation coverage: which docs were updated, which were verified accurate, which were flagged with tracking issues.
Fail condition: user-visible change landed without CHANGELOG entry or documentation coverage statement.

#### `debug -> test-first`
Requirement: root cause is established before a fix is attempted through `test-first`. Investigation output identifies the specific cause and the transition point where valid data becomes invalid.
Fail condition: fix-bug entered without root-cause analysis when the cause was unclear — agent proposed a fix from symptoms alone.

#### `debug -> ground`
Requirement: the 3-fix escalation rule triggers architectural re-examination via `ground` when 3 fix attempts have failed. The debugging scope ends and the architectural scope begins.
Fail condition: fourth fix attempt without questioning the architecture — agent continued to apply fixes past the point where the methodology indicated an architectural problem.

#### `third-force -> documentation`
Requirement: structural fixes that change operational instructions are reflected in CLAUDE.md or relevant documentation.
Fail condition: tool installed or config fixed without updating relevant documentation.

#### `third-force -> issue-craft`
Requirement: friction that exceeds side-quest scope is filed as an issue, not deferred silently.
Fail condition: agent applied a workaround without filing an issue for the structural fix.

## Anti-Divergence Rules

1. Do not document BDD as specification-only.
2. Do not present BDD and TDD as user-selectable alternatives.
3. Do not accept completion evidence that lacks behavior mapping.
4. Do not allow plan decomposition to lose behavior traceability.
5. Do not accept completion evidence that lacks documentation review.
6. Do not land user-visible changes without a CHANGELOG entry.
7. Do not treat stale documentation as authoritative over code behavior.
8. Do not route around operational friction — resolve it structurally via `third-force` or file an issue. Workarounds compound.
9. Do not propose fixes without root-cause investigation when the cause is unclear.

## Quick Compliance Checklist

- [ ] Behavior statements are explicitly defined (`bdd`).
- [ ] Design and issue decomposition reference behavior statements (`plan`, `issue-craft`).
- [ ] Execution tests map to behavior statements (`test-first`).
- [ ] Verification cites behavior-level evidence (`verification-before-completion`).
- [ ] Documentation review completed before verification (`documentation`).
- [ ] User-facing changes include CHANGELOG entry (`documentation`).
- [ ] Root-cause investigation completed before fix when cause was unclear (`debug`).
- [ ] Operational friction resolved structurally or filed as issue (`third-force`).

## Version History

- **v0.4** — March 16, 2026. Added "Two Visibility Channels" section articulating the artifact/signal architectural distinction. Removed `completion-record` from artifact chain — `land` and `propose` produce infrastructure state changes, not methodology artifacts. Updated handoff contracts accordingly. Removed `bdd -> land` and compliance items that referenced completion records.
- **v0.3** — Previous version. Six artifact types, completion-record in chain.
