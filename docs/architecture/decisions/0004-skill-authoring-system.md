# ADR-0004: Skill Authoring System

**Status:** Accepted
**Date:** 2026-03-07

## Context

Groundwork is a runtime methodology library. It ships skills that agents load
while doing project work, plus the CLI and documentation that define the
pipeline. It does not need to ship the tooling used to author or evaluate those
skills.

Issue #42 removed `writing-skills` from Groundwork's runtime docs and config.
That resolved the inventory question but left the authoring question partially
implicit: several docs and issues still treated `writing-skills` as the live
reference for creating or regenerating skills.

The local sibling repository `/home/pentaxis93/src/skill-creator` now provides
the missing authoring boundary. It is platform-agnostic at the output layer,
supports iterative skill improvement, and includes an evaluation loop. This ADR
records how Groundwork uses it.

## Decision Drivers

- Groundwork runtime docs should stay focused on the live methodology pipeline
- Skill authoring is a contributor workflow, not a runtime stage
- The adopted authoring system must produce platform-agnostic skill outputs
- Future sessions need one clear default for skill creation and regeneration
- Groundwork should preserve historical research findings without presenting
  removed skills as current guidance

## Decision

**Groundwork adopts the external `skill-creator` repository as its
skill-authoring system.**

Concretely:

- Use `/home/pentaxis93/src/skill-creator` for creating, regenerating, and
  evaluating skills
- Keep authoring guidance in contributor-facing documentation, not in runtime
  pipeline docs
- Do not add `skill-creator` to Groundwork's runtime inventory in
  `agents.toml` or `skills/skills.toml`
- Treat `writing-skills` as historical research context only, not as an active
  Groundwork skill or default authoring reference

## Consequences

### Good

- Contributors and agents have one explicit place to go for skill authoring
- Runtime docs remain about the pipeline Groundwork actually ships
- Historical research can be preserved without confusing present-day guidance

### Neutral

- Groundwork now depends on an adjacent repository for authoring workflow, but
  not for runtime behavior

### Bad

- Contributors need access to a second repository when doing skill-authoring
  work
- Authoring workflow changes in `skill-creator` are not versioned inside this
  repository

### Risks

- If the local `skill-creator` path changes, contributor docs will drift until
  updated
- Future research docs could accidentally reintroduce `writing-skills` as live
  guidance unless documentation review keeps the historical/current distinction
  explicit
