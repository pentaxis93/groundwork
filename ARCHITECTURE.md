# Architecture

Audience: contributors and agents who need to understand Groundwork's structure before making changes.

## What This Is

Groundwork is a methodology library — a set of skills that guide AI coding agents from problem framing through shipped code. It is not a framework, not a runtime, and not a plugin system. There is no code to import, no API to call. The deliverable is SKILL.md files that agents load as procedural instructions.

## System Components

### Skills (the product)

Each skill is a single SKILL.md file with YAML frontmatter (`name`, `description`) and markdown body. Skills define procedures, constraints, corruption modes, and triggers. The tracked `skills/` directory stores the local skill files plus one manifest that makes the shipped inventory explicit:

```
skills/
  skills.toml        authoritative shipped-skill manifest
  using-groundwork/  methodology orientation
  ground/            first-principles grounding
  research/          external evidence gathering
  bdd/               behavior contract definition
  plan/              design convergence
  issue-craft/       issue lifecycle
  begin/             work initiation
  documentation/     documentation review/update
  land/              closeout workflow
```

The directory structure is storage, not the methodology model. Order and inventory come from `skills/skills.toml`; the workflow narrative explains how those skills relate.

### CLI (the installer)

`crates/groundwork-cli/` is a Rust binary that reads `skills/skills.toml`, fetches skills via [`sk`](https://github.com/nickarora/sk), and populates the consumer's `agents.toml`. Commands: `init`, `update`, `list`, `doctor`. The CLI is a thin install layer — it has no role at runtime.

`skills/skills.toml` pins upstream skill sources to specific commits and lists the local ones by path. The CLI embeds this manifest at compile time, so it works outside the repository.

### agents.toml (the configuration)

`agents.toml` is an [`sk`](https://github.com/nickarora/sk)-compatible configuration file that declares which agents receive skills and where skills come from (local paths or GitHub repositories). `sk sync` reads this file to install SKILL.md files into each agent's native skill directory (e.g., `.claude/skills/` for Claude Code).

### Lock file

`.groundwork/installed.lock.toml` records what was installed, when, and from where. The CLI uses this for idempotent reconciliation — `update` compares the lock against the manifest and upserts or prunes as needed.

## Composition Model

Groundwork ships skills from two maintenance locations, but inventory is unified in one manifest:

Skills maintained in this repository are listed in `skills/skills.toml` with local paths under `skills/`. These skills define the pipeline's structure — what stages exist, what handoff contracts connect them, and what cognitive discipline the pipeline enforces.

Skills maintained upstream (from [obra/superpowers](https://github.com/obra/superpowers)) are listed in the same manifest with pinned commits and fetched at install time. They fill the execution phase — TDD, debugging, subagent orchestration, code review, verification — where high-quality implementations already exist.

Curated skills are pinned to a specific commit. They are not forked, vendored, or modified. Integration happens through documentation: WORKFLOW.md defines handoff rules that connect curated skills to the pipeline's input/output contracts.

This model works when upstream skills have general context assumptions (TDD, debugging). It is under tension for boundary skills where pipeline-specific behavior matters — see `docs/architecture/decisions/` for the design record on curate-vs-own decisions.

## Pipeline as Integration Architecture

The five stages are not a taxonomy — they are an integration architecture. Each stage produces artifacts that the next stage consumes:

1. **Frame constraints** (`ground`, `research`) produces verified constraints and substantiated evidence
2. **Define behavior** (`bdd`) produces Given/When/Then behavior contracts
3. **Decompose** (`issue-craft`, `begin`, `plan`) produces executable issues and implementation designs
4. **Execute and verify** (curated skills) produces tested implementations and review evidence
5. **Land** (`land`) produces closed issues, merged code, and behavior coverage records

Two cross-cutting disciplines run through all stages rather than belonging to one:
- **BDD thread**: behavior contracts from stage 2 thread through planning, execution, verification, and closure
- **Documentation thread** (`documentation`): documentation review fires at every stage, not just at the end

Formal handoff contracts and anti-divergence rules are defined in `docs/architecture/pipeline-contract.md`.

## Key Files

| File | Purpose |
|------|---------|
| `WORKFLOW.md` | Integration manual — the authoritative reference for operating the pipeline |
| `docs/architecture/pipeline-design.md` | Canonical pipeline design: as-built baseline + clearly marked target-state design |
| `docs/architecture/pipeline-contract.md` | Formal handoff contracts and anti-divergence rules |
| `skills/skills.toml` | Shipped skill inventory and upstream pinning |
| `agents.toml` | Skill system configuration (sk-compatible) |
| `crates/groundwork-cli/src/main.rs` | CLI source — init, update, list, doctor |
