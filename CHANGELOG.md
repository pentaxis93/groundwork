# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Five-stage methodology pipeline: frame constraints, define behavior, decompose, execute and verify, land
- Nine core skills: `ground`, `research`, `bdd`, `issue-craft`, `next-issue`, `plan`, `documentation`, `land`, and `using-groundwork` (methodology orientation)
- Seven curated skills from [obra/superpowers](https://github.com/obra/superpowers): `brainstorming`, `subagent-driven-development`, `test-driven-development`, `systematic-debugging`, `verification-before-completion`, `requesting-code-review`, `receiving-code-review`
- Rust CLI (`groundwork init`, `update`, `list`, `doctor`) with curated manifest and `sk` integration
- Automatic `gh-issue-sync` installation during `groundwork init`
- Schema distribution in CLI: `init/update` now provision `.groundwork/schemas/`, create `.groundwork/artifacts/`, and `doctor` reports schema completeness/drift
- Pipeline contract with formal handoff rules and anti-divergence checks (`docs/architecture/pipeline-contract.md`)
- Integration manual (`WORKFLOW.md`) covering pipeline stages, BDD thread, documentation thread, issue-based development, and skill routing
- Local issue mirroring via `gh-issue-sync`

### Changed

- Migrated license from Apache-2.0 to MIT
- Rewrote README from first principles around the pipeline concept
- Renamed `planning` skill to `next-issue`; added separate `plan` skill for design convergence
- Reframed sovereignty as a fractal principle (applies at every interface, not just human-agent)
- Removed the prescriptive step-script decomposition skill from the curated set and live pipeline docs; rationale recorded in [`docs/research/epic-7-methodology-research.md`](docs/research/epic-7-methodology-research.md). Groundwork intentionally overrides `brainstorming`'s upstream `writing-plans` handoff through `WORKFLOW.md` and `using-groundwork`.
- Removed the curated skill-authoring meta-skill from runtime docs/config and adopted external `skill-creator` as the contributor-facing skill-authoring system, so first-session agents only load pipeline-relevant skills.

### Fixed

- Removed namespace stutter from skill aliases (e.g., `ground` instead of prefixed variants)
- Corrected dependency ordering across pipeline documentation
- Unified skills table by pipeline stage instead of by source
