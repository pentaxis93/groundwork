# Groundwork

A methodology plugin for AI coding agents. One connected topology from problem framing through shipped change to closed loop.

Groundwork is the methodology layer in a three-layer stack:

- **Daemon** — the orchestration surface (e.g., Claude Code, Codex CLI)
- **Runtime** — [runa](https://github.com/pentaxis93/runa) monitors artifacts, evaluates triggers, enforces contracts
- **Methodology** — groundwork defines *what* cognitive discipline agents follow; runa enforces *when* and *whether*

Groundwork owns the skills, artifact schemas, and topology. It has no runtime, no CLI, no installer. It registers with runa through a single [manifest file](groundwork.toml).

## The Problem

AI agents fail in predictable ways between receiving a task and delivering working code:

- **Inherited framing** — accepting problem statements without questioning scope, premises, or fit
- **Premature generation** — coding before the design exists
- **Vague specifications** — behavior contracts that don't survive contact with implementation
- **Non-executable work** — issues that agents can't complete without clarification
- **Unverified claims** — declaring "done" without behavior-level evidence
- **Incomplete shipping** — merged code with no closure, no cleanup, no record

These aren't random. They're structural failure modes of agents operating without cognitive discipline. Groundwork prevents each one with a specific skill at the point where the failure occurs.

## The Topology

Every piece of work flows through five stages:

**1. Frame constraints** — `ground` establishes what the work must enable before any design begins. It strips inherited assumptions and builds from verified constraints. This fires on every new generative act, not just once at the start.

**2. Define behavior** — `bdd` defines the behavior contract in Given/When/Then scenarios. This contract threads through every subsequent stage — it is the integration mechanism, not a planning artifact.

**3. Decompose** — `issue-craft` produces agent-executable issues with binary acceptance criteria from the behavior contract. `begin` selects session-sized work from the issue graph, prepares the workspace, and declares the session's direction. `plan` converges to a decision-complete implementation design. Approved designs become executable work through `issue-craft`. The issue graph is the project's working memory across sessions.

**4. Execute and verify** — `test-first` implements behavior through RED-GREEN-REFACTOR — each RED test maps to a named scenario from stage 2. `debug` finds root cause before proposing fixes. `verification-before-completion` gates completion with behavior-level evidence. `propose` packages verified changes into a PR with derived title/body and issue linkage.

**5. Land** — `land` closes the loop: merge, push, delete branch, comment on issue, close issue. Closure records behavior coverage and remaining gaps. Do not stop after merge.

For the full integration manual, see [WORKFLOW.md](WORKFLOW.md). For formal handoff contracts and anti-divergence rules, see [docs/architecture/topology-contract.md](docs/architecture/topology-contract.md).

## Skills

The [manifest](groundwork.toml) is the canonical inventory of all skills and their interface declarations.

| Skill | Stage | What it prevents |
|---|---|---|
| `ground` | Foundation | Inherited framing, anchoring, premature assumptions |
| `research` | Foundation | Unsubstantiated decisions, hallucinated facts |
| `bdd` | Specification | Vague specs, testing implementation instead of behavior |
| `issue-craft` | Decomposition | Non-executable tasks, vague acceptance criteria |
| `begin` | Decomposition | Recency drift, scope creep, blocker bypass |
| `plan` | Decomposition | Unclear scope, design choices left to implementer |
| `test-first` | Execution | Implementation-first regressions |
| `debug` | Cross-cutting | Thrashing and symptom-fixing |
| `verification-before-completion` | Verification | False completion claims without evidence |
| `documentation` | Verification | Drifted docs, missing artifact updates |
| `propose` | Delivery | Manual ad-hoc commit/push/PR between implementation and merge |
| `land` | Completion | Branch rot, unclosed issues, incomplete delivery |
| `using-groundwork` | Meta | Using skills in isolation instead of as a connected topology |
| `third-force` | Cross-cutting | Routing around operational friction instead of resolving it |

## Project Layout

```
groundwork.toml             # Methodology manifest — artifact types, skill declarations
schemas/                    # JSON Schema contracts for artifact types
skills/                     # Skill definitions (SKILL.md + references)
  using-groundwork/         #   methodology orientation
  ground/                   #   first-principles grounding
  research/                 #   external evidence gathering
  bdd/                      #   behavior contract definition
  plan/                     #   design convergence
  issue-craft/              #   issue lifecycle
  begin/                    #   work initiation
  test-first/               #   RED-GREEN-REFACTOR execution
  debug/     #   root-cause investigation
  third-force/              #   friction resolution
  documentation/            #   documentation review/update
  verification-before-completion/ # completion gate
  propose/                  #   commit, push, PR creation
  land/                     #   closeout workflow
docs/
  architecture/             # Topology contract, ADRs
tests/
  fixtures/artifacts/       # Valid/invalid artifact examples for schema testing
WORKFLOW.md                 # Integration manual — the authoritative reference
```

## Design Commitments

Groundwork's design commitments derive from the bedrock principles at [`pentaxis93/commons`](https://github.com/pentaxis93/commons).

**Connected topology.** Skills are not independently selectable utilities. They form a connected topology with handoff contracts between stages. Skipping a stage means the next stage receives malformed input.

**BDD threads everything.** Behavior contracts defined in stage 2 thread through planning, execution, verification, and closure. Completion evidence is behavior-level, not "tests pass."

**Issues are working memory.** Agent sessions end. Context windows close. The issue graph survives. Work from the graph, not from memory.

**Ground re-fires.** `ground` is not step-one-once. Any new generative work — a design, a spec, an architecture — requires re-grounding. The trigger is creation, not sequence position.

**Sovereignty.** Each boundary has an owner. Skills don't override agent judgment. Agents don't override human intent. The principle is fractal — it applies at every interface, not just the human-agent boundary.

## License

[MIT](LICENSE)
