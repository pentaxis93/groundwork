# CLAUDE.md

## Skill Management

Skills are managed by `sk` (Skills Supply CLI). This project uses a **forked version** of `skills-supply` — not the upstream release.

The fork adds `--skill-target name` which installs skills with unprefixed directory names (e.g., `using-groundwork/`) instead of the upstream's prefixed format (`using_groundwork-using-groundwork/`).

### Installing the fork

```bash
gh repo clone pentaxis93/skills-supply && cd skills-supply
npm run build --workspace=packages/sk
cp packages/sk/bin/sk ~/.local/bin/sk
```

### Syncing skills

```bash
sk sync --skill-target name --non-interactive
```

Dependencies are declared in `agents.toml`. Skills with `gh = "pentaxis93/groundwork"` resolve from this repo on GitHub — local changes propagate after push.

### Primary source vs installed copies

- Primary source: `skills/<skill-name>/SKILL.md`
- Installed copies: `.claude/skills/<skill-name>/SKILL.md`, `.agents/skills/<skill-name>/SKILL.md`
- Installed copies are managed by `sk sync` — do not edit them directly.

## Agent Principles

### Introduce third force on friction

When you encounter operational friction — a missing tool, a broken config, a stale convention, an undocumented requirement — do not route around it. Stop, step outside, and resolve the structural cause permanently before continuing your original task. Structural fixes include: installing a tool, fixing a configuration, updating documentation (including this file), adding a CLAUDE.md instruction, filing an issue for deeper work.

For the full methodology, see the `third-force` skill.
