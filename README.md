# Groundwork

A methodology library for AI coding agents. One connected pipeline from problem framing through shipped change to closed loop.

## The Problem

AI agents fail in predictable ways between receiving a task and delivering working code:

- **Inherited framing** — accepting problem statements without questioning scope, premises, or fit
- **Premature generation** — coding before the design exists
- **Vague specifications** — behavior contracts that don't survive contact with implementation
- **Non-executable work** — issues that agents can't complete without clarification
- **Unverified claims** — declaring "done" without behavior-level evidence
- **Incomplete shipping** — merged code with no closure, no cleanup, no record

These aren't random. They're structural failure modes of agents operating without cognitive discipline. Groundwork prevents each one with a specific skill at the point where the failure occurs.

## The Pipeline

There is one path, not a menu. Every piece of work flows through five stages:

**1. Frame constraints** — `ground` establishes what the work must enable before any design begins. It strips inherited assumptions and builds from verified constraints. This fires on every new generative act, not just once at the start.

**2. Define behavior** — `bdd` defines the behavior contract in Given/When/Then scenarios. This contract threads through every subsequent stage — it is the integration mechanism, not a planning artifact.

**3. Decompose** — `next-issue` selects session-sized work from the issue graph. `issue-craft` produces agent-executable issues with binary acceptance criteria. `plan` converges to a decision-complete implementation design. The issue graph is the project's working memory across sessions.

**4. Execute and verify** — Curated skills from [Superpowers](https://github.com/obra/superpowers) handle the middle: `test-driven-development`, `systematic-debugging`, `subagent-driven-development`, `verification-before-completion`, and code review. Each RED test maps to a named behavior scenario from stage 2.

**5. Land** — `land` closes the loop: merge, push, delete branch, comment on issue, close issue. Closure records behavior coverage and remaining gaps. Do not stop after merge.

For the full integration manual, see [WORKFLOW.md](WORKFLOW.md). For formal handoff contracts and anti-divergence rules, see [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md).

## Skills

Groundwork ships 9 original skills and curates 9 from [Superpowers](https://github.com/obra/superpowers).

### Original

| Skill | Stage | What it prevents |
|---|---|---|
| `ground` | Foundation | Inherited framing, anchoring, premature assumptions |
| `research` | Foundation | Unsubstantiated decisions, hallucinated facts |
| `bdd` | Specification | Vague specs, testing implementation instead of behavior |
| `next-issue` | Decomposition | Recency drift, scope creep, blocker bypass |
| `issue-craft` | Decomposition | Non-executable tasks, vague acceptance criteria |
| `plan` | Decomposition | Unclear scope, design choices left to implementer |
| `documentation` | Verification | Drifted docs, missing artifact updates |
| `land` | Completion | Branch rot, unclosed issues, incomplete delivery |
| `using-groundwork` | Meta | Using skills in isolation instead of as a connected pipeline |

### Curated from Superpowers

| Skill | What it prevents |
|---|---|
| `brainstorming` | Coding before design is approved |
| `writing-plans` | Vague execution plans without file-level specificity |
| `writing-skills` | Deploying untested process documentation |
| `test-driven-development` | Implementation-first regressions |
| `subagent-driven-development` | Context drift in parallel work |
| `systematic-debugging` | Thrashing and symptom-fixing |
| `verification-before-completion` | False completion claims without evidence |
| `requesting-code-review` | Unreviewed changes reaching main |
| `receiving-code-review` | Performative agreement with review feedback |

## Install

```bash
groundwork init
```

This reads the curated manifest, fetches skills from their upstream sources via [`sk`](https://github.com/nickarora/sk), populates `agents.toml`, and syncs skills into your agent's skill directory.

Prerequisites: Node.js (for `sk`). Optional: `gh-issue-sync` (auto-installed if `curl` or `go` is available).

### Other commands

```bash
groundwork update    # Re-sync to latest manifest
groundwork list      # Show installed skills and versions
groundwork doctor    # Check prerequisites and configuration
```

## Project Layout

```
skills/                     # Original Groundwork skills
  foundation/               #   ground, research
  specification/            #   bdd
  decomposition/            #   next-issue, issue-craft, plan
  completion/               #   land
  verification/             #   documentation
  using-groundwork/         #   methodology orientation

manifests/
  curation.v1.toml          # Curated upstream skills with pinned refs

crates/
  groundwork-cli/           # Rust installer (groundwork init/update/list/doctor)

docs/
  architecture/             # Pipeline contract, integration rules
  research/                 # Ecosystem analysis, design rationale

WORKFLOW.md                 # Integration manual — the authoritative reference
agents.toml                 # Skill system configuration (sk-compatible)
```

## Design Principles

**One pipeline, not a menu.** Skills are not independently selectable utilities. They form a single path with handoff contracts between stages. Skipping a stage means the next stage receives malformed input.

**BDD threads everything.** Behavior contracts defined in stage 2 thread through planning, execution, verification, and closure. Completion evidence is behavior-level, not "tests pass."

**Issues are working memory.** Agent sessions end. Context windows close. The issue graph survives. Work from the graph, not from memory.

**Ground re-fires.** `ground` is not step-one-once. Any new generative work — a design, a spec, an architecture — requires re-grounding. The trigger is creation, not sequence position.

**Sovereignty.** Humans own WHAT. Agents own HOW.

## License

[MIT](LICENSE)
