# Groundwork

A methodology plugin for AI coding agents. One connected topology from problem framing through shipped change to closed loop.

Groundwork owns the protocols, skills, artifact schemas, and topology. It registers with [runa](https://github.com/pentaxis93/runa) through a single [manifest file](groundwork.toml) and has no runtime, no CLI, no installer.

## Why Groundwork Exists

Reliable engineering has a discipline: constraints get verified before design, behavior gets defined before code, completion gets proven before it's claimed, and changes land with merge, cleanup, and issue resolution. This discipline is what makes outcomes repeatable. Groundwork encodes it as a connected set of protocols and skills, giving AI agents the structure to produce verified, shipped work without the need for human course-correction — not through restrictions, but through a topology that makes the right sequence the natural one.

## The Topology

Five stages form the forward flow. Cross-cutting skills fire at any stage, creating branches and feedback loops. The behavior contract from stage 2 is the central integration thread — it flows through design, execution, verification, and landing.

### Stages

| Stage | Protocols | Produces |
|-------|-----------|----------|
| **1. Survey and select work** | `survey`, `decompose`, `begin` | Assessment, executable issues, session scope |
| **2. Define behavior** | `specify` | `behavior-contract` — threads through all subsequent stages |
| **3. Design** | `plan` | Decision-complete design |
| **4. Execute and verify** | `test`, `document`, `verify`, `propose` | Test evidence, documentation record, completion evidence, open PR |
| **5. Land** | `land` | Completion record — merge, cleanup, coverage status, issue closure |

Enter where the work needs you. A bug with an existing issue enters at Execute and verify. A new capability enters at Survey and select work. The constraint is sequence — you can't land before executing — not completeness.

### Cross-cutting skills

These fire at any stage when their trigger condition appears, not at a fixed position:

- **`orient`** provides methodology orientation and carries the documentation discipline the agent should keep active at all times
- **`reckon`** re-fires on any new generative act (design, spec, architecture) — not step-one-once
- **`research`** fires when a decision needs evidence outside the codebase
- **`debug`** fires on failures; hands off to `test` (fix), `reckon` (3-fix escalation), or `resolve` (environmental cause)
- **`resolve`** fires on operational friction; resolves structurally or files an issue via `decompose`
- **`contract`** carries the behavior contract through implementation and verification so execution does not drift from specified behavior

The connecting structure — artifacts, manifest edges, schemas, and protocol topology — is defined in [`connecting-structure.md`](docs/architecture/connecting-structure.md). The issue persistence model is defined in [`issue-model.md`](docs/architecture/issue-model.md).

### Protocol Routing

| Protocol | Trigger |
|----------|---------|
| `survey` | when the work is under-specified and needs a grounded assessment before selection or decomposition |
| `decompose` | creating, refining, or closing executable issues from an assessment or work definition |
| `begin` | initiating a work session: selecting work, preparing workspace, declaring direction |
| `specify` | when defining or refining behavior expectations |
| `plan` | implementation needs design convergence — multiple approaches, unclear scope, or cross-cutting changes |
| `test` | when implementing any feature or bugfix — RED → GREEN → REFACTOR |
| `document` | after code changes that may cause drift, before verification, or when docs need classification |
| `verify` | before claiming work is complete, fixed, or passing — evidence first |
| `propose` | packaging changes for review: `propose`, `submit pr`, `create pr`, `open pr`, `send for review` |
| `land` | merge-and-close completion events: `land`, `merge and close`, `ship it` |

### Skill Routing

| Skill | Trigger |
|-------|---------|
| `orient` | session start, task initiation, or any moment requiring methodology orientation |
| `reckon` | before creating designs/specs/architectures/processes |
| `research` | when reliable external evidence is needed for decisions |
| `debug` | when a test fails or behavior is unexpected, before proposing any fix |
| `resolve` | operational friction — missing tools, broken configs, stale conventions, undocumented requirements |
| `contract` | carrying a behavior contract through implementation and verification without drift |

## Key Files

| File | Purpose |
|------|---------|
| `groundwork.toml` | **Canonical manifest** — all artifact types and protocol declarations with interface edges |
| `docs/architecture/connecting-structure.md` | Connecting structure design — artifacts, manifest edges, schemas, protocol topology |
| `docs/architecture/issue-model.md` | Issue state model, dependency graph format, graph maintenance |
| `schemas/` | JSON Schema contracts for artifact types |
| `protocols/` | Protocol definitions — runa-managed stages declared in the manifest |
| `skills/` | Skill definitions — agent-managed cognitive tools, each a `SKILL.md` with YAML frontmatter |

## Project Layout

```
groundwork.toml             # Methodology manifest — artifact types, protocol declarations
schemas/                    # JSON Schema contracts for artifact types
protocols/                  # Runa-managed topology stages (SKILL.md + references)
  survey/                   #   territory assessment
  decompose/                #   issue lifecycle
  begin/                    #   work initiation
  specify/                  #   behavior contract definition
  plan/                     #   design convergence
  test/                     #   RED-GREEN-REFACTOR execution
  document/                 #   documentation review/update
  verify/                   #   completion gate
  propose/                  #   commit, push, PR creation
  land/                     #   closeout workflow
skills/                     # Agent-managed cognitive tools (SKILL.md + references)
  orient/                   #   methodology orientation
  reckon/                   #   first-principles reckoning
  research/                 #   external evidence gathering
  debug/                    #   root-cause investigation
  resolve/                  #   friction resolution
  contract/                 #   behavior-contract discipline
docs/
  architecture/             # Topology contract, issue model, ADRs
tests/
  fixtures/artifacts/       # Valid/invalid artifact examples for schema testing
```

## License

[MIT](LICENSE)
