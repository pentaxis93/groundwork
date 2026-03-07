# ADR-0001: Pipeline Integration Strategy

**Status:** Accepted
**Date:** 2026-03-05

## Context

Groundwork integrates skills from multiple sources into one pipeline. Currently: 9 core skills maintained in this repository and 8 curated skills fetched from [obra/superpowers](https://github.com/obra/superpowers) at install time.

The pipeline enforces handoff contracts between stages — each skill produces artifacts that the next skill consumes. These contracts are documented in `WORKFLOW.md` and formalized in `docs/architecture/pipeline-contract.md`.

The question this ADR addresses: **how should skills from different sources integrate into one pipeline, and when should a curated skill become a core skill?**

## Decision Drivers

- Skills must reliably hand off to each other at pipeline boundaries
- Upstream skills are maintained by other authors with their own design goals
- Curated skills reduce maintenance burden when upstream quality is high
- Some skills embed context assumptions (specific forges, specific agents) that documentation overlays cannot override
- The pipeline contract requires behavior at the skill level (what a skill produces, what it consumes) — not just at the documentation level

## Decision

**Skills form one pipeline with formal handoff contracts, not a menu of independent utilities.**

Integration strategy depends on where a skill sits relative to pipeline boundaries:

**Curate** when the skill is context-independent — its upstream author's assumptions are general and it does not need to know about adjacent pipeline stages. Examples: TDD, debugging, verification, code review. These skills operate within a stage without needing to produce specific handoff artifacts.

**Own (make first-party)** when the skill is at a pipeline boundary and must produce or consume specific handoff artifacts, or when the upstream skill embeds context assumptions that conflict with Groundwork's general-purpose scope. The signal is: documentation alone cannot make the skill behave correctly in the pipeline.

**Current evidence for this distinction:**
- `land` (core, boundary skill) uses `gh` CLI for forge operations. This is a boundary skill that must remain forge-agnostic.
- `research` (core, boundary skill) declares `compatibility: opencode` and references an opencode-specific agent file. Same pattern — context assumptions from a prior environment.
- `test-driven-development` (curated, mid-pipeline) works with any codebase and any agent. No pipeline-specific adaptation needed. Curation is correct here.

## Consequences

### Good

- Contributors have a clear framework for deciding how to integrate a new skill
- The distinction between "curate" and "own" is grounded in pipeline mechanics, not preference
- Legacy skills with embedded context assumptions are recognized as technical debt, not design choices

### Neutral

- The pipeline-contract.md already defines handoff requirements. This ADR explains *why* some skills must be first-party to meet those requirements.

### Bad

- Owning a skill means maintaining it. Forking an upstream skill creates a diverging copy.
- The boundary between "context-independent" and "context-dependent" is a judgment call, not a bright line.

### Risks

- Legacy skill debt (`research`) is acknowledged but not resolved by this ADR. `land` was generalized in issue #23; `research` still needs a separate issue to generalize its context assumptions.
- Future upstream skills may change in ways that break curation assumptions. Pinning to commits mitigates this but does not eliminate it.
