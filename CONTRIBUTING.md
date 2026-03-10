# Contributing

Audience: new contributors and agents starting development work on Groundwork.

## Prerequisites

- **Rust** (edition 2021) — for building and testing the CLI
- **Node.js** — required by [`sk`](https://github.com/nickarora/sk) for skill syncing
- **gh CLI** — required for verified `gh-issue-sync` release downloads during auto-install

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

## Upstream Attribution

Any skill in `skills/` that adapts, derives from, or reproduces substantial portions of upstream work must include a `LICENSE-UPSTREAM` file co-located with `SKILL.md`.

### When LICENSE-UPSTREAM is required

If the skill preserves structural frameworks, tables, terminology, enumerated lists, or near-verbatim text from an upstream source, it requires a `LICENSE-UPSTREAM` file. Skills that are merely "inspired by" a concept without reproducing protected expression do not.

### What LICENSE-UPSTREAM must contain

1. **Prose preamble** identifying the upstream project name, URL, and pinned revision (commit hash), plus an honest accounting of which elements are derived from the original and which are original to this adaptation.
2. **Full upstream copyright notice and license text** — the complete notice as it appears in the upstream project, not a summary or paraphrase.

### Origin metadata standard

The `origin:` field in `SKILL.md` frontmatter is a terse attribution pointer,
not a development narrative. See
[ADR-0004](docs/architecture/decisions/0004-internal-development-history-policy.md)
for the full policy.

- **Upstream-adapted skills:** Format:
  `"Adapted from <project> (<license>). See LICENSE-UPSTREAM."`
  Detailed preservation/adaptation accounting belongs in `LICENSE-UPSTREAM`,
  not in frontmatter.
- **Internal-only skills:** No `origin:` field. Internal genealogy belongs in
  git log and issue threads, not frontmatter.
- The `replaces:` field is not used. Do not add it to new skills.

### Reference examples

- `skills/plan/LICENSE-UPSTREAM` — adapted from OpenAI Codex (Apache-2.0), with itemized derived vs. original elements
- `skills/test-first/LICENSE-UPSTREAM` — adapted from obra/superpowers (MIT), with description of preserved portions

## Skill Quality Standards

Groundwork cares about the quality and compatibility of the skills it ships,
not which authoring tool produced them. A contribution meets the skill
authoring bar when the tracked output is well-formed and integrates cleanly
with the live methodology.

A repo-tracked skill contribution must:

- live at `skills/<skill-name>/SKILL.md`
- use YAML frontmatter with, at minimum, `name` and `description`
- include `metadata` and `origin` fields when they apply (see Origin metadata
  standard above; `replaces` is not used)
- include a co-located `LICENSE-UPSTREAM` file when the skill adapts upstream
  material, and reference it from `origin:` metadata
- include a valid `groundwork:` frontmatter block when the skill participates
  in runtime pipeline contracts; follow the accepted format in
  `docs/architecture/decisions/0002-groundwork-frontmatter-format.md`
- use valid Markdown/plain text encoding and stable relative references so the
  skill can be installed and read without local-environment assumptions

When a skill contribution changes the shipped Groundwork inventory or
methodology, update the corresponding manifest and documentation surfaces in
this repository (`skills/skills.toml`, `agents.toml`, README/WORKFLOW entries,
pipeline docs, ADRs, CHANGELOG) so the tracked project state stays accurate.

Contributor tooling is not part of Groundwork's runtime skill inventory. Only
add entries to `agents.toml` or `skills/skills.toml` for skills Groundwork
actually ships.

## Agent Workspace Policy

- `.codex/` is agent-local runtime/workspace state and is intentionally gitignored.
- Do not add or edit canonical project content under `.codex/`.
- Canonical skill content belongs in tracked project paths (`skills/`) and the shipped-skill manifest at `skills/skills.toml`.
- When a `.codex/**` path is accidentally tracked, remove it from git index (`git rm --cached <path>`) and move/preserve canonical content in tracked locations.

## PR Process

- Branch from `main`
- Ensure `cargo test -p groundwork-cli` passes
- Include a documentation review: check which docs need updating per the changes (see the `documentation` skill for the full procedure)
- Add a CHANGELOG entry for user-visible changes (see CHANGELOG discipline
  below)

## CHANGELOG Discipline

Only log user-visible changes. An entry belongs in CHANGELOG when an end user
or adopting agent would notice the difference. Internal iteration — version
bumps to individual skills, renames of internal concepts, add-then-remove
cycles, policy doc rewrites — does not.

The `[Unreleased]` section describes what ships, not what happened during
development. See
[ADR-0004](docs/architecture/decisions/0004-internal-development-history-policy.md).

## ADR Lifecycle

When an ADR is superseded:

1. Update the `Status:` line: `Accepted — Superseded by: <reference>`
2. Add a dated note after the status block explaining what changed
3. Preserve the full body as historical record

ADRs document *why* a decision was made. The superseded note tells readers to
follow the pointer for current policy; the body preserves the reasoning. See
[ADR-0004](docs/architecture/decisions/0004-internal-development-history-policy.md).

## Where to Look

| Document | What it covers |
|----------|---------------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | System structure, composition model, component overview |
| [WORKFLOW.md](WORKFLOW.md) | Integration manual — pipeline stages, skill routing, handoff rules |
| [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md) | Formal handoff contracts and anti-divergence rules |
| [README.md](README.md) | Project overview, install, design principles |
