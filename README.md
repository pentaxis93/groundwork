# Groundwork

A methodology plugin for AI coding agents. One connected topology from problem framing through shipped change to closed loop.

Groundwork owns the skills, artifact schemas, and topology. It registers with [runa](https://github.com/pentaxis93/runa) through a single [manifest file](groundwork.toml) and has no runtime, no CLI, no installer.

## Why Groundwork Exists

Reliable engineering has a discipline: constraints get verified before design, behavior gets defined before code, completion gets proven before it's claimed, and changes land with merge, cleanup, and issue resolution. This discipline is what makes outcomes repeatable. Groundwork encodes it as a connected set of skills, giving AI agents the structure to produce verified, shipped work without the need for human course-correction — not through restrictions, but through a topology that makes the right sequence the natural one.

## The Topology

Five stages form the forward flow. Cross-cutting disciplines fire at any stage, creating branches and feedback loops. The behavior contract from stage 2 is the central integration thread — it flows through design, execution, verification, and landing.

### Stages

| Stage | Skills | Produces |
|-------|--------|----------|
| **1. Frame constraints** | `ground` | Grounded constraints — what the work must enable |
| **2. Define behavior** | `bdd` | `behavior-contract` — threads through all subsequent stages |
| **3. Decompose** | `plan`, `issue-craft`, `begin` | Decision-complete design, executable issues, session scope |
| **4. Execute and verify** | `test-first`, `verification-before-completion`, `propose` | Test evidence, completion evidence, open PR |
| **5. Land** | `land` | Completion record — merge, cleanup, coverage status, issue closure |

Enter where the work needs you. A bug with an existing issue enters at Execute. A new capability enters at Frame. The constraint is sequence — you can't land before executing — not completeness.

### Cross-cutting disciplines

These fire at any stage when their trigger condition appears, not at a fixed position:

- **`ground`** re-fires on any new generative act (design, spec, architecture) — not step-one-once
- **`research`** fires when a decision needs evidence outside the codebase
- **`debug`** fires on failures; hands off to `test-first` (fix), `ground` (3-fix escalation), or `third-force` (environmental cause)
- **`third-force`** fires on operational friction; resolves structurally or files an issue via `issue-craft`
- **`documentation`** threads through every stage; drift blocks completion
- **`using-groundwork`** provides methodology orientation at any point

Handoff contracts between skills are defined in [`topology-contract.md`](docs/architecture/topology-contract.md). The issue persistence model is defined in [`issue-model.md`](docs/architecture/issue-model.md).

### Skill Routing

| Skill | Trigger |
|-------|---------|
| `using-groundwork` | session start, task initiation, or any moment requiring methodology orientation |
| `ground` | before creating designs/specs/architectures/processes |
| `research` | when reliable external evidence is needed for decisions |
| `bdd` | when defining or refining behavior expectations |
| `issue-craft` | creating/refining task/epic/bug/spike issues |
| `begin` | initiating a work session: selecting work, preparing workspace, declaring direction |
| `plan` | implementation needs design convergence — multiple approaches, unclear scope, or cross-cutting changes |
| `test-first` | when implementing any feature or bugfix — RED → GREEN → REFACTOR |
| `debug` | when a test fails or behavior is unexpected, before proposing any fix |
| `third-force` | operational friction — missing tools, broken configs, stale conventions, undocumented requirements |
| `documentation` | after code changes that may cause drift, at project initialization, when architectural decisions are made, or when docs fail the audience test |
| `verification-before-completion` | before claiming work is complete, fixed, or passing — evidence first |
| `propose` | packaging changes for review: `propose`, `submit pr`, `create pr`, `open pr`, `send for review` |
| `land` | merge-and-close completion events: `land`, `merge and close`, `ship it` |

## Key Files

| File | Purpose |
|------|---------|
| `groundwork.toml` | **Canonical manifest** — all artifact types and skill declarations with interface edges |
| `docs/architecture/topology-contract.md` | Formal handoff contracts and anti-divergence rules |
| `docs/architecture/issue-model.md` | Issue state model, dependency graph format, graph maintenance |
| `schemas/` | JSON Schema contracts for artifact types |
| `skills/` | Skill definitions — each is a `SKILL.md` with YAML frontmatter |

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
  architecture/             # Topology contract, issue model, ADRs
tests/
  fixtures/artifacts/       # Valid/invalid artifact examples for schema testing
```

## License

[MIT](LICENSE)
