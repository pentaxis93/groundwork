# Contributing

Audience: new contributors and agents starting development work on Groundwork.

## Prerequisites

- **Rust** (edition 2021) — for building and testing the CLI
- **Node.js** — required by [`sk`](https://github.com/nickarora/sk) for skill syncing
- **Optional:** `go` or `curl` — for `gh-issue-sync` auto-installation

## Build and Test

```bash
cargo build -p groundwork-cli
cargo test -p groundwork-cli
```

The binary is `groundwork`. After building, it is available at `target/debug/groundwork`.

## Adding a Skill

Tracked skill files live in `skills/`. Each skill is a single `SKILL.md` file with YAML frontmatter:

```yaml
---
name: skill-name
description: >-
  When to use this skill.
---

# Skill Name

[skill content]
```

To add a new shipped skill maintained in this repository:

1. Create `skills/<skill-name>/SKILL.md`
2. Add the skill to `skills/skills.toml` in the position that makes its invocation relationship clear
3. Add a `gh` dependency entry in `agents.toml`
4. Add the skill to the skills table in `README.md` and the routing table in `WORKFLOW.md`
5. If the skill participates in handoff contracts, update `docs/architecture/pipeline-contract.md`

To add a shipped skill maintained upstream:

1. Add the skill entry to `skills/skills.toml` with its pinned `rev`
2. Run `groundwork update` to sync

## Skill Authoring Boundary

Groundwork stores the skills it ships, but it does not ship its own
skill-authoring toolchain. Use the external sibling repository
`/home/pentaxis93/src/skill-creator` when you need to:

- create a new skill
- regenerate an existing Groundwork skill
- evaluate whether a skill change actually improves behavior

Treat Groundwork as the destination for committed skill outputs and pipeline
documentation:

1. Do the authoring or regeneration work in `skill-creator`
2. Bring the resulting `SKILL.md` content or curation change back into this repo
3. Update `skills/skills.toml`, `agents.toml`, README/WORKFLOW entries,
   ADRs, and CHANGELOG only if the shipped Groundwork inventory or methodology
   changes

Do not add `skill-creator` to `agents.toml` or `skills/skills.toml`.
It is contributor tooling, not part of Groundwork's runtime skill inventory.

## Agent Workspace Policy

- `.codex/` is agent-local runtime/workspace state and is intentionally gitignored.
- Do not add or edit canonical project content under `.codex/`.
- Canonical skill content belongs in tracked project paths (`skills/`) and the shipped-skill manifest at `skills/skills.toml`.
- When a `.codex/**` path is accidentally tracked, remove it from git index (`git rm --cached <path>`) and move/preserve canonical content in tracked locations.

## PR Process

- Branch from `main`
- Ensure `cargo test -p groundwork-cli` passes
- Include a documentation review: check which docs need updating per the changes (see the `documentation` skill for the full procedure)
- Add a CHANGELOG entry for user-visible changes

## Where to Look

| Document | What it covers |
|----------|---------------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | System structure, composition model, component overview |
| [WORKFLOW.md](WORKFLOW.md) | Integration manual — pipeline stages, skill routing, handoff rules |
| [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md) | Formal handoff contracts and anti-divergence rules |
| [README.md](README.md) | Project overview, install, design principles |
