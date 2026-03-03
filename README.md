# Groundwork

Groundwork is a methodology distribution for AI coding agents.

It curates methodology skills from multiple upstream sources into one coherent workflow, while maintaining a strict boundary:
- This repo contains Groundwork's original skills and curation metadata.
- Curated skills stay in their upstream repositories and are fetched at install time.

## v0.1 Architecture

Groundwork v0.1 uses:
- **Groundwork originals**: `ground`, `bdd`, `planning`, `issue-craft`, `land`
- **Superpowers middle** (curated): planning/execution/verification discipline skills

Pipeline invariant for v0.1:
- **There is one pipeline, not two.** `bdd` defines and maintains the behavior contract; curated implementation skills execute and verify that same contract.

Kata Orchestrator was evaluated as an integrated middle layer. For v0.1, Superpowers is selected because it currently provides a cleaner minimum path across Claude Code, Codex, and OpenCode while preserving strict execution guardrails.

## One-Command Install

Install with a single command:

```bash
groundwork init
```

`groundwork init` will:
1. Read Groundwork's curated manifest.
2. Auto-install `sk` if needed (npm-first, `npx` fallback).
3. Update your `agents.toml` with Groundwork-managed dependencies.
4. Run `sk sync`.
5. Report what it installed and from where.

Re-run safely anytime:

```bash
groundwork update
```

`groundwork update` converges Groundwork-managed dependencies to the current manifest,
including pruning obsolete `groundwork_*` entries that are no longer curated.

Preview changes without writing:

```bash
groundwork update --dry-run
```

## Build the CLI Locally

```bash
cargo run -p groundwork-cli -- init
cargo run -p groundwork-cli -- update
cargo run -p groundwork-cli -- list
cargo run -p groundwork-cli -- doctor
```

## Key Documents

- [WORKFLOW.md](WORKFLOW.md) - integrated alpha-to-omega workflow guide
- [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md) - canonical pipeline and handoff contract
- [CURATED.md](CURATED.md) - curated skill selection and failure modes
- [ATTRIBUTION.md](ATTRIBUTION.md) - source, author, license, and pinning details
- [`manifests/curation.v1.toml`](manifests/curation.v1.toml) - installer source-of-truth

## Original Skills in This Repo

- `skills/foundation/ground`
- `skills/specification/bdd`
- `skills/decomposition/planning`
- `skills/decomposition/issue-craft`
- `skills/completion/land`

## License

Apache-2.0.
