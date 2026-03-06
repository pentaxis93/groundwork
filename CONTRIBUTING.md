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

Core skills live in `skills/` organized by pipeline stage. Each skill is a single SKILL.md file with YAML frontmatter:

```yaml
---
name: skill-name
description: >-
  When to use this skill.
---

# Skill Name

[skill content]
```

To add a new core skill:

1. Create `skills/<stage>/<skill-name>/SKILL.md`
2. Add the skill to `ORIGINAL_SKILLS` in `crates/groundwork-cli/src/main.rs` (rename to `CORE_SKILLS` pending #22)
3. Add a `gh` dependency entry in `agents.toml`
4. Add the skill to the skills table in `README.md` and the routing table in `WORKFLOW.md`
5. If the skill participates in handoff contracts, update `docs/architecture/pipeline-contract.md`

To curate an upstream skill:

1. Add the source and skill entries to `manifests/curation.v1.toml` with a pinned `rev`
2. Run `groundwork update` to sync

## Agent Workspace Policy

- `.codex/` is agent-local runtime/workspace state and is intentionally gitignored.
- Do not add or edit canonical project content under `.codex/`.
- Canonical skill content belongs in tracked project paths (`skills/`) or upstream curated sources (`manifests/curation.v1.toml`).
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
