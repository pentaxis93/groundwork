# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Five-stage methodology pipeline: frame constraints, define behavior, decompose, execute and verify, land
- Fourteen shipped skills: `using-groundwork` (methodology orientation), `ground`, `research`, `bdd`, `plan`, `issue-craft`, `begin`, `test-first`, `systematic-debugging`, `third-force`, `documentation`, `verification-before-completion`, `propose`, `land`
- Rust CLI (`groundwork init`, `update`, `list`, `doctor`) with shipped-skill manifest and `sk` integration
- Automatic `gh-issue-sync` installation with pinned release assets and SHA-256 verification
- Schema distribution: `init/update` provision `.groundwork/schemas/` and `.groundwork/artifacts/`; `doctor` reports schema completeness and drift
- Pipeline contract with formal handoff rules and anti-divergence checks (`docs/architecture/pipeline-contract.md`)
- Integration manual (`WORKFLOW.md`) covering pipeline stages, BDD thread, documentation thread, issue-based development, and skill routing
- Local issue mirroring via `gh-issue-sync`
- Upstream attribution convention: `LICENSE-UPSTREAM` requirements for adapted skills, origin metadata standard (CONTRIBUTING.md)
- Runtime manifest freshness: CLI fetches current `skills/skills.toml` from GitHub, falls back to embedded manifest with warning; `doctor` reports when they differ

### Changed

- License: MIT (migrated from Apache-2.0)
