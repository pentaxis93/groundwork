# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Changed

- Broadened `next-issue` from work selection to work initiation — now covers selection, workspace preparation (branch + draft PR), and session open/close as the opening bookend to `land`
- `next-issue` accepts issue number(s) directly (skipping selection) or a topic string (narrowing selection), in addition to no-args full selection
- Multi-issue batching into a single PR is an explicitly supported pattern
- Session open captures starting direction instead of requiring a rigid binary done condition upfront
- Added `next-issue -> execution` and `next-issue <-> land` handoff contracts to pipeline contract

### Added

- Five-stage methodology pipeline: frame constraints, define behavior, decompose, execute and verify, land
- Nine core skills: `ground`, `research`, `bdd`, `issue-craft`, `next-issue`, `plan`, `documentation`, `land`, and `using-groundwork` (methodology orientation)
- Nine curated skills from [obra/superpowers](https://github.com/obra/superpowers): `brainstorming`, `writing-skills`, `writing-plans`, `subagent-driven-development`, `test-driven-development`, `systematic-debugging`, `verification-before-completion`, `requesting-code-review`, `receiving-code-review`
- Rust CLI (`groundwork init`, `update`, `list`, `doctor`) with curated manifest and `sk` integration
- Automatic `gh-issue-sync` installation during `groundwork init`
- Pipeline contract with formal handoff rules and anti-divergence checks (`docs/architecture/pipeline-contract.md`)
- Integration manual (`WORKFLOW.md`) covering pipeline stages, BDD thread, documentation thread, issue-based development, and skill routing
- Local issue mirroring via `gh-issue-sync`

### Changed

- Migrated license from Apache-2.0 to MIT
- Rewrote README from first principles around the pipeline concept
- Renamed `planning` skill to `next-issue`; added separate `plan` skill for design convergence
- Reframed sovereignty as a fractal principle (applies at every interface, not just human-agent)

### Fixed

- Removed namespace stutter from skill aliases (e.g., `ground` instead of prefixed variants)
- Corrected dependency ordering across pipeline documentation
- Unified skills table by pipeline stage instead of by source
