# Groundwork

Groundwork is a methodology distribution for AI coding agents.

It curates methodology skills from multiple upstream sources into one coherent workflow, while maintaining a strict boundary:
- This repo contains Groundwork's original skills and curation metadata.
- Curated skills stay in their upstream repositories and are fetched at install time.

## v0.1 Architecture

Groundwork v0.1 uses:
- **Groundwork originals**: `ground`, `research`, `bdd`, `planning`, `issue-craft`, `land`
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
groundwork init --dry-run
groundwork update --dry-run
```

## Local Issue Mirroring

Groundwork integrates with [gh-issue-sync](https://github.com/mitsuhiko/gh-issue-sync)
for local issue mirroring. Issues are synced to a gitignored `.issues/` directory —
GitHub remains the source of truth; the local copy is a working surface.

### Setup

Install gh-issue-sync ([installation options](https://github.com/mitsuhiko/gh-issue-sync#installation)),
then run `groundwork init`. The CLI will detect the tool and initialize `.issues/` automatically.

Or initialize manually:

    gh-issue-sync init
    gh-issue-sync pull

### How it works

Groundwork skills handle sync at the right moments:
- `planning` pulls fresh issues at session-open, pushes state updates at session-close
- `issue-craft` pushes after creating or closing issues
- `land` pulls after closing to update the local mirror

For manual sync: `gh-issue-sync pull`, `gh-issue-sync push`, or `gh-issue-sync sync` (bidirectional).

## CLI Behavior Notes

- `groundwork init` / `groundwork update` reconcile Groundwork-managed dependencies in `agents.toml`.
- Groundwork originals are managed as per-skill aliases (`groundwork_original_*`) that point to pinned GitHub paths.
- `groundwork update` prunes obsolete managed aliases (`groundwork_*`) that are no longer present in the current curation manifest.
- `groundwork list` reports from `.groundwork/installed.lock.toml`.
- The curation manifest is embedded in the CLI binary at build time; commands do not require `manifests/curation.v1.toml` in the target project directory.

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

- `skills/using-groundwork`
- `skills/foundation/ground`
- `skills/foundation/research`
- `skills/specification/bdd`
- `skills/decomposition/planning`
- `skills/decomposition/issue-craft`
- `skills/completion/land`

## License

MIT.
