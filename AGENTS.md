# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

Groundwork is a **pure methodology plugin** for AI coding agents — no runtime, no CLI, no build system. It defines cognitive discipline through skills, artifact schemas, and pipeline topology. It has no code to build, no tests to run, and no dependencies to install.

It is part of a three-layer stack:
- **Daemon** — orchestration surface (Claude Code, Codex CLI)
- **Runtime** — [runa](https://github.com/pentaxis93/runa) monitors artifacts, evaluates triggers, enforces contracts
- **Methodology** — groundwork defines *what* discipline agents follow; runa enforces *when* and *whether*

## Key Files

| File | Purpose |
|------|---------|
| `groundwork.toml` | **Canonical manifest** — all artifact types and skill declarations with interface edges |
| `WORKFLOW.md` | **Integration manual** — pipeline stages, skill routing, handoff rules |
| `docs/architecture/pipeline-contract.md` | Formal handoff contracts and anti-divergence rules |
| `CONTRIBUTING.md` | Contributor guide, skill submission standards, LICENSE-UPSTREAM policy |
| `schemas/` | JSON Schema contracts for artifact types |
| `skills/` | Skill definitions — each is a `SKILL.md` with YAML frontmatter |

## Pipeline

One path, not a menu. Every piece of work flows through five stages:

1. **Frame constraints** (`ground`) — strips assumptions, builds from verified constraints. Re-fires on every new generative act, not just once.
2. **Define behavior** (`bdd`) — Given/When/Then scenarios that thread through every subsequent stage.
3. **Decompose** (`issue-craft`, `begin`, `plan`) — executable issues, session selection, decision-complete designs.
4. **Execute and verify** (`test-first`, `systematic-debugging`, `verification-before-completion`, `propose`) — RED-GREEN-REFACTOR mapped to behavior scenarios, root-cause before fix, behavior-level evidence.
5. **Land** (`land`) — merge, push, delete branch, close issue. Do not stop after merge.

## Modifying the Methodology

When adding or changing a skill:

1. Create/edit `skills/<skill-name>/SKILL.md` (YAML frontmatter with `name`, `description`, and runa interface fields: `requires`, `accepts`, `produces`, `may_produce`, `trigger`)
2. Update `groundwork.toml` with the skill declaration
3. Update the routing table in `WORKFLOW.md`
4. If the skill participates in handoff contracts, update `docs/architecture/pipeline-contract.md`
5. Skills adapting upstream work must include a co-located `LICENSE-UPSTREAM` file

The manifest format is **TOML**, not YAML or JSON.

## Conventions

- **Document state, not transitions.** Repo artifacts describe what exists now and why. Development history belongs in git log and issue threads, not in user-facing documents. (See ADR-0001.)
- **CHANGELOG discipline:** only log user-visible changes. Internal iteration (version bumps, renames, add-then-remove cycles) does not belong.
- **`origin:` in skill frontmatter** is terse attribution for upstream-adapted skills only. Internal-only skills have no `origin:` field. The `replaces:` field is not used.
- **CLAUDE.md and AGENTS.md are the same file** — CLAUDE.md is a symlink to AGENTS.md. Edit AGENTS.md. Never break the symlink.

## Agent Principles

### Introduce third force on friction

When you encounter operational friction — a missing tool, a broken config, a stale convention, an undocumented requirement — do not route around it. Stop, step outside, and resolve the structural cause permanently before continuing your original task. Structural fixes include: installing a tool, fixing a configuration, updating documentation (including this file), adding a CLAUDE.md instruction, filing an issue for deeper work.

For the full methodology, see the `third-force` skill.
