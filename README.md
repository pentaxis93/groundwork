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

**3. Decompose** — `issue-craft` produces agent-executable issues with binary acceptance criteria from the behavior contract. `begin` selects session-sized work from the issue graph, prepares the workspace, and declares the session's direction. `plan` converges to a decision-complete implementation design. Approved designs become executable work through `issue-craft`. The issue graph is the project's working memory across sessions.

**4. Execute and verify** — `test-driven-development` implements behavior through RED-GREEN-REFACTOR — each RED test maps to a named scenario from stage 2. `systematic-debugging` finds root cause before proposing fixes. Code review and `verification-before-completion` gate completion with behavior-level evidence. `propose` packages verified changes into a PR with derived title/body and issue linkage.

**5. Land** — `land` closes the loop: merge, push, delete branch, comment on issue, close issue. Closure records behavior coverage and remaining gaps. Do not stop after merge.

For the full integration manual, see [WORKFLOW.md](WORKFLOW.md). For formal handoff contracts and anti-divergence rules, see [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md).
For the concise inventory and shipped order reference, see [`skills/skills.toml`](skills/skills.toml).

## Skills

| Skill | Stage | What it prevents |
|---|---|---|
| `ground` | Foundation | Inherited framing, anchoring, premature assumptions |
| `research` | Foundation | Unsubstantiated decisions, hallucinated facts |
| `bdd` | Specification | Vague specs, testing implementation instead of behavior |
| `issue-craft` | Decomposition | Non-executable tasks, vague acceptance criteria |
| `begin` | Decomposition | Recency drift, scope creep, blocker bypass |
| `plan` | Decomposition | Unclear scope, design choices left to implementer |
| `test-driven-development` | Execution | Implementation-first regressions |
| `subagent-driven-development` | Execution | Context drift in parallel work |
| `systematic-debugging` | Execution | Thrashing and symptom-fixing |
| `requesting-code-review` | Verification | Unreviewed changes reaching main |
| `receiving-code-review` | Verification | Performative agreement with review feedback |
| `verification-before-completion` | Verification | False completion claims without evidence |
| `documentation` | Verification | Drifted docs, missing artifact updates |
| `propose` | Delivery | Manual ad-hoc commit/push/PR between implementation and merge |
| `land` | Completion | Branch rot, unclosed issues, incomplete delivery |
| `using-groundwork` | Meta | Using skills in isolation instead of as a connected pipeline |

## Install

```bash
groundwork init
```

This reads `skills/skills.toml`, fetches skills from their upstream sources via [`sk`](https://github.com/nickarora/sk), populates `agents.toml`, and syncs skills into your agent's skill directory.
It also provisions `.groundwork/schemas/` with embedded artifact schemas and creates `.groundwork/artifacts/` for project artifacts.

Prerequisites: Node.js (for `sk`), Git, npm, and `gh` CLI.
`groundwork init` requires operational issue sync and will fail unless `gh-issue-sync status` reports a non-`never` `Last full pull`.
`gh-issue-sync` is auto-installed from a pinned release asset with SHA-256 verification when a supported OS/arch asset is available.
If issue sync touches GitHub Projects metadata, refresh GH scopes before first pull:
`gh auth refresh -h github.com -s read:project`.

### Commands

| Command | What it does | Flag |
|---------|-------------|------|
| `groundwork init` | Reads `skills/skills.toml`, populates `agents.toml`, fetches skills via `sk sync`, bootstraps issue mirror tooling, and requires a successful `gh-issue-sync` full pull (`Last full pull` must be non-`never`) before succeeding | `--dry-run` |
| `groundwork update` | Re-syncs to the current shipped-skill manifest — upserts new or changed skills, prunes removed ones, and reconciles embedded schemas (create/update only; extras preserved) | `--dry-run` |
| `groundwork list` | Shows installed skills in shipped-manifest order, along with source repos and pinned refs from the lock file | |
| `groundwork doctor` | Checks prerequisites (`sk`, `gh`, `gh-issue-sync`, `agents.toml`, shipped-skill manifest), plus `.groundwork/schemas/` completeness and drift, and reports status | |

Both `init` and `update` are idempotent. They reconcile the manifest against `agents.toml` and embedded schemas, writing only what changed. State is tracked in `.groundwork/installed.lock.toml`.
If tool version capture fails, `init`/`update` abort instead of writing unsubstantiated lock provenance.

Issue sync troubleshooting:
- If `groundwork doctor` reports "local issue mirror has never completed a full pull", run:
  `gh auth refresh -h github.com -s read:project`
  `gh-issue-sync pull`
  `gh-issue-sync status`

## Project Layout

```
skills/                     # Groundwork's tracked skills and shipped inventory
  skills.toml               #   authoritative shipped-skill manifest
  using-groundwork/         #   methodology orientation
  ground/                   #   first-principles grounding
  research/                 #   external evidence gathering
  bdd/                      #   behavior contract definition
  plan/                     #   design convergence
  issue-craft/              #   issue lifecycle
  begin/                    #   work initiation
  documentation/            #   documentation review/update
  propose/                  #   commit, push, PR creation
  land/                     #   closeout workflow

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

**Sovereignty.** Each boundary has an owner. Skills don't override agent judgment. Agents don't override human intent. The principle is fractal — it applies at every interface, not just the human-agent boundary.

## License

[MIT](LICENSE)
