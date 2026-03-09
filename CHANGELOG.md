# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Five-stage methodology pipeline: frame constraints, define behavior, decompose, execute and verify, land
- Ten core skills: `ground`, `research`, `bdd`, `issue-craft`, `begin`, `plan`, `test-first`, `documentation`, `land`, and `using-groundwork` (methodology orientation)
- Two curated skills from [obra/superpowers](https://github.com/obra/superpowers): `subagent-driven-development`, `systematic-debugging`
- Rust CLI (`groundwork init`, `update`, `list`, `doctor`) with curated manifest and `sk` integration
- Automatic `gh-issue-sync` installation during `groundwork init`
- Schema distribution in CLI: `init/update` now provision `.groundwork/schemas/`, create `.groundwork/artifacts/`, and `doctor` reports schema completeness/drift
- Pipeline contract with formal handoff rules and anti-divergence checks (`docs/architecture/pipeline-contract.md`)
- Integration manual (`WORKFLOW.md`) covering pipeline stages, BDD thread, documentation thread, issue-based development, and skill routing
- Local issue mirroring via `gh-issue-sync`

- `propose` skill (v1.0): commit strategy, push, and PR creation — the middle phase of the session lifecycle between `begin` and `land`. Pipeline contract, routing table, and cross-references updated.

### Changed

- Replaced curated `verification-before-completion` (obra/superpowers) with groundwork-native original (v1.0.0). Preserves core discipline (Iron Law, gate function, common-failures table, anti-rationalization patterns, red flags). Adds Lifecycle Role section establishing pipeline position, Corruption Modes section, and cross-references. Removes superpowers-specific vocabulary.
- Documented upstream attribution convention in CONTRIBUTING.md: LICENSE-UPSTREAM requirements for adapted skills, origin metadata standard, curated-skills attribution reasoning, and attribution checklist at the skill-authoring boundary
- Replaced curated `test-driven-development` (obra/superpowers) with groundwork-native `test-first` skill (v1.0.0). Preserves core discipline (Iron Law, red-green-refactor, delete-and-start-over, anti-rationalization patterns) while adding bidirectional composition with `bdd`, `verification-before-completion`, `systematic-debugging`, and `documentation`. Lifecycle Role section establishes pipeline position. Corruption Modes section added. Language-agnostic examples replace TypeScript-only. Companion `testing-anti-patterns.md` migrated to `references/`.
- `ground` skill upgraded to v3.0.0: broadened from design-only to full first-principles cognitive discipline covering strategic analysis, cost decomposition, and problem reframing. Added Active Excavation section (Socratic Drilling, Recursive Why). Decompose step now supports three modes (requirements, constituent, process) with Orient determining which applies. Orient step broadened with mode-specific questions. New "Structure survey as analysis" corruption mode. All existing patterns and corruption modes retained.
- Renamed `next-issue` skill to `begin`; session lifecycle is now `begin` → `propose` → `land`
- `ground` skill upgraded to v2.2.0: consolidated patterns from 17 to 12 by merging overlaps (#2+#10 → "Description as Design", #3+#4 → "Borrowed Structure", #5+BC#3+BC#5 → "Precedent as Constraint"). Merged Decision Protocol into The Move as steps 4–5 (Compare, Default). Folded backward-compat patterns into main section as "Preservation Variants" subsection. Deleted Exponential Context section. Trimmed "When to Ground" to trigger question only. 193 → 153 lines, no recognition power lost. Patch bump: structural consolidation, no behavioral change.
- `ground` skill upgraded to v2.1.0: added Local Coherence as assumed-constraint pattern #9 — captures details that follow valid engineering patterns but contradict the purpose of the work they belong to. Renumbered Descriptive-Normative Confusion to #10.
- `using-groundwork` skill upgraded to v2.1.0: added Entry Point section, folded routing table triggers into flow stages (single source of truth), compressed BDD/Documentation threading from enumerated arrows to principle-based guidance, consolidated corruption modes from 11 items to 5 named categories, added relationship cues to stage 4 execution skills. 108 → 70 lines, same skill coverage. Patch bump: structural refinement, no behavioral change.
- `using-groundwork` skill upgraded to v2.0.0: restructured as five numbered pipeline stages, added sovereignty guardrail with anti-prescription principle (WHAT/HOW boundary), expanded cross-cutting disciplines (documentation threading, research at any stage), converted routing to table format, added corruption modes for prescription and documentation gaps. Major version bump reflects structural rewrite of the methodology orientation document.
- `land` skill upgraded to v1.1: added CHANGELOG verification step before merge, updated overview to reflect 7-step procedure. Minor version bump: additive behavioral change (new precondition check).
- `pipeline-contract.md` updated to v0.2: fixed heading levels (## → ###), clarified `documentation-review` reference to "`documentation` skill's review mode"
- Added `CLAUDE.md` documenting skill management with `sk` fork
- Migrated license from Apache-2.0 to MIT
- `land` skill upgraded to v1.3: added pre-merge acceptance criteria evaluation — satisfied issues are closed, partial issues receive a progress comment listing delivered and remaining criteria, issues without extractable criteria stay open for human review. Prevents premature closure when a branch delivers part of an issue's scope.
- `land` skill upgraded to v1.4: added conditional squash step — evaluates commit history pre-merge and squashes when iterative refinement (feature + fix-ups on same files) would read as noise on `main`. Preserves history for distinct work units. Uses `git merge --squash` with a consolidated commit message when squashing, `--no-ff` otherwise. Branch deletion upgraded to `-D` to handle squash merges. Defaults to preserve when uncertain. Minor version bump: additive behavioral change (new decision step).
- `land` skill upgraded to v1.5: rewrote from bash-prescriptive to prose-reasoning style — instructions communicate intent rather than copy-paste scripts. Documentation drift scan demoted from merge blocker to best-effort warning. PR discovery moved before branch deletion. Branch deletion failure no longer blocks issue closure. `gh-issue-sync` degrades gracefully when unavailable.
- `land` skill upgraded to v1.6: replaced local `git merge` + `git push` with `gh pr merge` API call so PRs are recorded as "merged" on GitHub, not silently closed when the branch is deleted. PR discovery moved before merge (required by API). Local fallback retained for branches without PRs. Fixes PRs #60, #62, #63 showing as closed-not-merged.
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
