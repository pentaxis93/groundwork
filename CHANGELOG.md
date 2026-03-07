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

- `using-groundwork` skill upgraded to v2.0.0: restructured as five numbered pipeline stages, added sovereignty guardrail with anti-prescription principle (WHAT/HOW boundary), expanded cross-cutting disciplines (documentation threading, research at any stage), converted routing to table format, added corruption modes for prescription and documentation gaps. Major version bump reflects structural rewrite of the methodology orientation document.
- `land` skill upgraded to v1.1: added CHANGELOG verification step before merge, updated overview to reflect 7-step procedure. Minor version bump: additive behavioral change (new precondition check).
- `pipeline-contract.md` updated to v0.2: fixed heading levels (## → ###), updated step 3 to include brainstorming, clarified `documentation-review` reference to "`documentation` skill's review mode"
- Added `CLAUDE.md` documenting skill management with `sk` fork
- Migrated license from Apache-2.0 to MIT
- `land` skill upgraded to v1.3: added pre-merge acceptance criteria evaluation — satisfied issues are closed, partial issues receive a progress comment listing delivered and remaining criteria, issues without extractable criteria stay open for human review. Prevents premature closure when a branch delivers part of an issue's scope.
- `land` skill now supports closing multiple issues inferred from `issues-<N>-<M>-.../<slug>` branches, while preserving `issue-<N>/<slug>` behavior
- Rewrote README from first principles around the pipeline concept
- Renamed `planning` skill to `next-issue`; added separate `plan` skill for design convergence
- Reframed sovereignty as a fractal principle (applies at every interface, not just human-agent)
- Removed the prescriptive step-script decomposition skill from the curated set and live pipeline docs; rationale recorded in [`docs/research/epic-7-methodology-research.md`](docs/research/epic-7-methodology-research.md). Live methodology docs no longer route work through the deprecated intermediary planning handoff.
- Removed the curated skill-authoring meta-skill from runtime docs/config and adopted external `skill-creator` as the contributor-facing skill-authoring system, so first-session agents only load pipeline-relevant skills.
- Replaced the separate curated manifest plus hardcoded local-skill inventory with one shipped-skill manifest at `skills/skills.toml`, flattened tracked skill paths under `skills/`, and made `groundwork list` follow that manifest order.

### Fixed

- Removed namespace stutter from skill aliases (e.g., `ground` instead of prefixed variants)
- Corrected dependency ordering across pipeline documentation
- Unified skills table by pipeline stage instead of by source
- Hardened CLI tool bootstrap security: `gh-issue-sync` auto-install now uses pinned release assets with SHA-256 verification, and install lock writing now fails if tool version capture is missing/empty instead of silently recording unknown provenance
