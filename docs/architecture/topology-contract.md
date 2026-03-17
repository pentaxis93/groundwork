# Groundwork Topology Contract (v0.3)

The topology — stages, cross-cutting disciplines, routing, and entry rules — is defined in [README.md](../../README.md). This document specifies what must hold at each protocol and skill handoff for that topology to maintain coherence: what artifacts carry integration state, what each handoff requires, and what behaviors would break the system.

The authoritative source for protocol interfaces is `groundwork.toml`; this document captures the semantic contracts that the manifest's structural edges enforce.

## Artifact Flow

Eight artifact types carry the integration contract. Each has a JSON Schema in `schemas/`.

The forward-flow artifact chain:

```
behavior-contract → implementation-plan → test-evidence → completion-evidence → completion-record
   (specify)            (plan)              (test)          (verify)               (land)
```

`research-record`, `assessment`, and `documentation-record` are the cross-cutting artifacts — produced by `research`, `survey`, and `document`, then accepted by downstream protocols and skills as needed.

Edge semantics from `groundwork.toml`:
- **requires** — artifact must exist and validate before protocol executes (hard dependency)
- **accepts** — artifact may be consumed if available (soft dependency)
- **produces** — artifact will exist and validate after protocol executes

## Handoff Contracts

### Forward-flow handoffs

#### `survey -> decompose`
Requirement: `survey` produces an assessment that gives `decompose` grounded context for issue selection, scope, and next actions.
Fail condition: issue decomposition starts from inherited momentum or guesswork when an assessment exists or was needed.

#### `specify -> plan`
Requirement: design decisions preserve explicit links to the behavior statements they implement.
Fail condition: an implementation design exists but its behavior coverage is implicit.

#### `plan -> test`
Requirement: the implementation plan informs execution — `test` consumes design decisions, not just behavior statements.
Fail condition: execution proceeds without consulting the implementation plan when one exists.

#### `specify -> test`
Requirement: each RED test corresponds to a named behavior scenario.
Fail condition: tests are created from implementation convenience rather than behavior contract.

#### `test -> verify`
Requirement: `test-evidence` is produced and validated before verification begins. Verification consumes both `test-evidence` (required) and `behavior-contract` (accepted) to assess coverage.
Fail condition: completion claimed without `test-evidence` artifact, or verification checks only command exit status without behavior mapping.

#### `verify -> propose`
Requirement: `completion-evidence` is produced and validated before changes are proposed for review.
Fail condition: PR created for unverified work — no `completion-evidence` exists.

#### `propose -> land`
Requirement: `propose` produces an open PR on a feature branch. `land` consumes that PR.
Fail condition: `land` invoked on a branch with no PR, falling back to local merge and losing PR merge metadata.

### Behavior-contract thread

The `behavior-contract` produced by `specify` is accepted or required by five downstream protocols and skills. These contracts ensure behavior traceability is not lost as work flows through the topology.

#### `specify -> decompose`
Requirement: every executable work unit maps to one or more behavior statements.
Fail condition: issue acceptance criteria exist but no behavior mapping is stated.

#### `specify -> verify`
Requirement: completion claims include behavior-level evidence, not only command status.
Fail condition: "tests pass" without explicit behavior coverage statement.

#### `specify -> land`
Requirement: closure output identifies behavior coverage status and any deferred behavior gaps.
Fail condition: merged work with no behavior coverage summary.

### Cross-cutting handoffs

#### `research -> ground | specify | plan`
Requirement: when `research-record` exists, consuming skills and protocols incorporate its evidence rather than re-deriving or assuming.
Fail condition: a design or behavior decision contradicts available `research-record` without stated rationale.

#### `document -> decompose`
Requirement: user-facing changes include documentation updates as explicit acceptance criteria.
Fail condition: issue for a user-facing change has no documentation criterion.

#### `document -> verify`
Requirement: when the change affects documented behavior or user-facing surfaces, completion claims include documentation accuracy evidence via `documentation-record`.
Fail condition: work with documentation impact claimed complete without documentation review; drifted docs remain untracked.

#### `document -> land`
Requirement: landing records documentation coverage status: which docs were updated, which were verified accurate, which were flagged with tracking issues.
Fail condition: user-visible change landed without CHANGELOG entry or documentation coverage statement.

#### `debug -> test`
Requirement: root cause is established before a fix is attempted through `test`. Investigation output identifies the specific cause and the transition point where valid data becomes invalid.
Fail condition: fix-bug entered without root-cause analysis when the cause was unclear — agent proposed a fix from symptoms alone.

#### `debug -> ground`
Requirement: the 3-fix escalation rule triggers architectural re-examination via `ground` when 3 fix attempts have failed. The debugging scope ends and the architectural scope begins.
Fail condition: fourth fix attempt without questioning the architecture — agent continued to apply fixes past the point where the methodology indicated an architectural problem.

#### `resolve -> document`
Requirement: structural fixes that change operational instructions are reflected in CLAUDE.md or relevant documentation.
Fail condition: tool installed or config fixed without updating relevant documentation.

#### `resolve -> decompose`
Requirement: friction that exceeds side-quest scope is filed as an issue, not deferred silently.
Fail condition: agent applied a workaround without filing an issue for the structural fix.

## Anti-Divergence Rules

1. Do not document `specify` as specification-only.
2. Do not present `specify` and TDD as user-selectable alternatives.
3. Do not accept completion evidence that lacks behavior mapping.
4. Do not allow plan decomposition to lose behavior traceability.
5. Do not accept completion evidence that lacks documentation review.
6. Do not land user-visible changes without a CHANGELOG entry.
7. Do not treat stale documentation as authoritative over code behavior.
8. Do not route around operational friction — resolve it structurally via `resolve` or file an issue. Workarounds compound.
9. Do not propose fixes without root-cause investigation when the cause is unclear.

## Quick Compliance Checklist

- [ ] Behavior statements are explicitly defined (`specify`).
- [ ] Design and issue decomposition reference behavior statements (`plan`, `decompose`).
- [ ] Execution tests map to behavior statements (`test`).
- [ ] Verification cites behavior-level evidence (`verify`).
- [ ] Completion records behavior coverage and gaps (`land`).
- [ ] Documentation review completed before verification (`document`).
- [ ] User-facing changes include CHANGELOG entry (`document`).
- [ ] Documentation coverage status recorded at completion (`land`).
- [ ] Root-cause investigation completed before fix when cause was unclear (`debug`).
- [ ] Operational friction resolved structurally or filed as issue (`resolve`).
