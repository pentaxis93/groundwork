# Contributing

Audience: new contributors and agents starting development work on Groundwork.

## Prerequisites

- **gh CLI** — used for GitHub operations

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
2. Add the skill declaration to `groundwork.toml` with requires/accepts/produces/may_produce/trigger
3. Add the skill to the routing table in `WORKFLOW.md`
4. If the skill participates in handoff contracts, update `docs/architecture/pipeline-contract.md`

## Upstream Attribution

Any skill in `skills/` that adapts, derives from, or reproduces substantial portions of upstream work must include a `LICENSE-UPSTREAM` file co-located with `SKILL.md`.

### When LICENSE-UPSTREAM is required

If the skill preserves structural frameworks, tables, terminology, enumerated lists, or near-verbatim text from an upstream source, it requires a `LICENSE-UPSTREAM` file. Skills that are merely "inspired by" a concept without reproducing protected expression do not.

### What LICENSE-UPSTREAM must contain

1. **Prose preamble** identifying the upstream project name, URL, and pinned revision (commit hash), plus an honest accounting of which elements are derived from the original and which are original to this adaptation.
2. **Full upstream copyright notice and license text** — the complete notice as it appears in the upstream project, not a summary or paraphrase.

### Origin metadata standard

The `origin:` field in `SKILL.md` frontmatter is a terse attribution pointer,
not a development narrative.

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
- include runa interface fields (requires, accepts, produces, may_produce, trigger) in frontmatter when the skill participates in the methodology pipeline
- use valid Markdown/plain text encoding and stable relative references so the
  skill can be installed and read without local-environment assumptions

When a skill contribution changes the shipped Groundwork inventory or
methodology, update the corresponding manifest and documentation surfaces in
this repository (`groundwork.toml`, WORKFLOW entries, pipeline docs) so the
tracked project state stays accurate.

## PR Process

- Branch from `main`
- Include a documentation review: check which docs need updating per the changes (see the `documentation` skill for the full procedure)

## Where to Look

| Document | What it covers |
|----------|---------------|
| [WORKFLOW.md](WORKFLOW.md) | Integration manual — pipeline stages, skill routing, handoff rules |
| [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md) | Formal handoff contracts and anti-divergence rules |
| [README.md](README.md) | Project overview, design principles |
| [groundwork.toml](groundwork.toml) | Methodology manifest — artifact types and skill declarations |
