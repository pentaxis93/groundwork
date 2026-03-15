## What This Project Is

Groundwork is a **methodology plugin** for [runa](https://github.com/pentaxis93/runa). It defines cognitive discipline through skills, artifact schemas, and topology.

## Key Files

| File | Purpose |
|------|---------|
| `groundwork.toml` | **Canonical manifest** — all artifact types and skill declarations with interface edges |
| `WORKFLOW.md` | **Integration manual** — stages, skill routing, handoff rules |
| `docs/architecture/topology-contract.md` | Formal handoff contracts and anti-divergence rules |
| `schemas/` | JSON Schema contracts for artifact types |
| `skills/` | Skill definitions — each is a `SKILL.md` with YAML frontmatter |

## Topology

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
- **`systematic-debugging`** fires on failures; hands off to `test-first` (fix), `ground` (3-fix escalation), or `third-force` (environmental cause)
- **`third-force`** fires on operational friction; resolves structurally or files an issue via `issue-craft`
- **`documentation`** threads through every stage; drift blocks completion
- **`using-groundwork`** provides methodology orientation at any point

Handoff contracts between skills are defined in [`topology-contract.md`](docs/architecture/topology-contract.md). The artifact graph is declared in [`groundwork.toml`](groundwork.toml).
