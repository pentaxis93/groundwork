# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.1.0] — 2026-04-04

First release. Groundwork is a methodology plugin for
[runa](https://github.com/tesserine/runa) that encodes opinions about how
software should be built — from problem framing through shipped change — into
protocols, skills, and artifact schemas that a runa instance orchestrates.

### Protocol topology

- `manifest.toml` declares 12 artifact types and 10 protocols with their
  dependency edges (`requires`, `accepts`, `produces`, `may_produce`), trigger
  conditions, and scoping. This is the single file runa reads to understand
  the methodology.
- Two planning-phase protocols (unscoped): **survey** produces requirements
  from an external request; **decompose** produces session-sized issues with
  acceptance criteria and dependency edges.
- Eight execution-phase protocols (all `scoped = true`, work-unit threaded):
  **begin** claims an issue and opens the session → **specify** writes the
  behavior contract as Given/When/Then scenarios → **plan** converges on a
  decision-complete design → **implement** executes through RED-GREEN-REFACTOR
  → **verify** gates completion with behavior-level evidence → **document**
  ensures documentation accuracy → **submit** packages the change into a PR →
  **land** merges and closes the loop.
- Artifacts form a directed acyclic graph from request through
  completion-record. Execution order emerges from the dependency graph — it is
  not declared.
- All protocol triggers are `on_artifact`. No signal-based triggers.
- Three protocols (implement, plan, verify) carry upstream attribution via
  `LICENSE-UPSTREAM` files.
- Reference materials bundled with protocols: issue templates
  (`decompose/references/templates.md`), Given/When/Then language patterns for
  Rust, Python, TypeScript, Go, and Java
  (`specify/references/language-patterns.md`), and testing anti-patterns
  (`implement/references/testing-anti-patterns.md`).
- Structural linter (`decompose/scripts/issue_lint.py`) validates issue bodies
  against template schemas.

### Skills

- Six cross-cutting disciplines, agent-managed rather than runa-triggered.
  Each fires when its trigger condition matches the current work.
- **orient** — methodology map and documentation discipline. Loads the
  connected skill system at session start so subsequent skills operate as one
  methodology rather than in isolation.
- **reckon** — first-principles reasoning. Position and momentum as one act:
  establish what is actually needed, reason from ground with traced chains,
  catch inherited assumptions.
- **debug** — root-cause investigation before fixes. Stop, read, reproduce,
  trace, then hypothesize-and-test. Three-fix escalation rule: after three
  failed attempts, question the architecture. Carries upstream attribution via
  `LICENSE-UPSTREAM`.
- **resolve** — friction resolution through the reconciling force. When
  operational friction appears, stop and resolve structurally instead of
  routing around. Scope guidance distinguishes inline side quests from issues.
- **research** — systematic multi-source research with citations. Six-phase
  workflow (clarify, decompose, gather, evaluate, resolve, synthesize).
  Produces a typed artifact (research-record) that other protocols can accept.
- **contract** — behavior traceability through implementation and verification.
  Carries the behavior contract forward so tests, code, and completion claims
  map to named scenarios.
- Reference material: defense-in-depth validation pattern
  (`debug/references/defense-in-depth.md`).

### Artifact schemas

- Twelve JSON Schemas (draft 2020-12), one per artifact type declared in
  `manifest.toml`.
- Planning-phase artifacts carry no `work_unit` field: **request** (external
  input), **requirements** (scope, constraints, priorities),
  **issue** (work unit with acceptance criteria and dependencies).
- Execution-phase artifacts carry a `work_unit` envelope for runa's scoped
  validation: **claim** (threading root), **behavior-contract** (Given/When/Then
  scenarios), **implementation-plan** (design decisions, affected files,
  behavior mapping), **test-evidence** (results mapped to scenarios),
  **completion-evidence** (criterion-level coverage),
  **documentation-record** (docs reviewed and tracked), **patch** (PR
  reference), **completion-record** (final state with coverage summary and
  gaps).
- Cross-cutting: **research-record** with optional `work_unit` — produced by
  the research skill, accepted by survey, decompose, specify, and plan.
- Test fixtures (`tests/fixtures/artifacts/`) provide valid and invalid
  examples for all 12 artifact types.

### Architecture documentation

- `docs/architecture/connecting-structure.md` — artifact flow design, trigger
  semantics, work-unit scoping model, schema design rationale, and agent
  interface.
- `docs/architecture/issue-model.md` — issue working states (draft, ready,
  in-progress, blocked, closed, stale), dependency graph format, and graph
  maintenance rules.
- `docs/architecture/decisions/0001-internal-development-history-policy.md` —
  ADR establishing that repo artifacts document state, not transitions.
