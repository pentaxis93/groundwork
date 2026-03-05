# Architecture

Audience: contributors and agents who need to understand Groundwork's structure before making changes.

## What This Is

Groundwork is a methodology library — a set of skills that guide AI coding agents from problem framing through shipped code. It is not a framework, not a runtime, and not a plugin system. There is no code to import, no API to call. The deliverable is SKILL.md files that agents load as procedural instructions.

## System Components

### Skills (the product)

Each skill is a single SKILL.md file with YAML frontmatter (`name`, `description`) and markdown body. Skills define procedures, constraints, corruption modes, and triggers. They are organized by pipeline stage:

```
skills/
  foundation/       ground, research
  specification/    bdd
  decomposition/    issue-craft, next-issue, plan
  completion/       land
  verification/     documentation
  using-groundwork  (meta-skill: methodology orientation)
```

The directory structure reflects the pipeline, not the filesystem. A skill's stage determines when it fires and what it hands off to.

### CLI (the installer)

`crates/groundwork-cli/` is a Rust binary that reads a curated manifest, fetches skills via [`sk`](https://github.com/nickarora/sk), and populates the consumer's `agents.toml`. Commands: `init`, `update`, `list`, `doctor`. The CLI is a thin install layer — it has no role at runtime.

The curated manifest (`manifests/curation.v1.toml`) pins upstream skill sources to specific commits. The CLI embeds this manifest at compile time, so it works outside the repository.

### agents.toml (the configuration)

`agents.toml` is an [`sk`](https://github.com/nickarora/sk)-compatible configuration file that declares which agents receive skills and where skills come from (local paths or GitHub repositories). `sk sync` reads this file to install SKILL.md files into each agent's native skill directory (e.g., `.claude/skills/` for Claude Code).

### Lock file

`.groundwork/installed.lock.toml` records what was installed, when, and from where. The CLI uses this for idempotent reconciliation — `update` compares the lock against the manifest and upserts or prunes as needed.

## Composition Model

Groundwork has two kinds of skills, distinguished by where they are maintained:

**Core skills** (9) are maintained in this repository. They define the pipeline's structure — what stages exist, what handoff contracts connect them, and what cognitive discipline the pipeline enforces: `ground`, `research`, `bdd`, `issue-craft`, `next-issue`, `plan`, `documentation`, `land`, and the `using-groundwork` meta-skill.

**Curated skills** (9, from [obra/superpowers](https://github.com/obra/superpowers)) are referenced by the manifest and fetched at install time. They fill the execution phase — TDD, debugging, subagent orchestration, code review, verification — where high-quality implementations already exist.

Curated skills are pinned to a specific commit. They are not forked, vendored, or modified. Integration happens through documentation: WORKFLOW.md defines handoff rules that connect curated skills to the pipeline's input/output contracts.

This model works when upstream skills have general context assumptions (TDD, debugging). It is under tension for boundary skills where pipeline-specific behavior matters — see `docs/architecture/decisions/` for the design record on curate-vs-own decisions.

## Pipeline as Integration Architecture

The five stages are not a taxonomy — they are an integration architecture. Each stage produces artifacts that the next stage consumes:

1. **Frame constraints** (`ground`) produces verified constraints
2. **Define behavior** (`bdd`) produces Given/When/Then behavior contracts
3. **Decompose** (`issue-craft`, `next-issue`, `plan`) produces executable issues and implementation designs
4. **Execute and verify** (curated skills) produces tested implementations and review evidence
5. **Land** (`land`) produces closed issues, merged code, and behavior coverage records

Two cross-cutting threads run through all stages:
- **BDD thread**: behavior contracts from stage 2 thread through planning, execution, verification, and closure
- **Documentation thread**: documentation review fires at every stage, not just at the end

Formal handoff contracts and anti-divergence rules are defined in `docs/architecture/pipeline-contract.md`.

## Key Files

| File | Purpose |
|------|---------|
| `WORKFLOW.md` | Integration manual — the authoritative reference for operating the pipeline |
| `docs/architecture/pipeline-contract.md` | Formal handoff contracts and anti-divergence rules |
| `manifests/curation.v1.toml` | Curated upstream skills with pinned commits |
| `agents.toml` | Skill system configuration (sk-compatible) |
| `crates/groundwork-cli/src/main.rs` | CLI source — init, update, list, doctor |
